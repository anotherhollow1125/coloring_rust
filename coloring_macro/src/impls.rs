use proc_macro2::{Delimiter, Group, Span, TokenStream, TokenTree};
use quote::quote;
use quote::ToTokens;
use syn::parse::ParseStream;
use syn::Token;
use syn::{braced, bracketed};
use syn::{parse::Parse, Ident, Result};

pub fn repeat_for_types(
    RepeatInput {
        variable,
        types,
        block,
    }: RepeatInput,
) -> TokenStream {
    let tokens = RecTokensWithPlaceHolders::new(variable, block);

    tokens.render(types)
}

pub struct RepeatInput {
    variable: Ident,
    types: Vec<syn::Type>,
    block: TokenStream,
}

impl Parse for RepeatInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // for
        let _for: Token![for] = input.parse()?;

        // T
        let variable = input.parse()?;
        // in
        let _in: Token![in] = input.parse()?;

        // types in [..]
        let content;
        let _ = bracketed!(content in input);
        let types = content
            .parse_terminated(syn::Type::parse, Token![,])?
            .into_iter()
            .collect();

        // block in {..}
        let content;
        let _ = braced!(content in input);
        let block = TokenStream::parse(&content)?;

        Ok(Self {
            variable,
            types,
            block,
        })
    }
}

#[derive(Debug)]
enum RecToken {
    Group(Delimiter, Vec<RecToken>, Span),
    PlaceHolder(Ident),
    Other(TokenTree),
}

impl ToTokens for RecToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            RecToken::Group(delimiter, vec, span) => {
                let mut stream = TokenStream::new();
                vec.iter().for_each(|st| st.to_tokens(&mut stream));
                let mut group = Group::new(*delimiter, stream);
                group.set_span(*span);
                group.to_tokens(tokens);
            }
            RecToken::PlaceHolder(ident) => ident.to_tokens(tokens),
            RecToken::Other(token_tree) => token_tree.to_tokens(tokens),
        }
    }
}

impl RecToken {
    fn render(&self, type_: &syn::Type) -> TokenStream {
        match self {
            RecToken::Group(delimiter, tokens, span) => {
                let tokens = tokens.iter().map(|t| t.render(type_)).collect::<Vec<_>>();

                let mut group = Group::new(*delimiter, quote! { #(#tokens)* });
                group.set_span(*span);

                group.into_token_stream()
            }
            RecToken::PlaceHolder(_ident) => type_.clone().into_token_stream(),
            RecToken::Other(token_tree) => token_tree.into_token_stream(),
        }
    }
}

#[derive(Debug)]
struct RecTokensWithPlaceHolders(Vec<RecToken>);

impl RecTokensWithPlaceHolders {
    fn new(place_holder_ident: Ident, stream: TokenStream) -> Self {
        let mut tokens = Vec::new();

        parse_stream_rec(place_holder_ident, stream, &mut tokens);

        Self(tokens)
    }

    fn render(&self, types: Vec<syn::Type>) -> TokenStream {
        let tokens = types
            .into_iter()
            .flat_map(|type_| self.0.iter().map(move |t| t.render(&type_)))
            .collect::<Vec<_>>();

        quote! { #(#tokens)* }
    }
}

fn parse_stream_rec(place_holder_ident: Ident, stream: TokenStream, tokens: &mut Vec<RecToken>) {
    for token in stream {
        match token {
            TokenTree::Group(group) => {
                let delim = group.delimiter();
                let mut inner = Vec::new();
                parse_stream_rec(place_holder_ident.clone(), group.stream(), &mut inner);
                tokens.push(RecToken::Group(delim, inner, group.span()));
            }
            TokenTree::Ident(ident) if ident == place_holder_ident => {
                tokens.push(RecToken::PlaceHolder(ident));
            }
            t => tokens.push(RecToken::Other(t)),
        }
    }
}
