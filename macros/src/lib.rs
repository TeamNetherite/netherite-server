pub(crate) trait ProcMacro: Sized {
    fn invoke(self, input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream>;

    fn or_compile_err(self, input: TokenStream) -> TokenStream {
        self.invoke(input.into()).unwrap_or_else(|e| e.to_compile_error()).into()
    }
}

mod packet;

extern crate proc_macro;
use proc_macro::TokenStream;
use crate::packet::DerivePacket;

#[proc_macro_derive(Packet)]
pub fn derive_packet(input: TokenStream) -> TokenStream {
    DerivePacket.or_compile_err(input)
}