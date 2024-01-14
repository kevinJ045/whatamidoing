#[derive(Debug)]
enum Token {
	Identifier(String),
	Colon,
	Type(String),
	Equal,
	Expression(String),
	StringLiteral(String),
	Operation(char),
	OpenBrace,
	CloseBrace,
	OpenBracket,
	CloseBracket,
	OpenParen,
	CloseParen,
	SemiColon,
	Or,
	And,
	Not,
	Comma,
	Question,
	Number(String),
	Boolean(String),
}

fn tokenize(input: &str) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut current_token = String::new();
	let mut in_string_literal = false;
	let mut string_literal_delimiter = ' ';

	for c in input.chars() {
		if in_string_literal {
			if c == string_literal_delimiter {
				tokens.push(Token::StringLiteral(current_token.clone()));
				current_token.clear();
				in_string_literal = false;
			} else {
				current_token.push(c);
			}
	} else {
		match c {
			' ' | '\n' | '\t' => {
				if !current_token.is_empty() {
						tokens.push(process_token(&current_token));
						current_token.clear();
				}
			}
			'=' | ':' => {
				if !current_token.is_empty() {
						tokens.push(process_token(&current_token));
						current_token.clear();
				}
				tokens.push(match c {
						'=' => Token::Equal,
						':' => Token::Colon,
						_ => unreachable!(),
				});
			}
				'\'' | '\"' => {
					if !current_token.is_empty() {
						tokens.push(process_token(&current_token));
						current_token.clear();
					}
					in_string_literal = true;
					string_literal_delimiter = c;
				}
				'{' => {
					if !current_token.is_empty() {
						tokens.push(process_token(&current_token));
						current_token.clear();
					}
					tokens.push(Token::OpenBrace);
				}
				'}' => {
					if !current_token.is_empty() {
						tokens.push(process_token(&current_token));
						current_token.clear();
					}
					tokens.push(Token::CloseBrace);
				}
				'[' => {
					if !current_token.is_empty() {
						tokens.push(process_token(&current_token));
						current_token.clear();
					}
					tokens.push(Token::OpenBracket);
				}
				']' => {
					if !current_token.is_empty() {
						tokens.push(process_token(&current_token));
						current_token.clear();
					}
					tokens.push(Token::CloseBracket);
				}
				'(' => {
					if !current_token.is_empty() {
						tokens.push(process_token(&current_token));
						current_token.clear();
					}
					tokens.push(Token::OpenParen);
				}
				')' => {
					if !current_token.is_empty() {
						tokens.push(process_token(&current_token));
						current_token.clear();
					}
					tokens.push(Token::CloseParen);
				}
				',' => {
					if !current_token.is_empty() {
						tokens.push(process_token(&current_token));
						current_token.clear();
					}
					tokens.push(Token::Comma);
				}
				';' => {
					if !current_token.is_empty() {
						tokens.push(process_token(&current_token));
						current_token.clear();
					}
					tokens.push(Token::SemiColon);
				}
				'|' => {
					if !current_token.is_empty() {
						tokens.push(process_token(&current_token));
						current_token.clear();
					}
					tokens.push(Token::Or);
				}
				'&' => {
					if !current_token.is_empty() {
						tokens.push(process_token(&current_token));
						current_token.clear();
					}
					tokens.push(Token::And);
				}
				'!' => {
					if !current_token.is_empty() {
						tokens.push(process_token(&current_token));
						current_token.clear();
					}
					tokens.push(Token::Not);
				}
				'?' => {
					if !current_token.is_empty() {
						tokens.push(process_token(&current_token));
						current_token.clear();
					}
					tokens.push(Token::Question);
				}
				'+' | '-' | '*' | '/' | '%' | '^' | '~' | '<' | '>' => {
					if !current_token.is_empty() {
						tokens.push(process_token(&current_token));
						current_token.clear();
					}
					tokens.push(Token::Operation(c));
				}
				_ => {
					current_token.push(c);
				}
			}
		}
	}

	if !current_token.is_empty() {
		tokens.push(process_token(&current_token));
	}

	tokens
}

fn process_token(token: &str) -> Token {
	match token {
		_ if token == "true" => Token::Boolean(token.to_string()),
		_ if token == "false" => Token::Boolean(token.to_string()),
		_ if token.chars().all(|x: char| char::is_ascii_alphabetic(&x)) => Token::Identifier(token.to_string()),
		_ if token.chars().all(char::is_numeric) || (token.starts_with('-') && token[1..].chars().all(|x| char::is_numeric(x))) => {
			Token::Number(token.to_string())
		}
		_ => {
			if token.contains(':') {
				let parts: Vec<&str> = token.split(':').collect();
				if parts.len() == 2 {
					Token::Identifier(parts[0].to_string())
				} else {
					Token::Type(token.to_string())
				}
			} else {
				Token::Expression(token.to_string())
			}
		}
	}
}

fn main() {
	let input = "let name: str | int = 'Hello, Rust!' || true && (x + y) || [1, 2, 3];\n func name(str name) : str | int { println!('Hello'); }";
	let tokens = tokenize(input);

	println!("{:?}", tokens);
}
