use std::collections::HashMap;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Field, Fields, Ident, Type,
};

#[proc_macro_derive(Marshalable, attributes(marshalable))]
pub fn derive_tpm_marshal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_tpm_marshal_inner(input).into()
}

fn derive_tpm_marshal_inner(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let Data::Struct(stru) = input.data else {
        unimplemented!()
    };
    let field_unmarsh = get_field_unmarshal(&stru.fields);
    let expanded = quote! {
        impl Marshalable for #name  {
            fn try_unmarshal() {
                #field_unmarsh
            }
        }
    };

    expanded
}

fn get_named_field_unmarshal<'a>(
    basic_field_types: &mut HashMap<&'a Ident, Type>,
    field: &'a Field,
) -> TokenStream {
    let name = &field.ident;
    let field_type = &field.ty;
    if let Some(ident) = name {
        basic_field_types.insert(ident, field_type.clone());
    }
    quote_spanned! {field.span()=>
        let #name = <#field_type>::try_unmarshal();
    } // replacing quote_spanned with quote, eliminates the warning
}

fn get_field_unmarshal(all_fields: &Fields) -> TokenStream {
    let mut basic_field_types = HashMap::new();
    let Fields::Named(ref fields) = all_fields else {
        unimplemented!()
    };
    let mut recurse = Vec::new();
    for field in fields.named.iter() {
        let r = get_named_field_unmarshal(&mut basic_field_types, field);
        recurse.push(r);
    }
    quote! {
        #(#recurse)*
    }
}
