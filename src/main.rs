mod ast;
mod parser;
mod utils;
mod cfg;

use ast::ASTNode;
use cfg::{StateMachine, Structure};

use crate::parser::parse;
use std::{env::var, fs};

fn main() {
    let file = fs::read_to_string("example.aql").unwrap();
    let ret = parse(&file);
    // let mut output = Vec::new();
    // println!("{:?}", &ret);

    // let mut env = HashMap::new();
    
    let test = cfg::convert(ret.clone());
    println!("{}", generate(test));
}

fn generate(cfgs: StateMachine) -> String {
    let cfg_vec = cfgs.cfgs;

    let mut fsm_machine = format!("fsm.machine @{}(%arg0: i1, %arg1: i1) attributes {{initialState = \"{}\"}}", cfgs.fsm_name, cfgs.entry);

    fsm_machine += " {\n";

    for cfg in cfg_vec.clone() {
        let c = cfg.as_ref();
        let insts = &c.insts;

        for inst in insts {
            match inst {
                cfg::Inst::Stmt(stmt) => {
                    fsm_machine += "\t";
                    fsm_machine += &generate_decl(stmt);
                    fsm_machine += "\n";
                },
            }
        }
    }

    for cfg in cfg_vec {
        // Transitions only exist in the states, should filter the other structures.
        if let Structure::State = cfg.scope.label {
            let state_name = format!("\tfsm.state @{} transitions {{\n", &cfg.scope.name);
            fsm_machine += &state_name;

            // print transition.
            for tran in &cfg.next.trans {
                // guard and actions are all optional.
                let transition = format!("\t\tfsm.transition @{}\n", tran.target);
                fsm_machine += &transition;
            }
            fsm_machine += "\t}\n\n";
        }
    }

    fsm_machine += "}\n";
    fsm_machine
}

fn generate_stmt(stmt: &ASTNode) -> String {
    let ret = match stmt {
        ASTNode::VariableDeclaration { typed_identifier, expr } => {
            generate_decl(stmt)
        },
        ASTNode::Assignment { name, expr } => { String::new() },
        ASTNode::Transition { action, ident } => todo!(),
        _ => {
            panic!("Grammar error for instructions.");
        }
    };

    ret
}

// These are all the initilzation process, so should add one indent.
fn generate_decl(decl: &ASTNode) -> String {
    let mut ret = String::new();
    let tbs = utils::ConversionTable::new();

    if let ASTNode::VariableDeclaration { typed_identifier, expr } = decl {
        if let ASTNode::TypedIdentifier { aql_type, variable } = *typed_identifier.clone() {
            let aql_type = tbs.convert(&aql_type);
                
            match expr {
                Some(val) => {
                    // println!("Expr: {:?}", *val);
                    // ret = format!("fsm.variable \"{}\" {{initValue = {} : {} }} : {}", variable, init_value, aql_type, aql_type);
                    if let ASTNode::ConstVal(mut val) = *val.clone() {
                        let aql_type = match aql_type {
                            utils::AQLType::Base(t) => { t },
                            utils::AQLType::Ordering => {
                                if val == "FIFO" {
                                    val =  String::from("0");
                                } else if variable == "Hash" {
                                    val = String::from("1");
                                } else {
                                    val = String::from("2");
                                }

                                String::from("i32")
                            },
                        };
                        ret = format!("%{} = fsm.variable \"{}\" {{initValue = {} : {} }} : {}", variable, variable, val, aql_type, aql_type);
                    }
                },
                None => {
                    let aql_type = match aql_type {
                        utils::AQLType::Base(t) => { t },
                        utils::AQLType::Ordering => { String::from("i32") },
                    };
                    ret = format!("%{} = fsm.variable \"{}\" {{initValue = 0 : {} }} : {}", variable, variable, aql_type, aql_type);
                },
            }
        } else {
            panic!("invalid grammar.")
        }
    };
    ret
}

/*
fn print_assign(assign: ASTNode) -> String {
    let mut ret = String::from("fsm.variable");
    if let ASTNode::Assignment {name, expr} = assign { 
        if let ASTNode::TypedIdentifier { aql_type, variable } = *epxr {
            ret = format!("fsm.variable \"{}\" {{initValue = 0 : {} }} : {}", variable, aql_type, aql_type);
        }
    } else {
        panic!("invalid grammar.")
    }

    ret
}
*/

/*
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
*/
