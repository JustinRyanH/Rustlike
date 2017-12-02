use syn;
use quote;

use context::MacroContext;
use errors::{MacroResult, MacroError};
use utils::{StructKind, fields};

pub fn get_uniforms(_: MacroContext, fields: &[syn::Field]) -> MacroResult<Vec<quote::Tokens>> {
    let mut to_return = Vec::new();
    for (_, field) in fields.iter().enumerate() {
        let name = field.clone().ident.unwrap();
        if name == "changed_uniforms" {
        } else {
            let str_lit = syn::Lit::Str(name.as_ref().into(), syn::StrStyle::Cooked);
            to_return.push(quote!{
                rl_gl::program::uniforms::NamedUniform::new(#str_lit, self.#name)
            })
        }
    }
    Ok(to_return)
}

pub fn get_changed_uniforms(
    _: MacroContext,
    fields: &[syn::Field],
) -> MacroResult<Vec<quote::Tokens>> {
    let mut to_return = Vec::new();
    for (_, field) in fields.iter().enumerate() {
        let name = field.clone().ident.unwrap();
        if name == "changed_uniforms" {
        } else {
            let str_lit = syn::Lit::Str(name.as_ref().into(), syn::StrStyle::Cooked);
            to_return.push(quote!{
                &#str_lit => {
                    changed_values.push(
                        rl_gl::program::uniforms::NamedUniform::new(#str_lit, self.#name));
                },
            })
        }
    }
    Ok(to_return)
}

fn does_have_dirty(_: MacroContext, fields: &[syn::Field]) -> bool {
    for (_, field) in fields.iter().enumerate() {
        let name = field.clone().ident.unwrap();
        if name == "changed_uniforms" {
            return true;
        }
    }
    false
}

pub fn impl_updatable_uniforms(ast: &syn::MacroInput) -> MacroResult<quote::Tokens> {
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
                "enums currently cannot be used to update uniforms"
                    .to_string(),
            ))
        }
    };

    let ctx = MacroContext::new(&ast).with_struct_kind(&struct_data);
    match ctx.clone().struct_kind {
        StructKind::Struct => (),
        _ => {
            return Err(MacroError::BodyError(
                quote!{
            struct #name {
                #struct_data
            }
        },
                "only structs with named fields are allowed for uniforms"
                    .into(),
            ))
        }
    }

    let named_uniforms: Vec<quote::Tokens> =
        get_uniforms(ctx.clone(), fields(ctx.clone(), struct_data)?)?;


    let changed_values = if does_have_dirty(ctx.clone(), fields(ctx.clone(), struct_data)?) {
        let changed_uniforms: Vec<quote::Tokens> =
            get_changed_uniforms(ctx.clone(), fields(ctx.clone(), struct_data)?)?;
        quote!{
            fn changed_uniform_values(&self) -> Vec<NamedUniform> {
                let mut changed_values: Vec<NamedUniform> = Vec::new();
                for uniform in &self.changed_uniforms {
                    match uniform {
                        #(#changed_uniforms)*
                        _ => (),
                    }
                }
                return changed_values;
            }
        }
    } else {
        let uniforms = named_uniforms.clone();
        quote!{
            fn changed_uniform_values(&self) -> Vec<NamedUniform> {
                vec![
                    #(#uniforms),*
                ]
            }
        }
    };

    Ok(quote! {
        impl UpdatableUniforms for #name {
            fn uniform_values(&self) -> Vec<NamedUniform> {
                vec![
                    #(#named_uniforms),*
                ]
            }
            #changed_values
        }
    })
}
