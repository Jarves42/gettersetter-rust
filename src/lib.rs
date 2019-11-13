#![crate_type = "proc-macro"]
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::Lit;
use syn::MetaItem;
use syn::NestedMetaItem;

extern crate syn;
#[macro_use]
extern crate quote;

#[proc_macro_derive(GetSet, attributes(getset))]
pub fn qqq(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();

    let struct_name = &ast.ident;
    println!("struct_name {:?}", struct_name);
    let mut code = quote!();

    if let syn::Body::Struct(s) = ast.body {
        for field in s.fields() {
            let field_name = field.ident.clone().unwrap();
            let field_type = field.ty.clone();
            let field_type_2 = field_type.clone();

            let mut skip_setter = false;
            let mut skip_getter = false;
            let mut setter_name = format!("set_{}", field_name);
            let mut getter_name = format!("get_{}", field_name);

            for attr in &field.attrs {
                let attr_name = attr.name();
                if let "getset" = attr_name {
                    if let MetaItem::List(_, meta_items) = &attr.value {
                        for nested_meta_item in meta_items {
                            if let NestedMetaItem::MetaItem(meta_item) = nested_meta_item {
                                match meta_item {
                                    MetaItem::Word(name) => {
                                        let name = name.to_string();
                                        match name.as_str() {
                                            "skip_getter" => skip_getter = true,
                                            "skip_setter" => skip_setter = true,
                                            _ => {}
                                        }
                                    }
                                    MetaItem::NameValue(name, value) => {
                                        let name = name.to_string();
                                        let attr_value = if let Lit::Str(value, _) = value {
                                            Some(value.to_string())
                                        } else {
                                            None
                                        };
                                        match name.as_str() {
                                            "getter_name" => getter_name = attr_value.unwrap(),
                                            "setter_name" => setter_name = attr_value.unwrap(),
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            let setter_name = syn::Ident::new(format!("{}", setter_name).as_str());
            let getter_name = syn::Ident::new(format!("{}", getter_name).as_str());

            if !skip_setter {
                code = quote!(
                    #code

                    pub fn #setter_name(&mut self, x : #field_type) {
                        self.#field_name = x;
                    }
                );
            }

            if !skip_getter {
                code = quote!(
                    #code

                    pub fn #getter_name(&self) -> &#field_type_2 {
                        return &self.#field_name;
                    }
                );
            }
        }
    }

    quote!(
        impl #struct_name {
            #code
        }
    )
    .parse()
    .unwrap()
}
