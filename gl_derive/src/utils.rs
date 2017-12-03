use syn;
use quote;

use errors::{ MacroResult, MacroError };
use context::MacroContext;

#[derive(Clone, Copy, Debug)]
pub enum StructKind {
    Struct,
    Tuple,
    None,
}

pub fn get_field_name(location: usize, field: &syn::Field) -> quote::Tokens {
    if let Some(ref ident) = field.ident {
        return quote!{ #ident };
    }
    quote!{ #location }
}

pub fn fields<'a>(ctx: MacroContext, s: &'a syn::VariantData) -> MacroResult<&'a [syn::Field]> {
    let fields = s.fields();
    if fields.len() <= 0 {
        return Err(MacroError::BodyError(
            ctx.quote(),
            "empty structs cannot be described to OpenGL".into(),
        ));
    }
    Ok(fields)
}

