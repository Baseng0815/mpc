// A 1-n oblivious transfer allows a party P1 to receive one out of n messages known to another
// party P2 without P1 revealing their choice to P2 and P2 revealing any other messages to P1

use crate::pke::{TextbookRSA, OSPK, PKE};

pub trait ObliviousTransfer<M> {
    fn eval(x1: Vec<M>, x2: usize) -> ((), M);
}

pub struct PKEObliviousTransfer { }

impl ObliviousTransfer<u32> for PKEObliviousTransfer {
    fn eval(x1: Vec<u32>, x2: usize) -> ((), u32) {
        // P1
        let key = TextbookRSA::new(0).gen_key();
        let pubkeys = (0..x1.len()).map(|i| if i == x2 { key.1 } else { TextbookRSA::new(0).sample_pubkey() });;
        // send pubkeys to P2

        // P2
        let ciphers = x1.iter().zip(pubkeys).map(|(x, k)| {
            TextbookRSA::encrypt(k, *x)
        }).collect::<Vec<_>>();
        // send ciphers to P1

        // P1
        let x = TextbookRSA::decrypt(key.0, ciphers[x2]);

        ((), x)
    }
}

mod test {
    use crate::ot::{ObliviousTransfer, PKEObliviousTransfer};

    #[test]
    fn pke_oblivious_transfer_correctness() {
        let x1 = vec![0, 7, 19, 69, 8, 12];
        let x2 = 3;
        assert_eq!(PKEObliviousTransfer::eval(x1, x2), ((), 69));
    }
}
