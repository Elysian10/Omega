use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    // Parse the input as an identifier
    let ident = parse_macro_input!(input as Ident);
    let ident_str = ident.to_string();
    
    // Convert to PascalCase for the struct name
    let struct_name = if ident_str == "div" {
        "Div".to_string()
    } else {
        // Handle other tags as needed
        let mut chars = ident_str.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    };
    
    let struct_ident = Ident::new(&struct_name, ident.span());
    
    // Generate the code to create an instance of the struct and return ()
    let expanded = quote! {
        {
            let _ = #struct_ident::default();
        }
    };
    
    TokenStream::from(expanded)
}