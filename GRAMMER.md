# Grammer

```
program        → declaration* EOF ;

declaration    → funDecl | varDecl | statement ;
funDecl        → "\" function ;
function       → IDENTIFIER "(" parameter? ")" block;
paramenters    → IDENTIFIER ( "," IDENTIFIER )* ;

statement      → printStmt | expression | ifStmt | block ;
block          → "{" declaration* "}"
ifStmt         → "if" expression "then" statement ( "else" statement )? ;
varDecl        → IDENTIFIER ( "=" expression )? ;
printStmt      → "print" expression
expression     → logic_or ;
logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;

unary          → ( "!" | "-" ) unary | call ;
call           → primary ( "(" arguments? ")" )* ;
arguments      → expression ( "," expression)* ;

primary        → NUMBER | STRING | "true" | "false" | "(" expression ")" | IDENTIFIER ;
```
