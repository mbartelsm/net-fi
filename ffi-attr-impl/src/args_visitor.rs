use syn::{
    Ident,
    Pat,
    spanned::Spanned,
    visit::Visit,
};

/// Visitor that collects all function argument identifiers. Panics/Errors out
/// when the function signature uses pattern-matching. It is too complex to
/// implement and also undesireable for foreign functions.
pub struct ArgsVisitor<'ast> {
    pub idents: Vec<&'ast Ident>
}

impl<'ast> ArgsVisitor<'ast> {
    pub fn new() -> Self {
        ArgsVisitor { idents: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for ArgsVisitor<'ast> {
    fn visit_pat(&mut self, pat: &'ast Pat) {
        match pat {
            Pat::Ident(pat_ident) => self.idents.push(&pat_ident.ident),
            Pat::Reference(pat_ref) => self.visit_pat(&pat_ref.pat),
            _ => {
                let msg = "Pattern matching is not supported in net-fi ffi function signatures";
                syn::Error::new(pat.span(), msg).to_compile_error();
                panic!("{msg}");
            },
        }
    }
}