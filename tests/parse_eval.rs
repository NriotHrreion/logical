use std::collections::HashMap;

use logical::{logic::{eval::eval_ast, parser::parse_to_ast}, mode::Mode};

fn eval_expr(expr: &str) -> Result<bool, String> {
	let ast = parse_to_ast(expr, Mode::Default, false)?;
	eval_ast(ast, &HashMap::new())
}

#[test]
fn eval_literal_values() {
	assert_eq!(eval_expr("0").unwrap(), false);
	assert_eq!(eval_expr("1").unwrap(), true);
}

#[test]
fn eval_not_operator() {
	assert_eq!(eval_expr("!0").unwrap(), true);
	assert_eq!(eval_expr("!1").unwrap(), false);
}

#[test]
fn eval_and_operator() {
	assert_eq!(eval_expr("1&1").unwrap(), true);
	assert_eq!(eval_expr("1&0").unwrap(), false);
	assert_eq!(eval_expr("0&1").unwrap(), false);
	assert_eq!(eval_expr("0&0").unwrap(), false);
}

#[test]
fn eval_or_operator() {
	assert_eq!(eval_expr("1|1").unwrap(), true);
	assert_eq!(eval_expr("1|0").unwrap(), true);
	assert_eq!(eval_expr("0|1").unwrap(), true);
	assert_eq!(eval_expr("0|0").unwrap(), false);
}

#[test]
fn eval_strict_or_operator() {
	assert_eq!(eval_expr("1||1").unwrap(), false);
	assert_eq!(eval_expr("1||0").unwrap(), true);
	assert_eq!(eval_expr("0||1").unwrap(), true);
	assert_eq!(eval_expr("0||0").unwrap(), false);
}

#[test]
fn eval_chained_operators_without_parentheses() {
	assert_eq!(eval_expr("1&1&0").unwrap(), false);
	assert_eq!(eval_expr("0|0|1").unwrap(), true);
	assert_eq!(eval_expr("!1|1").unwrap(), true);
	assert_eq!(eval_expr("!0&1").unwrap(), true);
	assert_eq!(eval_expr("1|0&0").unwrap(), false);
	assert_eq!(eval_expr("1|0&!0").unwrap(), true);
	assert_eq!(eval_expr("1&!1").unwrap(), false);
}

#[test]
fn eval_implication_operators() {
	assert_eq!(eval_expr("1>0").unwrap(), false);
	assert_eq!(eval_expr("1>1").unwrap(), true);
	assert_eq!(eval_expr("0>0").unwrap(), true);
	assert_eq!(eval_expr("0>1").unwrap(), true);

	assert_eq!(eval_expr("1<0").unwrap(), true);
	assert_eq!(eval_expr("0<1").unwrap(), false);

	assert_eq!(eval_expr("1-1").unwrap(), true);
	assert_eq!(eval_expr("1-0").unwrap(), false);

	assert_eq!(eval_expr("1>1>0").unwrap(), false);
	assert_eq!(eval_expr("1>0>1").unwrap(), true);
}

#[test]
fn eval_chained_not_operators() {
	assert_eq!(eval_expr("!!1").unwrap(), true);
	assert_eq!(eval_expr("!!!1").unwrap(), false);
	assert_eq!(eval_expr("!!0").unwrap(), false);
	assert_eq!(eval_expr("!!!0").unwrap(), true);
}

#[test]
fn eval_parenthesized_expressions() {
	assert_eq!(eval_expr("(1)").unwrap(), true);
	assert_eq!(eval_expr("(0)").unwrap(), false);
	assert_eq!(eval_expr("(1&0)").unwrap(), false);
	assert_eq!(eval_expr("(1|0)").unwrap(), true);
	assert_eq!(eval_expr("1&(0|1)").unwrap(), true);
	assert_eq!(eval_expr("1&(0|0)").unwrap(), false);
	assert_eq!(eval_expr("(1&0)|1").unwrap(), true);
	assert_eq!(eval_expr("(1|0)&0").unwrap(), false);
	assert_eq!(eval_expr("(1|0)&(1|1)").unwrap(), true);
	assert_eq!(eval_expr("(1|0)&(1||1)").unwrap(), false);
	assert_eq!(eval_expr("((1&1)|0)&1").unwrap(), true);
}

#[test]
fn eval_parenthesized_expressions_with_not() {
	assert_eq!(eval_expr("!(1)").unwrap(), false);
	assert_eq!(eval_expr("!(0)").unwrap(), true);
	assert_eq!(eval_expr("!(1&0)").unwrap(), true);
	assert_eq!(eval_expr("!(1|0)").unwrap(), false);
	assert_eq!(eval_expr("1&!(0|1)").unwrap(), false);
	assert_eq!(eval_expr("1|!(0&1)").unwrap(), true);
	assert_eq!(eval_expr("!(1&0)|0").unwrap(), true);
	assert_eq!(eval_expr("(!(1|0))&1").unwrap(), false);
	assert_eq!(eval_expr("!((1|0)&(1|1))").unwrap(), false);
	assert_eq!(eval_expr("!((1&1)||1)&1").unwrap(), true);
}
