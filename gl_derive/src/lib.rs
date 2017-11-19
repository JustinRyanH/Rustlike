// extern crate proc_macro;
// extern crate syn;
// #[macro_use] extern crate quote;


// use proc_macro::TokenStream;

// #[proc_macr_derive(rustlike::gl::attributes::DescribeAttributes)]
// pub fn describe_attributes(input: TokenSteam) -> TokenStream {
//     let s = input.to_string();
//     let ast = syn::parse_macro_input(&s).unwrap();
// }

// pub fn impl_describe_attributes(ast: &syn::MacroInput) -> quote::Tokens {
//     let name = &ast.ident;
//     quote! {
//         impl #name {
//             unsafe fn attributes() -> Vec<Attribute> {
//                 Vec::new()
//             }
//         }
//     }
// }



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
