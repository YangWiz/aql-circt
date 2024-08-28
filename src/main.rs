mod ast;
mod parser;
mod utils;

use ast::{ASTNode, Statement};

use crate::parser::parse;
use std::fs;
use std::collections::HashMap;

fn main() {
    let file = fs::read_to_string("example.aql").unwrap();
    let ret = parse(&file);
    let mut output = Vec::new();
    println!("{:?}", &ret);

    let mut env = HashMap::new();
    pretty_print(ret, &mut output, 0, &mut env);

    for el in output {
         print!("{}", el);
    }

}

fn pretty_print(tree: ASTNode, output: &mut Vec<String>, indent: u8, env: &mut HashMap<String, Option<String>>) -> String {
    // Top level structure is the declaration.
    match tree {
        ASTNode::Top(nodes) => {
            output.push(String::from("fsm.machine @LoadInst(%arg0: i1, %arg1: i1) attributes {initialState = \"ReadyToDispatch\"} {\n"));
            for node in nodes {
                pretty_print(node, output, indent, env);
            }
            output.push(String::from("}\n"));
        },
        ASTNode::Integer(_) => todo!(),
        ASTNode::Decimal(_) => todo!(),
        ASTNode::Str(_) => todo!(),
        ASTNode::Ident(ident) => {
            output.push(ident + " ");
        },
        ASTNode::ConstVal(_) => todo!(),
        ASTNode::QualifiedName { names } => {
            for (i, name) in names.iter().enumerate() {
                match name {
                    ASTNode::Ident(ident) => {
                        output.push(String::from(ident));
                        if i != names.len() - 1 {
                            output.push(".".to_string());
                        }
                    }
                    _ => {

                    }
                }
            }
        },
        ASTNode::Declaration(decl) => {
            // top-level structure, machine, load instruction.
            pretty_print(*decl, output, indent + 1, env);
        },
        ASTNode::Transition { action, ident } => {
            output.push(String::from("\t".repeat(indent.into())));
            output.push(String::from("fsm.transition @"));
            if let ASTNode::Ident(next_state) = *ident {
                output.push(next_state);
            }
            output.push(";\n".to_string());
        },
        ASTNode::StructureDelcaration { s_type, statement, name } => {
            output.push(String::from("\t".repeat(indent.into())));

            if s_type == String::from("state") {
                output.push(String::from("fsm.state") + " ");
                output.push(String::from("@") + name.as_str() + " transitions ");
            }
            if s_type == String::from("controller_entry") {
                // a series of initilizations. 
            }

            pretty_print(*statement, output, indent, env);
        },
        ASTNode::Stmt(stmt) => {
            match stmt {
                Statement::LabeledStatement { label, stmt } => todo!(),
                Statement::DSLTransition { label, stmt } => todo!(),
                Statement::VariableDeclaration { t_ident, expr } => {
                    print!("hi!");
                },
                Statement::Assignment { name, expr } => todo!(),
            }
        },
        ASTNode::InternalFuncDecl(_) => todo!(),
        ASTNode::CatchBlock { keyword, qualified_name, idents, block } => {
            output.push(String::from("\t".repeat(indent.into())));
            output.push("handle ".to_string() + &keyword + " ");
            pretty_print(*qualified_name, output, indent, env);
            for ident in idents {
                pretty_print(ident, output, indent, env);
            }
            pretty_print(*block, output, indent, env);
        },
        ASTNode::Block(stmts) => {
            output.push(String::from("{\n"));

            for stmt in stmts {
                pretty_print(stmt, output, indent+1, env);
            }

            output.push("\t".repeat(indent.into()) + "}\n");
        },
        ASTNode::Expr(expr) => {
            match expr {
                ast::Expr::UnuaryOp { verb, term } => todo!(),
                ast::Expr::BinOp { verb, lhs, rhs } => todo!(),
                ast::Expr::List(_) => todo!(),
                ast::Expr::DSLTerm => {
                },
            }
        },
        ASTNode::Listen { block, catch_block } => {
            output.push("\t".repeat(indent.into()) + "listen ");
            pretty_print(*block, output, indent, env);
            output.push(String::from(" "));
            pretty_print(*catch_block, output, indent, env);
        },
        ASTNode::Call { qualified_name, list } => {
            output.push("\t".repeat(indent.into()));
            pretty_print(*qualified_name, output, indent, env);
            pretty_print(*list, output, indent, env);
            output.push(";\n".to_string());
        },
        ASTNode::ExprList(list) => {
            output.push("(".to_string());
            for expr in list {
                pretty_print(expr, output, indent, env);
                output.push(", ".to_string());
            }
            output.push(")".to_string());
        },
        ASTNode::None => {},
        ASTNode::Await { keyword, call, when_block } => {
            // ignore function call now.
            // ignore await keyword.
            pretty_print(*when_block, output, indent, env);
        },
        ASTNode::When { keyword, call, ident, block } => {
            // igore function call.

            // ignore the block.
            match *block {
                ASTNode::Block(blk) => {
                    for stmt in blk {
                        pretty_print(stmt, output, indent, env);
                    }
                },
                _ => {

                }
            } 
        },

    }
    String::new()
}

