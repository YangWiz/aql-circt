#[derive(Debug)]
#[derive(PartialEq)]
pub enum BinVerb {
    Plus,
    Minus,
    Times,
    Divide,
    And,
    Or,
    Xor,
    SmallerThan,
    LargerThan,
    SmallerOrEqual,
    LargerOrEqual,
    LeftShift,
    RightShift,
    Equal,
    NotEqual
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum UniVerb {
    Not,
    Tiled,
    Minus,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Label {
    ResultRewrite,
    InstSource,
    Commit
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum DSLKeyword {
    Transition,
    Reset,
    Complete
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Statement{
    LabeledStatement {
        label: Label,
        stmt: Box<Statement>,
    },

    DSLTransition {
        label: DSLKeyword,
        stmt: Box<Statement>
    },

    VariableDeclaration {
        t_ident: Box<ASTNode>,
        expr: Box<ASTNode>,
    },

    Assignment {
        name: Box<ASTNode>,
        expr: Box<ASTNode>,
    }
}

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Integer(i32),

    Decimal(f64),

    Str(String),

    Ident(String),

    ConstVal(Box<ASTNode>),

    QualifiedName {
        names: Vec<ASTNode>, // list of ident (name)
    },

    Declaration(Box<ASTNode>),

    Transition {
        action: String,
        ident: Box<ASTNode>,
    },

    StructureDelcaration {
        s_type: String,
        name: String,
        statement: Box<ASTNode>
    },
    
    Stmt(Statement),

    InternalFuncDecl(Box<ASTNode>),

    CatchBlock {
        keyword: String, 
        qualified_name: Box<ASTNode>, // function call, etc.
        idents: Vec<ASTNode>, // arguments
        block: Box<ASTNode> // statements
    },

    Block(Vec<ASTNode>),

    Expr(Expr),

    Listen {
        block: Box<ASTNode>,
        catch_block: Box<ASTNode>,
    },

    Call {
        qualified_name: Box<ASTNode>,
        list: Box<ASTNode>
    },

    ExprList(Vec<ASTNode>),

    None,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Expr {
    UnuaryOp {
        verb: UniVerb,
        term: Box<ASTNode> // dsl_term
    },

    BinOp {
        verb: BinVerb,
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>
    },

    List(Vec<Expr>),

    DSLTerm
}
