use std::{collections::HashMap, hash::Hash};

use colored::Colorize;

use crate::{
  executors::traits::Executor,
  logic::{parser::ASTNode, eval::eval_ast, parser::parse_to_ast},
  mode::Mode,
  mode::get_mode,
  variables::Variables
};

fn get_ast_var_names(ast: Box<ASTNode>) -> Vec<char> {
  let mut result: Vec<char> = Vec::new();

  let resolved_list = match *ast {
    ASTNode::Empty | ASTNode::Value(_) => vec![],
    ASTNode::Var(name) => vec![name],
    ASTNode::Not(node) => get_ast_var_names(node),
    ASTNode::And(node1, node2)
    | ASTNode::Or(node1, node2)
    | ASTNode::Ifthen(_, node1, node2) => {
      [get_ast_var_names(node1), get_ast_var_names(node2)].concat()
    }
  };

  for item in resolved_list {
    if !result.contains(&item) {
      result.push(item);
    }
  }

  result
}

fn vecs_to_map<K, V>(key_vec: Vec<K>, val_vec: Vec<V>) -> Result<HashMap<K, V>, String>
where K: Eq + Hash {
  if key_vec.len() != val_vec.len() {
    return Err("The lengths of 2 vecs to be converted to hash map should be equal.".to_string());
  }

  let map: HashMap<K, V> = key_vec.into_iter().zip(val_vec.into_iter()).collect();
  Ok(map)
}

fn print_truth_table(ast: Box<ASTNode>) -> Result<(), String> {
  let var_names = get_ast_var_names(ast.clone());
  let var_amount = var_names.len();

  // table head
  for name in &var_names {
    print!("| {} ", name);
  }
  print!("| Result |\n");

  // separate line
  for _ in 0..var_amount {
    print!("|---");
  }
  print!("|--------|\n");

  // table body
  let mut val_vec: Vec<bool> = var_names.iter().map(|_| false).collect();
  loop {
    let var_map = vecs_to_map(var_names.clone(), val_vec.clone())?;
    let result = eval_ast(ast.clone(), &var_map)?;
    for val in &val_vec {
      match val {
        true => print!("| {} ", 1),
        false => print!("| {} ", 0),
      }
    }
    match result {
      true => print!("| {}      |\n", 1),
      false => print!("| {}      |\n", 0),
    }

    let mut carry = false;
    for i in 0..var_amount {
      if i == var_amount - 1 && (i == 0 || carry) && val_vec[i] {
        return Ok(());
      }
      if (i == 0 || carry) && !val_vec[i] {
        val_vec[i] = true;
        break;
      }
      val_vec[i] = false;
      carry = true;
    }
  }
}

pub struct ExprExecutor;

impl Executor for ExprExecutor {
  fn execute(&self, input: &str) -> Result<bool, String> {
    let mode = get_mode();

    let variables = Variables::get_instance().read().unwrap();
    let ast = parse_to_ast(input, mode)?;

    match mode {
      Mode::Default => {
        let val = eval_ast(ast, variables.get_all_vars())?;
        
        if val { println!("{}{}", "=".bright_black(), "1".bold()); }
        else { println!("{}{}", "=".bright_black(), "0".bold()); }
      }
      Mode::Table => {
        match print_truth_table(ast) {
          Ok(()) => {}
          Err(e) => {
            eprintln!("Failed to print the truth table: {}", e);
          }
        }
      }
    }

    Ok(true)
  }
}
