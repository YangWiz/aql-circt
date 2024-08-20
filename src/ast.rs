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

    pub enum UniVerb {
        Not,
        Tiled,
        Minus,
    }

    pub enum Label {
        ResultRewrite,
        InstSource,
        Commit
    }

    pub enum DSLKeyword {
        Transition,
        Reset,
        Complete
    }

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

        StructureDelcaration {
            s_type: String,
            statement: Box<ASTNode>
        },
        
        Stmt(Statement),

        InternalFuncDecl(Box<ASTNode>),

        CatchBlock {
            keyword: String, 
            qualified_name: Vec<ASTNode>, // function call, etc.
            idents: Vec<ASTNode>, // arguments
            stmts: Vec<ASTNode> // statements
        },

        Block(Vec<ASTNode>),

        Expr(Expr),

        Listen {
            keyword: String,
            block: Box<ASTNode>,
            catch_block: Box<ASTNode>,
        },

        None,
    }

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
