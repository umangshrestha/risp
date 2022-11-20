use crate::{visitor, ErrorInfo, Expr, Interpretor, Stmt};

impl visitor::Stmt for Interpretor {
    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), ErrorInfo> {
        let out = self.eval(expr)?;
        println!("{}", out);
        Ok(())
    }

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<(), ErrorInfo> {
        self.eval(expr)?;
        Ok(())
    }

    fn visit_let_stmt(
        &mut self,
        name: &String,
        value: &Option<Expr>,
        is_const: bool,
    ) -> Result<(), ErrorInfo>;
    fn visit_block_stmt(&mut self, stmt: &Vec<Stmt>) -> Result<(), ErrorInfo>;
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        truthy: &Stmt,
        falsy: &Option<Box<Stmt>>,
    ) -> Result<(), ErrorInfo>;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), ErrorInfo>;
    fn visit_function_stmt(
        &mut self,
        name: &String,
        params: &Vec<String>,
        body: &Stmt,
    ) -> Result<(), ErrorInfo>;
    fn visit_return_stmt(&mut self, value: &Option<Expr>) -> Result<(), ErrorInfo>;
    fn visit_class_stmt(
        &mut self,
        name: &String,
        super_class: &Option<String>,
        methods: &Vec<Stmt>,
    ) -> Result<(), ErrorInfo>;
    fn visit_for_stmt(
        &mut self,
        initializer: &Option<Box<Stmt>>,
        condition: &Option<Box<Expr>>,
        increment: &Option<Box<Expr>>,
        body: &Stmt,
    ) -> Result<(), ErrorInfo>;
    fn visit_break_stmt(&mut self) -> Result<(), ErrorInfo>;
    fn visit_continue_stmt(&mut self) -> Result<(), ErrorInfo>;
}
