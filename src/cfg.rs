use std::{cell::{Ref, RefCell}, collections::{HashMap, VecDeque}, hash::Hash, rc::Rc};

use crate::ASTNode;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Transition {
    pub target: String, // State.
    pub guards: Option<Vec<ASTNode>>, // Conditional expr.
}

#[derive(Debug, Clone)]
struct Vertex {
    transition_target: Option<String>,
    label: Uuid,
    condition: Option<ASTNode>,
    is_else: bool 
}

impl Vertex {
    fn comparator(label: Uuid) -> Self {
        Self {
            transition_target: None,
            label,
            condition: None,
            is_else: false,
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Transition {
    fn new(target: String) -> Self {
        Transition {
            target,
            guards: None
        }
    }

    fn insert_guard(&mut self, guard: ASTNode) {
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

#[derive(Debug, Clone)]
pub struct ControlFlow {
    // This is the control flow inside the State.
    // State itself is also a control flow but with a higher level (Transitions between graphs).
    pub label: Uuid,
    pub insts: Vec<Inst>,
    pub cond: Option<ASTNode>, // expr.
    pub lhs: Option<Rc<ControlFlow>>, // when if (true)
    pub rhs: Option<Rc<ControlFlow>>, // when if (false)
}

impl ControlFlow {
    fn new() -> Self {
        ControlFlow {
            label: Uuid::new_v4(),
            cond: None,
            insts: vec![],
            lhs: None,
            rhs: None,
        }
    }

    fn add_next_lhs(&mut self, cfg: Rc<ControlFlow>) {
        self.lhs = Some(cfg)
    }

    fn add_next_rhs(&mut self, cfg: Rc<ControlFlow>) {
        self.rhs = Some(cfg)
    }

    fn add_new_inst(&mut self, inst: Inst) {
        self.insts.push(inst)
    }

    fn add_cond(&mut self, cond: ASTNode) {
        self.cond = Some(cond); 
    }
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

fn collect_transitions(root: &ControlFlow, label2cfg: &HashMap<Uuid, Rc<ControlFlow>>, transition_labels: &Vec<Uuid>) -> Transitions {
    // let mut conditionals = vec![];
    // let mut actions = vec![];
    let mut transitions = Transitions::new();

    for target in transition_labels {
        let mut paths: Vec<VecDeque<Vertex>> = vec![];
        let mut path: VecDeque<Vertex> = VecDeque::new();
        dfs(&target, root, &mut paths, &mut path, None); // The first stmt is unconditional.

        // println!("paths: {:?}\n", paths);

        let mut conditonals = vec![];
        let mut actions = vec![];
        for path in paths {
            for vertex in path {
                // add condition along the way. todo(add else condition, negation)
                match vertex.condition {
                    Some(cond) => {
                        conditonals.push(cond);
                    },
                    None => {},
                }

                let control_flow = label2cfg.get(&vertex.label).unwrap();
                for inst in &control_flow.insts {
                    // We ignore the transition and after that.
                    let Inst::Stmt(stmt) = inst;
                    if let ASTNode::Transition { .. } = stmt {
                        break;
                    }

                    actions.push(stmt);
                }

                match vertex.transition_target {
                    Some(target) => {
                        let mut transition = Transition::new(target);
                        for guard in &conditonals {
                            transition.insert_guard(guard.clone());
                        }
                        transitions.insert(transition);
                    },
                    None => {},
                }
            }
        }

        // We need to hack the first one, since it's not in the label2cfg due to my implementations.
        // for mut path in paths {
        //     let parent_uuid = path.pop_front().unwrap();
        //     let parent = root;

        //     for inst in &parent.insts {
        //         
        //     }

        //     // for label in path {
        //     //     let current_cf = label2cfg.get(&label.lebel).unwrap();
        //     //     match &parent.lhs {
        //     //         Some(cf) => {

        //     //         },
        //     //         None => todo!(),
        //     //     }
        //     // }
        // }
    }

    transitions
}

fn dfs(target: &Uuid, node: &ControlFlow, paths: &mut Vec<VecDeque<Vertex>>, path: &mut VecDeque<Vertex>, cond: Option<ASTNode>) {
    // search transition, and record the actions and conditions through the traveral.
    // conds must be expr and actions must be statements.
    // if let ASTNode::Transition { action, ident } = root {
    //     // Reach the terminator of this path.
    // }
    // if let ASTNode::Conditional { expr, if_blk, else_blk } = root {
    //     conds.push(*expr.clone());
    // }

    // last inst is transition.
    let transition_target;
    match node.insts.last() {
        Some(stmt) => {
            let Inst::Stmt(transition) = stmt;
            if let ASTNode::Transition { action: _, ident } = transition {
                // We only care about transition as action. todo(implement other types of transitions)
                if let ASTNode::Ident(target) = *ident.clone() {
                    transition_target = Some(target);
                } else {
                    panic!("The last stmt is not the transition, invalid syntax.");
                }
            } else {
                panic!("Invalid Control Flow Gra_ph.");
            }
        },
        None => {
            transition_target = None;
        },
    };

    if node.label == *target {
        paths.push(path.clone());
    } else {
        match &node.lhs {
            Some(lhs) => {
                if !path.contains(&Vertex::comparator(lhs.label.clone())) {
                    path.push_back(Vertex { transition_target: transition_target.clone(), label: node.label, condition: cond.clone(), is_else: false });
                    dfs(target, &lhs, paths, path, node.cond.clone());
                }
            },
            None => (),
        }

        match &node.rhs {
            Some(rhs) => {
                if !path.contains(&Vertex::comparator(rhs.label.clone())) {
                    path.push_back(Vertex { transition_target, label: node.label, condition: cond, is_else: true });
                    dfs(target, &rhs, paths, path, node.cond.clone());
                }
            },
            None => (),
        }
    }

    path.pop_back();
}

// We use the control flow graph here to execute some analysis (DFS, BFS, etc).
fn get_cfg(blk: &ASTNode, transition_labels: &mut Vec<Uuid>, label2cfg: &mut HashMap<Uuid, Rc<ControlFlow>>) -> Rc::<ControlFlow> {
    // Get the structure_decl inner block.
    let mut cfg = ControlFlow::new();

    if let ASTNode::Block(blk) = blk {
        for stmt_raw in blk {
            match stmt_raw.clone() {
                ASTNode::Assignment { .. } => {
                    let inst = Inst::Stmt(stmt_raw.clone());
                    cfg.add_new_inst(inst.clone());
                },
                ASTNode::VariableDeclaration { .. } => {
                    let inst = Inst::Stmt(stmt_raw.clone());
                    cfg.add_new_inst(inst);
                },
                ASTNode::Transition { .. } => {
                    // Direct transition without any conditions.
                    let inst = Inst::Stmt(stmt_raw.clone());
                    cfg.add_new_inst(inst);
                    transition_labels.push(cfg.label);
                }
                ASTNode::Conditional { expr, if_blk, else_blk } => {
                    cfg.add_cond(*expr);
                    cfg.add_next_lhs(get_cfg(&*if_blk, transition_labels, label2cfg));

                    if let ASTNode::None = *else_blk {
                    } else {
                        cfg.add_next_rhs(get_cfg(&*else_blk, transition_labels, label2cfg));
                        break; // reaheability.
                    }

                    // reacheability: 
                    /*
                        The last two lines here is not reachable, we can use else statement to check it.
                        state iq_schedule_inst {
                        if (i) {
                            transition init_rob_entry; 
                        } else {
                            transition init_rob_entry; 
                        }
                        i32 write_value;
                        transition init_rob_entry;
                        }
                     */
                }
                _ => {
                    println!("Unimplementated stmt.");
                }
            }
        }
    }
    
    let cfg = Rc::new(cfg);
    label2cfg.insert(cfg.label, cfg.clone());

    cfg
}

fn convert_struct(s_type: &str, name: &str, node: ASTNode, cfgs: &mut StateMachine) {
    // node is the structure_declaration.

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
    
    if let Structure::State = structure {
        let mut transitions_labels = vec![];
        let mut label2cfg = HashMap::new();
        let mut cfg = get_cfg(&node, &mut transitions_labels, &mut label2cfg);
        println!("{:?}", transitions_labels);

        // println!("{:?}", label2cfg);
        // println!("{:?}", test);
        // println!("{:?}", test.lhs);
        // println!("{:?}", test.rhs);

        collect_transitions(&cfg, &label2cfg, &transitions_labels);
    }
    

    if let ASTNode::Block(blk) = node {
        let mut cfg = State::new(scope.clone());
        let mut transitions = Transitions::new();

        // we should carry these information when parsing (actions and conditions).
        let mut actions = vec![];
        // let mut conditions = vec![];

        for stmt_raw in blk {
            // println!("{:?}", stmt_raw);

            /*
            if let ASTNode::Stmt(stmt) = stmt_raw.clone() {
                cfg.insert_inst(Inst::Stmt(scope, stmt));
            }
            */

            match stmt_raw.clone() {
                ASTNode::Assignment { .. } => {
                    let inst = Inst::Stmt(stmt_raw);
                    cfg.insert_inst(inst.clone());
                    actions.push(inst);
                },
                ASTNode::VariableDeclaration { .. } => {
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
                ASTNode::Conditional { expr, if_blk, else_blk } => {
                    println!("if")
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
            if "init_entry".eq(name.trim()) {
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