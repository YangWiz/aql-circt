#[derive(Debug)]
#[derive(PartialEq, Clone)]
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
#[derive(PartialEq, Clone)]
pub enum UniVerb {
    Not,
    Tiled,
    Minus,
}

#[derive(Debug)]
#[derive(PartialEq, Clone)]
pub enum Label {
    ResultRewrite,
    InstSource,
    Commit
}

#[derive(Debug)]
#[derive(PartialEq, Clone)]
pub enum DSLKeyword {
    Transition,
    Reset,
    Complete
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Top(Vec<ASTNode>),

    TypedIdentifier {
        aql_type: String,
        variable: String,
    },

    Integer(i32),

    Decimal(f64),

    Str(String),

    Ident(String),

    ConstVal(Box<ASTNode>),

    QualifiedName {
        names: Vec<ASTNode>, // list of ident (name)
    },

    VariableDeclaration {
        typed_identifier: Box<ASTNode>, // type_identifier
        expr: Option<Box<ASTNode>>
    }, 

    Assignment {
        name: String,
        expr: Box<ASTNode>
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

    Await {
        keyword: String,
        call: Option<Box<ASTNode>>,
        when_block: Box<ASTNode>,
    },

    When {
        keyword: String,
        call: Box<ASTNode>,
        ident: Box<ASTNode>,
        block: Box<ASTNode> 
    },


    None,
}

#[derive(Debug)]
#[derive(PartialEq, Clone)]
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
