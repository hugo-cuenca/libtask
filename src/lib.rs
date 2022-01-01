//! The advanced, extensible dependency management library. **CURRENTLY IN DEVELOPMENT**
//!
//! # What?
//! `libtask` is a flexible dependency manager. This crate currently serves the purpose of reserving
//! the name `libtask` in crates.io, and contains no other code than the standard `assert_eq!()`
//! test.
#![crate_name = "libtask"]
#![cfg_attr(test, deny(warnings))]
#![deny(unused)]
#![deny(unstable_features)]
#![warn(missing_docs)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
