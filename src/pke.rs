// A PKE scheme allows encrypting a message using a public key and decrypting a cipher using the
// corresponding private key

use crate::util::modpow;

pub trait PKE<M,C,SK,PK> {
    fn gen_key(&self) -> (SK,PK);
    fn encrypt(pk: PK, m: M) -> C;
    fn decrypt(sk: SK, c: C) -> M;
}

// obliviously samplable pubkeys
pub trait OSPK<PK> {
    fn sample_pubkey(&self) -> PK;
}

#[derive(Clone, Copy, Debug)]
pub struct TextbookRSA {
    prime_bits: u32,
}

impl TextbookRSA {
    pub fn new(prime_bits: u32) -> TextbookRSA {
        TextbookRSA {
            prime_bits,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TextbookRSASecretKey {
    pub N: u32,
    pub d: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct TextbookRSAPublicKey {
    pub N: u32,
    pub e: u32,
}

impl PKE<u32, u32, TextbookRSASecretKey, TextbookRSAPublicKey> for TextbookRSA {
    fn gen_key(&self) -> (TextbookRSASecretKey, TextbookRSAPublicKey) {
        // TODO: generate primes use a probabilistic primality test
        // 16-bit prime numbers
        let p = 63691;
        let q = 65171;
        let N = p * q; // order of modulus group
        let phi = (p - 1) * (q - 1); // order of unit group
        let e = 13;
        let d = 1277131477; // hardcoded; TODO: calculate

        (TextbookRSASecretKey {
            N, d
        }, TextbookRSAPublicKey {
            N, e
        })
    }

    fn encrypt(pk: TextbookRSAPublicKey, m: u32) -> u32 {
        modpow(m, pk.e, pk.N)
    }

    fn decrypt(sk: TextbookRSASecretKey, c: u32) -> u32 {
        modpow(c, sk.d, sk.N)
    }
}

impl OSPK<TextbookRSAPublicKey> for TextbookRSA {
    fn sample_pubkey(&self) -> TextbookRSAPublicKey {
        TextbookRSAPublicKey {
            N: rand::random::<u32>() % u32::MAX, // this is not correct. too bad!
            e: rand::random::<u32>() % u32::MAX,
        }
    }
}

mod test {
    use rand::Rng;

    use super::TextbookRSA;
    use super::PKE;

    #[test]
    fn textbook_rsa_correctness() {
        let (sk, pk) = TextbookRSA::new(0).gen_key();
        // let m: u32 = rand::random::<u32>() % pk.N;
        let m: u32 = 11;
        let c = TextbookRSA::encrypt(pk, m);
        let m_prime = TextbookRSA::decrypt(sk, c);
        assert_eq!(m_prime, m);
    }
}
