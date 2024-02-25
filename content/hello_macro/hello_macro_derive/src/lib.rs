// example of the procedural macro
// we need the dependencies syn and quote

use proc_macro::TokenStream;
use quote::quote;
// syn will parse the code in the data struct, which we can manipulate
use syn;

#[proc_macro_derive(HelloMacro)]
// this function will be called when the user will write #[derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // #name will be replaced with the variable name as the rust code
    let gen = quote! {impl HelloMacro for #name{
        fn hello_macro(){
            println!("Hello, Macro! My name is {}!", stringify!(#name));
        }
    }};
    gen.into()
}

// the output of syn
// DeriveInput {
//     // --snip--
//
//     ident: Ident {
//         ident: "Pancakes",
//         span: #0 bytes(95..103)
//     },
//     data: Struct(
//         DataStruct {
//             struct_token: Struct,
//             fields: Unit,
//             semi_token: Some(
//                 Semi
//             )
//         }
//     )
// }
