//! Type checker for Nova
//!
//! Performs type inference and checking on the AST.
//!
//! # TODO
//!
//! This module is a work in progress. Contributors welcome!
//!
//! See: https://github.com/nova-lang/nova/issues/4

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::ast::*;
use crate::error::NovaError;

/// A typed version of the program
#[derive(Debug)]
pub struct TypedProgram {
    pub items: Vec<TypedItem>,
}

/// A typed item
#[derive(Debug)]
pub enum TypedItem {
    Function(TypedFunction),
}

/// A typed function
#[derive(Debug)]
pub struct TypedFunction {
    pub name: String,
    pub params: Vec<(String, TypeInfo)>,
    pub return_type: TypeInfo,
    pub body: TypedBlock,
}

/// A typed block
#[derive(Debug)]
pub struct TypedBlock {
    pub stmts: Vec<TypedStmt>,
    pub ty: TypeInfo,
}

/// A typed statement
#[derive(Debug)]
pub enum TypedStmt {
    Let {
        name: String,
        ty: TypeInfo,
        value: Option<TypedExpr>,
    },
    Expr(TypedExpr),
}

/// A typed expression
#[derive(Debug)]
pub struct TypedExpr {
    pub kind: TypedExprKind,
    pub ty: TypeInfo,
}

/// Typed expression kinds
#[derive(Debug)]
pub enum TypedExprKind {
    Literal(Literal),
    Variable(String),
    Binary(Box<TypedExpr>, BinOp, Box<TypedExpr>),
    Unary(UnaryOp, Box<TypedExpr>),
    Call(Box<TypedExpr>, Vec<TypedExpr>),
    If(Box<TypedExpr>, TypedBlock, Option<Box<TypedExpr>>),
    Block(TypedBlock),
    Return(Option<Box<TypedExpr>>),
}

/// Type information
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeInfo {
    // Primitives
    Int,
    Float,
    Bool,
    String,
    Char,
    Unit,
    Never,

    // Compound
    Array(Box<TypeInfo>, usize),
    Tuple(Vec<TypeInfo>),
    Function(Vec<TypeInfo>, Box<TypeInfo>),
    Reference(bool, Box<TypeInfo>), // mutable?

    // Named types
    Named(String),

    // Generic/unknown
    Variable(usize), // Type variable for inference
    Unknown,
}

/// Type check a program
pub fn check(program: &Program) -> Result<TypedProgram, NovaError> {
    let mut checker = TypeChecker::new();
    checker.check_program(program)
}

/// The type checker state
struct TypeChecker {
    /// Type environment: name -> type
    env: Vec<(String, TypeInfo)>,
    /// Current function's return type
    return_type: Option<TypeInfo>,
    /// Next type variable ID
    next_var: usize,
}

impl TypeChecker {
    fn new() -> Self {
        Self {
            env: Vec::new(),
            return_type: None,
            next_var: 0,
        }
    }

    /// Check a complete program
    fn check_program(&mut self, program: &Program) -> Result<TypedProgram, NovaError> {
        let mut items = Vec::new();

        for item in &program.items {
            // TODO: Handle other items (struct, enum, etc.)
            if let Item::Function(f) = item {
                items.push(TypedItem::Function(self.check_function(f)?));
            }
        }

        Ok(TypedProgram { items })
    }

    /// Check a function
    fn check_function(&mut self, f: &Function) -> Result<TypedFunction, NovaError> {
        // Parse parameter types
        let mut params = Vec::new();
        for param in &f.params {
            let ty = self.resolve_type(&param.ty)?;
            let name = self.pattern_name(&param.pattern);
            self.env.push((name.clone(), ty.clone()));
            params.push((name, ty));
        }

        // Parse return type
        let return_type = if let Some(ref ty) = f.return_type {
            self.resolve_type(ty)?
        } else {
            TypeInfo::Unit
        };
        self.return_type = Some(return_type.clone());

        // Check body
        let body = self.check_block(&f.body)?;

        // Verify return type
        // TODO: More sophisticated type unification

        // Clean up environment
        for _ in &f.params {
            self.env.pop();
        }
        self.return_type = None;

        Ok(TypedFunction {
            name: f.name.name.clone(),
            params,
            return_type,
            body,
        })
    }

    /// Check a block
    fn check_block(&mut self, block: &Block) -> Result<TypedBlock, NovaError> {
        let mut stmts = Vec::new();
        let mut last_ty = TypeInfo::Unit;

        for stmt in &block.stmts {
            let (typed_stmt, ty) = self.check_stmt(stmt)?;
            last_ty = ty;
            stmts.push(typed_stmt);
        }

        Ok(TypedBlock { stmts, ty: last_ty })
    }

    /// Check a statement
    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(TypedStmt, TypeInfo), NovaError> {
        match stmt {
            Stmt::Let(let_stmt) => {
                let name = self.pattern_name(&let_stmt.pattern);

                let (value, inferred_ty) = if let Some(ref expr) = let_stmt.value {
                    let typed_expr = self.check_expr(expr)?;
                    let ty = typed_expr.ty.clone();
                    (Some(typed_expr), ty)
                } else {
                    (None, TypeInfo::Unknown)
                };

                let ty = if let Some(ref explicit_ty) = let_stmt.ty {
                    self.resolve_type(explicit_ty)?
                } else {
                    inferred_ty
                };

                self.env.push((name.clone(), ty.clone()));

                Ok((TypedStmt::Let { name, ty, value }, TypeInfo::Unit))
            }
            Stmt::Expr(expr_stmt) => {
                let typed_expr = self.check_expr(&expr_stmt.expr)?;
                let ty = if expr_stmt.has_semi {
                    TypeInfo::Unit
                } else {
                    typed_expr.ty.clone()
                };
                Ok((TypedStmt::Expr(typed_expr), ty))
            }
            Stmt::Item(_) => {
                // TODO: Handle nested items
                Ok((
                    TypedStmt::Expr(TypedExpr {
                        kind: TypedExprKind::Literal(Literal::Bool(true)),
                        ty: TypeInfo::Unit,
                    }),
                    TypeInfo::Unit,
                ))
            }
        }
    }

    /// Check an expression
    fn check_expr(&mut self, expr: &Expr) -> Result<TypedExpr, NovaError> {
        match &expr.kind {
            ExprKind::Literal(lit) => {
                let ty = match lit {
                    Literal::Int(_) => TypeInfo::Int,
                    Literal::Float(_) => TypeInfo::Float,
                    Literal::String(_) => TypeInfo::String,
                    Literal::Bool(_) => TypeInfo::Bool,
                    Literal::Char(_) => TypeInfo::Char,
                };
                Ok(TypedExpr {
                    kind: TypedExprKind::Literal(lit.clone()),
                    ty,
                })
            }
            ExprKind::Path(path) => {
                let name = &path.segments[0].ident.name;
                let ty = self
                    .env
                    .iter()
                    .rev()
                    .find(|(n, _)| n == name)
                    .map(|(_, t)| t.clone())
                    .unwrap_or(TypeInfo::Unknown);
                Ok(TypedExpr {
                    kind: TypedExprKind::Variable(name.clone()),
                    ty,
                })
            }
            ExprKind::Binary(left, op, right) => {
                let left_typed = self.check_expr(left)?;
                let right_typed = self.check_expr(right)?;

                let ty = self.binary_result_type(&left_typed.ty, *op, &right_typed.ty)?;

                Ok(TypedExpr {
                    kind: TypedExprKind::Binary(Box::new(left_typed), *op, Box::new(right_typed)),
                    ty,
                })
            }
            ExprKind::Unary(op, inner) => {
                let inner_typed = self.check_expr(inner)?;
                let ty = inner_typed.ty.clone();
                Ok(TypedExpr {
                    kind: TypedExprKind::Unary(*op, Box::new(inner_typed)),
                    ty,
                })
            }
            ExprKind::Call(func, args) => {
                let func_typed = self.check_expr(func)?;
                let mut args_typed = Vec::new();
                for arg in args {
                    args_typed.push(self.check_expr(arg)?);
                }

                let ty = match &func_typed.ty {
                    TypeInfo::Function(_, ret) => (**ret).clone(),
                    _ => TypeInfo::Unknown,
                };

                Ok(TypedExpr {
                    kind: TypedExprKind::Call(Box::new(func_typed), args_typed),
                    ty,
                })
            }
            ExprKind::If(cond, then_block, else_expr) => {
                let cond_typed = self.check_expr(cond)?;
                let then_typed = self.check_block(then_block)?;
                let else_typed = if let Some(e) = else_expr {
                    Some(Box::new(self.check_expr(e)?))
                } else {
                    None
                };

                let ty = then_typed.ty.clone();

                Ok(TypedExpr {
                    kind: TypedExprKind::If(Box::new(cond_typed), then_typed, else_typed),
                    ty,
                })
            }
            ExprKind::Block(block) => {
                let typed_block = self.check_block(block)?;
                let ty = typed_block.ty.clone();
                Ok(TypedExpr {
                    kind: TypedExprKind::Block(typed_block),
                    ty,
                })
            }
            ExprKind::Return(value) => {
                let typed_value = if let Some(v) = value {
                    Some(Box::new(self.check_expr(v)?))
                } else {
                    None
                };
                Ok(TypedExpr {
                    kind: TypedExprKind::Return(typed_value),
                    ty: TypeInfo::Never,
                })
            }
            // TODO: Implement remaining expression types
            _ => Ok(TypedExpr {
                kind: TypedExprKind::Literal(Literal::Bool(true)),
                ty: TypeInfo::Unknown,
            }),
        }
    }

    /// Resolve a type annotation to a TypeInfo
    fn resolve_type(&self, ty: &Type) -> Result<TypeInfo, NovaError> {
        match &ty.kind {
            TypeKind::Path(path) => {
                let name = &path.segments[0].ident.name;
                match name.as_str() {
                    "i8" | "i16" | "i32" | "i64" | "i128" => Ok(TypeInfo::Int),
                    "u8" | "u16" | "u32" | "u64" | "u128" => Ok(TypeInfo::Int),
                    "f32" | "f64" => Ok(TypeInfo::Float),
                    "bool" => Ok(TypeInfo::Bool),
                    "String" | "str" => Ok(TypeInfo::String),
                    "char" => Ok(TypeInfo::Char),
                    "()" => Ok(TypeInfo::Unit),
                    "!" => Ok(TypeInfo::Never),
                    _ => Ok(TypeInfo::Named(name.clone())),
                }
            }
            TypeKind::Tuple(types) => {
                let mut resolved = Vec::new();
                for t in types {
                    resolved.push(self.resolve_type(t)?);
                }
                Ok(TypeInfo::Tuple(resolved))
            }
            TypeKind::Reference(mutable, inner) => Ok(TypeInfo::Reference(
                *mutable,
                Box::new(self.resolve_type(inner)?),
            )),
            TypeKind::Never => Ok(TypeInfo::Never),
            TypeKind::Infer => Ok(TypeInfo::Unknown),
            _ => Ok(TypeInfo::Unknown),
        }
    }

    /// Get the result type of a binary operation
    fn binary_result_type(
        &self,
        left: &TypeInfo,
        op: BinOp,
        _right: &TypeInfo,
    ) -> Result<TypeInfo, NovaError> {
        // Simplified type rules
        match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Rem => {
                // Arithmetic: both operands should be numbers
                Ok(left.clone())
            }
            BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                // Comparison: result is bool
                Ok(TypeInfo::Bool)
            }
            BinOp::And | BinOp::Or => {
                // Logical: result is bool
                Ok(TypeInfo::Bool)
            }
            BinOp::Assign => {
                // Assignment: result is unit
                Ok(TypeInfo::Unit)
            }
            _ => Ok(left.clone()),
        }
    }

    /// Get the name from a pattern
    fn pattern_name(&self, pattern: &Pattern) -> String {
        match &pattern.kind {
            PatternKind::Ident(ident, _) => ident.name.clone(),
            _ => "_".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use crate::parser::parse;

    #[test]
    fn test_typecheck_simple() {
        let source = "fn main() { let x: i32 = 42; }";
        let tokens = lex(source).unwrap();
        let ast = parse(source, tokens).unwrap();
        let typed = check(&ast).unwrap();
        assert_eq!(typed.items.len(), 1);
    }
}
