mod ast;
mod parser;

use ast::ASTNode;

use crate::parser::parse;
use std::{fs, os::linux::raw::stat};

fn main() {
    let file = fs::read_to_string("ReadyToIssue.aql").unwrap();
    let ret = parse(&file);
    println!("{:?}", ret);
}

fn pretty_print(tree: ASTNode, mut output: String, indent: u8) -> String {
    // Top level structure is the declaration.
    match tree {
        ASTNode::Integer(_) => todo!(),
        ASTNode::Decimal(_) => todo!(),
        ASTNode::Str(_) => todo!(),
        ASTNode::Ident(_) => todo!(),
        ASTNode::ConstVal(_) => todo!(),
        ASTNode::QualifiedName { names } => todo!(),
        ASTNode::Declaration(_) => todo!(),
        ASTNode::Transition { action, ident } => todo!(),
        ASTNode::StructureDelcaration { s_type, statement, name } => {
            output += &s_type;
            output += " ";
            output += &name;
            output += " ";
            pretty_print(*statement, output, indent + 1);
        },
        ASTNode::Stmt(_) => todo!(),
        ASTNode::InternalFuncDecl(_) => todo!(),
        ASTNode::CatchBlock { keyword, qualified_name, idents, stmts } => todo!(),
        ASTNode::Block(_) => {
            output += "{\n";
            output += "\n}";
        },
        ASTNode::Expr(_) => todo!(),
        ASTNode::Listen { block, catch_block } => todo!(),
        ASTNode::Call { qualified_name, list } => todo!(),
        ASTNode::ExprList(_) => todo!(),
        ASTNode::None => todo!(),

    }
    String::new()
}

