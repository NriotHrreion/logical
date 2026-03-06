use crate::logic::parser::{ASTNode, ImplicationType};
use crate::utils::*;

pub fn eval_ast(ast: Box<ASTNode>) -> Result<bool, String> {
  match *ast {
    ASTNode::Value(val) => Ok(val),
    ASTNode::Not(node) => {
      let val = eval_ast(node)?;
      Ok(!val)
    }
    ASTNode::And(node1, node2) => {
      let val1 = eval_ast(node1)?;
      let val2 = eval_ast(node2)?;
      Ok(val1 && val2)
    }
    ASTNode::Or(node1, node2) => {
      let val1 = eval_ast(node1)?;
      let val2 = eval_ast(node2)?;
      Ok(val1 || val2)
    }
    ASTNode::Ifthen(implication_type, node1, node2) => {
      let val1 = eval_ast(node1)?;
      let val2 = eval_ast(node2)?;
      match implication_type {
        ImplicationType::Forward => { Ok(implication_forward(val1, val2)) }
        ImplicationType::Reverse => { Ok(implication_reverse(val1, val2)) }
        ImplicationType::Bidirectional => { Ok(implication_bidirectional(val1, val2)) }
      }
    }
    _ => Ok(false),
  }
}
