use std::collections::HashSet;
use std::ops::Range;
use std::path::PathBuf;
use std::{collections::HashMap, sync::LazyLock};

use anyhow::Result;
use clap::{Args, Parser};
use coloring_common::{Colored, FragSpecs};
use coloring_macro::repeat_for_types;
use console::{Color, Style, StyledObject};
use dialoguer::Input;
use quote::ToTokens;
use syn::parse::Parse;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    content: Option<String>,

    #[arg(short, long)]
    file_path: Option<PathBuf>,

    #[command(flatten)]
    include_exclude: IncludeExclude,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
struct IncludeExclude {
    #[arg(short, long)]
    exclude_frags: Option<Vec<FragSpecs>>,

    #[arg(short, long)]
    include_frags: Option<Vec<FragSpecs>>,
}

enum Mode {
    Include(HashSet<FragSpecs>),
    Exclude(HashSet<FragSpecs>),
}

impl IncludeExclude {
    fn into_mode(self) -> Mode {
        match (self.exclude_frags, self.include_frags) {
            (Some(exclude_frags), None) => Mode::Exclude(exclude_frags.into_iter().collect()),
            (None, Some(include_frags)) => Mode::Include(include_frags.into_iter().collect()),
            _ => Mode::Exclude(HashSet::new()),
        }
    }
}

fn main() -> Result<()> {
    let Cli {
        content,
        file_path,
        include_exclude,
    } = Cli::parse();

    let mode = include_exclude.into_mode();

    let content = get_content(content, file_path)?;

    let range_info = parser(&content)?;

    let colored = colored_content(&content, range_info, mode);

    let res = colored
        .into_iter()
        .map(|c| c.to_string())
        .collect::<String>();

    println!("{}", res);

    Ok(())
}

fn get_content(content: Option<String>, file_path: Option<PathBuf>) -> Result<String> {
    let res = match (file_path, content) {
        (Some(path), _) => std::fs::read_to_string(path)?,
        (_, Some(content)) => content,
        _ => Input::new().with_prompt("Rust?").interact_text()?,
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

fn parser(content: &str) -> Result<HashMap<FragSpecs, Vec<Range<usize>>>> {
    let mut errors = Vec::new();

    repeat_for_types!(for T in [
        syn::File,
        syn::Item,
        syn::Block,
        syn::Stmt,
        syn::Expr,
        syn::Type,
        syn::Path,
        syn::Visibility,
        syn::Ident,
        syn::Lifetime,
        syn::Lit,
        syn::Meta,
        // syn::Pat,
        // CustomPat,
    ] {
        let e = match syn::parse_str::<T>(content) {
            Ok(p) => {
                let mut colored = Colored::<T>::new(p);
                colored.visit();

                return Ok(colored.ranges);
            }
            Err(e) => e,
        };
        errors.push(e);
    });

    Err(anyhow::anyhow!("All Match Failed!: {:?}", errors))
}

#[derive(Debug, Clone, Copy)]
enum BgFg {
    Bg,
    Fg,
}

#[derive(Debug, Clone, Copy)]
struct CustomStyle {
    color: Color,
    bgfg: BgFg,
}

impl CustomStyle {
    fn new(color: Color) -> Self {
        Self {
            color,
            bgfg: BgFg::Fg,
        }
    }

    fn set_bgfg(self, bgfg: BgFg) -> Self {
        Self { bgfg, ..self }
    }

    fn to_style(self) -> Style {
        match self.bgfg {
            BgFg::Bg => Style::new().bg(self.color),
            BgFg::Fg => Style::new().fg(self.color),
        }
    }
}

static COLOR_MAP: LazyLock<HashMap<FragSpecs, CustomStyle>> = LazyLock::new(|| {
    HashMap::from_iter([
        (FragSpecs::Block, CustomStyle::new(Color::Blue)),
        (
            FragSpecs::Expr,
            CustomStyle::new(Color::Blue).set_bgfg(BgFg::Bg),
        ),
        (
            FragSpecs::Ident,
            CustomStyle::new(Color::Yellow).set_bgfg(BgFg::Bg),
        ),
        (FragSpecs::Item, CustomStyle::new(Color::Yellow)),
        (
            FragSpecs::Lifetime,
            CustomStyle::new(Color::Red).set_bgfg(BgFg::Bg),
        ),
        (
            FragSpecs::Literal,
            CustomStyle::new(Color::Magenta).set_bgfg(BgFg::Bg),
        ),
        (FragSpecs::Meta, CustomStyle::new(Color::Magenta)),
        (
            FragSpecs::Pat,
            CustomStyle::new(Color::Green).set_bgfg(BgFg::Bg),
        ),
        (FragSpecs::Path, CustomStyle::new(Color::Green)),
        (
            FragSpecs::Stmt,
            CustomStyle::new(Color::Cyan).set_bgfg(BgFg::Bg),
        ),
        (
            FragSpecs::Ty,
            CustomStyle::new(Color::Yellow).set_bgfg(BgFg::Bg),
        ),
        (FragSpecs::Vis, CustomStyle::new(Color::Yellow)),
    ])
});

const ORDERED_FRAG_SPECS: [FragSpecs; 12] = [
    FragSpecs::Item,
    FragSpecs::Block,
    FragSpecs::Meta,
    FragSpecs::Stmt,
    FragSpecs::Expr,
    FragSpecs::Vis,
    FragSpecs::Lifetime,
    FragSpecs::Ty,
    FragSpecs::Pat,
    FragSpecs::Path,
    FragSpecs::Ident,
    FragSpecs::Literal,
];

fn colored_content(
    content: &str,
    range_info: HashMap<FragSpecs, Vec<Range<usize>>>,
    mode: Mode,
) -> Vec<StyledObject<char>> {
    let mut styles: Vec<Option<Style>> = vec![None; content.bytes().len()];

    for frag in ORDERED_FRAG_SPECS {
        match &mode {
            Mode::Include(inc) => {
                if !inc.contains(&frag) {
                    continue;
                }
            }
            Mode::Exclude(exc) => {
                if exc.contains(&frag) {
                    continue;
                }
            }
        }

        let style = COLOR_MAP.get(&frag).unwrap().to_style();

        if let Some(ranges) = range_info.get(&frag) {
            for range in ranges {
                for i in range.clone() {
                    styles[i] = Some(style.clone());
                }
            }
        }
    }

    let res = content
        .char_indices()
        .map(|(i, c)| {
            if c == '\n' {
                return Style::new().apply_to(c);
            }

            let style = styles[i].clone().unwrap_or_default();
            style.apply_to(c)
        })
        .collect();

    res
}

#[allow(unused)]
type 日本語名の型 = i32;

#[allow(unused)]
fn 日本語名の関数() {}
