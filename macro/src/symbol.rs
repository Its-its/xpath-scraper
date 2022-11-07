use std::fmt::{self, Display};
use syn::{Ident, Path};


pub const BASE_SCRAPE: Symbol = Symbol("scrape");


pub const DEFAULT: Symbol = Symbol("default");
pub const XPATH: Symbol = Symbol("xpath");
pub const TRANSFORM: Symbol = Symbol("transform");


// From Serde Symbol
#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

impl PartialEq<Symbol> for Ident {
	fn eq(&self, word: &Symbol) -> bool {
		self == word.0
	}
}

impl<'a> PartialEq<Symbol> for &'a Ident {
	fn eq(&self, word: &Symbol) -> bool {
		*self == word.0
	}
}

impl PartialEq<Symbol> for Path {
	fn eq(&self, word: &Symbol) -> bool {
		self.is_ident(word.0)
	}
}

impl<'a> PartialEq<Symbol> for &'a Path {
	fn eq(&self, word: &Symbol) -> bool {
		self.is_ident(word.0)
	}
}

impl Display for Symbol {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str(self.0)
	}
}