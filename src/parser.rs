use crate::ast::{self, ASTNode};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "aql.pest"]
pub struct AQLParser;

pub fn parse(source: &str) -> ASTNode {
    // top-level parser
    let pairs = AQLParser::parse(Rule::program, source).unwrap();
    let mut ret = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::declaration => {
                ret.push(ASTNode::Declaration(Box::new(parse_decl(pair))));                
            },
            _ => {}
        }
    };
    ASTNode::Top(ret)
}

fn parse_decl(pair: pest::iterators::Pair<Rule>) -> ASTNode {
    let pair = pair.into_inner().next().unwrap();
    let ret = match pair.as_rule() {
        Rule::structure_declaration => {
            let mut pairs = pair.into_inner();

            let s_type = pairs.next().unwrap().as_str().to_string();
            let ident = pairs.next().unwrap();
            let stmt = pairs.next().unwrap();

            let structure_declaration = ast::ASTNode::StructureDelcaration { 
                s_type, 
                name: ident.as_str().to_string(),
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

fn parse_typed_identifier(pair: pest::iterators::Pair<Rule>) -> ASTNode {
    // two idents.
    let mut pairs = pair.clone().into_inner();

    let aql_type = pairs.next().unwrap().as_str().to_string();
    let variable = pairs.next().unwrap().as_str().to_string();

    ASTNode::TypedIdentifier { aql_type, variable }
}

fn parse_state(pair: pest::iterators::Pair<Rule>) -> ASTNode {
    let mut pairs = pair.clone().into_inner();
    match pair.as_rule() {
        Rule::labeled_statement => {

        }
        Rule::dsl_transition => {
            let action = pairs.next().unwrap().as_str().to_string();
            let ident = Box::new(ASTNode::Ident(pairs.next().unwrap().as_str().to_string()));

            return ASTNode::Transition { action, ident };
        }
        Rule::variable_declaration => {
            let typed_identifier = pairs.next().unwrap();
            let typed_identifier = Box::new(parse_typed_identifier(typed_identifier));
            if let Some(expr_raw) = pairs.next() {
                let expr = Some(Box::new(parse_expr(expr_raw)));
                return ASTNode::VariableDeclaration { typed_identifier, expr };
            } else {
                return ASTNode::VariableDeclaration { typed_identifier, expr: None };
            }

        }
        Rule::assignment => {
            let name = pairs.next().unwrap().as_str().to_string();

            let expr = pairs.next().unwrap();
            let expr = Box::new(parse_expr(expr));
            return ASTNode::Assignment { name, expr };
        }
        Rule::conditional => {
            let expr = Box::new(parse_expr(pairs.next().unwrap())); 
            let if_blk = Box::new(parse_state(pairs.next().unwrap()));
            let mut else_blk = Box::new(ASTNode::None);

            // else block is optional.

            let next_pair = pairs.next();
            if next_pair.is_some() {
                else_blk = Box::new(parse_state(next_pair.unwrap()));
            }

            return ASTNode::Conditional { expr, if_blk, else_blk };
        }
        Rule::block => {
            let mut stmts = vec![];

            for pair in pairs {
                stmts.push(parse_state(pair));
            }

            return ASTNode::Block(stmts);
        }
        Rule::await_block => {
            let keyword = String::from("await");
            let mut call = None;
            let mut when_block = Box::new(ASTNode::None);

            for pair in pairs {
                match pair.as_rule() {
                    Rule::call => {
                        call = Some(Box::new(parse_dsl(pair)));
                    },
                    Rule::when_block => {
                        when_block = Box::new(parse_when(pair));
                    },
                    _ => {

                    }
                }
            }
            return ASTNode::Await { keyword, call, when_block };
        }

        Rule::listen_handle => {
            let block = Box::new(parse_state(pairs.next().unwrap()));
            let catch_block = Box::new(parse_catch(pairs.next().unwrap()));

            return ASTNode::Listen { block, catch_block }
        }

        Rule::return_stmt => {

        }

        Rule::expr => {
            return parse_expr(pair);
        }
        _ => {

        }
    }

    ASTNode::None
}

fn parse_when(pair: pest::iterators::Pair<Rule>) -> ASTNode {
    let mut pairs = pair.into_inner();
    let call = Box::new(parse_dsl(pairs.next().unwrap()));
    let ident = Box::new(ASTNode::Ident(pairs.next().unwrap().as_str().to_string()));
    let block = Box::new(parse_state(pairs.next().unwrap()));
    ASTNode::When { keyword: String::from("when"), call, ident, block }
}

fn parse_expr(pair: pest::iterators::Pair<Rule>) -> ASTNode {
    let pair = pair.into_inner().next().unwrap();
    // println!("{:?}", pair.as_rule());
    match pair.as_rule() {
        Rule::dsl_term => {
            return parse_dsl(pair);
        }


        Rule::unuaryop => {

        }
        
        Rule::binop => {

        }

        Rule::list => {

        }

        _ => {}
    }

    ASTNode::None
}

fn parse_dsl(pair: pest::iterators::Pair<Rule>) -> ASTNode {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::call => {
            let mut pairs = pair.into_inner();
            let qualified_name_raw = pairs.next().unwrap();
            let list_raw = pairs.next().unwrap();

            let qualified_name = Box::new(parse_qualified_name(qualified_name_raw));
            let mut args_list = vec![];

            for expr in list_raw.into_inner() {
                // println!("{:?}", expr.as_rule());
                args_list.push(parse_expr(expr));
            }

            let list = Box::new(ASTNode::ExprList(args_list));

            return ASTNode::Call { qualified_name, list }
        }
        Rule::ident => {
            return ASTNode::Ident(pair.as_str().to_string());
        }
        Rule::qualified_name => {
            return parse_qualified_name(pair);
        }
        Rule::constval => {
            let constval = pair.into_inner().next().unwrap();
            return ASTNode::ConstVal(constval.as_str().to_string());
        }
        _ => {

        }
    }
    ASTNode::None
}

fn parse_catch(pair: pest::iterators::Pair<Rule>) -> ASTNode {
    let mut pairs = pair.into_inner();
    let mut idents = vec![];

    let keyword = pairs.next().unwrap().as_str().to_string();
    let qualified_name = Box::new(parse_qualified_name(pairs.next().unwrap()));
    let mut block = Box::new(ASTNode::None);

    for pair in pairs {
        match pair.as_rule() {
            Rule::ident => {
                idents.push(ASTNode::Ident(pair.as_str().to_string()))
            }
            Rule::statement => {
                block = Box::new(parse_state(pair));
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
                        block 
                    }
}

fn parse_qualified_name(pair: pest::iterators::Pair<Rule>) -> ASTNode {
    // name.name.name.var
    let pairs = pair.into_inner();
    let mut ret = vec![];

    for pair in pairs {
        ret.push(ASTNode::Ident(pair.as_str().to_string()));
    }

    ASTNode::QualifiedName { names: ret } 
}
