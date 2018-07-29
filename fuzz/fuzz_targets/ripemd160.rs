
extern crate bitcoin_hashes;
extern crate crypto;

use bitcoin_hashes::Hash;
use bitcoin_hashes::ripemd160::Ripemd160Hash;
use crypto::digest::Digest;
use crypto::ripemd160::Ripemd160;

fn do_test(data: &[u8]) {
    let our_hash = Ripemd160Hash::hash(data);

    let mut rc_hash = [0u8; 20];
    let mut rc_engine = Ripemd160::new();
    rc_engine.input(data);
    rc_engine.result(&mut rc_hash);

    assert_eq!(&our_hash[..], &rc_hash[..]);
}

#[cfg(feature="honggfuzz")]
#[macro_use]
extern crate honggfuzz;

#[cfg(feature="honggfuzz")]
fn main() {
    loop {
        fuzz!(|d| { do_test(d) });
    }
}

#[cfg(test)]
mod tests {
    fn extend_vec_from_hex(hex: &str, out: &mut Vec<u8>) {
        let mut b = 0;
        for (idx, c) in hex.as_bytes().iter().enumerate() {
            b <<= 4;
            match *c {
                b'A'...b'F' => b |= c - b'A' + 10,
                b'a'...b'f' => b |= c - b'a' + 10,
                b'0'...b'9' => b |= c - b'0',
                _ => panic!("Bad hex"),
            }
            if (idx & 1) == 1 {
                out.push(b);
                b = 0;
            }
        }
    }

    #[test]
    fn duplicate_crash() {
        let mut a = Vec::new();
        extend_vec_from_hex("00000", &mut a);
        super::do_test(&a);
    }
}

