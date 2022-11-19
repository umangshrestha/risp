use crate::{Error, Expr};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expr {
        expr: Expr,
    },
    Print {
        expr: Expr,
    },
    Let {
        name: String,
        value: Option<Expr>,
        is_const: bool,
    },
    Block {
        stmt: Vec<Stmt>,
    },
    If {
        condition: Expr,
        truthy: Box<Stmt>,
        falsy: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Box<Stmt>,
    },
    Return {
        value: Option<Expr>,
    },
    Class {
        name: String,
        super_class: Option<String>,
        methods: Vec<Stmt>,
    },
    For {
        initializer: Option<Box<Stmt>>,
        condition: Option<Box<Expr>>,
        increment: Option<Box<Expr>>,
        body: Box<Stmt>,
    },
    Break,
    Continue,
}

pub trait Visitor {
    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<(), Error>;
    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), Error>;
    fn visit_let_stmt(
        &mut self,
        name: &String,
        value: &Option<Expr>,
        is_const: bool,
    ) -> Result<(), Error>;
    fn visit_block_stmt(&mut self, stmt: &Vec<Stmt>) -> Result<(), Error>;
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        truthy: &Stmt,
        falsy: &Option<Box<Stmt>>,
    ) -> Result<(), Error>;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), Error>;
    fn visit_function_stmt(
        &mut self,
        name: &String,
        params: &Vec<String>,
        body: &Stmt,
    ) -> Result<(), Error>;
    fn visit_return_stmt(&mut self, value: &Option<Expr>) -> Result<(), Error>;
    fn visit_class_stmt(
        &mut self,
        name: &String,
        super_class: &Option<String>,
        methods: &Vec<Stmt>,
    ) -> Result<(), Error>;
    fn visit_for_stmt(
        &mut self,
        initializer: &Option<Box<Stmt>>,
        condition: &Option<Box<Expr>>,
        increment: &Option<Box<Expr>>,
        body: &Stmt,
    ) -> Result<(), Error>;
    fn visit_break_stmt(&mut self) -> Result<(), Error>;
    fn visit_continue_stmt(&mut self) -> Result<(), Error>;
}

impl Stmt {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), Error> {
        match self {
            Stmt::Expr { expr } => visitor.visit_expr_stmt(expr),
            Stmt::Print { expr } => visitor.visit_print_stmt(expr),
            Stmt::Let {
                name,
                value,
                is_const,
            } => visitor.visit_let_stmt(name, value, *is_const),
            Stmt::Block { stmt } => visitor.visit_block_stmt(stmt),
            Stmt::If {
                condition,
                truthy,
                falsy,
            } => visitor.visit_if_stmt(condition, truthy, falsy),
            Stmt::While { condition, body } => visitor.visit_while_stmt(condition, body),
            Stmt::Function { name, params, body } => {
                visitor.visit_function_stmt(name, params, body)
            }
            Stmt::Return { value } => visitor.visit_return_stmt(value),
            Stmt::Class {
                name,
                super_class,
                methods,
            } => visitor.visit_class_stmt(name, super_class, methods),
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
            } => visitor.visit_for_stmt(initializer, condition, increment, body),
            Stmt::Break => visitor.visit_break_stmt(),
            Stmt::Continue => visitor.visit_continue_stmt(),
        }
    }
}
