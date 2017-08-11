#![feature(proc_macro)]
extern crate proc_macro;

extern crate syn;
#[macro_use] extern crate synom;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;

named!(parse_facade -> (syn::Ident, syn::Block),
    preceded!(keyword!("mod"), tuple!(call!(syn::parse::ident), call!(syn::parse::block)))
);

named!(parse_args -> (),
    do_parse!(delimited!(punct!("("), punct!("hide"), punct!(")")) >> (()))
);

#[proc_macro_attribute]
pub fn facade(args: TokenStream, input: TokenStream) -> TokenStream {
    let (mut mod_ident, mod_block) = parse_facade(&input.to_string()).expect("Expecting a module definition.");

    if let synom::IResult::Done(..) = parse_args(&args.to_string()) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        mod_ident = syn::Ident::from(&format!("{}{}", mod_ident, secs_since_epoch) as &str);
    }

    (quote! {
        mod #mod_ident #mod_block
        pub use self::#mod_ident::*;
    }).to_string().parse().unwrap()
}
