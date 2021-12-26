//! The macro which converts a struct or tuple into one which is able to be scraped easily.
//!
//! An example of this would be here:
//! ```rust
//! #[derive(Scraper)]
//! pub struct RedditListItem {
//!     #[scrape(xpath = r#"//a[@data-click-id="body"]/@href"#)]
//!     pub urls: Vec<String>
//! }
//! ```

#[macro_use] extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use quote::__private::Span;
use symbol::Symbol;
use syn::{Attribute, Data, DeriveInput, ExprAssign, Fields, Meta, NestedMeta, spanned::Spanned, Path};



mod symbol;



// https://doc.rust-lang.org/reference/procedural-macros.html

/// The macro which converts a struct or tuple into one which is able to be scraped easily.
///
/// An example of this would be here:
/// ```rust
/// #[derive(Scraper)]
/// pub struct RedditListItem {
///     #[scrape(xpath = r#"//a[@data-click-id="body"]/@href"#)]
///     pub urls: Vec<String>
/// }
/// ```
#[proc_macro_derive(Scraper, attributes(scrape))]
pub fn derive_scraper(input: TokenStream) -> TokenStream {
	let mut input = parse_macro_input!(input as DeriveInput);

	let body = define_body(&mut input.data);

	let name = input.ident;

	TokenStream::from(quote! {
		impl ::scraper_main::ScraperMain for #name {
			fn scrape(doc: &::scraper_main::Document, container: Option<&::scraper_main::Node>) -> ::scraper_main::Result<Self> {
				fn wrap_values<T>(name: &'static str, value: ::scraper_main::Result<T>) -> ::scraper_main::Result<T> {
					match value {
						Ok(v) => Ok(v),
						Err(v) => {
							Err(::scraper_main::Error::FieldValueError(name, Box::new(v)))
						}
					}
				}

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
				let name = field.ident.as_ref().unwrap();

				let scrape = Scrape::new(field.span(), &field.attrs);

				let eval = scrape.generate_evaluation(name.to_string());

				quote! {
					#name: #eval
				}
			}).collect::<Vec<_>>();

			// TODO: Remove allows

			quote! {
				#[allow(clippy::redundant_clone, clippy::default_trait_access)]
				Self {
					#(#recurse),*
				}
			}
		}

		Fields::Unnamed(fields) => {
			let recurse = fields.unnamed.iter()
				.enumerate()
				.map(|(index, field)|
					Scrape::new(field.span(), &field.attrs)
					.generate_evaluation(index.to_string())
				)
				.collect::<Vec<_>>();

			quote! {
				#[allow(clippy::redundant_clone, clippy::default_trait_access)]
				Self(
					#(#recurse),*
				)
			}
		}

		Fields::Unit => unimplemented!("Unimplemented Field")
	}
}


struct Scrape {
	span: Span,

	is_ignored: bool,

	xpath: Option<String>,
	transform_fn: Option<String>
}

impl Scrape {
	pub fn new(span: Span, attributes: &[Attribute]) -> Self {
		Self {
			span,
			is_ignored: does_attribute_exist(symbol::IGNORE, attributes),
			xpath: get_scrape_attr_value(symbol::XPATH, attributes),
			transform_fn: get_scrape_attr_value(symbol::TRANSFORM, attributes)
		}
	}

	pub fn generate_evaluation(self, field_name: String) -> syn::__private::TokenStream2 {
		if self.is_ignored {
			quote! {
				Default::default()
			}
		} else {
			let xpath = self.xpath.expect("Expected #[scrape(xpath = \"\")] macro");
			let span = self.span;

			if let Some(transform_fn) = self.transform_fn {
				let transform_ident = format_ident!("{}", transform_fn);

				quote_spanned! {span=>
					#transform_ident(wrap_values(#field_name, ::scraper_main::evaluate(#xpath, doc, container).convert_from(doc))?)
				}
			} else {
				quote_spanned! {span=>
					wrap_values(#field_name, ::scraper_main::evaluate(#xpath, doc, container).convert_from(doc))?
				}
			}
		}
	}
}


fn get_scrape_attr_value(attr_name: Symbol, attributes: &[Attribute]) -> Option<String> {
	for attr in attributes {
		if attr.path == symbol::BASE_SCRAPE {
			let parsed = parse_attr(attr)?;

			if parsed.0 == attr_name {
				return Some(parsed.1);
			}
		}
	}

	None
}

fn does_attribute_exist(name: Symbol, attributes: &[Attribute]) -> bool {
	for attr in attributes {
		if attr.path == symbol::BASE_SCRAPE {
			if let Some(parsed) = parse_attr_name(attr) {
				if parsed == name {
					return true;
				}
			}
		}
	}

	false
}


fn parse_attr(attr: &Attribute) -> Option<(Path, String)> {
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

	Some((left.path, right_value))
}

fn parse_attr_name(attr: &Attribute) -> Option<Path> {
	// TODO: Actually use parse_meta() for all attributes instead of just this one.

	let parse = attr.parse_meta().expect("--------------------------------------------");

	if let Meta::List(val) = parse {
		let ret = val.nested.into_iter().next();

		if let NestedMeta::Meta(Meta::Path(path)) = ret? {
			return Some(path);
		}
	}

	None
}