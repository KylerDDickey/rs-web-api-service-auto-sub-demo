extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

static METHOD_TOKENS: &'static [&str] = &[
    "get",
    "post",
    "put",
    "delete",
    "head",
    "connect",
    "option",
    "trace",
    "patch",
    "route",
];

/// Generates a function that returns a configuration containing all
/// of the functions with route macro attributes. Must be placed in
/// a module block (which is removed upon compilation).
#[proc_macro_attribute]
pub fn service(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_m = syn::parse_macro_input!(item as syn::ItemMod);
    let items = get_mod_items(&item_m);
    let ep_functions = get_ep_functions(&item_m);

    // Codegen deletes the old mod block and replaces it with all the
    // code inside it and adds a function named "config". This function
    // will be scoped with the rest of the code (the scope of the mod
    // this attribute is applied to).
    let gen = quote! {
        #(#items)*
        pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
            cfg
                #(.service(#ep_functions))*;
        }
    };

    // The reason we have to apply this to an "redundant" mod is
    // because custom inner attributes are unstable. Until this
    // is resolved, this is the work-around we must use.

    gen.into()
}

fn get_ep_functions(item_m: &syn::ItemMod) -> Vec<syn::Ident> {
    get_mod_items(&item_m)
        .iter()
        .filter_map(get_method_token_or_none)
        .filter(has_ep_attribute)
        .map(|item_f| item_f.sig.ident)
        .collect()
}

fn get_mod_items(item_m: &syn::ItemMod) -> Vec<syn::Item> {
    item_m.content
        .as_ref()
        .unwrap()
        .1
        .to_owned()
}

fn get_method_token_or_none(item: &syn::Item) -> Option<syn::ItemFn> {
    match item {
        syn::Item::Fn(item_f) => Some(item_f.to_owned()),
        _ => None,
    }
}

fn has_ep_attribute(item_f: &syn::ItemFn) -> bool {
    item_f.attrs
        .iter()
        .any(method_attr_exists)
}

fn method_attr_exists(attr: &syn::Attribute) -> bool {
    METHOD_TOKENS
        .iter()
        .any(|ident| attr.path.is_ident(ident))
}
