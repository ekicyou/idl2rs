#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate regex;
#[macro_use]
extern crate nom;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod ast;
mod parser;


#[cfg(test)]
mod tests;