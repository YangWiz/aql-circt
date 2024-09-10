use std::{collections::HashMap, rc::Rc};

use crate::ASTNode;

#[derive(Debug, PartialEq, Clone)]
pub struct Transition {
    pub target: String, // State.
    pub guards: Option<Vec<Box<ASTNode>>>, // Conditional expr.
}

impl Transition {
    fn new(target: String) -> Self {
        Transition {
            target,
            guards: None
        }
    }

    fn insert_guard(&mut self, guard: Box<ASTNode>) {
        match &mut self.guards {
            Some(guards) => {
                guards.push(guard);
            },
            None => {
                let mut temp = vec![];
                temp.push(guard);
                self.guards = Some(temp);
            },
        }
    }
}

// We put it at the end of the CFG, since it's the only way to transfer the control (at this stage.)
#[derive(Debug, PartialEq, Clone)]
pub struct Transitions {
    pub trans: Vec<Transition>
}

impl Transitions {
    fn new() -> Self {
        Transitions {
            trans: vec![]
        }
    }

    fn insert(&mut self, transition: Transition) {
        self.trans.push(transition);
    }
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct Scope {
    pub label: Structure,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Inst {
    Stmt(ASTNode), // We don't have a strict type strictions.
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
pub struct State {
    pub scope: Scope,
    pub insts: Vec<Inst>,
    pub next: Transitions
}

#[derive(Debug)]
pub struct StateMachine {
    pub fsm_name: String,
    pub entry: String,
    pub map: HashMap<Scope, Rc<State>>,
    pub cfgs: Vec<Rc<State>>
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            fsm_name: String::new(),
            entry: String::new(),
            map: HashMap::new(),
            cfgs: vec![]
        }
    }

    pub fn get_cfg(&self, scope: &Scope) -> Rc<State> {
        Rc::clone(&self.map[scope])
    }

    pub fn get_cfg_structure(&self, key: Structure) -> Rc<State> {
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

    pub fn insert_cfg(&mut self, scope: Scope, cfg: State) {
        let cfg_cell = Rc::new(cfg);
        self.map.insert(scope, Rc::clone(&cfg_cell));
        self.cfgs.push(Rc::clone(&cfg_cell));
    }
}

impl State {
    pub fn new(scope: Scope) -> Self {
        Self {
            scope,
            insts: vec![],
            next: Transitions::new(),
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

fn convert_struct(s_type: &str, name: &str, node: ASTNode, cfgs: &mut StateMachine) {
    // node is the structure_declaration.
    let ret = StateMachine::new();

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
        let mut cfg = State::new(scope.clone());
        let mut transitions = Transitions::new();

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
                ASTNode::Transition { action, ident } => {
                    // Direct transition without any conditions.
                    if action == "transition" {
                        // ident should be the string.
                        if let ASTNode::Ident(target) = *ident {
                            let transition = Transition::new(target.clone());
                            transitions.insert(transition);
                        }
                    } else if action == "complete" {

                    } else if action == "reset" {

                    } else {
                        panic!("unknown action.");
                    }
                }
                _ => {
                    println!("Unimplementated stmt.");
                }
            }
        }
        
        cfg.next = transitions;
        cfgs.insert_cfg(scope, cfg)
    }
}

pub fn convert(node: ASTNode) -> StateMachine {
    let mut cfgs = StateMachine::new();

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
        _ => todo!()
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

    cfgs
}