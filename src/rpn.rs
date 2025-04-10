// src/rpn.rs
use crate::types::{BinaryOperator, Token, UnaryOperator};

/// 检查RPN（逆波兰表达式）是否有效
pub fn is_valid_rpn(tokens: &[Token]) -> bool {
    let mut stack_size: isize = 0; // 用于跟踪栈的大小
    if tokens.is_empty() {
        return false; // 空序列无效
    }
    for token in tokens {
        match token {
            Token::Operand(_) => {
                stack_size += 1; // 操作数入栈
            }
            Token::UnaryOperator(_) => {
                if stack_size < 1 {
                    return false; // 一元运算符需要至少一个操作数
                }
                // 一元运算符消耗一个操作数，产生一个结果，栈大小不变
            }
            Token::BinaryOperator(_) => {
                if stack_size < 2 {
                    return false; // 二元运算符需要至少两个操作数
                }
                stack_size -= 1; // 二元运算符消耗两个操作数，产生一个结果
            }
        }
        if stack_size < 0 {
            // This check might be redundant if logic above is correct, but good for safety.
            return false; // 栈大小不能为负
        }
    }
    stack_size == 1 // 最终栈中应该只有一个结果
}

/// 计算RPN表达式的值
///
/// # Panics
///
/// 如果输入的 `tokens` 不是一个有效的 RPN 序列，该函数会 panic。
/// 调用者应该首先使用 `is_valid_rpn` 来验证输入。
pub fn calculate(tokens: &[Token]) -> f64 {
    // 虽然主逻辑会检查，但这里加断言更明确
    assert!(is_valid_rpn(tokens), "Invalid RPN sequence passed to calculate");

    let mut stack = Vec::new(); // 使用向量模拟栈
    for token in tokens {
        match token {
            Token::Operand(operand) => stack.push(operand.value), // 操作数直接入栈
            Token::UnaryOperator(UnaryOperator { function, .. }) => {
                // is_valid_rpn 保证了此时栈不为空
                let value = stack.pop().unwrap(); // 弹出操作数
                stack.push(function(value)); // 应用一元运算符并压入结果
            }
            Token::BinaryOperator(BinaryOperator { function, .. }) => {
                 // is_valid_rpn 保证了此时栈至少有两个元素
                let right = stack.pop().unwrap(); // 弹出右操作数
                let left = stack.pop().unwrap(); // 弹出左操作数
                stack.push(function(left, right)); // 应用二元运算符并压入结果
            }
        } // End match
    }
    // is_valid_rpn 保证了最终栈中只有一个元素
    stack.pop().unwrap() // 返回最终结果
}
