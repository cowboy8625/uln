// program        → declaration* EOF ;
//
// declaration    → funDecl | varDecl | statement ;
// funDecl        → "\" function ;
// function       → parameter? block;
// paramenters    → IDENTIFIER ( "," IDENTIFIER )* ;
//
//
// statement      → printStmt | expression | ifStmt | block ;
// block          → "{" declaration* "}"
// ifStmt         → "if" expression "then" statement ( "else" statement )? ;
// varDecl        → IDENTIFIER ( "=" ( expression | funDecl )? ;
// printStmt      → "print" expression
// expression     → logic_or ;
// logic_or       → logic_and ( "or" logic_and )* ;
// logic_and      → equality ( "and" equality )* ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
//
// unary          → ( "!" | "-" ) unary | call ;
// call           → primary ( arguments? )* ;
// arguments      → expression ( "," expression)* ;
//
// primary        → NUMBER | STRING | "true" | "false" | "(" expression ")" | IDENTIFIER ;
use crate::combinators::*;
use crate::node::{Node, Operator};

// program        → declaration* EOF ;
pub fn program<'a>() -> impl Parser<'a, Vec<Node>> {
    zero_or_more(declaration()) // .dbg("Program")
}

// declaration    → funDecl | varDecl | statement ;
fn declaration<'a>() -> impl Parser<'a, Node> {
    either(var_decl(), statement()) // .dbg("DECLARATION")
}

// funDecl        → "\" function ;
fn fun_decl<'a>() -> impl Parser<'a, (Vec<String>, Node)> {
    pair(tag("fun"), function()).map(|(_, n)| n)
    // .dbg("FUNCTION DEC")
}

// function       → parameter? block;
fn function<'a>() -> impl Parser<'a, (Vec<String>, Node)> {
    pair(parameter(), block()) // .dbg("FUNCTION")
}

// paramenters    → IDENTIFIER ( "," IDENTIFIER )* ;
fn parameter<'a>() -> impl Parser<'a, Vec<String>> {
    zero_or_more(trim(identifier)) // .dbg("Parameters")
}

// varDecl        → IDENTIFIER ( "=" expression )? ;
fn var_decl<'a>() -> impl Parser<'a, Node> {
    either(
        pair(
            pair(
                identifier,     // .dbg("Function Ident"),
                trim(tag("=")), // .dbg("Assign Maybe to Function")),
            ),
            fun_decl(),
        )
        .map(|((ident, _), (param, block))| Node::Variable {
            ident,
            param,
            block: Box::new(block),
        }),
        //.dbg("Var Dec For Function"),
        pair(
            pair(
                identifier,     // .dbg("Var Ident"),
                trim(tag("=")), // .dbg("Assign Maybe to this Var")),
            ),
            statement(),
        )
        .map(|((ident, _), block)| Node::Variable {
            ident,
            param: Vec::new(),
            block: Box::new(block),
        }),
        // .dbg("Failed for Function on to Variable"),
    )
}

// statement      → printStmt | ifStmt | expression | block ;
fn statement<'a>() -> impl Parser<'a, Node> {
    either(
        either(
            print_statement(),
            either(if_else_statement(), if_statement()),
        ),
        either(
            expression(), /*.dbg("EXPRESSION")*/
            block(),      //.dbg("BLOCK"),
        ),
    )
}

// block          → "{" declaration* "}"
fn block<'a>() -> impl Parser<'a, Node> {
    move |input| {
        trim(tag("{"))
            // .dbg("Entering BLOCK MAYBE")
            .parse(input)
            .and_then(|(i1, _)| {
                zero_or_more(declaration())
                    .parse(i1)
                    .map(|(i2, r2)| (i2, r2.iter().map(|n| Box::new(n.clone())).collect()))
                    .map(|(i, r)| (i, Node::Block(r)))
            })
            .and_then(|(i1, r)| {
                trim(tag("}"))
                    // .dbg("EXITING BLOCK MAYBE")
                    .parse(i1)
                    .map(|(i2, _)| (i2, r))
            })
    }
}

// ifStmt         → "if" expression "then" statement ( "else" statement )? ;
fn if_else_statement<'a>() -> impl Parser<'a, Node> {
    move |input| {
        trim(tag("if"))
            .parse(input)
            .and_then(|(i1, _)| expression().parse(i1).map(|(i2, r2)| (i2, r2)))
            .and_then(|(i1, r1)| {
                trim(tag("then"))
                    .parse(i1)
                    .and_then(|(i2, _)| statement().parse(i2).map(|(i3, r2)| (i3, (r1, r2))))
            })
            .and_then(|(i1, (r1, r2))| {
                trim(tag("else"))
                    .parse(i1)
                    .and_then(|(i2, _)| statement().parse(i2).map(|(i3, r3)| (i3, (r1, r2, r3))))
            })
            .map(|(i, (exp, stmt, else_stmt))| {
                (
                    i,
                    Node::Conditional {
                        condition: Box::new(exp),
                        if_branch: Box::new(stmt),
                        else_branch: Some(Box::new(else_stmt)),
                    },
                )
            })
    }
}

// ifStmt         → "if" expression "then" statement ( "else" statement )? ;
fn if_statement<'a>() -> impl Parser<'a, Node> {
    move |input| {
        trim(tag("if"))
            .parse(input)
            .and_then(|(i1, _)| expression().parse(i1).map(|(i2, r2)| (i2, r2)))
            .and_then(|(i1, r1)| {
                trim(tag("then"))
                    .parse(i1)
                    .and_then(|(i2, _)| statement().parse(i2).map(|(i3, r2)| (i3, (r1, r2))))
            })
            .map(|(i, (exp, stmt))| {
                (
                    i,
                    Node::Conditional {
                        condition: Box::new(exp),
                        if_branch: Box::new(stmt),
                        else_branch: None,
                    },
                )
            })
    }
}

// printStmt → "print" expression ";" ;
fn print_statement<'a>() -> impl Parser<'a, Node> {
    pair(tag("print"), expression()).map(|(_, exp)| Node::Print(Box::new(exp)))
    // .dbg("PRINT")
}

// expression → assignment ;
fn expression<'a>() -> impl Parser<'a, Node> {
    logic_or() // .dbg("OR")
}

// logic_or → logic_and ( "or" logic_and )* ;
fn logic_or<'a>() -> impl Parser<'a, Node> {
    either(
        pair(pair(logic_and(), trim(tag("or"))), logic_and()).map(|((and1, _), and2)| {
            Node::BinaryExpr {
                op: Operator::Or,
                rhs: Box::new(and1),
                lhs: Box::new(and2),
            }
        }),
        logic_and(),
    )
}

// logic_and → equality ( "and" equality )* ;
fn logic_and<'a>() -> impl Parser<'a, Node> {
    either(
        pair(pair(equality(), trim(tag("and"))), equality()).map(|((equ1, _), equ2)| {
            Node::BinaryExpr {
                op: Operator::And,
                rhs: Box::new(equ1),
                lhs: Box::new(equ2),
            }
        }),
        equality(),
    )
}

// equality → ( "!=" | "==" ) ;
fn equality_op<'a>() -> impl Parser<'a, Operator> {
    trim(either(
        tag("!=").map(|_| Operator::NotEqual),
        tag("==").map(|_| Operator::Equality),
    ))
}
// equality → comparison ( ( "!=" | "==" ) comparison )* ;
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
// unary          → ( "!" | "-" ) unary | call ;
fn unary<'a>() -> impl Parser<'a, Node> {
    trim(either(call(), either(unary_neg(), unary_bang())))
    // either(unary_neg(), unary_bang())
}
// call           → primary ( arguments? )* ;
fn call<'a>() -> impl Parser<'a, Node> {
    move |input| {
        trim(identifier).parse(input).and_then(|(i1, func_name)| {
            arguments().parse(i1).map(|(i2, args)| {
                (
                    i2,
                    Node::Ident {
                        ident: func_name,
                        args,
                    },
                )
            })
        })
        // .map(|(ident, args)| Node::Ident {
        //     ident: func_name,
        //     args,
        // })
    }
    // pair(identifier, primary())
    //     .dbg("CALL")
    //     .map(|(ident, args)| Node::Ident {
    //         ident,
    //         args: vec![Box::new(args)],
    //     })
}
// arguments      → expression ( "," expression)* ;
fn arguments<'a>() -> impl Parser<'a, Vec<Box<Node>>> {
    zero_or_more(expression())
        .map(|vec_exp| vec_exp.iter().map(|exp| Box::new(exp.clone())).collect())

    // either(
    //     expression().map(|exp| vec![Box::new(exp)]),
    // zero_or_more(pair(trim(tag(",")), expression())).map(|vec_tag_exp| {
    //     vec_tag_exp
    //         .iter()
    //         .map(|(_, exp)| Box::new(exp.clone()))
    //         .collect()
    // })
    // )
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
    // .dbg("Entering into Primary")
}

// primary → IDENTIFIER
fn primary_ident<'a>() -> impl Parser<'a, Node> {
    trim(identifier).map(|ident| Node::Ident {
        ident,
        args: Vec::new(),
    })
    // .dbg("IDENTIFIER")
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
    // .dbg("Int or Float")
}
