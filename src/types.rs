// src/types.rs
use std::f64;

// 定义操作数结构体，包含符号和数值
#[derive(Clone, Debug)]
pub struct Operand {
    pub symbol: String, // 操作数的符号表示
    pub value: f64,     // 操作数的实际数值
}

// 定义运算符结构体，包含符号和对应的函数
#[derive(Clone)]
pub struct Operator<T> {
    pub symbol: String, // 运算符的符号表示
    pub function: T,    // 运算符对应的函数
}

// 定义一元运算符和二元运算符的类型别名
pub type UnaryOperator = Operator<fn(f64) -> f64>; // 一元运算符：接收一个f64参数，返回f64
pub type BinaryOperator = Operator<fn(f64, f64) -> f64>; // 二元运算符：接收两个f64参数，返回f64

// 为Operator实现构造函数
impl<T> Operator<T> {
    pub fn new(symbol: String, function: T) -> Self {
        Self { symbol, function }
    }
}

// 定义Token枚举，表示表达式中的各种元素
#[derive(Clone)]
pub enum Token {
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
pub struct TokenVec<'a>(pub &'a [Token]);

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
