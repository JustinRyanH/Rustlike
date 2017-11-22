extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod errors;

use proc_macro::TokenStream;
use quote::ToTokens;

use errors::{MacroError, MacroResult};


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

// fn size_of_array(ty: Box<syn::Ty>, size_expr: &syn::ConstExpr) -> quote::Tokens {
//     // let size: u64 = match size_expr {
//     //     &syn::ConstExpr::Lit(ref l) => match l {
//     //         &syn::Lit::Int(ref v, _) => *v,
//     //         _ => panic!("Unexpected Literal"),
//     //     },
//     //     _ => panic!("Unexpected Expression"),
//     // };

//     quote! {
//         rl_gl::attributes::AttributeSize::One
//     }
//     // let size: u64 = match expr {
//     //     &syn::ConstExpr::Lit(ref lt) => {
//     //         match lt {
//     //             &syn::Lit::Int(ref v, _) => {
//     //                 if  v > &4  {
//     //                     panic!("Only support up to 4d Vertices");
//     //                 }
//     //                 v
//     //             }
//     //             _ => panic!("Shouldn't be possible"),
//     //         }
//     //         unimplemented!()
//     //     },
//     //     _ => panic!("Somehow, you made it here. That shouldn't be possible... congratz I guess"),
//     // };
// }

struct MacroContext {
    name: syn::Ident,
}

impl MacroContext {
    pub fn new(ast: &syn::MacroInput) -> MacroContext {
        MacroContext { name: ast.ident.clone() }
    }
}

fn fields<'a>(ctx: &MacroContext, s: &'a syn::VariantData) -> MacroResult<&'a [syn::Field]> {
    let ref name = ctx.name;
    let fields = s.fields();
    if fields.len() <= 0 {
        return Err(MacroError::BodyError(
            quote!{
                struct #name {}
            },
            "empty structs cannot be described to OpenGL".into(),
        ));
    }
    Ok(fields)
}

fn get_attrs(ctx: &MacroContext, fields: &[syn::Field]) -> MacroResult<Vec<quote::Tokens>> {
    use syn::Ty;

    let ref name = ctx.name;
    let mut to_return = Vec::new();
    for field in fields {
        match field.ty {
            Ty::Slice(_) => return Err(MacroError::FieldError(
                quote!{
                    struct #name {
                        #field
                    }
                },
                "slice is an invalid field.".into()
            )),
            Ty::Tup(_) => return Err(MacroError::FieldError(
                quote!{
                    struct #name {
                        #fields
                    }
                },
                "tuples are not yet supported.".into()
            )),
            Ty::Array(_, ref expr) => {
                let size = match expr {
                    &syn::ConstExpr::Lit(ref lit) => {
                        match lit {
                            &syn::Lit::Int(ref size, _) => *size,
                            _ => return Err(MacroError::Unaccessible)
                        }
                    },
                    _ => return Err(MacroError::FieldError(
                        quote!{
                            struct #name {
                                #field
                            }
                        },
                        "unexpected field, should be a constant size array".into()
                    )),
                };
                if size > 4 {
                    return Err(MacroError::FieldError(
                        quote! {
                            struct #name {
                                #field
                            }
                        },
                        "To large".into()
                    ));
                }

                let size_token = match size {
                    1 => quote! { rl_gl::attributes::AttributeSize::One },
                    2 => quote! { rl_gl::attributes::AttributeSize::Two },
                    3 => quote! { rl_gl::attributes::AttributeSize::Three },
                    4 => quote! { rl_gl::attributes::AttributeSize::Four },
                    _ => return Err(MacroError::Unaccessible),
                };
                to_return.push(
                    quote! {
                        rl_gl::attributes::Attribute::new(
                            #size_token,
                            rl_gl::attributes::AttributeKind::Float,
                            false,
                            0,
                        )
                })
            },
            _ => unimplemented!(),
        }
    }
    Ok(to_return)
}

fn impl_describe_attributes(ast: &syn::MacroInput) -> MacroResult<quote::Tokens> {
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
            ));
        }
    };
    let ctx = MacroContext::new(&ast);

    let attrs: Vec<quote::Tokens> = get_attrs(&ctx, fields(&ctx, struct_data)?)?;

    // let attrs: Vec<quote::Tokens> = fields(&ctx, struct_data)?
    //     .iter()
    //     .map(|_| {
    //         quote!{
    //                 rl_gl::attributes::Attribute::new(
    //                     rl_gl::attributes::AttributeSize::One,
    //                     rl_gl::attributes::AttributeKind::Float,
    //                     false,
    //                     0,
    //                 )
    //             }
    //     })
    //     .collect();

    Ok(quote! {
            impl #name {
                unsafe fn attributes() -> Vec<Attribute> {
                    vec![
                        #(#attrs),*
                    ]
                }
            }
        })
}
