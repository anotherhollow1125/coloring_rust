use itertools::peek_nth;
use itertools::structs::PeekNth;
use proc_macro2::{Delimiter, Group, Span, TokenStream, TokenTree};
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use syn::parse::ParseStream;
use syn::parse_quote;
use syn::spanned::Spanned;
use syn::token::Pound;
use syn::token::Star;
use syn::Token;
use syn::TypePath;
use syn::{braced, bracketed};
use syn::{parse::Parse, Error, Ident, Result};

pub fn repeat_for_types(
    RepeatInput {
        variable,
        types,
        tt,
    }: RepeatInput,
) -> Result<TokenStream> {
    let tokens = RecTokensWithPlaceHolders::new(variable, tt);

    let res = tokens.render(&types)?;

    Ok(res)
}

pub struct RepeatInput {
    variable: Ident,
    types: Vec<syn::Type>,
    tt: TokenStream,
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

        // tt in {..}
        let content;
        let _ = braced!(content in input);
        let tt = TokenStream::parse(&content)?;

        Ok(Self {
            variable,
            types,
            tt,
        })
    }
}

#[derive(Debug)]
enum RecToken {
    Group(Delimiter, Vec<RecToken>, Span),
    PlaceHolder(PlaceHolder),
    Other(TokenTree),
    RepeatTarget(Token![#], Vec<RecToken>, Token![*], Span),
}

impl ToTokens for RecToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Group(delimiter, vec, span) => {
                let mut stream = TokenStream::new();
                vec.iter().for_each(|st| st.to_tokens(&mut stream));
                let mut group = Group::new(*delimiter, stream);
                group.set_span(*span);
                group.to_tokens(tokens);
            }
            Self::PlaceHolder(place_holder) => place_holder.to_tokens(tokens),
            Self::Other(token_tree) => token_tree.to_tokens(tokens),
            Self::RepeatTarget(pound, vec, star, span) => {
                pound.to_tokens(tokens);
                let mut stream = TokenStream::new();
                vec.iter().for_each(|t| t.to_tokens(&mut stream));
                let mut group = Group::new(Delimiter::Parenthesis, stream);
                group.set_span(*span);
                group.to_tokens(tokens);
                star.to_tokens(tokens);
            }
        }
    }
}

#[derive(Clone, Debug)]
enum PlaceHolder {
    AsType(Ident),
    AsIdent {
        // prefix~N~suffix
        prefix: Ident,
        tilde1: Token![~],
        target: Ident,
        suffix: Option<(Token![~], Ident)>,
    },
}

impl ToTokens for PlaceHolder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PlaceHolder::AsType(ident) => ident.to_tokens(tokens),
            PlaceHolder::AsIdent {
                prefix,
                tilde1,
                target,
                suffix,
            } => {
                prefix.to_tokens(tokens);
                tilde1.to_tokens(tokens);
                target.to_tokens(tokens);
                if let Some((t, i)) = suffix {
                    t.to_tokens(tokens);
                    i.to_tokens(tokens);
                }
            }
        }
    }
}

impl PlaceHolder {
    fn render(&self, type_: syn::Type) -> Result<TokenStream> {
        match self {
            Self::AsType(_ident) => {
                // type_.set_span(ident.span()); // 無理でした

                Ok(type_.into_token_stream())
            }
            as_ident @ Self::AsIdent { .. } => {
                let span = as_ident.span();

                let Self::AsIdent { prefix, suffix, .. } = as_ident else {
                    unreachable!()
                };

                let syn::Type::Path(TypePath {
                    path: type_path, ..
                }) = type_
                else {
                    return Err(Error::new(span, "invalid type expression."));
                };

                let Some(type_ident) = type_path.segments.last() else {
                    return Err(Error::new(span, "invalid type expression."));
                };

                let type_ident = type_ident.ident.clone();

                let mut ident = if let Some((_, suffix)) = suffix {
                    format_ident!("{}{}{}", prefix, type_ident, suffix)
                } else {
                    format_ident!("{}{}", prefix, type_ident)
                };

                ident.set_span(span);

                Ok(ident.into_token_stream())
            }
        }
    }
}

impl RecToken {
    fn render(&self, types: &[syn::Type]) -> Result<TokenStream> {
        match self {
            Self::Group(delimiter, tokens, span) => {
                let tokens = tokens
                    .iter()
                    .map(|t| t.render(types))
                    .collect::<Result<Vec<_>>>()?;

                let mut group = Group::new(*delimiter, quote! { #(#tokens)* });
                group.set_span(*span);

                Ok(group.into_token_stream())
            }
            Self::PlaceHolder(_) => Err(Error::new(
                self.span(),
                "invalid position of repeat variable place holder.",
            )),
            Self::RepeatTarget(_, tokens, _, _) => {
                let tokens = types
                    .iter()
                    .flat_map(|type_| {
                        tokens
                            .iter()
                            .map(move |token| token.render_in_repeat_target(type_))
                    })
                    .collect::<Result<Vec<_>>>()?;

                Ok(quote! { #(#tokens)* })
            }
            Self::Other(token_tree) => Ok(token_tree.into_token_stream()),
        }
    }

    fn render_in_repeat_target(&self, type_: &syn::Type) -> Result<TokenStream> {
        match self {
            Self::Group(delimiter, tokens, span) => {
                let tokens = tokens
                    .iter()
                    .map(|t| t.render_in_repeat_target(type_))
                    .collect::<Result<Vec<_>>>()?;

                let mut group = Group::new(*delimiter, quote! { #(#tokens)* });
                group.set_span(*span);

                Ok(group.into_token_stream())
            }
            Self::PlaceHolder(place_holder) => place_holder.render(type_.clone()),
            rt @ Self::RepeatTarget(..) => rt.render_in_deep_nest(type_),
            Self::Other(token_tree) => Ok(token_tree.into_token_stream()),
        }
    }

    // 2段以上のRepeatTarget( `#(...)*` )については現在対象のプレースホルダを置き換える以上のことはしない。例えば別のマクロのためのものと考える
    fn render_in_deep_nest(&self, type_: &syn::Type) -> Result<TokenStream> {
        match self {
            Self::Group(delimiter, tokens, span) => {
                let tokens = tokens
                    .iter()
                    .map(|t| t.render_in_deep_nest(type_))
                    .collect::<Result<Vec<_>>>()?;

                let mut group = Group::new(*delimiter, quote! { #(#tokens)* });
                group.set_span(*span);

                Ok(group.into_token_stream())
            }
            Self::PlaceHolder(place_holder) => place_holder.render(type_.clone()),
            Self::RepeatTarget(pound, tokens, star, span) => {
                let tokens = tokens
                    .iter()
                    .map(|t| t.render_in_deep_nest(type_))
                    .collect::<Result<Vec<_>>>()?;

                let mut group = Group::new(Delimiter::Parenthesis, quote! { #(#tokens)* });
                group.set_span(*span);

                Ok(quote! { #pound #group #star })
            }
            Self::Other(token_tree) => Ok(token_tree.into_token_stream()),
        }
    }
}

#[derive(Debug)]
struct RecTokensWithPlaceHolders(RecToken);

impl RecTokensWithPlaceHolders {
    fn new(place_holder_ident: Ident, stream: TokenStream) -> Self {
        let mut tokens = Vec::new();
        let mut has_repeat_block = false;

        parse_stream_rec(
            place_holder_ident,
            stream,
            &mut tokens,
            &mut has_repeat_block,
        );

        let tokens = if has_repeat_block {
            RecToken::Group(Delimiter::None, tokens, Span::call_site())
        } else {
            RecToken::RepeatTarget(Pound::default(), tokens, Star::default(), Span::call_site())
        };

        Self(tokens)
    }

    fn render(&self, types: &[syn::Type]) -> Result<TokenStream> {
        self.0.render(types)
    }
}

fn parse_stream_rec(
    place_holder_ident: Ident,
    stream: TokenStream,
    tokens: &mut Vec<RecToken>,
    has_repeat_block: &mut bool,
) {
    let mut trees = peek_nth(stream);
    while let Some(tree) = trees.next() {
        match tree {
            TokenTree::Group(group) => {
                let delim = group.delimiter();
                let mut inner = Vec::new();
                parse_stream_rec(
                    place_holder_ident.clone(),
                    group.stream(),
                    &mut inner,
                    has_repeat_block,
                );
                tokens.push(RecToken::Group(delim, inner, group.span()));
            }
            TokenTree::Ident(ident) => {
                let t = if ident == place_holder_ident {
                    RecToken::PlaceHolder(PlaceHolder::AsType(ident))
                } else if let Some(as_ident) =
                    parse_place_holder_ident(place_holder_ident.clone(), ident.clone(), &mut trees)
                {
                    RecToken::PlaceHolder(as_ident)
                } else {
                    RecToken::Other(TokenTree::Ident(ident))
                };
                tokens.push(t);
            }
            TokenTree::Punct(pound) if pound.as_char() == '#' => {
                let group = trees.peek().cloned();
                let star = trees.peek_nth(1).cloned();

                let (Some(TokenTree::Group(group)), Some(TokenTree::Punct(star))) = (group, star)
                else {
                    tokens.push(RecToken::Other(TokenTree::Punct(pound)));
                    continue;
                };

                if group.delimiter() != Delimiter::Parenthesis {
                    tokens.push(RecToken::Other(TokenTree::Punct(pound)));
                    continue;
                }

                if star.as_char() != '*' {
                    tokens.push(RecToken::Other(TokenTree::Punct(pound)));
                    continue;
                }

                *has_repeat_block = true;

                let (Some(TokenTree::Group(_group)), Some(TokenTree::Punct(_star))) =
                    (trees.next(), trees.next())
                else {
                    unreachable!()
                };

                let pound: Token![#] = parse_quote! { #pound };
                let mut inner = Vec::new();
                parse_stream_rec(
                    place_holder_ident.clone(),
                    group.stream(),
                    &mut inner,
                    has_repeat_block,
                );
                let star: Token![*] = parse_quote! { #star };

                tokens.push(RecToken::RepeatTarget(pound, inner, star, group.span()));
            }
            t => tokens.push(RecToken::Other(t)),
        }
    }
}

fn parse_place_holder_ident(
    place_holder_ident: Ident,
    prefix: Ident,
    trees: &mut PeekNth<impl Iterator<Item = TokenTree>>,
) -> Option<PlaceHolder> {
    // pre~N~suff

    // ~
    let Some(TokenTree::Punct(punct)) = trees.peek() else {
        return None;
    };

    // ~
    if punct.as_char() != '~' {
        return None;
    };

    // N
    let Some(TokenTree::Ident(ident)) = trees.peek_nth(1) else {
        return None;
    };

    // N
    if &place_holder_ident != ident {
        return None;
    }

    let suffix = (|| {
        // ~
        let Some(TokenTree::Punct(punct)) = trees.peek_nth(2) else {
            return None;
        };

        // ~
        if punct.as_char() != '~' {
            return None;
        }

        // suff
        if let Some(TokenTree::Ident(ident)) = trees.peek_nth(3) {
            Some(ident.clone())
        } else {
            None
        }
    })();

    // ~
    let tilde1 = trees.next().unwrap();
    let tilde1 = parse_quote!( #tilde1 );
    // N (target)
    let target = trees.next().unwrap();
    let target = parse_quote!( #target );
    let suffix = suffix.map(|suffix| {
        // ~
        let tilde2 = trees.next().unwrap();
        let tilde2 = parse_quote!( #tilde2 );
        // suff
        let _suffix = trees.next().unwrap();

        (tilde2, suffix)
    });

    Some(PlaceHolder::AsIdent {
        prefix,
        tilde1,
        target,
        suffix,
    })
}
