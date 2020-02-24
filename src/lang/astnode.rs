#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Assign(bool, String, Box<AstNode>),
    FunctionCall(DefaultFunction, Box<AstNode>),
    Ident(String),
    Str(String),
    Strs(Vec<AstNode>),
    Number(f64),
    Calc(CalcOp, Box<AstNode>, Box<AstNode>),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum CalcOp {
    Plus,
    Minus,
    Times,
    Divide,
    Modulus,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DefaultFunction {
    Print,
}

impl AstNode {
    pub fn calc<L, R>(op: CalcOp, lhs: L, rhs: R) -> Self
    where
        L: Into<AstNode>,
        R: Into<AstNode>,
    {
        AstNode::Calc(op.into(), Box::new(lhs.into()), Box::new(rhs.into()))
    }
}