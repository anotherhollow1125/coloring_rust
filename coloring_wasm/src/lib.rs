use std::{collections::HashMap, ops::Range};

use coloring_common::{Colored, FragSpecs};
use coloring_macro::repeat_for_types;
use html_escape::encode_text;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct Input {
    code: String,
    filters: Vec<Filter>,
}

#[derive(Serialize)]
struct Output {
    hit_top_filter: Option<Filter>,
    hit_filters: Vec<Filter>,
    colored: String,
}

#[wasm_bindgen]
pub fn colored(input: JsValue) -> Result<JsValue, JsValue> {
    let input = serde_wasm_bindgen::from_value(input)?;

    let output = colored_inner(input);

    Ok(serde_wasm_bindgen::to_value(&output)?)
}

struct TopFilter {
    filter: Filter,
    colored: String,
}

fn colored_inner(Input { code, filters }: Input) -> Output {
    let mut errors = String::new();

    let mut top_filter = None;
    for &filter in filters.iter() {
        match filter.try_parse(&code) {
            Ok(ranges) => {
                if top_filter.is_none() {
                    top_filter = Some(TopFilter {
                        filter,
                        colored: to_html_string(&code, ranges),
                    });
                }
            }
            Err(e) => errors.push_str(&e),
        }
    }

    match top_filter {
        Some(TopFilter { filter, colored }) => Output {
            hit_top_filter: Some(filter),
            hit_filters: filters
                .into_iter()
                .filter(|f| f.try_parse(&code).is_ok())
                .collect(),
            colored,
        },
        None => Output {
            hit_top_filter: None,
            hit_filters: vec![],
            colored: errors,
        },
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
    #[serde(rename = "ty")]
    Type,
    Path,
    #[serde(rename = "vis")]
    Visibility,
    Ident,
    Lifetime,
    #[serde(rename = "literal")]
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

#[derive(Debug, PartialEq, Eq, Clone)]
enum SpanTag {
    Start(FragSpecs, usize),
    End,
}

impl PartialOrd for SpanTag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SpanTag {
    fn cmp(&self, other: &Self) -> Ordering {
        // End < Start
        // Start 2 < Start 1
        match (self, other) {
            (SpanTag::Start(_, _), SpanTag::End) => Ordering::Greater,
            (SpanTag::End, SpanTag::Start(_, _)) => Ordering::Less,
            (SpanTag::Start(_, len_self), SpanTag::Start(_, len_other)) => len_other.cmp(len_self),
            (SpanTag::End, SpanTag::End) => Ordering::Equal,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum SpanTagForRender {
    Start(FragSpecs, Vec<FragSpecs>),
    End,
}

fn span_tag_map(
    len: usize,
    ranges: HashMap<FragSpecs, Vec<Range<usize>>>,
) -> Vec<Vec<SpanTagForRender>> {
    let mut tags_all: Vec<Vec<SpanTag>> = (0..len + 1).map(|_| Vec::new()).collect();

    for (frag, ranges) in ranges.into_iter() {
        for Range { start, end } in ranges {
            if end == 0 || end > len {
                // invalid range
                continue;
            }

            tags_all[start].push(SpanTag::Start(frag, end - start));
            tags_all[end].push(SpanTag::End);
        }
    }

    for tags in tags_all.iter_mut() {
        tags.sort(); // </span> </span> <span class="..."> <span class="...">
    }

    let mut stack = Vec::new();
    let mut tags_for_render_all: Vec<Vec<SpanTagForRender>> =
        (0..len + 1).map(|_| Vec::new()).collect();
    for (i, tags_for_render) in tags_for_render_all.iter_mut().enumerate() {
        for tag in tags_all[i].iter() {
            match tag {
                SpanTag::End => {
                    stack.pop();

                    tags_for_render.push(SpanTagForRender::End);
                }
                SpanTag::Start(frag, _) => {
                    stack.push(*frag);

                    tags_for_render.push(SpanTagForRender::Start(*frag, stack.clone()));
                }
            }
        }
    }

    tags_for_render_all
}

fn to_html_string(content: &str, ranges: HashMap<FragSpecs, Vec<Range<usize>>>) -> String {
    let span_tag_map = span_tag_map(content.bytes().len(), ranges);

    let mut res: String = content
        .char_indices()
        .map(|(i, c)| {
            let mut res = String::new();

            for tag in &span_tag_map[i] {
                let tag = match tag {
                    SpanTagForRender::End => "</span>".to_string(),
                    SpanTagForRender::Start(ref frag, ref frags) => {
                        let frags = frags
                            .iter()
                            .map(|f| f.to_string().to_ascii_lowercase())
                            .collect::<Vec<_>>()
                            .join(" ");

                        format!(
                            "<span data-frag=\"{}\" class=\"{}\">",
                            frag.to_string().to_ascii_lowercase(),
                            frags
                        )
                    }
                };
                res.push_str(&tag);
            }

            res.push_str(&encode_text(&c.to_string()));

            res
        })
        .collect();

    for tag in span_tag_map.last().unwrap() {
        if let &SpanTagForRender::End = tag {
            res.push_str("</span>");
        }
    }

    res
}
