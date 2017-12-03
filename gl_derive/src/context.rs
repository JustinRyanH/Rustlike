use syn;
use quote;

use utils::StructKind;

#[derive(Clone, Debug)]
pub struct MacroContext {
    pub name: syn::Ident,
    pub struct_kind: StructKind,
    pub field: Option<syn::Field>,
}

impl MacroContext {
    pub fn new(ast: &syn::MacroInput) -> MacroContext {
        MacroContext {
            name: ast.ident.clone(),
            field: None,
            struct_kind: StructKind::None,
        }
    }

    pub fn with_field(&self, field: syn::Field) -> MacroContext {
        MacroContext {
            name: self.name.clone(),
            field: Some(field.clone()),
            struct_kind: self.struct_kind,
        }
    }

    pub fn with_struct_kind(&self, kind: &syn::VariantData) -> MacroContext {
        MacroContext {
            name: self.name.clone(),
            field: self.field.clone(),
            struct_kind: match kind {
                &syn::VariantData::Struct(_) => StructKind::Struct,
                &syn::VariantData::Tuple(_) => StructKind::Tuple,
                &syn::VariantData::Unit => StructKind::None,
            },
        }
    }

    pub fn quote(&self) -> quote::Tokens {
        let ref name = self.name;
        match self.field {
            Some(ref field) => quote! { struct #name { #field } },
            None => quote! { struct #name {} },
        }
    }
}
