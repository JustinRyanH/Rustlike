extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod errors;
mod utils;
mod context;
mod describe_attributes;

use proc_macro::TokenStream;

use describe_attributes::impl_describe_attributes;


#[proc_macro_derive(DescribeAttributes)]
pub fn describe_attributes(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = match impl_describe_attributes(&ast) {
        Ok(gen) => gen,
        Err(e) => panic!("{}", e),
    };
    gen.parse().unwrap()
}

#[proc_macro_derive(UpdatableUniforms)]
pub fn updatable_uniforms(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = match impl_updatable_uniforms(&ast) {
        Ok(gen) => gen,
        Err(e) => panic!("{}", e),
    };
    gen.parse().unwrap()
}

fn impl_updatable_uniforms(ast: &syn::MacroInput) -> errors::MacroResult<quote::Tokens> {
    let name = &ast.ident;
    Ok(quote! {
        impl UpdatableUniforms for #name {
            fn uniform_values(&self) -> Vec<NamedUniform> {
                Vec::new()
            }

            fn changed_uniform_values(&self) -> Vec<NamedUniform> {
                Vec::new()
            }
        }
    })
}
