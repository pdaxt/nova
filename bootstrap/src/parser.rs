//! Parser for Nova
//!
//! Converts a stream of tokens into an AST using recursive descent with
//! Pratt parsing for expressions.
//!
//! # Example
//!
//! ```ignore
//! use nova::lexer::lex;
//! use nova::parser::parse;
//!
//! let source = "fn main() { return 42; }";
//! let tokens = lex(source)?;
//! let ast = parse(source, tokens)?;
//! ```
//!
//! # Design (ADR-005)
//!
//! The parser requires access to the source string because tokens do not store
//! literal values. Values are extracted from source using token spans:
//!
//! ```ignore
//! let text = &source[token.span().start() as usize..token.span().end() as usize];
//! let value: i64 = text.parse()?;
//! ```
//!
//! This keeps tokens at a fixed 12 bytes regardless of literal size.
//!
//! # Contributing
//!
//! **Good first issues in this module:**
//!
//! - [ ] Implement `parse_struct()` for struct definitions
//! - [ ] Implement `parse_enum()` for enum definitions
//! - [ ] Implement `parse_match_expr()` for match expressions
//! - [ ] Implement `parse_generics()` for generic parameters
//! - [ ] Add better error recovery (don't stop at first error)
//!
//! **How to add a new syntax construct:**
//!
//! 1. Add AST node types in `ast.rs` if needed
//! 2. Implement the parsing function here
//! 3. Add tests in the `tests` module at the bottom
//! 4. Update the spec in `spec/syntax.md`
//! 5. Run `cargo test parser` to verify
//!
//! **Pratt Parsing:**
//!
//! We use Pratt parsing for expressions. Each operator has:
//! - Left binding power (how tightly it binds on the left)
//! - Right binding power (how tightly it binds on the right)
//!
//! See `parse_expr_bp()` for the implementation.

use crate::ast::*;
use crate::error::NovaError;
use crate::token::{Span, Token, TokenKind};

// ============================================================================
// Security Constants
// ============================================================================

/// Maximum expression nesting depth (prevents stack overflow)
/// Set to 64 to stay well within typical 2MB test thread stack limits
const MAX_EXPR_DEPTH: usize = 64;

/// Maximum block nesting depth (prevents stack overflow)
/// Set to 64 to stay well within typical 2MB test thread stack limits
const MAX_BLOCK_DEPTH: usize = 64;

// ============================================================================
// Public API
// ============================================================================

/// Parse tokens into an AST.
///
/// # Arguments
///
/// * `source` - The original source code (needed to extract literal values)
/// * `tokens` - The tokens produced by the lexer
pub fn parse(source: &str, tokens: Vec<Token>) -> Result<Program, NovaError> {
    let mut parser = Parser::new(source, tokens);
    parser.parse_program()
}

/// The parser state.
struct Parser<'a> {
    /// The source code (for extracting literal values via span)
    source: &'a str,
    /// The token stream
    tokens: Vec<Token>,
    /// Current position in token stream
    current: usize,
    /// Current expression nesting depth (for security limits)
    expr_depth: usize,
    /// Current block nesting depth (for security limits)
    block_depth: usize,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str, tokens: Vec<Token>) -> Self {
        Self {
            source,
            tokens,
            current: 0,
            expr_depth: 0,
            block_depth: 0,
        }
    }

    // ========================================================================
    // Source extraction helpers
    // ========================================================================

    /// Extracts the source text for a span.
    #[inline]
    fn text(&self, span: Span) -> &str {
        &self.source[span.start() as usize..span.end() as usize]
    }

    /// Extracts the source text for a token.
    #[inline]
    #[allow(dead_code)]
    fn token_text(&self, token: &Token) -> &str {
        self.text(token.span())
    }

    /// Parses an integer from the source at the given span.
    fn parse_int(&self, span: Span) -> Result<i64, NovaError> {
        let text = self.text(span);

        // Handle different bases
        let (radix, text) = if text.starts_with("0x") || text.starts_with("0X") {
            (16, &text[2..])
        } else if text.starts_with("0b") || text.starts_with("0B") {
            (2, &text[2..])
        } else if text.starts_with("0o") || text.starts_with("0O") {
            (8, &text[2..])
        } else {
            (10, text)
        };

        // Remove underscores
        let clean: String = text.chars().filter(|c| *c != '_').collect();

        i64::from_str_radix(&clean, radix).map_err(|_| NovaError::InvalidLiteral {
            kind: "integer",
            span,
        })
    }

    /// Parses a float from the source at the given span.
    fn parse_float(&self, span: Span) -> Result<f64, NovaError> {
        let text = self.text(span);
        let clean: String = text.chars().filter(|c| *c != '_').collect();
        clean.parse().map_err(|_| NovaError::InvalidLiteral {
            kind: "float",
            span,
        })
    }

    /// Extracts a string literal (removes quotes and unescapes).
    fn parse_string(&self, span: Span) -> String {
        let text = self.text(span);
        // Remove surrounding quotes
        let inner = &text[1..text.len() - 1];
        // TODO: Proper escape sequence handling
        inner
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\r", "\r")
            .replace("\\\\", "\\")
            .replace("\\\"", "\"")
    }

    // ========================================================================
    // Program parsing
    // ========================================================================

    /// Parse a complete program.
    fn parse_program(&mut self) -> Result<Program, NovaError> {
        let mut items = Vec::new();

        while !self.is_at_end() {
            items.push(self.parse_item()?);
        }

        Ok(Program { items })
    }

    /// Parse a top-level item.
    fn parse_item(&mut self) -> Result<Item, NovaError> {
        match self.peek().kind() {
            TokenKind::Fn => self.parse_function().map(Item::Function),
            TokenKind::Struct => self.parse_struct().map(Item::Struct),
            TokenKind::Enum => self.parse_enum().map(Item::Enum),
            TokenKind::Impl => self.parse_impl().map(Item::Impl),
            TokenKind::Trait => self.parse_trait().map(Item::Trait),
            TokenKind::Use => self.parse_use().map(Item::Use),
            TokenKind::Type => self.parse_type_alias().map(Item::TypeAlias),
            _ => Err(NovaError::UnexpectedToken {
                expected: "item".to_string(),
                found: self.peek().kind(),
                span: self.peek().span(),
            }),
        }
    }

    /// Parse a function definition.
    fn parse_function(&mut self) -> Result<Function, NovaError> {
        let start = self.expect(TokenKind::Fn)?.span();

        let name = self.parse_ident()?;
        let generics = self.parse_generics()?;

        self.expect(TokenKind::LParen)?;
        let params = self.parse_params()?;
        self.expect(TokenKind::RParen)?;

        let return_type = if self.check(TokenKind::Arrow) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let where_clause = if self.check(TokenKind::Where) {
            Some(self.parse_where_clause()?)
        } else {
            None
        };

        let body = self.parse_block()?;
        let span = start.merge(body.span);

        Ok(Function {
            name,
            generics,
            params,
            return_type,
            where_clause,
            body,
            span,
        })
    }

    /// Parse function parameters.
    fn parse_params(&mut self) -> Result<Vec<Param>, NovaError> {
        let mut params = Vec::new();

        while !self.check(TokenKind::RParen) && !self.is_at_end() {
            let pattern = self.parse_pattern()?;
            self.expect(TokenKind::Colon)?;
            let ty = self.parse_type()?;
            let span = pattern.span.merge(ty.span);
            params.push(Param { pattern, ty, span });

            if !self.check(TokenKind::RParen) {
                self.expect(TokenKind::Comma)?;
            }
        }

        Ok(params)
    }

    /// Parse a block.
    fn parse_block(&mut self) -> Result<Block, NovaError> {
        // Security: Check block depth limit
        self.block_depth += 1;
        if self.block_depth > MAX_BLOCK_DEPTH {
            let span = self.peek().span();
            self.block_depth -= 1;
            return Err(NovaError::NestingTooDeep {
                depth: self.block_depth,
                max: MAX_BLOCK_DEPTH,
                span,
            });
        }

        let start = self.expect(TokenKind::LBrace)?.span();
        let mut stmts = Vec::new();

        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            stmts.push(self.parse_stmt()?);
        }

        let end = self.expect(TokenKind::RBrace)?.span();
        self.block_depth -= 1;

        Ok(Block {
            stmts,
            span: start.merge(end),
        })
    }

    /// Parse a statement.
    fn parse_stmt(&mut self) -> Result<Stmt, NovaError> {
        match self.peek().kind() {
            TokenKind::Let => self.parse_let_stmt().map(Stmt::Let),
            TokenKind::Fn | TokenKind::Struct | TokenKind::Enum => {
                self.parse_item().map(Stmt::Item)
            }
            _ => {
                let expr = self.parse_expr()?;
                let has_semi = self.check(TokenKind::Semi);
                if has_semi {
                    self.advance();
                }
                let span = expr.span;
                Ok(Stmt::Expr(ExprStmt {
                    expr,
                    has_semi,
                    span,
                }))
            }
        }
    }

    /// Parse a let statement.
    fn parse_let_stmt(&mut self) -> Result<LetStmt, NovaError> {
        let start = self.expect(TokenKind::Let)?.span();

        let pattern = self.parse_pattern()?;

        let ty = if self.check(TokenKind::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let value = if self.check(TokenKind::Eq) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };

        let end = self.expect(TokenKind::Semi)?.span();

        Ok(LetStmt {
            pattern,
            ty,
            value,
            span: start.merge(end),
        })
    }

    // ========================================================================
    // Expression parsing (Pratt parser)
    // ========================================================================

    /// Parse an expression.
    fn parse_expr(&mut self) -> Result<Expr, NovaError> {
        // Security: Check expression depth limit
        self.expr_depth += 1;
        if self.expr_depth > MAX_EXPR_DEPTH {
            let span = self.peek().span();
            self.expr_depth -= 1;
            return Err(NovaError::NestingTooDeep {
                depth: self.expr_depth,
                max: MAX_EXPR_DEPTH,
                span,
            });
        }
        let result = self.parse_expr_bp(0);
        self.expr_depth -= 1;
        result
    }

    /// Parse expression with binding power (Pratt parsing).
    fn parse_expr_bp(&mut self, min_bp: u8) -> Result<Expr, NovaError> {
        // Parse prefix
        let mut lhs = self.parse_prefix()?;

        loop {
            // Check for postfix or infix
            let op = match self.peek().kind() {
                // Binary operators
                TokenKind::Plus => Some((BinOp::Add, 10, 11)),
                TokenKind::Minus => Some((BinOp::Sub, 10, 11)),
                TokenKind::Star => Some((BinOp::Mul, 12, 13)),
                TokenKind::Slash => Some((BinOp::Div, 12, 13)),
                TokenKind::Percent => Some((BinOp::Rem, 12, 13)),
                TokenKind::AmpAmp => Some((BinOp::And, 4, 5)),
                TokenKind::PipePipe => Some((BinOp::Or, 2, 3)),
                TokenKind::Amp => Some((BinOp::BitAnd, 6, 7)),
                TokenKind::Pipe => Some((BinOp::BitOr, 4, 5)),
                TokenKind::Caret => Some((BinOp::BitXor, 5, 6)),
                TokenKind::LtLt => Some((BinOp::Shl, 9, 10)),
                TokenKind::GtGt => Some((BinOp::Shr, 9, 10)),
                TokenKind::EqEq => Some((BinOp::Eq, 7, 8)),
                TokenKind::BangEq => Some((BinOp::Ne, 7, 8)),
                TokenKind::Lt => Some((BinOp::Lt, 8, 9)),
                TokenKind::LtEq => Some((BinOp::Le, 8, 9)),
                TokenKind::Gt => Some((BinOp::Gt, 8, 9)),
                TokenKind::GtEq => Some((BinOp::Ge, 8, 9)),
                TokenKind::Eq => Some((BinOp::Assign, 1, 0)), // Right associative
                _ => None,
            };

            if let Some((op, l_bp, r_bp)) = op {
                if l_bp < min_bp {
                    break;
                }

                self.advance();
                let rhs = self.parse_expr_bp(r_bp)?;
                let span = lhs.span.merge(rhs.span);
                lhs = Expr {
                    kind: ExprKind::Binary(Box::new(lhs), op, Box::new(rhs)),
                    span,
                };
                continue;
            }

            // Postfix operators
            match self.peek().kind() {
                TokenKind::LParen => {
                    // Function call
                    self.advance();
                    let args = self.parse_args()?;
                    let end = self.expect(TokenKind::RParen)?.span();
                    let span = lhs.span.merge(end);
                    lhs = Expr {
                        kind: ExprKind::Call(Box::new(lhs), args),
                        span,
                    };
                }
                TokenKind::LBracket => {
                    // Index
                    self.advance();
                    let index = self.parse_expr()?;
                    let end = self.expect(TokenKind::RBracket)?.span();
                    let span = lhs.span.merge(end);
                    lhs = Expr {
                        kind: ExprKind::Index(Box::new(lhs), Box::new(index)),
                        span,
                    };
                }
                TokenKind::Dot => {
                    // Field access
                    self.advance();
                    let field = self.parse_ident()?;
                    let span = lhs.span.merge(field.span);
                    lhs = Expr {
                        kind: ExprKind::Field(Box::new(lhs), field),
                        span,
                    };
                }
                TokenKind::Question => {
                    // Try operator
                    let end = self.advance().span();
                    let span = lhs.span.merge(end);
                    lhs = Expr {
                        kind: ExprKind::Try(Box::new(lhs)),
                        span,
                    };
                }
                _ => break,
            }
        }

        Ok(lhs)
    }

    /// Parse a prefix expression (primary or unary).
    fn parse_prefix(&mut self) -> Result<Expr, NovaError> {
        match self.peek().kind() {
            TokenKind::Minus => {
                let start = self.advance().span();
                let expr = self.parse_expr_bp(14)?; // High precedence for unary
                let span = start.merge(expr.span);
                Ok(Expr {
                    kind: ExprKind::Unary(UnaryOp::Neg, Box::new(expr)),
                    span,
                })
            }
            TokenKind::Bang => {
                let start = self.advance().span();
                let expr = self.parse_expr_bp(14)?;
                let span = start.merge(expr.span);
                Ok(Expr {
                    kind: ExprKind::Unary(UnaryOp::Not, Box::new(expr)),
                    span,
                })
            }
            TokenKind::Amp => {
                let start = self.advance().span();
                let mutable = self.check(TokenKind::Mut);
                if mutable {
                    self.advance();
                }
                let expr = self.parse_expr_bp(14)?;
                let span = start.merge(expr.span);
                Ok(Expr {
                    kind: ExprKind::Ref(mutable, Box::new(expr)),
                    span,
                })
            }
            TokenKind::Star => {
                let start = self.advance().span();
                let expr = self.parse_expr_bp(14)?;
                let span = start.merge(expr.span);
                Ok(Expr {
                    kind: ExprKind::Deref(Box::new(expr)),
                    span,
                })
            }
            _ => self.parse_primary(),
        }
    }

    /// Parse a primary expression.
    fn parse_primary(&mut self) -> Result<Expr, NovaError> {
        let token = self.peek();
        let kind = token.kind();
        let span = token.span();

        match kind {
            TokenKind::IntLit => {
                self.advance();
                let value = self.parse_int(span)?;
                Ok(Expr {
                    kind: ExprKind::Literal(Literal::Int(value)),
                    span,
                })
            }
            TokenKind::FloatLit => {
                self.advance();
                let value = self.parse_float(span)?;
                Ok(Expr {
                    kind: ExprKind::Literal(Literal::Float(value)),
                    span,
                })
            }
            TokenKind::StringLit => {
                self.advance();
                let value = self.parse_string(span);
                Ok(Expr {
                    kind: ExprKind::Literal(Literal::String(value)),
                    span,
                })
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr {
                    kind: ExprKind::Literal(Literal::Bool(true)),
                    span,
                })
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr {
                    kind: ExprKind::Literal(Literal::Bool(false)),
                    span,
                })
            }
            TokenKind::Ident => {
                let path = self.parse_path()?;
                Ok(Expr {
                    span: path.span,
                    kind: ExprKind::Path(path),
                })
            }
            TokenKind::LParen => {
                let start = self.advance().span();
                if self.check(TokenKind::RParen) {
                    // Unit tuple
                    let end = self.advance().span();
                    Ok(Expr {
                        kind: ExprKind::Tuple(vec![]),
                        span: start.merge(end),
                    })
                } else {
                    let expr = self.parse_expr()?;
                    if self.check(TokenKind::Comma) {
                        // Tuple
                        let mut exprs = vec![expr];
                        while self.check(TokenKind::Comma) {
                            self.advance();
                            if self.check(TokenKind::RParen) {
                                break;
                            }
                            exprs.push(self.parse_expr()?);
                        }
                        let end = self.expect(TokenKind::RParen)?.span();
                        Ok(Expr {
                            kind: ExprKind::Tuple(exprs),
                            span: start.merge(end),
                        })
                    } else {
                        // Grouped expression
                        self.expect(TokenKind::RParen)?;
                        Ok(expr)
                    }
                }
            }
            TokenKind::LBracket => {
                let start = self.advance().span();
                let mut exprs = Vec::new();
                while !self.check(TokenKind::RBracket) && !self.is_at_end() {
                    exprs.push(self.parse_expr()?);
                    if !self.check(TokenKind::RBracket) {
                        self.expect(TokenKind::Comma)?;
                    }
                }
                let end = self.expect(TokenKind::RBracket)?.span();
                Ok(Expr {
                    kind: ExprKind::Array(exprs),
                    span: start.merge(end),
                })
            }
            TokenKind::LBrace => {
                let block = self.parse_block()?;
                Ok(Expr {
                    span: block.span,
                    kind: ExprKind::Block(block),
                })
            }
            TokenKind::If => self.parse_if_expr(),
            TokenKind::Match => self.parse_match_expr(),
            TokenKind::While => self.parse_while_expr(),
            TokenKind::For => self.parse_for_expr(),
            TokenKind::Return => {
                let start = self.advance().span();
                let value = if !self.check(TokenKind::Semi) && !self.check(TokenKind::RBrace) {
                    Some(Box::new(self.parse_expr()?))
                } else {
                    None
                };
                let span = if let Some(ref v) = value {
                    start.merge(v.span)
                } else {
                    start
                };
                Ok(Expr {
                    kind: ExprKind::Return(value),
                    span,
                })
            }
            TokenKind::Break => {
                let start = self.advance().span();
                let value = if !self.check(TokenKind::Semi) && !self.check(TokenKind::RBrace) {
                    Some(Box::new(self.parse_expr()?))
                } else {
                    None
                };
                let span = if let Some(ref v) = value {
                    start.merge(v.span)
                } else {
                    start
                };
                Ok(Expr {
                    kind: ExprKind::Break(value),
                    span,
                })
            }
            TokenKind::Continue => {
                let span = self.advance().span();
                Ok(Expr {
                    kind: ExprKind::Continue,
                    span,
                })
            }
            _ => Err(NovaError::UnexpectedToken {
                expected: "expression".to_string(),
                found: kind,
                span,
            }),
        }
    }

    // ========================================================================
    // Helpers
    // ========================================================================

    /// Parse an identifier.
    fn parse_ident(&mut self) -> Result<Ident, NovaError> {
        if self.peek().kind() == TokenKind::Ident {
            let token = self.advance();
            let span = token.span();
            let name = self.text(span).to_string();
            Ok(Ident { name, span })
        } else {
            Err(NovaError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: self.peek().kind(),
                span: self.peek().span(),
            })
        }
    }

    /// Parse a path.
    fn parse_path(&mut self) -> Result<Path, NovaError> {
        let mut segments = Vec::new();
        let start = self.peek().span();

        loop {
            let ident = self.parse_ident()?;
            let generics = if self.check(TokenKind::ColonColon) {
                // Check for turbofish ::<>
                let next = self.tokens.get(self.current + 1);
                if matches!(next, Some(t) if t.kind() == TokenKind::Lt) {
                    self.advance(); // ::
                    self.parse_generic_args()?
                } else {
                    vec![]
                }
            } else {
                vec![]
            };

            segments.push(PathSegment {
                span: ident.span,
                ident,
                generics,
            });

            if self.check(TokenKind::ColonColon) {
                self.advance();
            } else {
                break;
            }
        }

        let end = segments.last().map(|s| s.span).unwrap_or(start);
        Ok(Path {
            segments,
            span: start.merge(end),
        })
    }

    /// Parse a type.
    fn parse_type(&mut self) -> Result<Type, NovaError> {
        let start = self.peek().span();

        match self.peek().kind() {
            TokenKind::LParen => {
                self.advance();
                if self.check(TokenKind::RParen) {
                    let end = self.advance().span();
                    return Ok(Type {
                        kind: TypeKind::Tuple(vec![]),
                        span: start.merge(end),
                    });
                }
                let mut types = vec![self.parse_type()?];
                while self.check(TokenKind::Comma) {
                    self.advance();
                    if self.check(TokenKind::RParen) {
                        break;
                    }
                    types.push(self.parse_type()?);
                }
                let end = self.expect(TokenKind::RParen)?.span();
                Ok(Type {
                    kind: TypeKind::Tuple(types),
                    span: start.merge(end),
                })
            }
            TokenKind::LBracket => {
                self.advance();
                let elem_type = self.parse_type()?;
                if self.check(TokenKind::Semi) {
                    self.advance();
                    let size = self.parse_expr()?;
                    let end = self.expect(TokenKind::RBracket)?.span();
                    Ok(Type {
                        kind: TypeKind::Array(Box::new(elem_type), Box::new(size)),
                        span: start.merge(end),
                    })
                } else {
                    let end = self.expect(TokenKind::RBracket)?.span();
                    Ok(Type {
                        kind: TypeKind::Slice(Box::new(elem_type)),
                        span: start.merge(end),
                    })
                }
            }
            TokenKind::Amp => {
                self.advance();
                let mutable = self.check(TokenKind::Mut);
                if mutable {
                    self.advance();
                }
                let inner = self.parse_type()?;
                let span = start.merge(inner.span);
                Ok(Type {
                    kind: TypeKind::Reference(mutable, Box::new(inner)),
                    span,
                })
            }
            TokenKind::Bang => {
                let span = self.advance().span();
                Ok(Type {
                    kind: TypeKind::Never,
                    span,
                })
            }
            TokenKind::Ident => {
                let path = self.parse_path()?;
                Ok(Type {
                    span: path.span,
                    kind: TypeKind::Path(path),
                })
            }
            _ => Err(NovaError::UnexpectedToken {
                expected: "type".to_string(),
                found: self.peek().kind(),
                span: self.peek().span(),
            }),
        }
    }

    /// Parse a pattern.
    fn parse_pattern(&mut self) -> Result<Pattern, NovaError> {
        let start = self.peek().span();

        match self.peek().kind() {
            TokenKind::Ident => {
                let token = self.advance();
                let span = token.span();
                let name = self.text(span).to_string();
                Ok(Pattern {
                    kind: PatternKind::Ident(Ident { name, span }, false),
                    span,
                })
            }
            TokenKind::Mut => {
                self.advance();
                if self.peek().kind() == TokenKind::Ident {
                    let token = self.advance();
                    let span = token.span();
                    let name = self.text(span).to_string();
                    Ok(Pattern {
                        kind: PatternKind::Ident(Ident { name, span }, true),
                        span: start.merge(span),
                    })
                } else {
                    Err(NovaError::UnexpectedToken {
                        expected: "identifier".to_string(),
                        found: self.peek().kind(),
                        span: self.peek().span(),
                    })
                }
            }
            TokenKind::IntLit => {
                let token = self.advance();
                let span = token.span();
                let value = self.parse_int(span)?;
                Ok(Pattern {
                    kind: PatternKind::Literal(Literal::Int(value)),
                    span,
                })
            }
            TokenKind::StringLit => {
                let token = self.advance();
                let span = token.span();
                let value = self.parse_string(span);
                Ok(Pattern {
                    kind: PatternKind::Literal(Literal::String(value)),
                    span,
                })
            }
            TokenKind::True => {
                let span = self.advance().span();
                Ok(Pattern {
                    kind: PatternKind::Literal(Literal::Bool(true)),
                    span,
                })
            }
            TokenKind::False => {
                let span = self.advance().span();
                Ok(Pattern {
                    kind: PatternKind::Literal(Literal::Bool(false)),
                    span,
                })
            }
            TokenKind::Underscore => {
                let span = self.advance().span();
                Ok(Pattern {
                    kind: PatternKind::Wildcard,
                    span,
                })
            }
            _ => {
                // Default wildcard for unrecognized patterns
                let span = self.advance().span();
                Ok(Pattern {
                    kind: PatternKind::Wildcard,
                    span,
                })
            }
        }
    }

    // ========================================================================
    // Stub implementations (TODO: implement by contributors)
    // ========================================================================

    fn parse_generics(&mut self) -> Result<Vec<GenericParam>, NovaError> {
        // TODO: Implement generic parameter parsing
        Ok(vec![])
    }

    fn parse_generic_args(&mut self) -> Result<Vec<Type>, NovaError> {
        // TODO: Implement generic argument parsing
        Ok(vec![])
    }

    fn parse_where_clause(&mut self) -> Result<WhereClause, NovaError> {
        // TODO: Implement where clause parsing
        let span = self.advance().span();
        Ok(WhereClause {
            predicates: vec![],
            span,
        })
    }

    fn parse_struct(&mut self) -> Result<StructDef, NovaError> {
        // TODO: Implement struct parsing
        todo!("Struct parsing not yet implemented")
    }

    fn parse_enum(&mut self) -> Result<EnumDef, NovaError> {
        // TODO: Implement enum parsing
        todo!("Enum parsing not yet implemented")
    }

    fn parse_impl(&mut self) -> Result<ImplBlock, NovaError> {
        // TODO: Implement impl block parsing
        todo!("Impl parsing not yet implemented")
    }

    fn parse_trait(&mut self) -> Result<TraitDef, NovaError> {
        // TODO: Implement trait parsing
        todo!("Trait parsing not yet implemented")
    }

    fn parse_use(&mut self) -> Result<UseStmt, NovaError> {
        // TODO: Implement use statement parsing
        todo!("Use parsing not yet implemented")
    }

    fn parse_type_alias(&mut self) -> Result<TypeAlias, NovaError> {
        // TODO: Implement type alias parsing
        todo!("Type alias parsing not yet implemented")
    }

    fn parse_if_expr(&mut self) -> Result<Expr, NovaError> {
        let start = self.expect(TokenKind::If)?.span();
        let cond = self.parse_expr()?;
        let then_block = self.parse_block()?;
        let else_expr = if self.check(TokenKind::Else) {
            self.advance();
            if self.check(TokenKind::If) {
                Some(Box::new(self.parse_if_expr()?))
            } else {
                let block = self.parse_block()?;
                Some(Box::new(Expr {
                    span: block.span,
                    kind: ExprKind::Block(block),
                }))
            }
        } else {
            None
        };
        let span = if let Some(ref e) = else_expr {
            start.merge(e.span)
        } else {
            start.merge(then_block.span)
        };
        Ok(Expr {
            kind: ExprKind::If(Box::new(cond), then_block, else_expr),
            span,
        })
    }

    fn parse_match_expr(&mut self) -> Result<Expr, NovaError> {
        // TODO: Implement match expression parsing
        todo!("Match parsing not yet implemented")
    }

    fn parse_while_expr(&mut self) -> Result<Expr, NovaError> {
        let start = self.expect(TokenKind::While)?.span();
        let cond = self.parse_expr()?;
        let body = self.parse_block()?;
        let span = start.merge(body.span);
        Ok(Expr {
            kind: ExprKind::While(Box::new(cond), body),
            span,
        })
    }

    fn parse_for_expr(&mut self) -> Result<Expr, NovaError> {
        let start = self.expect(TokenKind::For)?.span();
        let pattern = self.parse_pattern()?;
        self.expect(TokenKind::In)?;
        let iter = self.parse_expr()?;
        let body = self.parse_block()?;
        let span = start.merge(body.span);
        Ok(Expr {
            kind: ExprKind::For(pattern, Box::new(iter), body),
            span,
        })
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, NovaError> {
        let mut args = Vec::new();
        while !self.check(TokenKind::RParen) && !self.is_at_end() {
            args.push(self.parse_expr()?);
            if !self.check(TokenKind::RParen) {
                self.expect(TokenKind::Comma)?;
            }
        }
        Ok(args)
    }

    // ========================================================================
    // Token helpers
    // ========================================================================

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current)
            .unwrap_or_else(|| self.tokens.last().expect("Token stream is empty"))
    }

    fn advance(&mut self) -> Token {
        let token = *self.peek();
        if !self.is_at_end() {
            self.current += 1;
        }
        token
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.peek().kind() == kind
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, NovaError> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(NovaError::UnexpectedToken {
                expected: kind.to_string(),
                found: self.peek().kind(),
                span: self.peek().span(),
            })
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind() == TokenKind::Eof
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;

    #[test]
    fn test_parse_simple_fn() {
        let source = "fn main() { return 42; }";
        let tokens = lex(source).unwrap();
        let program = parse(source, tokens).unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn test_parse_let() {
        let source = "fn main() { let x = 42; }";
        let tokens = lex(source).unwrap();
        let program = parse(source, tokens).unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn test_parse_hex_literal() {
        let source = "fn main() { let x = 0xFF; }";
        let tokens = lex(source).unwrap();
        let program = parse(source, tokens).unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn test_parse_binary_literal() {
        let source = "fn main() { let x = 0b1010; }";
        let tokens = lex(source).unwrap();
        let program = parse(source, tokens).unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn test_parse_underscore_pattern() {
        let source = "fn main() { let _ = 42; }";
        let tokens = lex(source).unwrap();
        let program = parse(source, tokens).unwrap();
        assert_eq!(program.items.len(), 1);
    }
}
