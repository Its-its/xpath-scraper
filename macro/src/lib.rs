#[macro_use] extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use syn::{Attribute, Data, DeriveInput, ExprAssign, Fields, spanned::Spanned};

// https://doc.rust-lang.org/reference/procedural-macros.html


#[proc_macro_derive(Scraper, attributes(scrape))]
pub fn derive_scraper(input: TokenStream) -> TokenStream {
	let mut input = parse_macro_input!(input as DeriveInput);

	// println!("item: {:#?}", input);

	let body = define_body(&mut input.data);

	let name = input.ident;

	TokenStream::from(quote! {
		impl scraper_main::ScraperMain for #name {
			fn scrape(doc: &scraper_main::Document, container: Option<scraper_main::Node>) -> scraper_main::Result<Self> {
				Ok(#body)
			}
		}
	})
}

fn define_body(data: &mut Data) -> syn::__private::TokenStream2 {
	match data {
		Data::Struct(s) => {
			define_fields(&mut s.fields)
		}

		Data::Enum(_) => unimplemented!("Enum"),
		Data::Union(_) => unimplemented!("Union"),
	}
}

fn define_fields(field_types: &mut Fields) -> syn::__private::TokenStream2 {
	match field_types {
		Fields::Named(fields) => {
			let recurse = fields.named.iter().map(|field| {
				let name = &field.ident;

				let xpath = get_xpath(&field.attrs).expect("Missing XPATH");

				let field_span = field.span();
				quote_spanned! {field_span=>
					#name: scraper_main::evaluate(#xpath, doc, container.clone()).convert_from(doc)?
				}
			}).collect::<Vec<_>>();

			quote! {
				#[allow(clippy::redundant_clone)]
				Self {
					#(#recurse),*
				}
			}
		}

		Fields::Unnamed(fields) => {
			let recurse = fields.unnamed.iter().map(|field| {
				let xpath = get_xpath(&field.attrs).expect("Missing XPATH");

				let field_span = field.span();
				quote_spanned! {field_span=>
					scraper_main::evaluate(#xpath, doc, container.clone()).convert_from(doc)?
				}
			}).collect::<Vec<_>>();

			quote! {
				#[allow(clippy::redundant_clone)]
				Self(
					#(#recurse),*
				)
			}
		}

		Fields::Unit => unimplemented!("Unimplemented Field")
	}
}


fn get_xpath(attributes: &[Attribute]) -> Option<String> {
	get_scrape_attr_value("xpath", attributes).map(|(_, v)| v)
}

fn get_scrape_attr_value(name: &str, attributes: &[Attribute]) -> Option<(String, String)> {
	for attr in attributes {
		if attr.path.get_ident()? == "scrape" {
			let parsed = parse_attr(attr)?;

			if parsed.0 == name {
				return Some(parsed);
			}
		}
	}

	None
}


fn parse_attr(attr: &Attribute) -> Option<(String, String)> {
	let stream = attr.parse_args::<ExprAssign>().ok()?;

	let left = if let syn::Expr::Path(value) = *stream.left {
		value
	} else {
		return None;
	};

	let right = if let syn::Expr::Lit(value) = *stream.right {
		value
	} else {
		return None;
	};

	let right_value = if let syn::Lit::Str(value) = right.lit {
		value.value()
	} else {
		return None;
	};

	Some((left.path.get_ident()?.to_string(), right_value))
}