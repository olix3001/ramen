use ramen_common::ast::{self, Attribute, Attributes, NodeId};
use crate::{error::SyntaxError, lex::{Token, Tokens}};

pub fn parse_ramen(module_name: String, tokens: &mut Tokens) -> Result<ast::Module, SyntaxError> {
    Ok(ast::Module {
        name: module_name,
        items: parse_item_stream(tokens)?
    })
}

fn parse_item_stream(tokens: &mut Tokens) -> Result<Vec<ast::Item>, SyntaxError> {
    let mut items = Vec::<ast::Item>::new();
    loop {
        match tokens.peek() {
            Some(_) => {
                items.push(parse_item(tokens)?);
                new_lines(tokens);
            }
            None => break,
        }
    }
    Ok(items)
}

fn parse_item(tokens: &mut Tokens) -> Result<ast::Item, SyntaxError> {
    let attributes = parse_attributes(tokens, false)?;

    tokens.begin_span();
    let kind = match tokens.peek() {
        Some(Token::FuncKW) => ast::ItemKind::Function(parse_function_definition(tokens)?),

        _ => return Err(SyntaxError::ExpectedItem { found: tokens.next_info().unwrap() }),
    };

    Ok(ast::Item {
        location: tokens.end_span(),
        attributes,
        kind,
        id: NodeId::next()
    }) 
}

fn parse_function_definition(tokens: &mut Tokens) -> Result<ast::Function, SyntaxError> {
    tokens.expect(Token::FuncKW)?;

    let name = tokens.expect(Token::Identifier)?.text();
    let parameters = parse_enclosed_value_parameter_list(tokens)?;

    let return_type = if tokens.is(Token::Colon) { Some(parse_type(tokens)?) }
    else { None };

    let body = parse_block_or_expression_shorthand(tokens)?;

    Ok(ast::Function {
        name,
        parameters,
        return_type,
        body
    })
}

fn parse_block_or_expression_shorthand(tokens: &mut Tokens) -> Result<ast::Block, SyntaxError> {
    if tokens.is(Token::FatArrow) {
        new_lines(tokens);
        let expression = parse_expression(tokens)?;
        semis(tokens);

        Ok(ast::Block {
            location: expression.location.clone(),
            statements: vec![ast::Statement {
                location: expression.location.clone(),
                kind: ast::StatementKind::Return(expression),
                id: NodeId::next(),
            }],
            id: NodeId::next(),
        })
    } else { unimplemented!("Only expression syntax is supported currently") }
}

fn parse_expression(tokens: &mut Tokens) -> Result<ast::Expression, SyntaxError> {
    parse_primary_expression(tokens)
}

fn parse_primary_expression(tokens: &mut Tokens) -> Result<ast::Expression, SyntaxError> {
    tokens.begin_span();
    let kind = match tokens.next() {
        Some(Token::IntegerLiteral) => ast::ExpressionKind::Literal(ast::Literal::Integer(
            tokens.text().unwrap().parse().unwrap() 
        )),
        _ => return Err(SyntaxError::ExpectedExpression { found: tokens.current_info().unwrap() })
    };

    Ok(ast::Expression {
        location: tokens.end_span(),
        kind,
        id: NodeId::next(),
    })
}

fn parse_enclosed_value_parameter_list(tokens: &mut Tokens) -> Result<Vec<ast::ValueParameter>, SyntaxError> {
    let mut parameters = Vec::<ast::ValueParameter>::new();
    tokens.expect(Token::LeftParen)?;
    new_lines(tokens);

    while !tokens.is(Token::RightParen) {
        parameters.push(parse_value_parameter(tokens)?);
        new_lines(tokens);
        if !tokens.is(Token::Comma) {
            tokens.expect(Token::RightParen)?;
            break;
        }
        new_lines(tokens);
    }

    Ok(parameters)
}

fn parse_value_parameter(tokens: &mut Tokens) -> Result<ast::ValueParameter, SyntaxError> {
    tokens.begin_span();
    let parameter = parse_parameter(tokens)?;
    let initializer = if tokens.is(Token::Assign) { Some(parse_expression(tokens)?) }
        else { None };

    Ok(ast::ValueParameter {
        location: tokens.end_span(),
        parameter,
        initializer,
        id: NodeId::next()
    })
}

fn parse_parameter(tokens: &mut Tokens) -> Result<ast::Parameter, SyntaxError> {
    tokens.begin_span();
    let name = tokens.expect(Token::Identifier)?.text();
    tokens.expect(Token::Colon)?;
    let ty = parse_type(tokens)?;

    Ok(ast::Parameter {
        name,
        location: tokens.end_span(),
        ty,
        id: NodeId::next()
    })
}

fn parse_type(tokens: &mut Tokens) -> Result<ast::Type, SyntaxError> {
    tokens.begin_span();
    let kind = match tokens.next() {
        Some(Token::IntegerType) => {
            let text = tokens.text().unwrap();
            let width = text[3..].parse::<usize>().unwrap();
            ast::TypeKind::Integer(width)
        }

        _ => return Err(SyntaxError::ExpectedType { found: tokens.current_info().unwrap() })
    };

    Ok(ast::Type {
        location: tokens.end_span(),
        kind,
        id: NodeId::next()
    })
}

fn parse_attributes(tokens: &mut Tokens, _top_level: bool) -> Result<ast::Attributes, SyntaxError> {
    let mut attributes = Vec::<ast::Attribute>::new();
    while tokens.is(Token::At) {
        let name = tokens.expect(Token::Identifier)?.text();
        // Currently only marker attributes are supported
        attributes.push(Attribute {
            location: tokens.loc().unwrap(),
            kind: ast::AttributeKind::Marker(name),
            id: NodeId::next()
        });

        new_lines(tokens);
    }
    Ok(Attributes(attributes))
}

fn new_lines(tokens: &mut Tokens) {
    while tokens.is(Token::NL) {}
}
fn semis(tokens: &mut Tokens) {
    while tokens.is_any(&[Token::NL, Token::Semicolon]).is_some() {}
}