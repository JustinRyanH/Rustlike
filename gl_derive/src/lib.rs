extern crate proc_macro;
extern crate syn;
#[cfg(test)]
extern crate rspec;
#[cfg(test)]
extern crate rl_gl;
#[macro_use]
extern crate quote;


use proc_macro::TokenStream;

#[proc_macro_derive(DescribeAttributes)]
pub fn describe_attributes(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = impl_describe_attributes(&ast);
    gen.parse().unwrap()
}

fn impl_describe_attributes(ast: &syn::MacroInput) -> quote::Tokens {
    let fields = match ast.body {
        syn::Body::Struct(ref data) => { data.fields() }
        syn::Body::Enum(_) => panic!("#[derive(DescribeAttributes)] can only be done with structs"),
    };

    let attrs: Vec<quote::Tokens> = fields.iter().map(|v| {
        quote!{
            rl_gl::attributes::Attribute::new(
                rl_gl::attributes::AttributeSize::One,
                rl_gl::attributes::AttributeKind::Float,
                false,
                0,
            )
        }
    }).collect();

    let name = &ast.ident;
    quote! {
        impl #name {
            unsafe fn attributes() -> Vec<Attribute> {
                vec![
                    #(#attrs),*
                ]
            }
        }
    }
}



#[cfg(test)]
mod tests {
    #[macro_use]
    extern crate derive_rl_gl;
    use super::*;
    use rl_gl::attributes::{Attribute, DescribeAttributes};
    use rspec::given;


    #[test]
    fn deriving_describe_attributes() {
        let tested: Vec<Attribute> = Vec::new();
        rspec::run(&given("DescribeAttributes", tested, |ctx| {
            ctx.when("struct contains attributes with floats", |ctx| {
                ctx.before(|example| {
                    #[derive(DescribeAttributes)]
                    struct ExampleStruct {
                        color: [f32; 4],
                        position: [f32; 3],
                        uv: [f32; 2],
                        another_var: f32,
                    }

                    example = ExampleStruct::attributes();
                });
                ctx.then("it has the appropriate attributes", |example| {
                    assert_eq!(example.len(), 4)
                });
            });
        }));
    }
}
