#![feature(float_gamma)] // 启用浮点数gamma函数特性

mod cli;
mod generator;
mod rpn;
mod types;

use clap::Parser;
use itertools::Itertools; // 导入迭代器工具集
use rayon::prelude::*;
use std::f64::consts::{E, PI}; // 导入数学常量e和π
use tracing::info; // 导入并行迭代器支持

use crate::cli::Args;
use crate::generator::*;
use crate::rpn::*;
use crate::types::*;

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

    if let Some(num_threads) = args.num_threads {
        rayon::ThreadPoolBuilder::default()
            .num_threads(num_threads)
            .build_global()
            .unwrap();
    }

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

#[cfg(test)]
mod tests {
    use crate::rpn::calculate;
    use crate::types::{BinaryOperator, Operand, Token, UnaryOperator};

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
            Token::UnaryOperator(UnaryOperator::new("^2".to_string(), |a| a * a)),
            Token::BinaryOperator(BinaryOperator::new("+".to_string(), |a, b| a + b)),
        ];
        let result = calculate(&tokens);
        assert_eq!(result, 5.0); // 验证计算结果是否正确

        let tokens2 = vec![
            Token::Operand(Operand {
                symbol: "1.0".to_string(),
                value: 1.0,
            }),
            Token::Operand(Operand {
                symbol: "2.0".to_string(),
                value: 2.0,
            }),
            Token::BinaryOperator(BinaryOperator::new("+".to_string(), |a, b| a + b)),
        ];
        let result2 = calculate(&tokens2);
        assert_eq!(result2, 3.0); // 验证计算结果是否正确

        let tokens3 = vec![
            Token::Operand(Operand {
                symbol: "2".to_string(),
                value: 2.0,
            }),
            Token::Operand(Operand {
                symbol: "3".to_string(),
                value: 3.0,
            }),
            Token::BinaryOperator(BinaryOperator::new("+".to_string(), |a, b| a + b)),
            Token::Operand(Operand {
                symbol: "4".to_string(),
                value: 4.0,
            }),
            Token::BinaryOperator(BinaryOperator::new("*".to_string(), |a, b| a * b)),
        ];
        assert_eq!(calculate(&tokens3), 20.0);

        let tokens4 = vec![
            Token::Operand(Operand {
                symbol: "5".to_string(),
                value: 5.0,
            }),
            Token::UnaryOperator(UnaryOperator::new("sqrt".to_string(), |a| a.sqrt())),
        ];
        // Fix the assertion for sqrt(5.0)
        assert!((calculate(&tokens4) - 5.0_f64.sqrt()).abs() < 1e-9);
    }
}
