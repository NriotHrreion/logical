use std::collections::HashMap;

use crate::logic::parser::{ASTNode, ImplicationType};
use crate::utils::*;

pub fn eval_ast(ast: Box<ASTNode>, vars: &HashMap<char, bool>) -> Result<bool, String> {
  match *ast {
    ASTNode::Value(val) => Ok(val),
    ASTNode::Var(var_name) => {
      let var_val = match vars.get(&var_name) {
        Some(val) => val,
        None => {
          return Err("No such variable.".to_string());
        }
      };
      Ok(*var_val)
    }
    ASTNode::Not(node) => {
      let val = eval_ast(node, vars)?;
      Ok(!val)
    }
    ASTNode::And(node1, node2) => {
      let val1 = eval_ast(node1, vars)?;
      let val2 = eval_ast(node2, vars)?;
      Ok(val1 && val2)
    }
    ASTNode::Or(node1, node2) => {
      let val1 = eval_ast(node1, vars)?;
      let val2 = eval_ast(node2, vars)?;
      Ok(val1 || val2)
    }
    ASTNode::StrictOr(node1, node2) => {
      let val1 = eval_ast(node1, vars)?;
      let val2 = eval_ast(node2, vars)?;
      Ok((val1 && !val2) || (!val1 && val2))
    }
    ASTNode::Ifthen(implication_type, node1, node2) => {
      let val1 = eval_ast(node1, vars)?;
      let val2 = eval_ast(node2, vars)?;
      match implication_type {
        ImplicationType::Forward => { Ok(implication_forward(val1, val2)) }
        ImplicationType::Reverse => { Ok(implication_reverse(val1, val2)) }
        ImplicationType::Bidirectional => { Ok(implication_bidirectional(val1, val2)) }
      }
    }
    _ => Ok(false),
  }
}
