use std::iter;

#[derive(Clone, Debug)]
struct Operand {
    symbol: String,
    value: f64,
}

#[derive(Clone)]
struct Operator<T> {
    symbol: String,
    function: T,
}

type UnaryOperator = Operator<fn(f64) -> f64>;
type BinaryOperator = Operator<fn(f64, f64) -> f64>;

#[derive(Clone)]
enum Token {
    Operand(Operand),
    UnaryOperator(UnaryOperator),
    BinaryOperator(BinaryOperator),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Operand(operand) => write!(f, "{}", operand.symbol),
            Token::UnaryOperator(operator) => write!(f, "{}", operator.symbol),
            Token::BinaryOperator(operator) => write!(f, "{}", operator.symbol),
        }
    }
}

struct TokenVec<'a>(&'a [Token]);

impl<'a> std::fmt::Display for TokenVec<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for token in self.0 {
            if !first {
                write!(f, " ")?;
            }
            write!(f, "{}", token)?;
            first = false;
        }
        Ok(())
    }
}

fn is_valid_rpn(tokens: &[Token]) -> bool {
    let mut stack_size: isize = 0;
    if tokens.is_empty() {
        return false;
    }
    for token in tokens {
        match token {
            Token::Operand(_) => {
                stack_size += 1;
            }
            Token::UnaryOperator(_) => {
                if stack_size < 1 {
                    return false;
                }
            }
            Token::BinaryOperator(_) => {
                if stack_size < 2 {
                    return false;
                }
                stack_size -= 1;
            }
        }
        if stack_size < 0 {
            return false;
        }
    }
    stack_size == 1
}

fn calculate(tokens: &[Token]) -> f64 {
    assert!(is_valid_rpn(tokens), "Invalid RPN sequence");

    let mut stack = Vec::new();
    for token in tokens {
        match token {
            Token::Operand(operand) => stack.push(operand.value),
            Token::UnaryOperator(operator) => {
                let value = stack.pop().unwrap();
                stack.push((operator.function)(value));
            }
            Token::BinaryOperator(operator) => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push((operator.function)(left, right));
            }
        }
    }
    stack.pop().unwrap()
}

fn generate_valid_tokens<'a>(
    operands: &'a [Operand],
    unary_operators: &'a [UnaryOperator],
    binary_operators: &'a [BinaryOperator],
    max_depth: usize,
) -> impl Iterator<Item = Vec<Token>> + 'a {
    if operands.is_empty() && max_depth > 0 {}

    (1..=max_depth).flat_map(move |depth| {
        generate_valid_tokens_with_depth(operands, unary_operators, binary_operators, depth)
    })
}

fn aux_generate<'a>(
    operands: &'a [Operand],
    unary_operators: &'a [UnaryOperator],
    binary_operators: &'a [BinaryOperator],
    operands_needed: usize,
    unary_ops_needed: usize,
    binary_ops_needed: usize,
    current_sequence: Vec<Token>,
    stack_size: usize,
) -> Box<dyn Iterator<Item = Vec<Token>> + 'a> {
    if operands_needed == 0 && unary_ops_needed == 0 && binary_ops_needed == 0 {
        if stack_size == 1 {
            return Box::new(iter::once(current_sequence));
        } else {
            return Box::new(iter::empty());
        }
    }

    let operand_iter: Box<dyn Iterator<Item = Vec<Token>> + 'a> =
        if operands_needed > 0 && !operands.is_empty() {
            let current_sequence = current_sequence.clone();
            let iter = operands.iter().flat_map(move |op| {
                let mut next_sequence = current_sequence.clone();
                next_sequence.push(Token::Operand(op.clone()));
                aux_generate(
                    operands,
                    unary_operators,
                    binary_operators,
                    operands_needed - 1,
                    unary_ops_needed,
                    binary_ops_needed,
                    next_sequence,
                    stack_size + 1,
                )
            });
            Box::new(iter)
        } else {
            Box::new(iter::empty())
        };

    let unary_iter: Box<dyn Iterator<Item = Vec<Token>> + 'a> =
        if unary_ops_needed > 0 && stack_size >= 1 && !unary_operators.is_empty() {
            let current_sequence = current_sequence.clone();
            let iter = unary_operators.iter().flat_map(move |uop| {
                let mut next_sequence = current_sequence.clone();
                next_sequence.push(Token::UnaryOperator(uop.clone()));
                aux_generate(
                    operands,
                    unary_operators,
                    binary_operators,
                    operands_needed,
                    unary_ops_needed - 1,
                    binary_ops_needed,
                    next_sequence,
                    stack_size,
                )
            });
            Box::new(iter)
        } else {
            Box::new(iter::empty())
        };

    let binary_iter: Box<dyn Iterator<Item = Vec<Token>> + 'a> =
        if binary_ops_needed > 0 && stack_size >= 2 && !binary_operators.is_empty() {
            let current_sequence = current_sequence.clone();
            let iter = binary_operators.iter().flat_map(move |bop| {
                let mut next_sequence = current_sequence.clone();
                next_sequence.push(Token::BinaryOperator(bop.clone()));
                aux_generate(
                    operands,
                    unary_operators,
                    binary_operators,
                    operands_needed,
                    unary_ops_needed,
                    binary_ops_needed - 1,
                    next_sequence,
                    stack_size - 1,
                )
            });
            Box::new(iter)
        } else {
            Box::new(iter::empty())
        };

    Box::new(operand_iter.chain(unary_iter).chain(binary_iter))
}

fn generate_valid_tokens_with_depth<'a>(
    operands: &'a [Operand],
    unary_operators: &'a [UnaryOperator],
    binary_operators: &'a [BinaryOperator],
    depth: usize,
) -> impl Iterator<Item = Vec<Token>> + 'a {
    if depth == 0 {
        let empty_iter: Box<dyn Iterator<Item = Vec<Token>> + 'a> = Box::new(iter::empty());
        return empty_iter;
    }

    Box::new((0..depth).flat_map(move |unary_ops_needed| {
        let operands_needed = depth - unary_ops_needed;
        let binary_ops_needed = operands_needed - 1;
        aux_generate(
            operands,
            unary_operators,
            binary_operators,
            operands_needed,
            unary_ops_needed,
            binary_ops_needed,
            Vec::new(),
            0,
        )
    }))
}

fn main() {
    let operands = vec![
        Operand {
            symbol: "1.0".to_string(),
            value: 1.0,
        },
        Operand {
            symbol: "2.0".to_string(),
            value: 2.0,
        },
    ];
    let unary_operators = vec![UnaryOperator {
        symbol: "^2".to_string(),
        function: |a| a * a,
    }];
    let binary_operators = vec![BinaryOperator {
        symbol: "+".to_string(),
        function: |a, b| a + b,
    }];
    let max_depth = 2;
    let valid_tokens =
        generate_valid_tokens(&operands, &unary_operators, &binary_operators, max_depth);
    for tokens in valid_tokens {
        println!("{}: {}", TokenVec(&tokens), calculate(&tokens));
    }
}

#[test]
fn test_calculate() {
    let tokens = vec![
        Token::Operand(Operand {
            symbol: "1.0".to_string(),
            value: 1.0,
        }),
        Token::Operand(Operand {
            symbol: "2.0".to_string(),
            value: 2.0,
        }),
        Token::UnaryOperator(UnaryOperator {
            symbol: "^2".to_string(),
            function: |a| a * a,
        }),
        Token::BinaryOperator(BinaryOperator {
            symbol: "+".to_string(),
            function: |a, b| a + b,
        }),
    ];
    let result = calculate(&tokens);
    assert_eq!(result, 5.0);
}
