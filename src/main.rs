mod ast;
mod parser;
mod utils;

use ast::ASTNode;

use crate::parser::parse;
use std::fs;

fn main() {
    let file = fs::read_to_string("example.aql").unwrap();
    let ret = parse(&file);
    let mut output = Vec::new();
    println!("{:?}", &ret);
    pretty_print(ret, &mut output, 0);

    // for el in output {
    //     print!("{}", el);
    // }
}

fn pretty_print(tree: ASTNode, output: &mut Vec<String>, indent: u8) -> String {
    // Top level structure is the declaration.
    match tree {
        ASTNode::Top(nodes) => {
            for node in nodes {
                pretty_print(node, output, indent);
            }
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
            output.push(String::from("fsm.machine @LoadInst(%arg0: i1, %arg1: i1) -> (i8) attributes {initialState = \"ReadyToDispatch\"} {\n"));
            pretty_print(*decl, output, indent + 1);
            output.push(String::from("}\n"))
        },
        ASTNode::Transition { action, ident } => {
            output.push(String::from("\t".repeat(indent.into())));
            output.push(action + " ");
            pretty_print(*ident, output, indent);
            output.push(";\n".to_string());
        },
        ASTNode::StructureDelcaration { s_type, statement, name } => {
            output.push(String::from("\t".repeat(indent.into())));

            if s_type == String::from("state") {
                output.push(String::from("fsm.state") + " ");
                output.push(String::from("@") + name.as_str() + " output ");
            }
            if s_type == String::from("controller_entry") {
                 
            }

            pretty_print(*statement, output, indent);
        },
        ASTNode::Stmt(_) => todo!(),
        ASTNode::InternalFuncDecl(_) => todo!(),
        ASTNode::CatchBlock { keyword, qualified_name, idents, block } => {
            output.push(String::from("\t".repeat(indent.into())));
            output.push("handle ".to_string() + &keyword + " ");
            pretty_print(*qualified_name, output, indent);
            for ident in idents {
                pretty_print(ident, output, indent);
            }
            pretty_print(*block, output, indent);
        },
        ASTNode::Block(stmts) => {
            output.push(String::from("{\n"));

            for stmt in stmts {
                pretty_print(stmt, output, indent+1);
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
            pretty_print(*block, output, indent);
            output.push(String::from(" "));
            pretty_print(*catch_block, output, indent);
        },
        ASTNode::Call { qualified_name, list } => {
            output.push("\t".repeat(indent.into()));
            pretty_print(*qualified_name, output, indent);
            pretty_print(*list, output, indent);
            output.push(";\n".to_string());
        },
        ASTNode::ExprList(list) => {
            output.push("(".to_string());
            for expr in list {
                pretty_print(expr, output, indent);
                output.push(", ".to_string());
            }
            output.push(")".to_string());
        },
        ASTNode::None => {},
        ASTNode::Await { keyword, call, when_block } => todo!(),
        ASTNode::When { keyword, call, ident, block } => todo!(),

    }
    String::new()
}

