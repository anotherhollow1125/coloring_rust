use proc_macro2::Span;
use std::collections::HashMap;
use std::fmt::Debug;
use syn::spanned::Spanned;
use syn::visit::Visit;

// ref: https://veykril.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum FragSpecs {
    Block,
    Expr,
    Ident,
    Item,
    Lifetime,
    Literal,
    Meta,
    Pat,
    Path,
    Stmt,
    // TT,
    Ty,
    Vis,
}

#[derive(Debug)]
pub struct Colored<T: Debug> {
    pub inner: T,
    pub visit: for<'a> fn(&'a mut Colored<T>, &T),
    pub spans: HashMap<FragSpecs, Vec<Span>>,
}

macro_rules! visit {
    ($elm:ident, $elmty:ty, $key:expr) => {
        fn $elm(&mut self, i: &'ast $elmty) {
            let spans = self.spans.entry($key).or_default();
            spans.push(i.span());

            ::syn::visit::$elm(self, i);
        }
    };
}

impl<'ast, T> Visit<'ast> for Colored<T>
where
    T: Debug,
{
    visit!(visit_block, syn::Block, FragSpecs::Block);
    visit!(visit_expr, syn::Expr, FragSpecs::Expr);
    visit!(visit_ident, syn::Ident, FragSpecs::Ident);
    visit!(visit_item, syn::Item, FragSpecs::Item);
    visit!(visit_lifetime, syn::Lifetime, FragSpecs::Lifetime);
    visit!(visit_lit, syn::Lit, FragSpecs::Literal);
    visit!(visit_meta, syn::Meta, FragSpecs::Meta);
    visit!(visit_pat, syn::Pat, FragSpecs::Pat);
    visit!(visit_path, syn::Path, FragSpecs::Path);
    visit!(visit_stmt, syn::Stmt, FragSpecs::Stmt);
    visit!(visit_type, syn::Type, FragSpecs::Ty);
    visit!(visit_visibility, syn::Visibility, FragSpecs::Vis);
}

impl<T> Colored<T>
where
    T: Clone + Debug,
{
    pub fn visit(&mut self) {
        let inner = self.inner.clone();

        (self.visit)(self, &inner);
    }
}

macro_rules! impl_colored {
    ($colored_ty:ident, $base_ty:ty, $fn_name:ident) => {
        pub type $colored_ty = Colored<$base_ty>;

        impl Colored<$base_ty> {
            pub fn new(inner: $base_ty) -> Self {
                Self {
                    inner,
                    visit: |v, node| ::syn::visit::$fn_name(v, node),
                    spans: HashMap::new(),
                }
            }
        }
    };
}

impl_colored!(ColoredBlock, syn::Block, visit_block);
impl_colored!(ColoredExpr, syn::Expr, visit_expr);
impl_colored!(ColoredIdent, syn::Ident, visit_ident);
impl_colored!(ColoredItem, syn::Item, visit_item);
impl_colored!(ColoredLifetime, syn::Lifetime, visit_lifetime);
impl_colored!(ColoredLit, syn::Lit, visit_lit);
impl_colored!(ColoredMeta, syn::Meta, visit_meta);
impl_colored!(ColoredPat, syn::Pat, visit_pat);
impl_colored!(ColoredPath, syn::Path, visit_path);
impl_colored!(ColoredStmt, syn::Stmt, visit_stmt);
impl_colored!(ColoredType, syn::Type, visit_type);
impl_colored!(ColoredVisibility, syn::Visibility, visit_visibility);
