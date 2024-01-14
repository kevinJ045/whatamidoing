use crate::tokenizer::Token;


#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    VariableDeclaration(String, String, Box<ASTNode>),
    FunctionDeclaration(String, Vec<(String, String)>, String, Vec<ASTNode>),
    Literal(String),
    BinaryOperation(char, Box<ASTNode>, Box<ASTNode>),
}

pub fn parse(tokens: Vec<Token>) -> ASTNode {
    // Your parsing logic here
    // This is a simple example, actual parsing logic can be more complex
    ASTNode::Program(tokens.into_iter().map(|t| parse_token(t)).collect())
}

fn parse_token(token: Token) -> ASTNode {
    match token {
        Token::Identifier(_, name) => ASTNode::Literal(name),
        Token::Equal(address) => ASTNode::VariableDeclaration("var".to_string(), "int".to_string(), Box::new(ASTNode::Literal("0".to_string()))),
        Token::SemiColon(_) => ASTNode::Literal(";".to_string()),
        Token::OpenBrace(_) | Token::CloseBrace(_) => ASTNode::Program(vec![]),
        _ => panic!("Unsupported token: {:?}", token),
    }
}