#[derive(Debug)]
enum Token {
    Keyword(String),
    Identifier(String),
    Colon,
    Type(String),
    Assignment,
    Dot,
    StringLiteral(String),
    NumberLiteral(i64), // Adjust type as needed
    Operator(char),
    Whitespace,
    Unknown,
    OpenBrace(char),
    CloseBrace(char),
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(ch) = chars.next() {
        match ch {
            ' ' => tokens.push(Token::Whitespace),
            '+' => tokens.push(Token::Operator('+')),
            // '{' => tokens.push(Token::OpenBrace('('))),
            // '}' => tokens.push(Token::CloseBrace(')')),
            // Add more token types as needed
            _ => tokens.push(Token::Unknown),
        }
    }

    tokens
}

#[derive(Debug)]
enum AstNode {
    VariableDeclaration {
        name: String,
        type_: String,
        expression: Box<AstNode>,
    },
    FunctionCall {
        name: String,
        argument: Box<AstNode>,
    },
    // ... (Define other AST node types)
}

fn build_ast(tokens: &[Token]) -> AstNode {
    match tokens.get(0) {
        Token::Keyword(keyword) => match keyword.as_str() {
            "let" => {
                let (name, ty) = match tokens.get() {
                    Token::Identifier(name), Token::Type(ty) => (name.to_string(), ty.to_string()),
                    _ => panic!("Expected identifier and type after keyword 'let'"),
                };
                AstNode::VariableDeclaration { name, ty, expression: Box::new(build_ast(&tokens[1..])), }
            }
            _ => panic!("Unexpected keyword"),
        }
        Token::Operator(op) => {
            let left = build_ast(&tokens[1..]);
            let right = build_ast(&tokens[left.end + 1..]);
            AstNode::BinaryOperation { left, op, right }
        }
        // Implement more AST node types as needed
        _ => panic!("Unexpected token"),
    }
}