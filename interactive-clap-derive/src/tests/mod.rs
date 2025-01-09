mod test_simple_enum;
mod test_simple_struct;

fn pretty_codegen(ts: &proc_macro2::TokenStream) -> String {
    let file = syn::parse_file(&ts.to_string()).unwrap();
    prettyplease::unparse(&file)
}
