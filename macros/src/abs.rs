use proc_macro2::TokenStream;
use std::iter::Map;
use syn::parenthesized;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

pub(crate) trait ProcMacro: Sized {
    fn parse(self, input: TokenStream) -> syn::Result<TokenStream>;

    fn or_compile_err(self, input: TokenStream) -> TokenStream {
        self.parse(input).unwrap_or_else(|e| e.to_compile_error())
    }
}

pub(crate) trait ProcMacroAttr: Sized {
    fn parse(self, args: TokenStream, input: TokenStream) -> syn::Result<TokenStream>;

    fn or_compile_err(self, args: TokenStream, input: TokenStream) -> TokenStream {
        self.parse(args, input)
            .unwrap_or_else(|e| e.to_compile_error())
    }
}

pub(crate) trait FlattenResult<T, E>: Iterator<Item = Result<T, E>> + Sized {
    type FlattenTo;

    fn flatten_result(self) -> Result<Self::FlattenTo, E>;
}

pub(crate) trait FlattenMapResult<T, E, R>: Iterator<Item = T> + Sized {
    fn flatten_map<F: FnMut(T) -> Result<R, E>>(
        self,
        map: F,
    ) -> Result<<<Map<Self, F> as FlattenResult<R, E>>::FlattenTo as IntoIterator>::IntoIter, E>;
}

impl<T, E, I: Iterator<Item = Result<T, E>>> FlattenResult<T, E> for I {
    type FlattenTo = Vec<T>;

    fn flatten_result(self) -> Result<Self::FlattenTo, E> {
        let mut result = vec![];

        for i in self {
            if let Err(e) = i {
                return Err(e);
            }

            if let Ok(ok) = i {
                result.push(ok);
            }
        }

        Ok(result)
    }
}

impl<T, E, I: Iterator<Item = T>, R> FlattenMapResult<T, E, R> for I {
    fn flatten_map<F: FnMut(T) -> Result<R, E>>(self, map: F) -> Result<<Vec<R> as IntoIterator>::IntoIter, E> {
        Ok(self.map(map).flatten_result()?.into_iter())
    }
}

#[repr(transparent)]
pub struct Parenthesized<T>(pub T);

pub auto trait Ok {}

impl<T, P> !Ok for Punctuated<T, P> {}

impl<T: Parse + Ok> Parse for Parenthesized<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        parenthesized!(content in input);

        Ok(Parenthesized(T::parse(&content)?))
    }
}

impl<T: Parse, P: Parse> Parse for Parenthesized<Punctuated<T, P>> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);

        Ok(Parenthesized(Punctuated::parse_terminated(&content)?))
    }
}