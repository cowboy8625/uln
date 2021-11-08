// program        → declaration* EOF ;
// declaration    → varDecl | statement ;
// statement      → printStmt | expression | ifStmt | block ;
// block          → "{" declaration "}"
// ifStmt         → "if" expression "then" statement ( "else" statement )? ;
// varDecl        → IDENTIFIER ( "=" expression )? ;
// printStmt      → "print" expression
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "(" expression ")" | IDENTIFIER ;
use crate::combinators::*;
use crate::node::{Node, Operator};

// program        → declaration* EOF ;
pub fn program<'a>() -> impl Parser<'a, Node> {
    declaration()
}

// declaration    → varDecl | statement ;
fn declaration<'a>() -> impl Parser<'a, Node> {
    either(var_decl(), statement())
}

// // block          → "{" declaration "}"
// fn block<'a>() -> impl Parser<'a, Node> {
//     pair(trim(tag("{")), pari(declaration(), trim(tag("}")))).map(|(_, (exp, _)) Node::Block(Box::new(exp))|
// }

// varDecl        → "let" IDENTIFIER ( "=" expression )? ";" ;
fn var_decl<'a>() -> impl Parser<'a, Node> {
    pair(pair(identifier, trim(tag("="))), expression()).map(|((ident, _), exp)| Node::Variable {
        ident,
        exp: Box::new(exp),
    })
}

// statement      → printStmt | expression ;
fn statement<'a>() -> impl Parser<'a, Node> {
    either(print_statement(), expression())
}

// printStmt      → "print" expression ";" ;
fn print_statement<'a>() -> impl Parser<'a, Node> {
    pair(tag("print"), expression()).map(|(_, exp)| Node::Print(Box::new(exp)))
}

fn expression<'a>() -> impl Parser<'a, Node> {
    equality()
}
// equality → ( "!=" | "==" ) ;
fn equality_op<'a>() -> impl Parser<'a, Operator> {
    trim(either(
        tag("!=").map(|_| Operator::NotEqual),
        tag("==").map(|_| Operator::Equality),
    ))
}
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
fn equality<'a>() -> impl Parser<'a, Node> {
    either(
        pair(comparison(), pair(equality_op(), comparison()))
            .map(|(node1, (op, node2))| Node::BinaryExpr {
                op,
                lhs: Box::new(node1),
                rhs: Box::new(node2),
            })
            .and_then(|node| {
                zero_or_more(pair(equality_op(), comparison())).map(move |vec| {
                    let mut node = node.clone();
                    for (op, unary_node) in vec {
                        node = Node::BinaryExpr {
                            op,
                            lhs: Box::new(node),
                            rhs: Box::new(unary_node),
                        };
                    }
                    node
                })
            }),
        comparison(),
    )
}

// comparison  → ( ">" | ">=" | "<" | "<=" ) ;
fn comparison_op<'a>() -> impl Parser<'a, Operator> {
    either(
        either(
            tag("<=").map(|_| Operator::LessEqual),
            tag("<").map(|_| Operator::LessThan),
        ),
        either(
            tag(">=").map(|_| Operator::GreaterEqual),
            tag(">").map(|_| Operator::GreaterThan),
        ),
    )
}

// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
fn comparison<'a>() -> impl Parser<'a, Node> {
    either(
        pair(term(), pair(comparison_op(), term()))
            .map(|(node1, (op, node2))| Node::BinaryExpr {
                op,
                lhs: Box::new(node1),
                rhs: Box::new(node2),
            })
            .and_then(|node| {
                zero_or_more(pair(comparison_op(), term())).map(move |vec| {
                    let mut node = node.clone();
                    for (op, unary_node) in vec {
                        node = Node::BinaryExpr {
                            op,
                            lhs: Box::new(node),
                            rhs: Box::new(unary_node),
                        };
                    }
                    node
                })
            }),
        term(),
    )
}

// term → ( "-" | "+" ) ;
fn term_op<'a>() -> impl Parser<'a, Operator> {
    either(
        tag("-").map(|_| Operator::Minus),
        tag("+").map(|_| Operator::Plus),
    )
}
// term           → factor ( ( "-" | "+" ) factor )* ;
fn term<'a>() -> impl Parser<'a, Node> {
    either(
        pair(factor(), pair(term_op(), factor()))
            .map(|(node1, (op, node2))| Node::BinaryExpr {
                op,
                lhs: Box::new(node1),
                rhs: Box::new(node2),
            })
            .and_then(|node| {
                zero_or_more(pair(term_op(), factor())).map(move |vec| {
                    let mut node = node.clone();
                    for (op, unary_node) in vec {
                        node = Node::BinaryExpr {
                            op,
                            lhs: Box::new(node),
                            rhs: Box::new(unary_node),
                        };
                    }
                    node
                })
            }),
        factor(),
    )
}

// factor → ( "/" | "*" ) ;
fn factor_op<'a>() -> impl Parser<'a, Operator> {
    either(
        tag("/").map(|_| Operator::Divide),
        tag("*").map(|_| Operator::Multiply),
    )
}

// factor         → unary ( ( "/" | "*" ) unary )* ;
fn factor<'a>() -> impl Parser<'a, Node> {
    either(
        pair(unary(), pair(factor_op(), unary()))
            .map(|(node1, (op, node2))| Node::BinaryExpr {
                op,
                lhs: Box::new(node1),
                rhs: Box::new(node2),
            })
            .and_then(|node| {
                zero_or_more(pair(factor_op(), unary())).map(move |vec| {
                    let mut node = node.clone();
                    for (op, unary_node) in vec {
                        node = Node::BinaryExpr {
                            op,
                            lhs: Box::new(node),
                            rhs: Box::new(unary_node),
                        };
                    }
                    node
                })
            }),
        unary(),
    )
}

// unary          → ( "!" | "-" ) unary | primary ;
fn unary<'a>() -> impl Parser<'a, Node> {
    trim(either(unary_neg(), unary_bang()))
}

// unary → "-"
fn unary_neg<'a>() -> impl Parser<'a, Node> {
    zero_or_more(trim(tag("-")))
        .map(|vec_of_op| {
            if vec_of_op.len() % 2 == 0 {
                Operator::Plus
            } else {
                Operator::Minus
            }
        })
        .and_then(|op| {
            primary().map(move |child| Node::UnaryExpr {
                op: op.clone(),
                child: Box::new(child),
            })
        })
}

// unary → "!"
fn unary_bang<'a>() -> impl Parser<'a, Node> {
    zero_or_more(trim(tag("!")))
        .map(|vec_of_op| {
            if vec_of_op.len() % 2 == 0 {
                Operator::Plus
            } else {
                Operator::Bang
            }
        })
        .and_then(|op| {
            primary().map(move |child| Node::UnaryExpr {
                op: op.clone(),
                child: Box::new(child),
            })
        })
}

// primary        → NUMBER | STRING | "true" | "false" | "(" expression ")" | IDENTIFIER ;
fn primary<'a>() -> impl Parser<'a, Node> {
    either(
        either(
            either(primary_number(), primary_string()),
            either(primary_bool(), primary_paren()),
        ),
        primary_ident(),
    )
}

// primary → IDENTIFIER
fn primary_ident<'a>() -> impl Parser<'a, Node> {
    identifier.map(|name| Node::Ident(name))
}

// primary → "(" expression ")"
fn primary_paren<'a>() -> impl Parser<'a, Node> {
    right(tag("("), left(expression(), tag(")")))
}

// primary → BOOL
fn primary_bool<'a>() -> impl Parser<'a, Node> {
    either(
        tag("true").map(|_| Node::True),
        tag("false").map(|_| Node::False),
    )
}

// primary → STRING
fn primary_string<'a>() -> impl Parser<'a, Node> {
    quoted_string().map(|s| Node::Str(s))
}

// primary → INT | FLOAT
fn primary_number<'a>() -> impl Parser<'a, Node> {
    number.map(|s| {
        if s.contains(".") {
            Node::Float(s.parse::<f64>().expect("Failed to parse String into f64"))
        } else {
            Node::Int(s.parse::<i128>().expect("Failed to parse String into i128"))
        }
    })
}
