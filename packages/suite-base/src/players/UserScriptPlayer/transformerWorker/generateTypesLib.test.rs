```rust
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{parse2, parse_quote};

fn generate_types_interface(datatypes: Vec<(String, syn::Type)>) -> TokenStream {
    let mut interface = quote! {pub trait TypesInterface {}};
    for (datatype_name, datatype_type) in datatypes {
        let type_definition = match datatype_type {
            syn::Type::Path(path) => {
                format!(
                    "pub struct {} {{\n\
                     *type: {},\n\
                     }}",
                    datatype_name,
                    path.to_string()
                )
            },
            _ => panic!("Unsupported datatype type"),
        };
        interface.extend(quote! {pub use self::{#datatype_name::*};});
        interface.extend(quote! {pub struct #datatype_name {{\n\
                         *type: {},\n\
                         }}},
                               [datatype_name.to_string()]);
    }
    interface.into()
}

fn generate_types_lib(topics: Vec<(&str, &syn::Type)>, datatypes: HashMap<String, syn::Type>) -> TokenStream {
    let mut types_lib = quote! {pub struct TypesLib {}};
    for (topic_name, schema_type) in topics {
        if !datatypes.contains_key(schema_type.to_string()) {
            continue;
        }
        let type_definition = match datatypes[schema_type.to_string()] {
            syn::Type::Path(path) => format!(
                "pub struct {} {{\n\
                 *type: {},\n\
                 }}",
                topic_name,
                path.to_string()
            ),
            _ => panic!("Unsupported datatype type"),
        };
        types_lib.extend(quote! {pub use self::{#topic_name::*};});
        types_lib.extend(quote! {pub struct #topic_name {{\n\
                         *type: {},\n\
                         }}},
                               [topic_name.to_string()]);
    }
    types_lib.into()
}

// ... rest of the code remains the same
```