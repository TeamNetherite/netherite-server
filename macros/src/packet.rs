use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{DeriveInput, Expr, ExprPath, LitInt, parenthesized, Token};
use syn::parse::{Parse, Parser, ParseStream};
use syn::spanned::Spanned;
use crate::abs::{netherite_protocol, Parenthesized, ProcMacro};

mod kw {
    use proc_macro2::TokenStream;
    use quote::ToTokens;
    use syn::custom_keyword;
    use syn::parse::{Parse, ParseStream};

    custom_keyword!(serverbound);
    custom_keyword!(clientbound);

    pub(super) enum Bound {
        Serverbound(serverbound),
        Clientbound(clientbound)
    }

    impl Parse for Bound {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let ahead = input.lookahead1();

            if ahead.peek(serverbound) {
                Ok(Bound::Serverbound(input.parse()?))
            } else if ahead.peek(clientbound) {
                Ok(Bound::Clientbound(input.parse()?))
            } else {
                Err(ahead.error())
            }
        }
    }

    impl ToTokens for Bound {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                Bound::Serverbound(s) => s.to_tokens(tokens),
                Bound::Clientbound(c) => c.to_tokens(tokens)
            }
        }
    }
}

struct PacketAttr(kw::Bound, LitInt, Token![in], ExprPath);
impl Parse for PacketAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let next;
        parenthesized!(next in input);
        let bound = kw::Bound::parse(&next)?;

        let ahead = next.lookahead1();

        if ahead.peek(LitInt) {
            let id: LitInt = next.parse()?;

            let ahead = next.lookahead1();
            if ahead.peek(Token![in]) {
                let in_t: Token![in] = next.parse()?;

                let expr: ExprPath = next.parse()?;
                Ok(PacketAttr(bound, id, in_t, expr))
            } else {
                Err(ahead.error())
            }
        } else {
            Err(ahead.error())
        }
    }
}

pub struct PacketDerive;

impl ProcMacro for PacketDerive {
    fn parse(self, input: TokenStream) -> syn::Result<TokenStream> {
        let input = Parser::parse2(DeriveInput::parse, input)?;
        let indent = input.ident;
        let n = netherite_protocol();

        if let Some(a) = input.attrs.into_iter().find(|a| a.path.to_token_stream().to_string() == "packet") {
            let PacketAttr(bound, id, _, state) = Parser::parse2(PacketAttr::parse, a.tokens)?;

            return Ok(quote! {
                impl #n::packet::Packet for #indent {
                    const ID: i32 = #id;
                    const TYPE: #n::packet::PacketType = #n::packet::PacketType::#bound;
                    const STATE: #n::packet::state::State = #state;
                }
            })
        }

        Err(syn::Error::new(Span::call_site(), "expected packet attribute"))
    }
}