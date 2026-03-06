use logical::logic::{parser::parse_to_ast, eval::eval_ast};

fn eval_expr(expr: &str) -> Result<bool, String> {
	let ast = parse_to_ast(expr)?;
	eval_ast(ast)
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
fn eval_chained_operators_without_parentheses() {
	assert_eq!(eval_expr("1&1&0").unwrap(), false);
	assert_eq!(eval_expr("0|0|1").unwrap(), true);
	assert_eq!(eval_expr("!1|1").unwrap(), true);
	assert_eq!(eval_expr("!0&1").unwrap(), true);
	assert_eq!(eval_expr("1|0&0").unwrap(), false);
	assert_eq!(eval_expr("1|0&!0").unwrap(), true);
}

#[test]
fn rejects_invalid_characters() {
	match parse_to_ast("1a0") {
		Ok(_) => panic!("Expected parser to reject invalid characters"),
		Err(err) => assert!(err.contains("Unexpected character")),
	}
}
