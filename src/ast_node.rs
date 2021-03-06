/**
 * 一个简单的语法解析器。
 * 能够解析简单的表达式、变量声明和初始化语句、赋值语句。
 * 它支持的语法规则为：
 *
 * programme -> intDeclare | expressionStatement | assignmentStatement
 * intDeclare -> 'int' Id ( = additive) ';'
 * expressionStatement -> additive ';'
 * additive -> multiplicative ( (+ | -) multiplicative)*
 * multiplicative -> primary ( (* | /) primary)*
 * primary -> IntLiteral | Id | (additive)
*/

pub trait ASTNode {
    fn get_parent(&self) -> Self;
    fn get_children(&self) -> Vec<&Self>;
    fn get_type(&self) -> Self;
    fn get_text(&self) -> String;
}
