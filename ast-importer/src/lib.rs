#![feature(rustc_private)]
extern crate cbor;
extern crate syntax;
extern crate idiomize;

pub mod renamer;
pub mod name_manager;
pub mod clang_ast;
pub mod convert_type;
pub mod translator;
pub mod typed_convert_type;
pub mod typed_translator;
pub mod c_ast;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
