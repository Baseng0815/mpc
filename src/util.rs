pub fn modpow(x: u32, e: u32, N: u32) -> u32 {
    /* b ** 13 = b ** (0b1101)
     * r = 1
     * r = r*r = 1   ; r = r*b = b    (b[3] = 1) (MSb)
     * r = r*r = b^2 ; r = r*b = b^3  (b[2] = 1)
     * r = r*r = b^6 ; r = r   = b^6  (b[1] = 0)
     * r = r*r = b^12; r = r*b = b^13 (b[0] = 1) (LSb) */

    let mut r = 1; // because multiplication might overflow (should use bigint but w/e)

    for i in (0..u32::BITS).rev() {
        let bi = (e >> i) & 0x1;
        r = r*r % N as u64;
        eprintln!("(r, N) = {:#?}", (r, N));
        if bi == 1 {
            r = r*x as u64 % N as u64;
        }
    }

    r as u32
}

mod test {
    use crate::util::modpow;

    #[test]
    fn modpow_correctness() {
        let x = 27;
        let e = 13;
        let N = 16;
        assert_eq!(modpow(x, e, N), 11);
    }
}
