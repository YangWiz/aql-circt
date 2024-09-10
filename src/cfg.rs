use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::ASTNode;

#[derive(Debug)]
#[derive(PartialEq, Clone)]
pub enum Statement{
    // Those are instructions with no control transfer.
    DSLTransition(Box<ASTNode>),

    VariableDeclaration(Box<ASTNode>),

    Assignment(Box<ASTNode>),

    // LabeledStatement(Box<ASTNode>),
    // Conditional(Box<ASTNode>),
    // AwaitBlock(Box<ASTNode>),
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct Scope {
    pub label: Structure,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Inst {
    Stmt(ASTNode),
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum Structure {
    ControllerEntry,
    Controller,
    StateQueue,
    State,
    ControllerControlFlow,
    None
}

#[derive(Debug, Clone)]
pub struct CFG {
    pub scope: Scope,
    pub insts: Vec<Inst>, // last inst as the terminator.
    pub next: Vec<Scope>
}

#[derive(Debug)]
pub struct CFGAccessor {
    pub fsm_name: String,
    pub entry: String,
    pub map: HashMap<Scope, Rc<CFG>>,
    pub cfgs: Vec<Rc<CFG>>
}

impl CFGAccessor {
    pub fn new() -> Self {
        Self {
            fsm_name: String::new(),
            entry: String::new(),
            map: HashMap::new(),
            cfgs: vec![]
        }
    }

    pub fn get_cfg(&self, scope: &Scope) -> Rc<CFG> {
        Rc::clone(&self.map[scope])
    }

    pub fn get_cfg_structure(&self, key: Structure) -> Rc<CFG> {
        for cfg in &self.cfgs {
            if cfg.scope.is_structure(&key) {
                return Rc::clone(&cfg)
            }
        }
        panic!("The program didn't provide us the init state.");
    }

    pub fn get_cfg_name_by_structure(&self, key: Structure) -> String {
        for cfg in &self.cfgs {
            if cfg.scope.is_structure(&key) {
                return cfg.scope.name.clone();
            }
        }
        String::new()
    }

    pub fn insert_cfg(&mut self, scope: Scope, cfg: CFG) {
        let cfg_cell = Rc::new(cfg);
        self.map.insert(scope, Rc::clone(&cfg_cell));
        self.cfgs.push(Rc::clone(&cfg_cell));
    }
}

impl CFG {
    pub fn new(scope: Scope) -> Self {
        Self {
            scope,
            insts: vec![],
            next: vec![],
        }
    }

    pub fn insert_inst(&mut self, inst: Inst) {
        self.insts.push(inst);
    }
}

impl Scope {
    pub fn new() -> Self {
        Scope { label: Structure::None, name: String::new() }
    }

    pub fn from(label: Structure, name: String) -> Self {
        Scope { label, name }
    }

    pub fn is_structure(&self, strut: &Structure) -> bool {
        *strut == self.label
    }
}

fn convert_struct(s_type: &str, name: &str, node: ASTNode, cfgs: &mut CFGAccessor) {
    // node is the structure_declaration.
    let ret = CFGAccessor::new();

    let structure;
    if s_type == "controller_entry" {
        structure = Structure::ControllerEntry; 
    } else if s_type == "controller" {
        structure = Structure::Controller; 
    } else if s_type == "state_queue" {
        structure = Structure::StateQueue; 
    } else if s_type == "state" {
        structure = Structure::State; 
    } else if s_type == "controller_control_flow" {
        structure = Structure::ControllerControlFlow; 
    } else {
        structure = Structure::None;
    }
    let scope = Scope::from(structure.clone(), String::from(name));

    if let ASTNode::Block(blk) = node {
        let mut cfg = CFG::new(scope.clone());
        for stmt_raw in blk {
            // println!("{:?}", stmt_raw);

            /*
            if let ASTNode::Stmt(stmt) = stmt_raw.clone() {
                cfg.insert_inst(Inst::Stmt(scope, stmt));
            }
            */

            match stmt_raw.clone() {
                ASTNode::Assignment { name, expr } => {
                    let inst = Inst::Stmt(stmt_raw);
                    cfg.insert_inst(inst);
                },
                ASTNode::VariableDeclaration { typed_identifier, expr } => {
                    let inst = Inst::Stmt(stmt_raw);
                    cfg.insert_inst(inst);
                },
                _ => {
                    println!("Unimplementated stmt.");
                }
            }
        }
        cfgs.insert_cfg(scope, cfg)
    }
}

pub fn convert(node: ASTNode) -> CFGAccessor {
    let mut cfgs = CFGAccessor::new();

    let ret = match node {
        ASTNode::Top(decls) => {
            // search for the 
            for decl in decls {
                // should only be structure declaration or internal_func_decl.
                if let ASTNode::Declaration(structure) = decl {
                    match *structure {
                        ASTNode::StructureDelcaration { s_type, name, statement } => {
                            convert_struct(&s_type, &name, *statement, &mut cfgs); // statement can be block or instructions, in this case, it's block.
                        }
                        ASTNode::InternalFuncDecl(_) => {
                            todo!()
                        }
                        _ => {
                            todo!() // report errors.
                        }
                    }
                }
            }
        },
        ASTNode::Integer(_) => todo!(),
        ASTNode::Decimal(_) => todo!(),
        ASTNode::Str(_) => todo!(),
        ASTNode::Ident(_) => todo!(),
        ASTNode::ConstVal(_) => todo!(),
        ASTNode::QualifiedName { names } => todo!(),
        ASTNode::Declaration(_) => todo!(),
        ASTNode::Transition { action, ident } => todo!(),
        ASTNode::StructureDelcaration { s_type, name, statement } => todo!(),
        ASTNode::InternalFuncDecl(_) => todo!(),
        ASTNode::CatchBlock { keyword, qualified_name, idents, block } => todo!(),
        ASTNode::Block(_) => todo!(),
        ASTNode::Expr(_) => todo!(),
        ASTNode::Listen { block, catch_block } => todo!(),
        ASTNode::Call { qualified_name, list } => todo!(),
        ASTNode::ExprList(_) => todo!(),
        ASTNode::Await { keyword, call, when_block } => todo!(),
        ASTNode::When { keyword, call, ident, block } => todo!(),
        ASTNode::None => todo!(),
        ASTNode::TypedIdentifier { aql_type, variable } => todo!(),
        ASTNode::VariableDeclaration { typed_identifier, expr } => todo!(),
        ASTNode::Assignment { name, expr } => todo!(),
    };

    // find the controller entry.
    // iterate all the instructions until find init = <State>.
    // if not find, panic an error.

    let key = Structure::ControllerEntry;
    let ret = cfgs.get_cfg_structure(key.clone());
    cfgs.fsm_name = cfgs.get_cfg_name_by_structure(key);

    for inst in &ret.insts {
        let Inst::Stmt(stmt) = inst;
        if let ASTNode::Assignment { name, expr } = stmt {
            if "init_entry ".eq(name) {
                // expr must be Qualified name.
                if let ASTNode::QualifiedName { names } = *expr.clone() {
                    let state = &names[0];
                    if let ASTNode::Ident(entry) = state {
                        cfgs.entry = String::from(entry);
                    }
                } else {
                    panic!("Init entry should be a leagal expression.");
                }
            }
        }
    }

    println!("{}", cfgs.entry);

    cfgs
}