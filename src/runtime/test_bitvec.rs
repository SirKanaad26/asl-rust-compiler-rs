// test_bitvec.rs — standalone crate root for testing bitvec.rs in isolation
// Compile with:
//   rustc +nightly --edition 2021 --crate-type lib src/runtime/test_bitvec.rs
//
// In production, this role is played by the generated .rs file which has:
//   #![feature(generic_const_exprs)]
//   mod bitvec;

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

mod bitvec;
