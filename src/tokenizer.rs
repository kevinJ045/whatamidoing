
#[derive(Debug, Copy, Clone)]
pub struct TokenAddress {
	line: i128,
	column: i128,
	index: i128,
	start: i128,
	end: i128
}

#[derive(Debug)]
pub enum Token {
	Identifier(TokenAddress, String),
	Colon(TokenAddress),
	Type(TokenAddress, String),
	Equal(TokenAddress),
	Expression(TokenAddress, String),
	StringLiteral(TokenAddress, String),
	Operation(TokenAddress, char),
	OpenBrace(TokenAddress),
	CloseBrace(TokenAddress),
	OpenBracket(TokenAddress),
	CloseBracket(TokenAddress),
	OpenParen(TokenAddress),
	CloseParen(TokenAddress),
	SemiColon(TokenAddress),
	Or(TokenAddress),
	And(TokenAddress),
	Not(TokenAddress),
	Comma(TokenAddress),
	Question(TokenAddress),
	Number(TokenAddress, String),
	Boolean(TokenAddress, String)
}


pub fn tokenize(input: &str) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut current_token = String::new();
	let mut in_string_literal = false;
	let mut string_literal_delimiter = ' ';
	let mut index = 0;
	let mut line_number = 1;
	let mut column_number = 1;
	let mut token_address_prev = TokenAddress {
		line: line_number,
		column: column_number,
		index,
		start: index,
		end: column_number,
	};

	for c in input.chars() {
		let mut token_address = TokenAddress {
			line: line_number,
			column: column_number,
			index,
			start: index,
			end: column_number - 1
		};
		if in_string_literal {
			if c == string_literal_delimiter {
				token_address.start = token_address_prev.start;
				tokens.push(Token::StringLiteral(token_address, current_token.clone()));
				current_token.clear();
				in_string_literal = false;
			} else {
				current_token.push(c);
			}
		} else {
			match c {
				' ' | '\n' | '\t' => {
					if !current_token.is_empty() {
						tokens.push(process_token(token_address, &current_token));
						current_token.clear();
					}
				}
				'=' | ':' => {
					if !current_token.is_empty() {
						tokens.push(process_token(token_address, &current_token));
						current_token.clear();
					}
					tokens.push(match c {
						'=' => Token::Equal(token_address),
						':' => Token::Colon(token_address),
						_ => unreachable!(),
					});
				}
				'\'' | '\"' => {
					if !current_token.is_empty() {
						tokens.push(process_token(token_address, &current_token));
						current_token.clear();
					}
					in_string_literal = true;
					string_literal_delimiter = c;
					token_address_prev.start = column_number;
				}
				'{' => {
					if !current_token.is_empty() {
						tokens.push(process_token(token_address, &current_token));
						current_token.clear();
					}
					tokens.push(Token::OpenBrace(token_address));
				}
				'}' => {
					if !current_token.is_empty() {
						tokens.push(process_token(token_address, &current_token));
						current_token.clear();
					}
					tokens.push(Token::CloseBrace(token_address));
				}
				'[' => {
					if !current_token.is_empty() {
						tokens.push(process_token(token_address, &current_token));
						current_token.clear();
					}
					tokens.push(Token::OpenBracket(token_address));
				}
				']' => {
					if !current_token.is_empty() {
						tokens.push(process_token(token_address, &current_token));
						current_token.clear();
					}
					tokens.push(Token::CloseBracket(token_address));
				}
				'(' => {
					if !current_token.is_empty() {
						tokens.push(process_token(token_address, &current_token));
						current_token.clear();
					}
					tokens.push(Token::OpenParen(token_address));
				}
				')' => {
					if !current_token.is_empty() {
						tokens.push(process_token(token_address, &current_token));
						current_token.clear();
					}
					tokens.push(Token::CloseParen(token_address));
				}
				',' => {
					if !current_token.is_empty() {
						tokens.push(process_token(token_address, &current_token));
						current_token.clear();
					}
					tokens.push(Token::Comma(token_address));
				}
				';' => {
					if !current_token.is_empty() {
						tokens.push(process_token(token_address, &current_token));
						current_token.clear();
					}
					tokens.push(Token::SemiColon(token_address));
				}
				'|' => {
					if !current_token.is_empty() {
						tokens.push(process_token(token_address, &current_token));
						current_token.clear();
					}
					tokens.push(Token::Or(token_address));
				}
				'&' => {
					if !current_token.is_empty() {
						tokens.push(process_token(token_address, &current_token));
						current_token.clear();
					}
					tokens.push(Token::And(token_address));
				}
				'!' => {
					if !current_token.is_empty() {
						tokens.push(process_token(token_address, &current_token));
						current_token.clear();
					}
					tokens.push(Token::Not(token_address));
				}
				'?' => {
					if !current_token.is_empty() {
						tokens.push(process_token(token_address, &current_token));
						current_token.clear();
					}
					tokens.push(Token::Question(token_address));
				}
				'+' | '-' | '*' | '/' | '%' | '^' | '~' | '<' | '>' => {
					if !current_token.is_empty() {
						tokens.push(process_token(token_address, &current_token));
						current_token.clear();
					}
					tokens.push(Token::Operation(token_address, c));
				}
				_ => {
					current_token.push(c);
				}
			}
		}
		match c {
			'\n'=> {
				line_number += 1;
				column_number = 1;
			}
			_ => {
				column_number += 1;
			}
		}
		index += 1;
	}

	if !current_token.is_empty() {
		let token_address = TokenAddress {
			line: line_number,
			column: column_number,
			index,
			start: index,
			end: index,
		}	;
		tokens.push(process_token(token_address, &current_token));
	}

	tokens
}

fn process_token(address: TokenAddress, token: &str) -> Token {
	match token {
		_ if token == "true" => Token::Boolean(address, token.to_string()),
		_ if token == "false" => Token::Boolean(address, token.to_string()),
		_ if token.chars().all(|x: char| char::is_ascii_alphabetic(&x)) => Token::Identifier(TokenAddress{
			line: address.line,
			column: address.column - token.to_string().len() as i128,
			index: address.index,
			start: address.start - token.to_string().len() as i128,
			end: address.end,
		}, token.to_string()),
		_ if token.chars().all(char::is_numeric) || (token.starts_with('-') && token[1..].chars().all(|x| char::is_numeric(x))) => {
			Token::Number(address, token.to_string())
		}
		_ => {
			if token.contains(':') {
				let parts: Vec<&str> = token.split(':').collect();
				if parts.len() == 2 {
					Token::Identifier(address, parts[0].to_string())
				} else {
					Token::Type(address, token.to_string())
				}
			} else {
				Token::Expression(address, token.to_string())
			}
		}
	}
}
