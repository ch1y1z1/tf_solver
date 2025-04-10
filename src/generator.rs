// src/generator.rs
use crate::types::{BinaryOperator, Operand, Token, UnaryOperator};
use std::iter;

/// 生成有效的token序列
pub fn generate_valid_tokens<'a>(
    operands: &'a [Operand],                // 可用的操作数列表
    unary_operators: &'a [UnaryOperator],   // 可用的一元运算符列表
    binary_operators: &'a [BinaryOperator], // 可用的二元运算符列表
    max_depth: usize,                       // 最大深度限制
) -> impl Iterator<Item = Vec<Token>> + 'a {
    // 从1到max_depth遍历所有可能的深度
    (1..=max_depth).flat_map(move |depth| {
        generate_valid_tokens_with_depth(operands, unary_operators, binary_operators, depth)
    })
}

/// 辅助函数：生成指定深度的有效token序列
/// 该函数使用递归方式生成所有可能的有效RPN表达式
fn aux_generate<'a>(
    operands: &'a [Operand],                // 可用的操作数列表
    unary_operators: &'a [UnaryOperator],   // 可用的一元运算符列表
    binary_operators: &'a [BinaryOperator], // 可用的二元运算符列表
    operands_needed: usize,                 // 还需要多少个操作数
    unary_ops_needed: usize,                // 还需要多少个一元运算符
    binary_ops_needed: usize,               // 还需要多少个二元运算符
    current_sequence: Vec<Token>,           // 当前已生成的序列
    stack_size: usize,                      // 当前栈的大小
) -> Box<dyn Iterator<Item = Vec<Token>> + 'a> {
    // 基本情况：所有需要的token都已生成
    // 此时检查栈大小是否为1，表示表达式有效
    if operands_needed == 0 && unary_ops_needed == 0 && binary_ops_needed == 0 {
        if stack_size == 1 {
            // 栈大小为1表示序列有效，返回当前序列
            return Box::new(iter::once(current_sequence));
        } else {
            // 栈大小不为1表示序列无效，返回空迭代器
            return Box::new(iter::empty());
        }
    }

    // 生成操作数的迭代器
    let operand_iter: Box<dyn Iterator<Item = Vec<Token>> + 'a> =
        if operands_needed > 0 && !operands.is_empty() {
            let current_sequence_clone = current_sequence.clone();
            let iter = operands.iter().flat_map(move |op| {
                let mut next_sequence = current_sequence_clone.clone();
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

    // 生成一元运算符的迭代器
    let unary_iter: Box<dyn Iterator<Item = Vec<Token>> + 'a> =
        if unary_ops_needed > 0 && stack_size >= 1 && !unary_operators.is_empty() {
            let current_sequence_clone = current_sequence.clone();
            let iter = unary_operators.iter().flat_map(move |uop| {
                let mut next_sequence = current_sequence_clone.clone();
                next_sequence.push(Token::UnaryOperator(uop.clone()));
                aux_generate(
                    operands,
                    unary_operators,
                    binary_operators,
                    operands_needed,
                    unary_ops_needed - 1,
                    binary_ops_needed,
                    next_sequence,
                    stack_size, // 一元操作符不改变栈大小
                )
            });
            Box::new(iter)
        } else {
            Box::new(iter::empty())
        };

    // 生成二元运算符的迭代器
    let binary_iter: Box<dyn Iterator<Item = Vec<Token>> + 'a> =
        if binary_ops_needed > 0 && stack_size >= 2 && !binary_operators.is_empty() {
            let current_sequence_clone = current_sequence.clone();
            let iter = binary_operators.iter().flat_map(move |bop| {
                let mut next_sequence = current_sequence_clone.clone();
                next_sequence.push(Token::BinaryOperator(bop.clone()));
                aux_generate(
                    operands,
                    unary_operators,
                    binary_operators,
                    operands_needed,
                    unary_ops_needed,
                    binary_ops_needed - 1,
                    next_sequence,
                    stack_size - 1, // 二元操作符使栈大小减1
                )
            });
            Box::new(iter)
        } else {
            Box::new(iter::empty())
        };

    // 合并所有生成的迭代器
    Box::new(operand_iter.chain(unary_iter).chain(binary_iter))
}


/// 生成指定深度的有效token序列
pub fn generate_valid_tokens_with_depth<'a>(
    operands: &'a [Operand],                // 可用的操作数列表
    unary_operators: &'a [UnaryOperator],   // 可用的一元运算符列表
    binary_operators: &'a [BinaryOperator], // 可用的二元运算符列表
    depth: usize,                           // 目标深度
) -> Box<dyn Iterator<Item = Vec<Token>> + 'a> { // Changed return type to Box<dyn Iterator>
    // 计算需要多少个操作数和运算符才能达到指定的深度
    // RPN 中，n 个操作数需要 n-1 个二元运算符
    // 简单的估计：depth 大约等于操作数数量
    let num_operands = depth; // 简化假设深度等于操作数数量

    if depth == 0 {
        // Depth 0 doesn't make sense for RPN, return empty.
        return Box::new(iter::empty());
    }

    let num_binary_ops = depth - 1;

    // We need to generate sequences with *exactly* num_operands and num_binary_ops,
    // potentially including some unary operators. How many unary operators?
    // The original code didn't explicitly limit unary operators based on depth.
    // Let's generate sequences with exactly `num_operands` operands and `num_binary_ops` binary operators,
    // allowing *up to* some number of unary operators. What limit? Let's try `depth`.
    let max_unary_ops = depth; // Arbitrary limit, needs refinement

    Box::new((0..=max_unary_ops)
        .flat_map(move |num_unary_ops| {
            aux_generate(
                operands,
                unary_operators,
                binary_operators,
                num_operands,
                num_unary_ops,
                num_binary_ops,
                Vec::new(), // Start with an empty sequence
                0,          // Start with stack size 0
            )
        })
        // Filter results that might not be valid RPN (although aux_generate aims for valid structure)
        // It's better to ensure aux_generate *only* produces valid RPN.
        // The base case `stack_size == 1` should guarantee validity if recursion is correct.
        // .filter(|tokens| crate::rpn::is_valid_rpn(tokens)) // Removed potential redundant check
    )
}
