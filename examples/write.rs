extern crate bitcoin_hashes;

use bitcoin_hashes::literacy::Write;
use bitcoin_hashes::{sha256d, Hash, hash_newtype, hex_fmt_impl, index_impl, serde_impl, borrow_slice_impl};

hash_newtype!(Txid, sha256d::Hash, 32, doc="A bitcoin transaction hash/transaction ID.");

fn do_it<W: Write>(mut w: W) {
    let _ = w.write_all(&[0u8; 1]);
}

fn main() {
    let mut enc = Txid::engine();
    do_it(&mut enc);
}