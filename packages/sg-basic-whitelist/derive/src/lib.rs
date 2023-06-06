use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DataEnum, DeriveInput};

/// Merges the variants of two enums.
///
/// Adapted from cw-plus-plus which is adapted from DAO DAO:
/// https://github.com/larry0x/cw-plus-plus/blob/main/packages/ownable/derive/src/lib.rs
fn merge_variants(metadata: TokenStream, left: TokenStream, right: TokenStream) -> TokenStream {
    use syn::Data::Enum;

    // parse metadata
    let args = parse_macro_input!(metadata as AttributeArgs);
    if let Some(first_arg) = args.first() {
        return syn::Error::new_spanned(first_arg, "macro takes no arguments")
            .to_compile_error()
            .into();
    }

    // parse the left enum
    let mut left: DeriveInput = parse_macro_input!(left);
    let Enum(DataEnum {
        variants,
        ..
    }) = &mut left.data else {
        return syn::Error::new(left.ident.span(), "only enums can accept variants")
            .to_compile_error()
            .into();
    };

    // parse the right enum
    let right: DeriveInput = parse_macro_input!(right);
    let Enum(DataEnum {
        variants: to_add,
        ..
    }) = right.data else {
        return syn::Error::new(left.ident.span(), "only enums can provide variants")
            .to_compile_error()
            .into();
    };

    // insert variants from the right to the left
    variants.extend(to_add.into_iter());

    quote! { #left }.into()
}

/// Append basic-whitelist-related query message variant(s) to an enum.
///
/// For example, apply the `sg_basic_whitelist_query` macro to the following enum:
///
/// ```rust
/// #[sg_basic_whitelist_query]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// enum QueryMsg {
///     #[returns(FooResponse)]
///     Foo {},
///     #[returns(BarResponse)]
///     Bar {},
/// }
/// ```
///
/// Is equivalent to:
///
/// ```rust
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// enum QueryMsg {
///     #[returns(bool)]
///     IncludesAddress { address: String },
///     #[returns(FooResponse)]
///     Foo {},
///     #[returns(BarResponse)]
///     Bar {},
/// }
/// ```
///
/// Note: `#[sg_basic_whitelist_query]` must be applied _before_ `#[cw_serde]`.
#[proc_macro_attribute]
pub fn sg_basic_whitelist_query(metadata: TokenStream, input: TokenStream) -> TokenStream {
    merge_variants(
        metadata,
        input,
        quote! {
            enum Right {
                /// Query the contract's basic whitelist info
                #[returns(bool)]
                IncludesAddress { address: String },
            }
        }
        .into(),
    )
}
