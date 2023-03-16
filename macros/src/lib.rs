#![feature(auto_traits)]
#![feature(negative_impls)]
mod abs;
mod enum_fields;
//mod packet;

extern crate proc_macro;
use proc_macro::TokenStream;
use crate::abs::ProcMacro;
use crate::enum_fields::EnumDerive;

#[proc_macro_derive(EnumFields, attributes(enum_field, ef))]
pub fn enum_fields_derive(input: TokenStream) -> TokenStream {
    EnumDerive.or_compile_err(input.into()).into()
}