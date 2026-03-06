use std::any::Any;

use crate::global::*;

#[derive(Clone, PartialEq)]
enum ImplicationType {
  Forward,
  Reverse,
  Bidirectional,
}
#[derive(PartialEq)]
enum OpType {
  Not,
  And,
  Or,
  Ifthen(ImplicationType),
}

#[derive(Clone, PartialEq)]
pub enum ASTNode {
  Empty,
  Value(bool),
  Var(char /* var name */),
  Not(Box<ASTNode>),
  And(Box<ASTNode>, Box<ASTNode>),
  Or(Box<ASTNode>, Box<ASTNode>),
  Ifthen(ImplicationType, Box<ASTNode>, Box<ASTNode>),
}

fn err_wrapper(e: &str, i: usize) -> Result<Box<ASTNode>, String> {
  Err(format!("{e} (at {i})"))
}

fn put_not_into_ast(ast: &mut ASTNode, val: ASTNode) -> Result<(), String> {
  *ast = ASTNode::Not(Box::new(val));

  Ok(())
}

fn put_op_into_ast(
  ast: &mut ASTNode,
  op_type: OpType,
  val1: ASTNode,
  val2: ASTNode,
) -> Result<(), &str> {
  match op_type {
    OpType::Not => {
      return Err("Cannot add this NOT operation into the ast.");
    }
    OpType::And => {
      *ast = ASTNode::And(Box::new(val1), Box::new(val2));
    }
    OpType::Or => {
      *ast = ASTNode::Or(Box::new(val1), Box::new(val2));
    }
    OpType::Ifthen(_) => {
      return Err("todo...");
    }
  }

  Ok(())
}

pub fn parse_to_ast(expr: &str) -> Result<Box<ASTNode>, String> {
  let mut ast = ASTNode::Empty;
  let mut temp_value: Option<ASTNode> = None;
  let mut op_type: Option<OpType> = None;

  let mut chars_peekable = expr.chars().enumerate().peekable();
  while let Some((i, char)) = chars_peekable.next() {
    match char {
      '0' | '1' => { // constant value
        if temp_value.is_some() && op_type.is_some() {
          match put_op_into_ast(
            &mut ast,
            op_type.take().unwrap(),
            temp_value.take().unwrap(),
            ASTNode::Value(char == '1'),
          ) {
            Ok(()) => { temp_value = None; }
            Err(e) => { return err_wrapper(e, i); }
          }
          continue;
        }

        if temp_value.is_none() && op_type.is_none() {
          temp_value = Some(ASTNode::Value(char == '1'));
        }
      }
      NOT_SYM => {
        if let Some(&(_, next_char)) = chars_peekable.peek() {
          if next_char != '0' && next_char != '1' {
            return err_wrapper("Expect a constant value.", i);
          }
          put_not_into_ast(&mut ast, ASTNode::Value(next_char == '1'))?;
          chars_peekable.next();
        }
      }
      AND_SYM => {
        if op_type.is_some() {
          return err_wrapper("Expect a value, but found an AND operation.", i);
        }

        op_type = Some(OpType::And);
        if !ast.eq(&ASTNode::Empty) {
          temp_value = Some(ast.clone());
        }
      }
      OR_SYM => {
        if op_type.is_some() {
          return err_wrapper("Expect a value, but found an OR operation.", i);
        }

        op_type = Some(OpType::Or);
        if !ast.eq(&ASTNode::Empty) {
          temp_value = Some(ast.clone());
        }
      }
      _ => return err_wrapper("Unexpected character.", i),
    }
  }

  if temp_value.is_some() {
    ast = temp_value.take().unwrap();
  }

  Ok(optimize_ast(Box::new(ast)))
}

/** todo */
fn optimize_ast(ast: Box<ASTNode>) -> Box<ASTNode> {
  ast
}
