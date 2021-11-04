use crate::combinators::{
    either, left, match_literal, pair, quoted_string, right, whitespace_wrap, zero_or_more,
    ParseResult, Parser,
};
use crate::node::{Node, Operator};

fn number(input: &str) -> ParseResult<f64> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_numeric() => matched.push(next),
        _ => return Err(input),
    }

    while let Some(next) = chars.next() {
        if next.is_numeric() || (next == '.' && !matched.contains('.')) {
            matched.push(next);
        } else {
            break;
        }
    }

    let next_index = matched.len();
    match matched.parse::<f64>() {
        Ok(n) => Ok((&input[next_index..], n)),
        Err(_) => panic!("Failed to Convert String to Float"),
    }
}

#[test]
fn number_parse() {
    assert_eq!(Ok(("", 1.)), number("1"));
    assert_eq!(Ok(("", 245.765)), number("245.765"));
    assert_eq!(Err("not a number"), number("not a number"));
}

fn operator_mul_div<'a>() -> impl Parser<'a, Operator> {
    whitespace_wrap(either(
        match_literal("*").map(|_| Operator::Multiply),
        match_literal("/").map(|_| Operator::Divide),
    ))
}

#[test]
fn operator_mul_div_parse() {
    assert_eq!(Ok(("", Operator::Multiply)), operator_mul_div().parse("*"));
    assert_eq!(Ok(("", Operator::Multiply)), operator_mul_div().parse(" *"));
    assert_eq!(Ok(("", Operator::Multiply)), operator_mul_div().parse("* "));
    assert_eq!(
        Ok(("", Operator::Multiply)),
        operator_mul_div().parse(" * ")
    );
    assert_eq!(Ok(("", Operator::Divide)), operator_mul_div().parse("/"));
    assert_eq!(Ok(("", Operator::Divide)), operator_mul_div().parse(" /"));
    assert_eq!(Ok(("", Operator::Divide)), operator_mul_div().parse("/ "));
    assert_eq!(Ok(("", Operator::Divide)), operator_mul_div().parse(" / "));
}

fn operator_add_sub<'a>() -> impl Parser<'a, Operator> {
    whitespace_wrap(either(
        match_literal("+").map(|_| Operator::Plus),
        match_literal("-").map(|_| Operator::Minus),
    ))
}

#[test]
fn operator_add_sub_parse() {
    assert_eq!(Ok(("", Operator::Plus)), operator_add_sub().parse("+"));
    assert_eq!(Ok(("", Operator::Plus)), operator_add_sub().parse(" +"));
    assert_eq!(Ok(("", Operator::Plus)), operator_add_sub().parse("+ "));
    assert_eq!(Ok(("", Operator::Plus)), operator_add_sub().parse(" + "));
    assert_eq!(Ok(("", Operator::Minus)), operator_add_sub().parse("-"));
    assert_eq!(Ok(("", Operator::Minus)), operator_add_sub().parse(" -"));
    assert_eq!(Ok(("", Operator::Minus)), operator_add_sub().parse("- "));
    assert_eq!(Ok(("", Operator::Minus)), operator_add_sub().parse(" - "));
}

fn primary_number<'a>() -> impl Parser<'a, Node> {
    whitespace_wrap(number.map(|num| Node::Int(num)))
}

fn primary_paren<'a>() -> impl Parser<'a, Node> {
    right(match_literal("("), left(expression(), match_literal(")")))
}

fn primary_string<'a>() -> impl Parser<'a, Node> {
    whitespace_wrap(quoted_string().map(|string| Node::Str(string)))
}

fn primary_bool<'a>() -> impl Parser<'a, Node> {
    whitespace_wrap(either(
        match_literal("true").map(|_| Node::True),
        match_literal("false").map(|_| Node::False),
    ))
}

// primary     →  NUMBER | STRING | "true" | "false" | "(" expression ")" ;
fn primary<'a>() -> impl Parser<'a, Node> {
    either(
        either(primary_number(), either(primary_string(), primary_paren())),
        primary_bool(),
    )
}

#[test]
fn primary_parse() {
    assert_eq!(Ok(("", Node::Int(1.))), primary().parse("1"));
    assert_eq!(Ok(("", Node::Int(1.))), primary().parse(" 1."));
    assert_eq!(Ok(("", Node::Int(876.909))), primary().parse(" 876.909 "));
}

fn unary_neg<'a>() -> impl Parser<'a, Node> {
    zero_or_more(whitespace_wrap(match_literal("-")))
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

fn unary_bang<'a>() -> impl Parser<'a, Node> {
    zero_or_more(whitespace_wrap(match_literal("!")))
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

// unary       →  ( "!" | "-" ) unary | primary ;
fn unary<'a>() -> impl Parser<'a, Node> {
    either(unary_neg(), unary_bang())
}

#[test]
fn unary_parse() {
    assert_eq!(
        Ok((
            "",
            Node::UnaryExpr {
                op: Operator::Minus,
                child: Box::new(Node::Int(1.))
            }
        )),
        unary().parse("-1")
    );
    assert_eq!(
        Ok((
            "",
            Node::UnaryExpr {
                op: Operator::Plus,
                child: Box::new(Node::Int(1.))
            }
        )),
        unary().parse("1")
    );
}

// factor      -> unary ( ( "/" | "*" ) unary )* ;
fn factor<'a>() -> impl Parser<'a, Node> {
    either(
        pair(pair(unary(), operator_mul_div()), unary())
            .map(|((lhs, op), rhs)| Node::BinaryExpr {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            })
            .and_then(|node| {
                zero_or_more(pair(operator_mul_div(), unary())).map(move |vec| {
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

#[test]
fn factor_parse() {
    assert_eq!(
        Ok((
            "",
            Node::BinaryExpr {
                op: Operator::Multiply,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(1.)),
                }),
                rhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(1.)),
                }),
            }
        )),
        factor().parse("1 * 1")
    );
    assert_eq!(
        Ok((
            "",
            Node::BinaryExpr {
                op: Operator::Multiply,
                lhs: Box::new(Node::BinaryExpr {
                    op: Operator::Multiply,
                    lhs: Box::new(Node::UnaryExpr {
                        op: Operator::Plus,
                        child: Box::new(Node::Int(1.)),
                    }),
                    rhs: Box::new(Node::UnaryExpr {
                        op: Operator::Plus,
                        child: Box::new(Node::Int(1.)),
                    }),
                }),
                rhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(2.3)),
                }),
            }
        )),
        factor().parse("1 * 1 * 2.3")
    );
    assert_eq!(
        Ok((
            "",
            Node::UnaryExpr {
                op: Operator::Plus,
                child: Box::new(Node::Int(1.))
            }
        )),
        factor().parse("1")
    );
}

// term        -> factor ( ( "-" | "+" ) factor )* ;
fn term<'a>() -> impl Parser<'a, Node> {
    either(
        pair(pair(factor(), operator_add_sub()), factor())
            .map(|((lhs, op), rhs)| Node::BinaryExpr {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            })
            .and_then(|node| {
                zero_or_more(pair(operator_add_sub(), factor())).map(move |vec| {
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

#[test]
fn term_parser() {
    assert_eq!(
        Ok((
            "",
            Node::BinaryExpr {
                op: Operator::Plus,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(1.)),
                }),
                rhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(1.)),
                }),
            }
        )),
        term().parse("1 + 1")
    );
    assert_eq!(
        Ok((
            "",
            Node::UnaryExpr {
                op: Operator::Plus,
                child: Box::new(Node::Int(1.)),
            },
        )),
        term().parse("1")
    );
}

fn comparison_greater<'a>() -> impl Parser<'a, Node> {
    pair(pair(term(), whitespace_wrap(match_literal(">"))), term()).map(|((n1, _), n2)| {
        Node::BinaryExpr {
            op: Operator::GreaterThan,
            lhs: Box::new(n1),
            rhs: Box::new(n2),
        }
    })
}

#[test]
fn comparison_greater_parser() {
    assert_eq!(
        Ok((
            "",
            Node::BinaryExpr {
                op: Operator::GreaterThan,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(1.)),
                }),
                rhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(3.)),
                }),
            }
        )),
        comparison_greater().parse("1 > 3")
    );
}

fn comparison_lesser<'a>() -> impl Parser<'a, Node> {
    pair(pair(term(), whitespace_wrap(match_literal("<"))), term()).map(|((n1, _), n2)| {
        Node::BinaryExpr {
            op: Operator::LessThan,
            lhs: Box::new(n1),
            rhs: Box::new(n2),
        }
    })
}

#[test]
fn comparison_lesser_parser() {
    assert_eq!(
        Ok((
            "",
            Node::BinaryExpr {
                op: Operator::LessThan,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(1.)),
                }),
                rhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(3.)),
                }),
            }
        )),
        comparison_lesser().parse("1 < 3")
    );
}

fn comparison_greater_equal<'a>() -> impl Parser<'a, Node> {
    pair(pair(term(), whitespace_wrap(match_literal(">="))), term()).map(|((n1, _), n2)| {
        Node::BinaryExpr {
            op: Operator::GreaterEqual,
            lhs: Box::new(n1),
            rhs: Box::new(n2),
        }
    })
}

#[test]
fn comparison_greater_equal_parser() {
    assert_eq!(
        Ok((
            "",
            Node::BinaryExpr {
                op: Operator::GreaterEqual,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(1.)),
                }),
                rhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(3.)),
                }),
            }
        )),
        comparison_greater_equal().parse("1 >= 3")
    );
}

fn comparison_lesser_equal<'a>() -> impl Parser<'a, Node> {
    pair(pair(term(), whitespace_wrap(match_literal("<="))), term()).map(|((n1, _), n2)| {
        Node::BinaryExpr {
            op: Operator::LessEqual,
            lhs: Box::new(n1),
            rhs: Box::new(n2),
        }
    })
}

#[test]
fn comparison_lesser_equal_parser() {
    assert_eq!(
        Ok((
            "",
            Node::BinaryExpr {
                op: Operator::LessEqual,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(1.)),
                }),
                rhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(3.)),
                }),
            }
        )),
        comparison_lesser_equal().parse("1 <= 3")
    );
}

// comparison  → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
fn comparison<'a>() -> impl Parser<'a, Node> {
    either(
        either(
            either(comparison_greater(), comparison_lesser()),
            comparison_greater_equal(),
        ),
        comparison_lesser_equal(),
    )
}

#[test]
fn comparison_parser() {
    assert_eq!(
        Ok((
            "",
            Node::BinaryExpr {
                op: Operator::LessEqual,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(1.)),
                }),
                rhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(3.)),
                }),
            }
        )),
        comparison().parse("1 <= 3")
    );
    assert_eq!(
        Ok((
            "",
            Node::UnaryExpr {
                op: Operator::Plus,
                child: Box::new(Node::Int(1.)),
            },
        )),
        comparison().parse("1")
    );
}

fn equality_comparison<'a>() -> impl Parser<'a, Node> {
    pair(
        pair(comparison(), whitespace_wrap(match_literal("=="))),
        comparison(),
    )
    .map(|((n1, _), n2)| Node::BinaryExpr {
        op: Operator::Equality,
        lhs: Box::new(n1),
        rhs: Box::new(n2),
    })
}

#[test]
fn equality_comparison_parser() {
    assert_eq!(
        Ok((
            "",
            Node::BinaryExpr {
                op: Operator::Equality,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(1.)),
                }),
                rhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(3.)),
                }),
            }
        )),
        equality_comparison().parse("1 == 3")
    );
}

fn equality_not_comparison<'a>() -> impl Parser<'a, Node> {
    pair(
        pair(comparison(), whitespace_wrap(match_literal("!="))),
        comparison(),
    )
    .map(|((n1, _), n2)| Node::BinaryExpr {
        op: Operator::NotEqual,
        lhs: Box::new(n1),
        rhs: Box::new(n2),
    })
}

#[test]
fn equality_not_comparison_parser() {
    assert_eq!(
        Ok((
            "",
            Node::BinaryExpr {
                op: Operator::NotEqual,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(1.)),
                }),
                rhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(3.)),
                }),
            }
        )),
        equality_not_comparison().parse("1 != 3")
    );
}

// equality    → comparison ( ( "!=" | "==" ) comparison )* ;
fn equality<'a>() -> impl Parser<'a, Node> {
    either(equality_comparison(), equality_not_comparison())
}

#[test]
fn equality_parser() {
    assert_eq!(
        Ok((
            "",
            Node::BinaryExpr {
                op: Operator::NotEqual,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(1.)),
                }),
                rhs: Box::new(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(Node::Int(3.)),
                }),
            }
        )),
        equality().parse("1 != 3")
    );
    assert_eq!(
        Ok((
            "",
            Node::UnaryExpr {
                op: Operator::Minus,
                child: Box::new(Node::Int(-1.)),
            },
        )),
        equality().parse("-1")
    );
}

pub fn expression<'a>() -> impl Parser<'a, Node> {
    comparison()
}
