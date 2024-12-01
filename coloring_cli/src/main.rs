use anyhow::Result;
use clap::Parser;
use coloring_macro::repeat_for_types;
use dialoguer::Input;
use quote::ToTokens;
use syn::parse::Parse;
use syn::spanned::Spanned;

#[derive(Parser)]
struct Args {
    content: Option<String>,
}

fn main() -> Result<()> {
    let Args { content } = Args::parse();

    let content = get_content(content)?;

    let ast_result = parser(&content);

    match ast_result {
        Ok(ast) => println!(
            "{:?}\nspan: {:?}-{:?}",
            ast,
            ast.span().start(),
            ast.span().end()
        ),
        Err(e) => eprintln!("{:?}", e),
    }

    Ok(())
}

fn get_content(content: Option<String>) -> Result<String> {
    let res = match content {
        Some(content) => content,
        None => Input::new().with_prompt("Rust?").interact_text()?,
    };

    Ok(res)
}

#[derive(Debug)]
struct CustomPat(syn::Pat);

impl Parse for CustomPat {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(res) = syn::Pat::parse_multi_with_leading_vert(input) {
            return Ok(CustomPat(res));
        }

        if let Ok(res) = syn::Pat::parse_multi(input) {
            return Ok(CustomPat(res));
        }

        let res = syn::Pat::parse_single(input)?;

        Ok(CustomPat(res))
    }
}

impl ToTokens for CustomPat {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

trait Parsed: std::fmt::Debug + ToTokens {}

/// とにかくマッチさせる実験的なパーサー
fn parser(content: &str) -> Result<Box<dyn Parsed>> {
    let mut errors = Vec::new();

    repeat_for_types!(for T in [
        syn::File,
        syn::Item,
        syn::Block,
        syn::Type,
        syn::Path,
        syn::Visibility,
        syn::Ident,
        syn::Lifetime,
        syn::Lit,
        syn::Meta,
        // syn::Pat,
        CustomPat,
        syn::Stmt,
        syn::Expr,
        proc_macro2::TokenTree,
        proc_macro2::TokenStream,
    ] {
        impl Parsed for T {}

        let e = match syn::parse_str::<T>(content) {
            Ok(p) => return Ok(Box::new(p)),
            Err(e) => e,
        };
        errors.push(e);
    });

    Err(anyhow::anyhow!("All Match Failed!: {:?}", errors))
}
