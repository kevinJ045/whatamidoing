mod parser;
mod tokenizer;


use crate::parser::parse;
use crate::tokenizer::tokenize;


fn main() {
	let input = "let name = n;";
	let tokens = tokenize(input);

	let parsed = parse(tokens);

	println!("{:?}", parsed);
}
