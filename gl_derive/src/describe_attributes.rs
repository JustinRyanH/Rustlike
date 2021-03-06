use syn;
use quote;

use context::MacroContext;
use errors::{MacroError, MacroResult};
use utils::{fields, get_field_name};

fn kind_from_type(ctx: MacroContext, ty: &syn::Ty) -> MacroResult<quote::Tokens> {
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

fn size_from_type(ctx: MacroContext, field: &syn::Field) -> MacroResult<quote::Tokens> {
    use syn::Ty;
    match field.ty {
        Ty::Slice(_) => {
            return Err(MacroError::FieldError(
                ctx.quote(),
                "slice is an invalid field.".into(),
            ))
        }
        Ty::Tup(_) => {
            return Err(MacroError::FieldError(
                ctx.quote(),
                "tuples are not yet supported.".into(),
            ))
        }
        Ty::Array(_, ref expr) => {

            let size = match expr {
                &syn::ConstExpr::Lit(ref lit) => {
                    match lit {
                        &syn::Lit::Int(ref size, _) => *size,
                        _ => return Err(MacroError::Unaccessible),
                    }
                }
                _ => {
                    return Err(MacroError::FieldError(
                        ctx.quote(),
                        "unexpected field, should be a constant size array".into(),
                    ))
                }
            };
            if size > 4 {
                return Err(MacroError::FieldError(ctx.quote(), "To large".into()));
            }

            Ok(match size {
                1 => quote! { rl_gl::attributes::AttributeSize::One },
                2 => quote! { rl_gl::attributes::AttributeSize::Two },
                3 => quote! { rl_gl::attributes::AttributeSize::Three },
                4 => quote! { rl_gl::attributes::AttributeSize::Four },
                _ => return Err(MacroError::Unaccessible),
            })
        }
        Ty::Path(_, _) => Ok(quote!{ rl_gl::attributes::AttributeSize::One }),
        _ => unimplemented!(),
    }
}

pub fn get_attrs(ctx: MacroContext, fields: &[syn::Field]) -> MacroResult<Vec<quote::Tokens>> {
    let ref name = ctx.name;
    let mut to_return = Vec::new();
    for (i, field) in fields.iter().enumerate() {
        let kind = kind_from_type(ctx.with_field(field.clone()), &field.ty)?;
        let size = size_from_type(ctx.with_field(field.clone()), &field)?;
        let ident = get_field_name(i, field);
        to_return.push(quote! {
            rl_gl::attributes::Attribute::new(
                #size,
                #kind,
                false,
                std::mem::size_of::<#name>(),
                &(*(std::ptr::null() as *const #name)).#ident as *const _ as usize
            )
        });
    }
    Ok(to_return)
}

pub fn impl_describe_attributes(ast: &syn::MacroInput) -> MacroResult<quote::Tokens> {
    let name = &ast.ident;

    let struct_data = match ast.body {
        syn::Body::Struct(ref data) => data,
        syn::Body::Enum(ref possiblities) => {
            return Err(MacroError::BodyError(
                quote!{
                    enum #name {
                        #(#possiblities),*
                    }
                },
                "enums currently cannot be described to OpenGL".to_string(),
            ))
        }
    };
    let ctx = MacroContext::new(&ast).with_struct_kind(&struct_data);
    let attrs: Vec<quote::Tokens> = get_attrs(ctx.clone(), fields(ctx.clone(), struct_data)?)?;
    Ok(quote! {
        impl DescribeAttributes for #name {
            unsafe fn attributes() -> Vec<Attribute> {
                vec![
                    #(#attrs),*
                ]
            }
        }
    })
}
