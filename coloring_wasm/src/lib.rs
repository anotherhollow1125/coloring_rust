use std::{collections::HashMap, ops::Range};

use coloring_common::{Colored, FragSpecs};
use coloring_macro::repeat_for_types;
use html_escape::encode_text;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct Input {
    code: String,
    filters: Vec<Filter>,
}

#[derive(Serialize)]
struct Output {
    hit_filter: Option<Filter>,
    colored: String,
}

#[wasm_bindgen]
pub fn colored(input: JsValue) -> Result<JsValue, JsValue> {
    let input = serde_wasm_bindgen::from_value(input)?;

    let output = colored_inner(input);

    Ok(serde_wasm_bindgen::to_value(&output)?)
}

fn colored_inner(Input { code, filters }: Input) -> Output {
    let mut errors = String::new();

    for filter in filters {
        match filter.try_parse(&code) {
            Ok(ranges) => {
                return Output {
                    hit_filter: Some(filter),
                    colored: to_html_string(&code, ranges),
                }
            }
            Err(e) => errors.push_str(&e),
        }
    }

    Output {
        hit_filter: None,
        colored: errors.replace("\n", "<br />"),
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum Filter {
    File,
    Item,
    Block,
    Stmt,
    Expr,
    Type,
    Path,
    Visibility,
    Ident,
    Lifetime,
    Lit,
    Meta,
}

impl Filter {
    fn try_parse(self, content: &str) -> Result<HashMap<FragSpecs, Vec<Range<usize>>>, String> {
        repeat_for_types!(for F in [
            File,
            Item,
            Block,
            Stmt,
            Expr,
            Type,
            Path,
            Visibility,
            Ident,
            Lifetime,
            Lit,
            Meta,
        ] {
            match self {
                #(
                    Filter::F => {
                        let parsed = syn::parse_str::<syn::F>(content)
                            .map_err(|e| format!("Failed to parse {}\n---\n{}", stringify!(F), e))?;

                        let mut colored = Colored::<syn::F>::new(parsed);
                        colored.visit();

                        Ok(colored.ranges)
                    },
                )*
            }
        })
    }
}

fn classes_map(len: usize, ranges: HashMap<FragSpecs, Vec<Range<usize>>>) -> Vec<Vec<String>> {
    let mut classes: Vec<Vec<String>> = (0..len).map(|_| Vec::new()).collect();

    for (frag, ranges) in ranges.into_iter() {
        for range in ranges {
            for i in range {
                classes[i].push(frag.to_string().to_ascii_lowercase());
            }
        }
    }

    classes
}

fn to_html_string(content: &str, ranges: HashMap<FragSpecs, Vec<Range<usize>>>) -> String {
    let classes_map = classes_map(content.bytes().len(), ranges);

    content
        .char_indices()
        .map(|(i, c)| spanned(c, &classes_map[i]))
        .collect()
}

fn spanned(s: char, classes: &[String]) -> String {
    let class = classes.join(" ");

    format!(
        "<span class=\"{}\">{}</span>",
        class,
        if s == '\n' {
            "<br />".to_string()
        } else {
            encode_text(&s.to_string()).to_string()
        }
    )
}
