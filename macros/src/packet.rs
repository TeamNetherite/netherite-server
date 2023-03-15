use proc_macro2::{TokenStream, Span};
use syn::{DeriveInput, parse::{Parser, Parse}};
use crate::ProcMacro;

pub struct DerivePacket;

impl ProcMacro for DerivePacket {
    fn invoke(self, input: TokenStream) -> syn::Result<TokenStream> {
        let input = Parser::parse2(DeriveInput::parse, input)?;



        Ok(TokenStream::new())
    }
}