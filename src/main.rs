use itertools::{Itertools, iproduct};

#[derive(Clone, Debug)]
struct Operand {
    value: f64,
}

#[derive(Clone)]
struct Operator<T> {
    symbol: String,
    function: T,
}
impl<T> std::fmt::Debug for Operator<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Operator")
            .field("symbol", &self.symbol)
            // Cannot easily print function pointer, so omit it or show placeholder
            .field("function", &format_args!("fn(...)"))
            .finish()
    }
}

type UnaryOperator = Operator<fn(f64) -> f64>;
type BinaryOperator = Operator<fn(f64, f64) -> f64>;

#[derive(Clone, Debug)] // Add Clone and Debug
enum Token {
    Operand(Operand),
    UnaryOperator(UnaryOperator),
    BinaryOperator(BinaryOperator),
}

// Helper function to check if a sequence of tokens is a valid RPN expression
fn is_valid_rpn(tokens: &[Token]) -> bool {
    let mut stack_size: isize = 0; // Use isize to allow temporary negative values if needed, though logic prevents it
    if tokens.is_empty() {
        return false; // An empty expression isn't valid in this context
    }
    for token in tokens {
        match token {
            Token::Operand(_) => {
                stack_size += 1;
            }
            Token::UnaryOperator(_) => {
                if stack_size < 1 {
                    return false; // Not enough operands for unary operator
                }
                // stack_size -= 1; // Pop one operand
                // stack_size += 1; // Push result - net change is 0
            }
            Token::BinaryOperator(_) => {
                if stack_size < 2 {
                    return false; // Not enough operands for binary operator
                }
                stack_size -= 1; // Pop two operands, push one result - net change is -1
            }
        }
        // It's also crucial that the stack never becomes empty *before* the final token if operators are involved
        // The checks stack_size < 1 and stack_size < 2 handle this implicitly.
    }
    // A valid RPN expression results in exactly one value left on the stack
    stack_size == 1
}

fn calculate(tokens: Vec<Token>) -> f64 {
    let mut stack = Vec::new();
    for token in tokens {
        match token {
            Token::Operand(operand) => stack.push(operand.value),
            Token::UnaryOperator(operator) => {
                // Check stack size before unwrapping for robustness, although is_valid_rpn should guarantee this
                if stack.is_empty() {
                    panic!("Invalid RPN sequence: unary operator needs 1 operand");
                }
                let value = stack.pop().unwrap();
                stack.push((operator.function)(value));
            }
            Token::BinaryOperator(operator) => {
                // Check stack size before unwrapping
                if stack.len() < 2 {
                    panic!("Invalid RPN sequence: binary operator needs 2 operands");
                }
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push((operator.function)(left, right));
            }
        }
    }
    // Check final stack size
    if stack.len() != 1 {
        panic!("Invalid RPN sequence: final stack size is not 1");
    }
    stack.pop().unwrap()
}

fn generate_valid_tokens(
    operands: &[Operand],
    unary_operators: &[UnaryOperator],
    binary_operators: &[BinaryOperator],
    max_depth: usize,
) -> impl Iterator<Item = Vec<Token>> {
    // Ensure input slices are not empty if they are expected to be used.
    // Although the inner function might handle it, checking here can be clearer.
    // Note: An expression might be valid with only operands (depth 1)
    // or only operands and unary operators.

    (1..=max_depth).flat_map(move |depth| {
        generate_valid_tokens_with_depth(operands, unary_operators, binary_operators, depth)
    })
}

/// Generates valid RPN token sequences based on a definition of 'depth'.
/// Following the comments provided:
/// - `depth` seems to relate to the total count of operands and unary operators.
/// - `unary_operator_num` (`u`) iterates from 0 up to `depth - 1`.
/// - `operand_num` (`n`) is defined as `depth - u`.
/// - A valid RPN sequence with `n` operands requires `n - 1` binary operators (`b`).
fn generate_valid_tokens_with_depth(
    operands: &[Operand],
    unary_operators: &[UnaryOperator],
    binary_operators: &[BinaryOperator],
    depth: usize, // Interpreted as: operand_num + unary_operator_num
) -> impl Iterator<Item = Vec<Token>> {
    (0..depth).flat_map(move |unary_operator_num| {
        let u = unary_operator_num;
        let n = depth - u; // Calculate operand_num based on depth and u

        // We need at least one operand for a meaningful expression that results in a value.
        // If n = 0, this combination is impossible according to RPN rules (need n = b + 1).
        if n == 0 {
            // Return an empty iterator for this invalid configuration
            return itertools::Either::Left(std::iter::empty());
            // Or using Vec::new().into_iter() if Either is not preferred and type matches
            // return Vec::new().into_iter(); // Requires turbofish or type hint
        }

        let b = n - 1; // Calculate required binary_operator_num

        // --- Pre-computation checks ---
        // Check if we have the necessary components available.
        // If we need operands but the operands slice is empty, we can't proceed.
        if n > 0 && operands.is_empty() {
            return itertools::Either::Left(std::iter::empty());
        }
        // If we need unary operators but the unary_operators slice is empty.
        if u > 0 && unary_operators.is_empty() {
            return itertools::Either::Left(std::iter::empty());
        }
        // If we need binary operators but the binary_operators slice is empty.
        if b > 0 && binary_operators.is_empty() {
            return itertools::Either::Left(std::iter::empty());
        }
        // --- End Pre-computation checks ---

        // 1. Generate all combinations (with repetition) of choosing n operands,
        //    u unary operators, and b binary operators.

        // Choose n operands
        let operand_choices = (0..n)
            .map(|_| operands.iter().cloned().map(Token::Operand))
            .multi_cartesian_product(); // Iterator over Vec<Token::Operand>

        // Choose u unary operators
        let unary_choices = (0..u)
            .map(|_| unary_operators.iter().cloned().map(Token::UnaryOperator))
            .multi_cartesian_product(); // Iterator over Vec<Token::UnaryOperator>

        // Choose b binary operators
        let binary_choices = (0..b)
            .map(|_| binary_operators.iter().cloned().map(Token::BinaryOperator))
            .multi_cartesian_product(); // Iterator over Vec<Token::BinaryOperator>

        // 2. Combine these choices using iproduct!
        //    iproduct! handles cases where one or more choice iterators might be empty
        //    (e.g., if u=0 or b=0).
        let combined_choices = iproduct!(operand_choices, unary_choices, binary_choices);

        // 3. For each combination of chosen tokens, generate all unique permutations
        //    and filter them for RPN validity.
        let result_iterator = combined_choices.flat_map(|(op_tokens, un_tokens, bin_tokens)| {
            // Create the pool of tokens for this specific combination
            let token_pool: Vec<Token> = op_tokens
                .into_iter()
                .chain(un_tokens.into_iter())
                .chain(bin_tokens.into_iter())
                .collect();

            // Generate all unique permutations of the token pool
            // The length of token_pool is n + u + b
            let len = token_pool.len();
            token_pool
                .into_iter()
                .permutations(len)
                // Filter each permutation to keep only valid RPN sequences
                .filter(|p| is_valid_rpn(p))
        });

        // Wrap the result iterator in Either to match the return type of the n=0 case
        itertools::Either::Right(result_iterator)
    }) // End of flat_map over unary_operator_num
}

fn main() {
    let operands = vec![Operand { value: 1.0 }, Operand { value: 2.0 }];
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
        println!("{:?}", tokens);
    }
}

// test
#[test]
fn test_calculate() {
    let tokens = vec![
        Token::Operand(Operand { value: 1.0 }),
        Token::Operand(Operand { value: 2.0 }),
        Token::UnaryOperator(UnaryOperator {
            symbol: "^2".to_string(),
            function: |a| a * a,
        }),
        Token::BinaryOperator(BinaryOperator {
            symbol: "+".to_string(),
            function: |a, b| a + b,
        }),
    ];
    let result = calculate(tokens);
    assert_eq!(result, 5.0);
}
