# Grammer

- ✅ Implemented
- ❌ Not Implemented
- ❌✅ Working on it

```
✅   program      → declaration* EOF ;
✅   declaration  → function | statement ;
✅   function      → IDENTIFIER parameter? "=" statement "\n" ;
✅   paramenters  → IDENTIFIER ( IDENTIFIER )* ;
✅   statement    → printStmt | expression | ifStmt | returnStmt | block ;
❌   returnStmt   → "return" expression? ;
❌✅ block        → "{" declaration* "}"
✅   ifStmt       → "if" expression "then" statement ( "else" statement )? ;
✅   printStmt    → "print" expression
✅   expression   → logic_or ;
✅   logic_or     → logic_and ( "or" logic_and )* ;
✅   logic_and    → equality ( "and" equality )* ;
✅   equality     → comparison ( ( "!=" | "==" ) comparison )* ;
✅   comparison   → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
✅   term         → factor ( ( "-" | "+" ) factor )* ;
✅   factor       → unary ( ( "/" | "*" ) unary )* ;
✅   unary        → ( "!" | "-" ) unary | call ;
✅   call         → primary ( arguments? )* ;
✅   arguments    → expression ( expression)* ;
✅   primary      → NUMBER | STRING | "true" | "false" | "(" expression ")" | IDENTIFIER ;

```
