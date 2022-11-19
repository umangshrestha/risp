use crate::{Expr, Error, Stmt};

pub trait Visitor {
    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<(), Error>;
    fn visit_print_stmt(&mut self) -> Result<(), Error>;
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
