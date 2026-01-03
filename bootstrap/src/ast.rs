//! Abstract Syntax Tree definitions for Nova
//!
//! This module defines the AST nodes that the parser produces.
//!
//! NOTE: Many AST variants are defined but not yet used.
//! This is intentional - the full grammar is defined upfront.

#![allow(dead_code)]

use crate::token::Span;

/// A complete Nova program
#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
}

/// A top-level item
#[derive(Debug, Clone)]
pub enum Item {
    Function(Function),
    Struct(StructDef),
    Enum(EnumDef),
    Impl(ImplBlock),
    Trait(TraitDef),
    Use(UseStmt),
    TypeAlias(TypeAlias),
}

/// A function definition
#[derive(Debug, Clone)]
pub struct Function {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub where_clause: Option<WhereClause>,
    pub body: Block,
    pub span: Span,
}

/// A function parameter
#[derive(Debug, Clone)]
pub struct Param {
    pub pattern: Pattern,
    pub ty: Type,
    pub span: Span,
}

/// A struct definition
#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub fields: Vec<Field>,
    pub span: Span,
}

/// A struct field
#[derive(Debug, Clone)]
pub struct Field {
    pub name: Ident,
    pub ty: Type,
    pub span: Span,
}

/// An enum definition
#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub variants: Vec<Variant>,
    pub span: Span,
}

/// An enum variant
#[derive(Debug, Clone)]
pub struct Variant {
    pub name: Ident,
    pub fields: VariantFields,
    pub span: Span,
}

/// Enum variant fields
#[derive(Debug, Clone)]
pub enum VariantFields {
    Unit,
    Tuple(Vec<Type>),
    Struct(Vec<Field>),
}

/// An impl block
#[derive(Debug, Clone)]
pub struct ImplBlock {
    pub generics: Vec<GenericParam>,
    pub trait_: Option<Type>,
    pub self_type: Type,
    pub items: Vec<ImplItem>,
    pub span: Span,
}

/// An item in an impl block
#[derive(Debug, Clone)]
pub enum ImplItem {
    Function(Function),
}

/// A trait definition
#[derive(Debug, Clone)]
pub struct TraitDef {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub bounds: Vec<Type>,
    pub items: Vec<TraitItem>,
    pub span: Span,
}

/// An item in a trait definition
#[derive(Debug, Clone)]
pub enum TraitItem {
    Function(TraitFunction),
}

/// A function signature in a trait
#[derive(Debug, Clone)]
pub struct TraitFunction {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub default_body: Option<Block>,
    pub span: Span,
}

/// A use statement
#[derive(Debug, Clone)]
pub struct UseStmt {
    pub path: Path,
    pub span: Span,
}

/// A type alias
#[derive(Debug, Clone)]
pub struct TypeAlias {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub ty: Type,
    pub span: Span,
}

/// A generic parameter
#[derive(Debug, Clone)]
pub struct GenericParam {
    pub name: Ident,
    pub bounds: Vec<Type>,
    pub span: Span,
}

/// A where clause
#[derive(Debug, Clone)]
pub struct WhereClause {
    pub predicates: Vec<WherePredicate>,
    pub span: Span,
}

/// A predicate in a where clause
#[derive(Debug, Clone)]
pub struct WherePredicate {
    pub ty: Type,
    pub bounds: Vec<Type>,
    pub span: Span,
}

/// A block of statements
#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

/// A statement
#[derive(Debug, Clone)]
pub enum Stmt {
    Let(LetStmt),
    Expr(ExprStmt),
    Item(Item),
}

/// A let statement
#[derive(Debug, Clone)]
pub struct LetStmt {
    pub pattern: Pattern,
    pub ty: Option<Type>,
    pub value: Option<Expr>,
    pub span: Span,
}

/// An expression statement
#[derive(Debug, Clone)]
pub struct ExprStmt {
    pub expr: Expr,
    pub has_semi: bool,
    pub span: Span,
}

/// An expression
#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

/// Expression kinds
#[derive(Debug, Clone)]
pub enum ExprKind {
    // Literals
    Literal(Literal),

    // Variables
    Path(Path),

    // Operators
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),

    // Function call
    Call(Box<Expr>, Vec<Expr>),

    // Field access
    Field(Box<Expr>, Ident),

    // Index
    Index(Box<Expr>, Box<Expr>),

    // Struct literal
    StructLit(Path, Vec<FieldInit>),

    // Array literal
    Array(Vec<Expr>),

    // Tuple
    Tuple(Vec<Expr>),

    // Control flow
    If(Box<Expr>, Block, Option<Box<Expr>>),
    Match(Box<Expr>, Vec<MatchArm>),
    While(Box<Expr>, Block),
    For(Pattern, Box<Expr>, Block),
    Loop(Block),

    // Block
    Block(Block),

    // Closures
    Closure(Vec<Param>, Option<Type>, Box<Expr>),

    // Return/break/continue
    Return(Option<Box<Expr>>),
    Break(Option<Box<Expr>>),
    Continue,

    // Range
    Range(Option<Box<Expr>>, Option<Box<Expr>>, bool), // inclusive?

    // Reference
    Ref(bool, Box<Expr>), // mutable?
    Deref(Box<Expr>),

    // Await
    Await(Box<Expr>),

    // Try (?)
    Try(Box<Expr>),
}

/// A literal value
#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Char(char),
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    Rem,    // %
    And,    // &&
    Or,     // ||
    BitAnd, // &
    BitOr,  // |
    BitXor, // ^
    Shl,    // <<
    Shr,    // >>
    Eq,     // ==
    Ne,     // !=
    Lt,     // <
    Le,     // <=
    Gt,     // >
    Ge,     // >=
    Assign, // =
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,    // -
    Not,    // !
    BitNot, // ~
}

/// A field initializer in a struct literal
#[derive(Debug, Clone)]
pub struct FieldInit {
    pub name: Ident,
    pub value: Expr,
    pub span: Span,
}

/// A match arm
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Expr,
    pub span: Span,
}

/// A pattern
#[derive(Debug, Clone)]
pub struct Pattern {
    pub kind: PatternKind,
    pub span: Span,
}

/// Pattern kinds
#[derive(Debug, Clone)]
pub enum PatternKind {
    Wildcard,
    Ident(Ident, bool), // mutable?
    Literal(Literal),
    Tuple(Vec<Pattern>),
    Struct(Path, Vec<FieldPattern>),
    TupleStruct(Path, Vec<Pattern>),
    Or(Vec<Pattern>),
    Ref(bool, Box<Pattern>), // mutable?
    Range(Option<Box<Pattern>>, Option<Box<Pattern>>, bool),
}

/// A field in a struct pattern
#[derive(Debug, Clone)]
pub struct FieldPattern {
    pub name: Ident,
    pub pattern: Option<Pattern>,
    pub span: Span,
}

/// A type
#[derive(Debug, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Span,
}

/// Type kinds
#[derive(Debug, Clone)]
pub enum TypeKind {
    Path(Path),
    Tuple(Vec<Type>),
    Array(Box<Type>, Box<Expr>),
    Slice(Box<Type>),
    Reference(bool, Box<Type>), // mutable?
    Fn(Vec<Type>, Option<Box<Type>>),
    Never,
    Infer,
}

/// A path (like `std::collections::HashMap`)
#[derive(Debug, Clone)]
pub struct Path {
    pub segments: Vec<PathSegment>,
    pub span: Span,
}

/// A segment in a path
#[derive(Debug, Clone)]
pub struct PathSegment {
    pub ident: Ident,
    pub generics: Vec<Type>,
    pub span: Span,
}

/// An identifier
#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl Ident {
    pub fn new(name: String, span: Span) -> Self {
        Self { name, span }
    }
}
