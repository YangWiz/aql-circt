mod ast;

use std::fs;

use ast::ASTNode;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "aql.pest"]
pub struct AQLParser;

fn main() {
    fs::read();
}

pub fn parse(source: &str) -> () {
    // top-level parser
    let pairs = AQLParser::parse(Rule::program, source).unwrap();
    for pair in pairs {
        match pair.as_rule() {
            Rule::declaration => {

            },
            _ => {}
        }
    };
    ()
}

fn parse_decl(pair: pest::iterators::Pair<Rule>) -> ASTNode {
    let ret = match pair.as_rule() {
        Rule::statement => {
            let mut pairs = pair.into_inner();
            let s_type = pairs.next().unwrap().to_string();
            let ident = pairs.next().unwrap();
            let stmt = pairs.next().unwrap();
            let mut structure_declaration = ast::ASTNode::StructureDelcaration { 
                s_type, 
                statement: Box::new(parse_state(stmt)),
            };
            structure_declaration
        },
        Rule::internal_func_decl => {
            ast::ASTNode::None 
        },
        _ => ast::ASTNode::None 
    };
    ret
}

fn parse_state(pair: pest::iterators::Pair<Rule>) -> ASTNode {
    let mut pairs = pair.clone().into_inner();
    match pair.as_rule() {
        Rule::labeled_statement => {

        }
        Rule::dsl_transition => {

        }
        Rule::variable_declaration => {

        }
        Rule::assignment => {

        }
        Rule::conditional => {

        }
        Rule::block => {
            let mut stmts = vec![];

            for pair in pairs {
                stmts.push(parse_state(pair));
            }

            return ASTNode::Block(stmts);
        }
        Rule::await_block => {

        }

        Rule::listen_handle => {
            let keyword = pairs.next().unwrap().to_string();
            let block = parse_state(pairs.next().unwrap());

            let catch_block = pairs.next().unwrap();

            return ASTNode::Listen { keyword, , catch_block }
        }

        Rule::return_stmt => {

        }

        Rule::expr => {

        }
        _ => {

        }
    }

    ASTNode::None
}

fn parse_catch(pair: pest::iterators::Pair<Rule>) -> ASTNode {
    let mut pairs = pair.into_inner();
    let mut stmts = vec![];
    let mut idents = vec![];

    let keyword = pairs.next().unwrap().to_string();
    let qualified_name = parse_qualified_name(pairs.next().unwrap());

    for pair in pairs {
        match pair.as_rule() {
            Rule::ident => {
                idents.push(ASTNode::Ident(pair.to_string()))
            }
            Rule::statement => {
                let stmt = parse_state(pair);
                stmts.push(stmt);
            }
            _ => {
                // Left or right quota.
                // Do nothing.
            }
        }
    }
    ASTNode::CatchBlock { keyword, 
                        qualified_name, 
                        idents, 
                        stmts 
                    }
}

fn parse_qualified_name(pair: pest::iterators::Pair<Rule>) -> Vec<ASTNode> {
    // name.name.name.var
    let pairs = pair.into_inner();
    let mut ret = vec![];

    for pair in pairs {
        ret.push(ASTNode::Ident(pair.to_string()));
    }

    ret
}

