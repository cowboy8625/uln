use crate::node::{Node, Operator};
use npc::{
    either, identifier, left, number, pair, quoted_string, right, tag, trim, zero_or_more, Error,
    Parser,
};

const FUNCTION_DECL: &str = "fn";
const EQUALS: &str = "=";
const PLUS: &str = "+";
const MINUS: &str = "-";
const DIVID: &str = "/";
const MULTIPLY: &str = "*";
const LBRACE: &str = "{";
const RBRACE: &str = "}";
const LPAREN: &str = "(";
const RPAREN: &str = ")";
const IF: &str = "if";
const THEN: &str = "then";
const ELSE: &str = "else";
const LESS_THEN: &str = "<";
const LESS_EQUAL: &str = "<=";
const GREATER_THAN: &str = ">";
const GREATER_EQUAL: &str = ">=";
const NOT_EQUAL: &str = "!=";
const EQUAL_EQUAL: &str = "==";
const TRUE: &str = "true";
const FALSE: &str = "false";

// program        → declaration* EOF ;
pub fn program<'a>() -> impl Parser<'a, Vec<Node>> {
    zero_or_more(declaration())
}

// declaration    → funDecl | varDecl | statement ;
fn declaration<'a>() -> impl Parser<'a, Node> {
    either(var_decl(), statement())
}

// funDecl        → "fn" function ;
fn fun_decl<'a>() -> impl Parser<'a, (Vec<String>, Node)> {
    pair(tag(FUNCTION_DECL), function()).map(|(_, n)| n)
}

// function       → parameter? block;
fn function<'a>() -> impl Parser<'a, (Vec<String>, Node)> {
    pair(parameter(), block())
}

// paramenters    → IDENTIFIER ( IDENTIFIER )* ;
fn parameter<'a>() -> impl Parser<'a, Vec<String>> {
    zero_or_more(trim(identifier))
}

// varDecl        → IDENTIFIER ( "=" expression )? ;
fn var_decl<'a>() -> impl Parser<'a, Node> {
    either(
        pair(pair(identifier, trim(tag(EQUALS))), fun_decl()).map(
            |((ident, _), (param, block))| Node::Variable {
                ident,
                param,
                block: Box::new(block),
                environment: None,
            },
        ),
        pair(pair(identifier, trim(tag(EQUALS))), statement()).map(|((ident, _), block)| {
            Node::Variable {
                ident,
                param: Vec::new(),
                block: Box::new(block),
                environment: None,
            }
        }),
    )
}

// statement      → printStmt | ifStmt | expression | block ;
fn statement<'a>() -> impl Parser<'a, Node> {
    either(
        either(
            print_statement(),
            either(if_else_statement(), if_statement()),
        ),
        either(expression(), block()),
    )
}

// block          → "{" declaration* "}"
fn block<'a>() -> impl Parser<'a, Node> {
    move |input| {
        trim(tag(LBRACE))
            .parse(input)
            .and_then(|(i1, _)| {
                zero_or_more(declaration())
                    .parse(i1)
                    .map(|(i2, r2)| (i2, r2.iter().map(|n| Box::new(n.clone())).collect()))
                    .map(|(i, r)| (i, Node::Block(r)))
            })
            .and_then(|(i1, r)| trim(tag(RBRACE)).parse(i1).map(|(i2, _)| (i2, r)))
    }
}

// ifStmt         → "if" expression "then" statement ( "else" statement )? ;
fn if_else_statement<'a>() -> impl Parser<'a, Node> {
    move |input| {
        trim(tag(IF))
            .parse(input)
            .and_then(|(i1, _)| expression().parse(i1).map(|(i2, r2)| (i2, r2)))
            .and_then(|(i1, r1)| {
                trim(tag(THEN))
                    .parse(i1)
                    .and_then(|(i2, _)| statement().parse(i2).map(|(i3, r2)| (i3, (r1, r2))))
            })
            .and_then(|(i1, (r1, r2))| {
                trim(tag(ELSE))
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
}

// expression → assignment ;
fn expression<'a>() -> impl Parser<'a, Node> {
    logic_or()
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
        tag(NOT_EQUAL).map(|_| Operator::NotEqual),
        tag(EQUAL_EQUAL).map(|_| Operator::Equality),
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
            tag(LESS_EQUAL).map(|_| Operator::LessEqual),
            tag(LESS_THEN).map(|_| Operator::LessThan),
        ),
        either(
            tag(GREATER_EQUAL).map(|_| Operator::GreaterEqual),
            tag(GREATER_THAN).map(|_| Operator::GreaterThan),
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
        tag(MINUS).map(|_| Operator::Minus),
        tag(PLUS).map(|_| Operator::Plus),
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
        tag(DIVID).map(|_| Operator::Divide),
        tag(MULTIPLY).map(|_| Operator::Multiply),
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

// unary          → ( "!" | "-" ) unary | call ;
fn unary<'a>() -> impl Parser<'a, Node> {
    trim(either(call(), either(unary_neg(), unary_bang())))
}

// call           → primary ( arguments? )* ;
fn call<'a>() -> impl Parser<'a, Node> {
    move |input: (String, Option<Error<String>>)| {
        trim(identifier)
            .parse(input.clone())
            .and_then(|(i1, func_name)| match func_name.as_str() {
                "true" | "false" => return Err(input),
                _ => arguments().parse(i1).map(|(i2, args)| {
                    (
                        i2,
                        Node::Ident {
                            ident: func_name,
                            args,
                        },
                    )
                }),
            })
    }
}
// arguments      → expression ( "," expression)* ;
fn arguments<'a>() -> impl Parser<'a, Vec<Box<Node>>> {
    zero_or_more(either(
        expression(),
        fun_decl().map(|(param, block)| Node::Variable {
            ident: "".into(),
            param,
            block: Box::new(block),
            environment: None,
        }),
    ))
    .map(|vec_exp| vec_exp.iter().map(|exp| Box::new(exp.clone())).collect())
}

// unary → "-"
fn unary_neg<'a>() -> impl Parser<'a, Node> {
    zero_or_more(trim(tag(MINUS)))
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
            either(primary_bool(), primary_string()),
            either(primary_number(), primary_paren()),
        ),
        primary_ident(),
    )
}

// primary → IDENTIFIER
fn primary_ident<'a>() -> impl Parser<'a, Node> {
    trim(identifier).map(|ident| Node::Ident {
        ident,
        args: Vec::new(),
    })
}

// primary → "(" expression ")"
fn primary_paren<'a>() -> impl Parser<'a, Node> {
    right(tag(LPAREN), left(declaration(), tag(RPAREN)))
}

// primary → BOOL
fn primary_bool<'a>() -> impl Parser<'a, Node> {
    either(
        tag(TRUE).map(|_| Node::True),
        tag(FALSE).map(|_| Node::False),
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
