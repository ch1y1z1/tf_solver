use crate::types::{BinaryOperator, Operand, UnaryOperator};
use std::f64::consts::{E, PI};

pub fn prepare_opes() -> (Vec<Operand>, Vec<UnaryOperator>, Vec<BinaryOperator>) {
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

    (operands, unary_operators, binary_operators)
}
