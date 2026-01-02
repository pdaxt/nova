//! Error types and reporting for Nova

use crate::token::{Span, TokenKind};
use ariadne::{Color, Label, Report, ReportKind, Source};

/// All possible Nova errors
#[derive(Debug, Clone)]
pub enum NovaError {
    // Lexer errors
    InvalidCharacter {
        char: char,
        span: Span,
    },
    UnterminatedString {
        span: Span,
    },
    InvalidEscape {
        char: char,
        span: Span,
    },
    InvalidNumber {
        span: Span,
    },

    // Parser errors
    UnexpectedToken {
        expected: String,
        found: TokenKind,
        span: Span,
    },
    UnexpectedEof {
        expected: String,
        span: Span,
    },

    // Type errors
    TypeMismatch {
        expected: String,
        found: String,
        span: Span,
    },
    UndefinedVariable {
        name: String,
        span: Span,
    },
    UndefinedType {
        name: String,
        span: Span,
    },
    UndefinedFunction {
        name: String,
        span: Span,
    },

    // General
    Custom {
        message: String,
        span: Span,
    },
}

impl NovaError {
    /// Get the span of this error
    pub fn span(&self) -> Span {
        match self {
            NovaError::InvalidCharacter { span, .. } => *span,
            NovaError::UnterminatedString { span } => *span,
            NovaError::InvalidEscape { span, .. } => *span,
            NovaError::InvalidNumber { span } => *span,
            NovaError::UnexpectedToken { span, .. } => *span,
            NovaError::UnexpectedEof { span, .. } => *span,
            NovaError::TypeMismatch { span, .. } => *span,
            NovaError::UndefinedVariable { span, .. } => *span,
            NovaError::UndefinedType { span, .. } => *span,
            NovaError::UndefinedFunction { span, .. } => *span,
            NovaError::Custom { span, .. } => *span,
        }
    }

    /// Get the error message
    pub fn message(&self) -> String {
        match self {
            NovaError::InvalidCharacter { char, .. } => {
                format!("Invalid character: {:?}", char)
            }
            NovaError::UnterminatedString { .. } => {
                "Unterminated string literal".to_string()
            }
            NovaError::InvalidEscape { char, .. } => {
                format!("Invalid escape sequence: \\{}", char)
            }
            NovaError::InvalidNumber { .. } => {
                "Invalid number literal".to_string()
            }
            NovaError::UnexpectedToken { expected, found, .. } => {
                format!("Expected {}, found {}", expected, found)
            }
            NovaError::UnexpectedEof { expected, .. } => {
                format!("Unexpected end of file, expected {}", expected)
            }
            NovaError::TypeMismatch { expected, found, .. } => {
                format!("Type mismatch: expected {}, found {}", expected, found)
            }
            NovaError::UndefinedVariable { name, .. } => {
                format!("Undefined variable: {}", name)
            }
            NovaError::UndefinedType { name, .. } => {
                format!("Undefined type: {}", name)
            }
            NovaError::UndefinedFunction { name, .. } => {
                format!("Undefined function: {}", name)
            }
            NovaError::Custom { message, .. } => message.clone(),
        }
    }

    /// Get the error code
    pub fn code(&self) -> &'static str {
        match self {
            NovaError::InvalidCharacter { .. } => "E0001",
            NovaError::UnterminatedString { .. } => "E0002",
            NovaError::InvalidEscape { .. } => "E0003",
            NovaError::InvalidNumber { .. } => "E0004",
            NovaError::UnexpectedToken { .. } => "E0100",
            NovaError::UnexpectedEof { .. } => "E0101",
            NovaError::TypeMismatch { .. } => "E0200",
            NovaError::UndefinedVariable { .. } => "E0201",
            NovaError::UndefinedType { .. } => "E0202",
            NovaError::UndefinedFunction { .. } => "E0203",
            NovaError::Custom { .. } => "E9999",
        }
    }
}

impl std::fmt::Display for NovaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code(), self.message())
    }
}

impl std::error::Error for NovaError {}

/// Report an error with nice formatting
pub fn report(source: &str, filename: &str, error: NovaError) {
    let span = error.span();
    let message = error.message();
    let code = error.code();

    Report::build(ReportKind::Error, filename, span.start)
        .with_code(code)
        .with_message(&message)
        .with_label(
            Label::new((filename, span.start..span.end))
                .with_message(&message)
                .with_color(Color::Red),
        )
        .finish()
        .print((filename, Source::from(source)))
        .unwrap();
}

/// Report multiple errors
pub fn report_all(source: &str, filename: &str, errors: Vec<NovaError>) {
    for error in errors {
        report(source, filename, error);
    }
}
