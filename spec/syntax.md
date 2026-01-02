# Nova Syntax Specification

This document defines the syntax of the Nova programming language.

## Notation

We use a modified BNF notation:
- `|` denotes alternatives
- `*` denotes zero or more
- `+` denotes one or more
- `?` denotes optional
- `( )` denotes grouping
- `" "` denotes literal text

## Lexical Grammar

### Whitespace and Comments

```
WHITESPACE  = " " | "\t" | "\n" | "\r"
LINE_COMMENT = "//" (~"\n")*
BLOCK_COMMENT = "/*" (~"*/" | BLOCK_COMMENT)* "*/"
```

Block comments nest.

### Identifiers and Keywords

```
IDENT = (ALPHA | "_") (ALPHA | DIGIT | "_")*
ALPHA = "a".."z" | "A".."Z"
DIGIT = "0".."9"

KEYWORD = "fn" | "let" | "mut" | "if" | "else" | "while" | "for" | "in"
        | "return" | "break" | "continue" | "struct" | "enum" | "impl"
        | "trait" | "type" | "pub" | "use" | "mod" | "where" | "match"
        | "true" | "false"
```

### Literals

```
INT_LIT    = DIGIT+ | "0x" HEX_DIGIT+ | "0b" BIN_DIGIT+ | "0o" OCT_DIGIT+
FLOAT_LIT  = DIGIT+ "." DIGIT+ EXPONENT?
EXPONENT   = ("e" | "E") ("+" | "-")? DIGIT+
STRING_LIT = '"' (CHAR | ESCAPE)* '"'
CHAR_LIT   = "'" (CHAR | ESCAPE) "'"
ESCAPE     = "\\" ("n" | "r" | "t" | "\\" | '"' | "'" | "0" | "x" HEX HEX)
BOOL_LIT   = "true" | "false"
```

### Operators and Punctuation

```
// Arithmetic
PLUS = "+"   MINUS = "-"   STAR = "*"   SLASH = "/"   PERCENT = "%"

// Comparison
EQ = "=="   NE = "!="   LT = "<"   LE = "<="   GT = ">"   GE = ">="

// Logical
AND = "&&"   OR = "||"   NOT = "!"

// Bitwise
AMP = "&"   PIPE = "|"   CARET = "^"   TILDE = "~"   SHL = "<<"   SHR = ">>"

// Assignment
ASSIGN = "="   PLUS_EQ = "+="   MINUS_EQ = "-="   STAR_EQ = "*="
SLASH_EQ = "/="   PERCENT_EQ = "%="

// Punctuation
LPAREN = "("   RPAREN = ")"   LBRACKET = "["   RBRACKET = "]"
LBRACE = "{"   RBRACE = "}"   COMMA = ","   SEMI = ";"   COLON = ":"
COLONCOLON = "::"   ARROW = "->"   FAT_ARROW = "=>"   DOT = "."
DOTDOT = ".."   DOTDOTEQ = "..="   QUESTION = "?"   AT = "@"
```

## Syntactic Grammar

### Program Structure

```
Program = Item*

Item = Function
     | StructDef
     | EnumDef
     | ImplBlock
     | TraitDef
     | TypeAlias
     | UseStmt
```

### Functions

```
Function = "fn" IDENT Generics? "(" Params? ")" ("->" Type)? WhereClause? Block

Params = Param ("," Param)* ","?
Param = Pattern ":" Type

Generics = "<" GenericParam ("," GenericParam)* ","? ">"
GenericParam = IDENT (":" TypeBound)?

WhereClause = "where" WherePred ("," WherePred)* ","?
WherePred = Type ":" TypeBound

TypeBound = Type ("+" Type)*
```

### Types

```
Type = TypePath
     | TupleType
     | ArrayType
     | SliceType
     | ReferenceType
     | FunctionType
     | NeverType
     | InferType

TypePath = Path ("<" TypeArgs ">")?
TupleType = "(" (Type ("," Type)* ","?)? ")"
ArrayType = "[" Type ";" Expr "]"
SliceType = "[" Type "]"
ReferenceType = "&" "mut"? Type
FunctionType = "fn" "(" (Type ("," Type)*)? ")" ("->" Type)?
NeverType = "!"
InferType = "_"

TypeArgs = Type ("," Type)* ","?
```

### Structs and Enums

```
StructDef = "struct" IDENT Generics? "{" StructFields? "}"
StructFields = StructField ("," StructField)* ","?
StructField = IDENT ":" Type

EnumDef = "enum" IDENT Generics? "{" EnumVariants? "}"
EnumVariants = EnumVariant ("," EnumVariant)* ","?
EnumVariant = IDENT VariantFields?
VariantFields = "(" TupleFields ")" | "{" StructFields "}"
TupleFields = Type ("," Type)* ","?
```

### Impl and Trait

```
ImplBlock = "impl" Generics? (Type "for")? Type WhereClause? "{" ImplItem* "}"
ImplItem = Function

TraitDef = "trait" IDENT Generics? (":" TypeBound)? WhereClause? "{" TraitItem* "}"
TraitItem = TraitFunction
TraitFunction = "fn" IDENT Generics? "(" Params? ")" ("->" Type)? Block?
```

### Statements

```
Stmt = LetStmt
     | ExprStmt
     | Item

LetStmt = "let" "mut"? Pattern (":" Type)? ("=" Expr)? ";"
ExprStmt = Expr ";"?
```

### Expressions

```
Expr = Literal
     | Path
     | UnaryExpr
     | BinaryExpr
     | CallExpr
     | FieldExpr
     | IndexExpr
     | TupleExpr
     | ArrayExpr
     | StructExpr
     | BlockExpr
     | IfExpr
     | MatchExpr
     | WhileExpr
     | ForExpr
     | LoopExpr
     | ClosureExpr
     | ReturnExpr
     | BreakExpr
     | ContinueExpr
     | RangeExpr
     | RefExpr
     | DerefExpr
     | TryExpr
     | AwaitExpr
```

#### Precedence (high to low)

| Level | Operators | Associativity |
|-------|-----------|---------------|
| 1 | Method call, field access, index, ? | Left |
| 2 | Unary -, !, &, &mut, * | Right |
| 3 | as (casting) | Left |
| 4 | *, /, % | Left |
| 5 | +, - | Left |
| 6 | <<, >> | Left |
| 7 | & (bitwise) | Left |
| 8 | ^ (bitwise) | Left |
| 9 | \| (bitwise) | Left |
| 10 | ==, !=, <, >, <=, >= | Left |
| 11 | && | Left |
| 12 | \|\| | Left |
| 13 | .., ..= | Left |
| 14 | = (assignment) | Right |

### Patterns

```
Pattern = WildcardPat
        | IdentPat
        | LiteralPat
        | TuplePat
        | StructPat
        | EnumPat
        | OrPat
        | RefPat
        | RangePat

WildcardPat = "_"
IdentPat = "mut"? IDENT
LiteralPat = Literal
TuplePat = "(" (Pattern ("," Pattern)* ","?)? ")"
StructPat = Path "{" FieldPats? "}"
FieldPats = FieldPat ("," FieldPat)* ","?
FieldPat = IDENT (":" Pattern)?
EnumPat = Path ("(" Patterns ")" | "{" FieldPats "}")?
Patterns = Pattern ("," Pattern)* ","?
OrPat = Pattern ("|" Pattern)+
RefPat = "&" "mut"? Pattern
RangePat = Pattern? (".." | "..=") Pattern?
```

### Paths

```
Path = PathSegment ("::" PathSegment)*
PathSegment = IDENT ("::" "<" TypeArgs ">")?
```

### Blocks

```
Block = "{" Stmt* Expr? "}"
```

## Examples

### Function Definition

```nova
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Struct with Methods

```nova
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(self, other: Point) -> f64 {
        let dx = self.x - other.x
        let dy = self.y - other.y
        (dx * dx + dy * dy).sqrt()
    }
}
```

### Pattern Matching

```nova
enum Option<T> {
    Some(T),
    None,
}

fn unwrap_or<T>(opt: Option<T>, default: T) -> T {
    match opt {
        Option::Some(value) => value,
        Option::None => default,
    }
}
```

### Closures

```nova
fn main() {
    let numbers = [1, 2, 3, 4, 5]
    let squares = numbers.map(|x| x * x)
}
```

---

*This specification is a work in progress. See [GitHub Issues](https://github.com/nova-lang/nova/issues) for discussion.*
