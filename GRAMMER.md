# Grammer

```
program        → declaration* EOF ;
declaration    → varDecl | statement ;
statement      → printStmt | expression | ifStmt | block ;
block          → "{" declaration "}"
ifStmt         → "if" expression "then" statement ( "else" statement )? ;
varDecl        → IDENTIFIER ( "=" expression )? ;
printStmt      → "print" expression
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary | primary ;
primary        → NUMBER | STRING | "true" | "false" | "(" expression ")" | IDENTIFIER ;
```
