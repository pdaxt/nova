//! Intermediate Representation for Nova
//!
//! A low-level representation suitable for code generation.
//!
//! # TODO
//!
//! This module is a work in progress. Contributors welcome!
//!
//! See: https://github.com/nova-lang/nova/issues/5

use crate::types::{TypedProgram, TypedItem, TypedFunction, TypedBlock, TypedStmt, TypedExpr, TypedExprKind, TypeInfo};
use crate::ast::{BinOp, UnaryOp, Literal};

/// An IR module (corresponds to a program)
#[derive(Debug)]
pub struct Module {
    pub functions: Vec<Function>,
}

/// An IR function
#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, IrType)>,
    pub return_type: IrType,
    pub blocks: Vec<BasicBlock>,
}

/// A basic block (sequence of instructions ending in a terminator)
#[derive(Debug)]
pub struct BasicBlock {
    pub id: BlockId,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

/// Block identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub usize);

/// Value identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub usize);

/// An IR instruction
#[derive(Debug)]
pub struct Instruction {
    pub result: ValueId,
    pub kind: InstructionKind,
}

/// Instruction kinds
#[derive(Debug)]
pub enum InstructionKind {
    // Constants
    ConstInt(i64),
    ConstFloat(f64),
    ConstBool(bool),
    ConstString(String),

    // Binary operations
    Add(ValueId, ValueId),
    Sub(ValueId, ValueId),
    Mul(ValueId, ValueId),
    Div(ValueId, ValueId),
    Rem(ValueId, ValueId),

    // Comparisons
    Eq(ValueId, ValueId),
    Ne(ValueId, ValueId),
    Lt(ValueId, ValueId),
    Le(ValueId, ValueId),
    Gt(ValueId, ValueId),
    Ge(ValueId, ValueId),

    // Logical
    And(ValueId, ValueId),
    Or(ValueId, ValueId),
    Not(ValueId),

    // Bitwise
    BitAnd(ValueId, ValueId),
    BitOr(ValueId, ValueId),
    BitXor(ValueId, ValueId),
    Shl(ValueId, ValueId),
    Shr(ValueId, ValueId),

    // Unary
    Neg(ValueId),

    // Memory
    Alloca(IrType),
    Load(ValueId),
    Store(ValueId, ValueId),

    // Function calls
    Call(String, Vec<ValueId>),

    // Phi nodes (for SSA)
    Phi(Vec<(BlockId, ValueId)>),

    // Get parameter
    GetParam(usize),
}

/// Block terminator
#[derive(Debug)]
pub enum Terminator {
    Return(Option<ValueId>),
    Branch(BlockId),
    CondBranch(ValueId, BlockId, BlockId),
    Unreachable,
}

/// IR type representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrType {
    I32,
    I64,
    F32,
    F64,
    Bool,
    Ptr(Box<IrType>),
    Void,
}

/// Lower typed AST to IR
pub fn lower(program: &TypedProgram) -> Module {
    let mut lowerer = Lowerer::new();
    lowerer.lower_program(program)
}

/// The lowering state
struct Lowerer {
    next_value: usize,
    next_block: usize,
    current_block: Vec<Instruction>,
    blocks: Vec<BasicBlock>,
    locals: Vec<(String, ValueId)>,
}

impl Lowerer {
    fn new() -> Self {
        Self {
            next_value: 0,
            next_block: 0,
            current_block: Vec::new(),
            blocks: Vec::new(),
            locals: Vec::new(),
        }
    }

    fn fresh_value(&mut self) -> ValueId {
        let id = ValueId(self.next_value);
        self.next_value += 1;
        id
    }

    fn fresh_block(&mut self) -> BlockId {
        let id = BlockId(self.next_block);
        self.next_block += 1;
        id
    }

    fn emit(&mut self, kind: InstructionKind) -> ValueId {
        let result = self.fresh_value();
        self.current_block.push(Instruction { result, kind });
        result
    }

    fn finish_block(&mut self, terminator: Terminator) -> BlockId {
        let id = self.fresh_block();
        let block = BasicBlock {
            id,
            instructions: std::mem::take(&mut self.current_block),
            terminator,
        };
        self.blocks.push(block);
        id
    }

    fn lower_program(&mut self, program: &TypedProgram) -> Module {
        let mut functions = Vec::new();

        for item in &program.items {
            match item {
                TypedItem::Function(f) => {
                    functions.push(self.lower_function(f));
                }
            }
        }

        Module { functions }
    }

    fn lower_function(&mut self, f: &TypedFunction) -> Function {
        // Reset state
        self.blocks.clear();
        self.current_block.clear();
        self.locals.clear();
        self.next_value = 0;
        self.next_block = 0;

        // Add parameters to locals
        for (i, (name, ty)) in f.params.iter().enumerate() {
            let value = self.emit(InstructionKind::GetParam(i));
            self.locals.push((name.clone(), value));
        }

        // Lower body
        let result = self.lower_block(&f.body);

        // Finish with return
        self.finish_block(Terminator::Return(result));

        Function {
            name: f.name.clone(),
            params: f.params.iter().map(|(n, t)| (n.clone(), self.lower_type(t))).collect(),
            return_type: self.lower_type(&f.return_type),
            blocks: std::mem::take(&mut self.blocks),
        }
    }

    fn lower_block(&mut self, block: &TypedBlock) -> Option<ValueId> {
        let mut last_value = None;

        for stmt in &block.stmts {
            last_value = self.lower_stmt(stmt);
        }

        last_value
    }

    fn lower_stmt(&mut self, stmt: &TypedStmt) -> Option<ValueId> {
        match stmt {
            TypedStmt::Let { name, ty, value } => {
                if let Some(expr) = value {
                    let v = self.lower_expr(expr);
                    self.locals.push((name.clone(), v));
                }
                None
            }
            TypedStmt::Expr(expr) => Some(self.lower_expr(expr)),
        }
    }

    fn lower_expr(&mut self, expr: &TypedExpr) -> ValueId {
        match &expr.kind {
            TypedExprKind::Literal(lit) => match lit {
                Literal::Int(n) => self.emit(InstructionKind::ConstInt(*n)),
                Literal::Float(n) => self.emit(InstructionKind::ConstFloat(*n)),
                Literal::Bool(b) => self.emit(InstructionKind::ConstBool(*b)),
                Literal::String(s) => self.emit(InstructionKind::ConstString(s.clone())),
                Literal::Char(c) => self.emit(InstructionKind::ConstInt(*c as i64)),
            },
            TypedExprKind::Variable(name) => {
                self.locals
                    .iter()
                    .rev()
                    .find(|(n, _)| n == name)
                    .map(|(_, v)| *v)
                    .unwrap_or_else(|| self.emit(InstructionKind::ConstInt(0)))
            }
            TypedExprKind::Binary(left, op, right) => {
                let l = self.lower_expr(left);
                let r = self.lower_expr(right);
                let kind = match op {
                    BinOp::Add => InstructionKind::Add(l, r),
                    BinOp::Sub => InstructionKind::Sub(l, r),
                    BinOp::Mul => InstructionKind::Mul(l, r),
                    BinOp::Div => InstructionKind::Div(l, r),
                    BinOp::Rem => InstructionKind::Rem(l, r),
                    BinOp::Eq => InstructionKind::Eq(l, r),
                    BinOp::Ne => InstructionKind::Ne(l, r),
                    BinOp::Lt => InstructionKind::Lt(l, r),
                    BinOp::Le => InstructionKind::Le(l, r),
                    BinOp::Gt => InstructionKind::Gt(l, r),
                    BinOp::Ge => InstructionKind::Ge(l, r),
                    BinOp::And => InstructionKind::And(l, r),
                    BinOp::Or => InstructionKind::Or(l, r),
                    BinOp::BitAnd => InstructionKind::BitAnd(l, r),
                    BinOp::BitOr => InstructionKind::BitOr(l, r),
                    BinOp::BitXor => InstructionKind::BitXor(l, r),
                    BinOp::Shl => InstructionKind::Shl(l, r),
                    BinOp::Shr => InstructionKind::Shr(l, r),
                    BinOp::Assign => {
                        // TODO: Proper assignment
                        InstructionKind::ConstInt(0)
                    }
                };
                self.emit(kind)
            }
            TypedExprKind::Unary(op, inner) => {
                let v = self.lower_expr(inner);
                let kind = match op {
                    UnaryOp::Neg => InstructionKind::Neg(v),
                    UnaryOp::Not => InstructionKind::Not(v),
                    UnaryOp::BitNot => InstructionKind::Not(v), // TODO: Different instruction
                };
                self.emit(kind)
            }
            TypedExprKind::Call(func, args) => {
                let name = match &func.kind {
                    TypedExprKind::Variable(n) => n.clone(),
                    _ => "unknown".to_string(),
                };
                let arg_values: Vec<_> = args.iter().map(|a| self.lower_expr(a)).collect();
                self.emit(InstructionKind::Call(name, arg_values))
            }
            TypedExprKind::If(cond, then_block, else_expr) => {
                // TODO: Proper control flow
                self.lower_block(then_block).unwrap_or_else(|| {
                    self.emit(InstructionKind::ConstInt(0))
                })
            }
            TypedExprKind::Block(block) => {
                self.lower_block(block).unwrap_or_else(|| {
                    self.emit(InstructionKind::ConstInt(0))
                })
            }
            TypedExprKind::Return(value) => {
                if let Some(v) = value {
                    self.lower_expr(v)
                } else {
                    self.emit(InstructionKind::ConstInt(0))
                }
            }
        }
    }

    fn lower_type(&self, ty: &TypeInfo) -> IrType {
        match ty {
            TypeInfo::Int => IrType::I64,
            TypeInfo::Float => IrType::F64,
            TypeInfo::Bool => IrType::Bool,
            TypeInfo::Unit | TypeInfo::Never => IrType::Void,
            _ => IrType::I64, // Default
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use crate::parser::parse;
    use crate::types::check;

    #[test]
    fn test_lower_simple() {
        let tokens = lex("fn main() { return 42; }").unwrap();
        let ast = parse(tokens).unwrap();
        let typed = check(&ast).unwrap();
        let ir = lower(&typed);
        assert_eq!(ir.functions.len(), 1);
    }
}
