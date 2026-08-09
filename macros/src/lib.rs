use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::Span;
use quote::quote;
use syn::{Error, Fields, Ident, ItemStruct, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn game_object(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemStruct);

    if let Fields::Named(ref mut fields) = item.fields {
        fields.named.push(parse_quote!(pub color: Rgba8));
        fields.named.push(parse_quote!(pub shader: &'static str));
    } else {
        return Error::new_spanned(
            &item,
            "Current attribute supports structs with named fields",
        )
        .to_compile_error()
        .into();
    }

    item.attrs
        .retain(|attr| !attr.path().is_ident("game_object"));

    let class = &item.ident;

    let found_crate =
        crate_name("snake-engine").expect("snake-engine must be present in Cargo.toml");
    let crate_ident = match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( ::#ident )
        }
    };

    let expanded = quote! {
        #item

        impl #crate_ident::Render for #class {
            fn color(&self) -> Rgba8 {
               self.color.clone()
            }

            fn shader(&self) -> &'static str {
                self.shader
            }
        }
    };

    TokenStream::from(expanded)
}
