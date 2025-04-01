#![feature(float_gamma)] // 启用浮点数gamma函数特性

use clap::Parser;
use itertools::Itertools; // 导入迭代器工具集
use rayon::prelude::*;
use std::{
    f64::consts::{E, PI}, // 导入数学常量e和π
    iter,
};
use tracing::info; // 导入并行迭代器支持

// 定义操作数结构体，包含符号和数值
#[derive(Clone, Debug)]
struct Operand {
    symbol: String, // 操作数的符号表示
    value: f64,     // 操作数的实际数值
}

// 定义运算符结构体，包含符号和对应的函数
#[derive(Clone)]
struct Operator<T> {
    symbol: String, // 运算符的符号表示
    function: T,    // 运算符对应的函数
}

// 定义一元运算符和二元运算符的类型别名
type UnaryOperator = Operator<fn(f64) -> f64>; // 一元运算符：接收一个f64参数，返回f64
type BinaryOperator = Operator<fn(f64, f64) -> f64>; // 二元运算符：接收两个f64参数，返回f64

// 为Operator实现构造函数
impl<T> Operator<T> {
    fn new(symbol: String, function: T) -> Self {
        Self { symbol, function }
    }
}

// 定义Token枚举，表示表达式中的各种元素
#[derive(Clone)]
enum Token {
    Operand(Operand),               // 操作数
    UnaryOperator(UnaryOperator),   // 一元运算符
    BinaryOperator(BinaryOperator), // 二元运算符
}

// 为Token实现Display trait，用于打印输出
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Operand(operand) => write!(f, "{}", operand.symbol),
            Token::UnaryOperator(operator) => write!(f, "{}", operator.symbol),
            Token::BinaryOperator(operator) => write!(f, "{}", operator.symbol),
        }
    }
}

// 定义TokenVec结构体，用于表示Token切片
struct TokenVec<'a>(&'a [Token]);

// 为TokenVec实现Display trait，用于打印输出
impl<'a> std::fmt::Display for TokenVec<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for token in self.0 {
            if !first {
                write!(f, " ")?; // 在token之间添加空格
            }
            write!(f, "{}", token)?;
            first = false;
        }
        Ok(())
    }
}

// 检查RPN（逆波兰表达式）是否有效
fn is_valid_rpn(tokens: &[Token]) -> bool {
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
            }
            Token::BinaryOperator(_) => {
                if stack_size < 2 {
                    return false; // 二元运算符需要至少两个操作数
                }
                stack_size -= 1; // 二元运算符消耗两个操作数，产生一个结果
            }
        }
        if stack_size < 0 {
            return false; // 栈大小不能为负
        }
    }
    stack_size == 1 // 最终栈中应该只有一个结果
}

// 计算RPN表达式的值
fn calculate(tokens: &[Token]) -> f64 {
    assert!(is_valid_rpn(tokens), "Invalid RPN sequence"); // 确保RPN序列有效

    let mut stack = Vec::new(); // 使用向量模拟栈
    for token in tokens {
        match token {
            Token::Operand(operand) => stack.push(operand.value), // 操作数直接入栈
            Token::UnaryOperator(operator) => {
                let value = stack.pop().unwrap(); // 弹出操作数
                stack.push((operator.function)(value)); // 应用一元运算符并压入结果
            }
            Token::BinaryOperator(operator) => {
                let right = stack.pop().unwrap(); // 弹出右操作数
                let left = stack.pop().unwrap(); // 弹出左操作数
                stack.push((operator.function)(left, right)); // 应用二元运算符并压入结果
            }
        }
    }
    stack.pop().unwrap() // 返回最终结果
}

// 生成有效的token序列
fn generate_valid_tokens<'a>(
    operands: &'a [Operand],                // 可用的操作数列表
    unary_operators: &'a [UnaryOperator],   // 可用的一元运算符列表
    binary_operators: &'a [BinaryOperator], // 可用的二元运算符列表
    max_depth: usize,                       // 最大深度限制
) -> impl Iterator<Item = Vec<Token>> + 'a {
    if operands.is_empty() && max_depth > 0 {}

    // 从1到max_depth遍历所有可能的深度
    (1..=max_depth).flat_map(move |depth| {
        generate_valid_tokens_with_depth(operands, unary_operators, binary_operators, depth)
    })
}

// 辅助函数：生成指定深度的有效token序列
// 该函数使用递归方式生成所有可能的有效RPN表达式
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
    // 当还需要操作数且操作数列表不为空时生成
    let operand_iter: Box<dyn Iterator<Item = Vec<Token>> + 'a> =
        if operands_needed > 0 && !operands.is_empty() {
            // 克隆当前序列以避免修改原始序列
            let current_sequence = current_sequence.clone();
            // 遍历所有可用的操作数
            let iter = operands.iter().flat_map(move |op| {
                // 创建新的序列并添加当前操作数
                let mut next_sequence = current_sequence.clone();
                next_sequence.push(Token::Operand(op.clone()));
                // 递归生成剩余部分，栈大小加1
                aux_generate(
                    operands,
                    unary_operators,
                    binary_operators,
                    operands_needed - 1, // 减少需要的操作数数量
                    unary_ops_needed,    // 一元运算符数量不变
                    binary_ops_needed,   // 二元运算符数量不变
                    next_sequence,
                    stack_size + 1, // 栈大小加1
                )
            });
            Box::new(iter)
        } else {
            Box::new(iter::empty())
        };

    // 生成一元运算符的迭代器
    // 当还需要一元运算符、栈大小至少为1且一元运算符列表不为空时生成
    let unary_iter: Box<dyn Iterator<Item = Vec<Token>> + 'a> =
        if unary_ops_needed > 0 && stack_size >= 1 && !unary_operators.is_empty() {
            // 克隆当前序列以避免修改原始序列
            let current_sequence = current_sequence.clone();
            // 遍历所有可用的一元运算符
            let iter = unary_operators.iter().flat_map(move |uop| {
                // 创建新的序列并添加当前一元运算符
                let mut next_sequence = current_sequence.clone();
                next_sequence.push(Token::UnaryOperator(uop.clone()));
                // 递归生成剩余部分，栈大小保持不变
                aux_generate(
                    operands,
                    unary_operators,
                    binary_operators,
                    operands_needed,      // 操作数数量不变
                    unary_ops_needed - 1, // 减少需要的一元运算符数量
                    binary_ops_needed,    // 二元运算符数量不变
                    next_sequence,
                    stack_size, // 栈大小不变
                )
            });
            Box::new(iter)
        } else {
            Box::new(iter::empty())
        };

    // 生成二元运算符的迭代器
    // 当还需要二元运算符、栈大小至少为2且二元运算符列表不为空时生成
    let binary_iter: Box<dyn Iterator<Item = Vec<Token>> + 'a> =
        if binary_ops_needed > 0 && stack_size >= 2 && !binary_operators.is_empty() {
            // 克隆当前序列以避免修改原始序列
            let current_sequence = current_sequence.clone();
            // 遍历所有可用的二元运算符
            let iter = binary_operators.iter().flat_map(move |bop| {
                // 创建新的序列并添加当前二元运算符
                let mut next_sequence = current_sequence.clone();
                next_sequence.push(Token::BinaryOperator(bop.clone()));
                // 递归生成剩余部分，栈大小减1
                aux_generate(
                    operands,
                    unary_operators,
                    binary_operators,
                    operands_needed,       // 操作数数量不变
                    unary_ops_needed,      // 一元运算符数量不变
                    binary_ops_needed - 1, // 减少需要的二元运算符数量
                    next_sequence,
                    stack_size - 1, // 栈大小减1
                )
            });
            Box::new(iter)
        } else {
            Box::new(iter::empty())
        };

    // 合并所有迭代器并返回
    // 使用chain方法将三个迭代器连接成一个
    Box::new(operand_iter.chain(unary_iter).chain(binary_iter))
}

// 生成指定深度的有效token序列
fn generate_valid_tokens_with_depth<'a>(
    operands: &'a [Operand],                // 可用的操作数列表
    unary_operators: &'a [UnaryOperator],   // 可用的一元运算符列表
    binary_operators: &'a [BinaryOperator], // 可用的二元运算符列表
    depth: usize,                           // 目标深度
) -> impl Iterator<Item = Vec<Token>> + 'a {
    // 基本情况：深度为0时返回空迭代器
    if depth == 0 {
        let empty_iter: Box<dyn Iterator<Item = Vec<Token>> + 'a> = Box::new(iter::empty());
        return empty_iter;
    }

    // 遍历所有可能的一元运算符数量
    Box::new((0..depth).flat_map(move |unary_ops_needed| {
        let operands_needed = depth - unary_ops_needed; // 需要的操作数数量
        let binary_ops_needed = operands_needed - 1; // 需要的二元运算符数量
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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 't', long)]
    target: f64,
    #[arg(short = 'd', long, default_value_t = 6)]
    max_depth: usize,
    #[arg(short = 'e', long, default_value_t = 1e-2)]
    tolerance: f64,
    #[arg(short = 'o', long)]
    output: Option<String>,
    #[arg(short = 'c', long, default_value_t = 2 ^ 16)]
    chunk_size: usize,
}

// 主函数
fn main() {
    let args = Args::parse();
    let file_appender = if let Some(output_file) = &args.output {
        Some(tracing_appender::rolling::RollingFileAppender::new(
            tracing_appender::rolling::Rotation::NEVER,
            ".",         // 文件夹
            output_file, // 文件路径
        ))
    } else {
        None
    };

    // 初始化日志订阅器
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(false)
        .with_writer(std::io::stdout);

    if let Some(appender) = file_appender {
        subscriber.with_writer(appender).init();
    } else {
        subscriber.init();
    }

    // 定义基本操作数：e和π
    let operands = vec![
        Operand {
            symbol: "e".to_string(),
            value: E,
        },
        Operand {
            symbol: "pi".to_string(),
            value: PI,
        },
        Operand {
            symbol: "γ".to_string(),
            value: 0.57721566490153286060651209,
        },
    ];

    // 定义所有可用的一元运算符
    let unary_operators = vec![
        UnaryOperator::new("sin".to_string(), |a| a.sin()),
        UnaryOperator::new("cos".to_string(), |a| a.cos()),
        UnaryOperator::new("tan".to_string(), |a| a.tan()),
        UnaryOperator::new("asin".to_string(), |a| a.asin()),
        UnaryOperator::new("acos".to_string(), |a| a.acos()),
        UnaryOperator::new("atan".to_string(), |a| a.atan()),
        UnaryOperator::new("sinh".to_string(), |a| a.sinh()),
        UnaryOperator::new("cosh".to_string(), |a| a.cosh()),
        UnaryOperator::new("tanh".to_string(), |a| a.tanh()),
        UnaryOperator::new("asinh".to_string(), |a| a.asinh()),
        UnaryOperator::new("acosh".to_string(), |a| a.acosh()),
        UnaryOperator::new("atanh".to_string(), |a| a.atanh()),
        UnaryOperator::new("coth".to_string(), |a| a.cosh() / a.sinh()),
        UnaryOperator::new("csch".to_string(), |a| 1.0 / a.sinh()),
        UnaryOperator::new("sech".to_string(), |a| 1.0 / a.cosh()),
        UnaryOperator::new("cot".to_string(), |a| a.cos() / a.sin()),
        UnaryOperator::new("csc".to_string(), |a| 1.0 / a.sin()),
        UnaryOperator::new("sec".to_string(), |a| 1.0 / a.cos()),
        UnaryOperator::new(
            "sinc".to_string(),
            |a| if a == 0.0 { 1.0 } else { a.sin() / a },
        ),
        UnaryOperator::new("sqrt".to_string(), |a| a.sqrt()),
        UnaryOperator::new("abs".to_string(), |a| a.abs()),
        UnaryOperator::new("ln".to_string(), |a| a.ln()),
        UnaryOperator::new("log".to_string(), |a| a.log10()),
        UnaryOperator::new("gamma".to_string(), |a| a.gamma()),
        UnaryOperator::new("!".to_string(), |a| (a - 1.0).gamma()),
        UnaryOperator::new("floor".to_string(), |a| a.floor()),
        UnaryOperator::new("ceil".to_string(), |a| a.ceil()),
    ];

    // 定义所有可用的二元运算符
    let binary_operators = vec![
        BinaryOperator::new("+".to_string(), |a, b| a + b),
        BinaryOperator::new("-".to_string(), |a, b| a - b),
        BinaryOperator::new("*".to_string(), |a, b| a * b),
        BinaryOperator::new("/".to_string(), |a, b| a / b),
        BinaryOperator::new("^".to_string(), |a, b| a.powf(b)),
        BinaryOperator::new("mod".to_string(), |a, b| a % b),
        BinaryOperator::new("min".to_string(), |a, b| a.min(b)),
        BinaryOperator::new("max".to_string(), |a, b| a.max(b)),
        BinaryOperator::new("atan2".to_string(), |a, b| a.atan2(b)),
    ];

    let max_depth = args.max_depth; // 设置最大深度
    let valid_tokens =
        generate_valid_tokens(&operands, &unary_operators, &binary_operators, max_depth);

    // 并行处理生成的token序列
    valid_tokens
        .chunks(args.chunk_size)
        .into_iter()
        .for_each(|chunk| {
            chunk
                .collect::<Vec<_>>()
                .par_iter()
                .filter(|tokens| (calculate(&tokens) - args.target).abs() < args.tolerance) // 筛选结果接近613的表达式
                .for_each(|tokens| {
                    println!("{}: {}", TokenVec(&tokens), calculate(&tokens));
                    info!("{}: {}", TokenVec(&tokens), calculate(&tokens));
                });
        });
}

// 测试calculate函数的单元测试
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
    assert_eq!(result, 5.0); // 验证计算结果是否正确
}
