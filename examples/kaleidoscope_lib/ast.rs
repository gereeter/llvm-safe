pub enum Expr {
    Number(f64),
    Variable(String),
    BinaryOp(char, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>)
}

pub struct Prototype {
    pub name: String,
    pub args: Vec<String>
}

pub struct Function {
    pub proto: Prototype,
    pub body: Expr
}
