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

pub fn kind_from_type(ctx: MacroContext, ty: &syn::Ty) -> MacroResult<quote::Tokens> {
    match ty {
        &syn::Ty::Path(_, ref path) => {
            if let Some(last_segment) = path.segments.last() {
                let ref ident = last_segment.ident;
                let name: &str = ident.as_ref();
                match name {
                    "i32" => return Ok(quote!{rl_gl::attributes::AttributeKind::Int}),
                    "u32" => return Ok(quote!{rl_gl::attributes::AttributeKind::UnsignedInt}),
                    "f32" => return Ok(quote!{rl_gl::attributes::AttributeKind::Float}),
                    "f64" => return Ok(quote!{rl_gl::attributes::AttributeKind::Double}),
                    "i8" => return Ok(quote!{rl_gl::attributes::AttributeKind::Byte}),
                    "u8" => return Ok(quote!{rl_gl::attributes::AttributeKind::UnsignedByte}),
                    "i16" => return Ok(quote!{rl_gl::attributes::AttributeKind::Short}),
                    "u16" => return Ok(quote!{rl_gl::attributes::AttributeKind::UnsignedShort}),
                    _ => {
                        return Err(MacroError::TypeError(
                            ctx.quote(),
                            "attribute kind must be [i8, u8, i16, u16, i32, f32, f64]"
                                .into(),
                        ))
                    }
                }
            }
            return Err(MacroError::Unaccessible);
        }
        &syn::Ty::Array(ref s_ty, _) => return kind_from_type(ctx, s_ty),
        _ => unimplemented!(),
    }
}
