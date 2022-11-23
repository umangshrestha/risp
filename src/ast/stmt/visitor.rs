use crate::{ErrorInfo, Expr, Span, Stmt};

pub trait Visitor {
    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<(), ErrorInfo>;
    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), ErrorInfo>;
    fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> Result<(), ErrorInfo>;
    fn visit_function_stmt(
        &mut self,
        name: &String,
        params: &Vec<String>,
        body: &Vec<Stmt>,
        span: &Span,
    ) -> Result<(), ErrorInfo>;
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        truthy: &Box<Stmt>,
        falsy: &Option<Box<Stmt>>,
    ) -> Result<(), ErrorInfo>;
    fn visit_let_stmt(
        &mut self,
        name: &String,
        value: &Option<Expr>,
        is_const: bool,
        span: &Span,
    ) -> Result<(), ErrorInfo>;
    fn visit_return_stmt(&mut self, value: &Option<Expr>, span: &Span) -> Result<(), ErrorInfo>;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Box<Stmt>) -> Result<(), ErrorInfo>;
    fn visit_class_stmt(
        &mut self,
        name: &String,
        super_class: &Option<String>,
        methods: &Vec<Stmt>,
        span: &Span,
    ) -> Result<(), ErrorInfo>;
    fn visit_break_stmt(&mut self, span: &Span) -> Result<(), ErrorInfo>;
    fn visit_continue_stmt(&mut self, span: &Span) -> Result<(), ErrorInfo>;
}
