use crate::{global::*, mode::Mode, utils::validate_var_name, variables::Variables};

use crate::logic::optimizer::optimize_ast;

#[derive(Clone, PartialEq)]
pub enum ImplicationType {
  Forward,
  Reverse,
  Bidirectional,
}
#[derive(PartialEq)]
enum OpType {
  Not(bool),
  And,
  Or,
  StrictOr,
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
  StrictOr(Box<ASTNode>, Box<ASTNode>),
  Ifthen(ImplicationType, Box<ASTNode>, Box<ASTNode>),
}

impl ASTNode {
  pub fn stringify(&self) -> String {
    match self {
      ASTNode::Value(val) => {
        match val {
          true => "1".to_string(),
          false => "0".to_string(),
        }
      }
      ASTNode::Var(var_name) => var_name.to_string(),
      ASTNode::Not(node) => format!("!{}", node.stringify()),
      ASTNode::And(node1, node2) => format!("({}&{})", node1.stringify(), node2.stringify()),
      ASTNode::Or(node1, node2) => format!("({}|{})", node1.stringify(), node2.stringify()),
      ASTNode::StrictOr(node1, node2) => format!("({}||{})", node1.stringify(), node2.stringify()),
      ASTNode::Ifthen(implication_type, node1, node2) => {
        match implication_type {
          ImplicationType::Forward => format!("({}>{})", node1.stringify(), node2.stringify()),
          ImplicationType::Reverse => format!("({}<{})", node1.stringify(), node2.stringify()),
          ImplicationType::Bidirectional => format!("({}-{})", node1.stringify(), node2.stringify()),
        }
      }
      _ => "".to_string()
    }
  }
}

fn err_wrapper(e: &str, i: usize) -> Result<Box<ASTNode>, String> {
  Err(format!("{e} (at {i})"))
}

fn put_ifthen_op_into_ast(
  ast: &mut ASTNode,
  implication_type: ImplicationType,
  val1: ASTNode,
  val2: ASTNode,
) -> Result<(), &str> {
  *ast = ASTNode::Ifthen(implication_type, Box::new(val1), Box::new(val2));

  Ok(())
}

fn put_common_op_into_ast(
  ast: &mut ASTNode,
  op_type: OpType,
  val1: ASTNode,
  val2: ASTNode,
) -> Result<(), &str> {
  match op_type {
    OpType::And => {
      *ast = ASTNode::And(Box::new(val1), Box::new(val2));
    }
    OpType::Or => {
      *ast = ASTNode::Or(Box::new(val1), Box::new(val2));
    }
    OpType::StrictOr => {
      *ast = ASTNode::StrictOr(Box::new(val1), Box::new(val2));
    }
    OpType::Not(_) => {
      return Err("Cannot add this NOT operation into the ast.");
    }
    OpType::Ifthen(_) => {
      return Err("Cannot add this IFTHEN operation into the ast.");
    }
  }

  Ok(())
}

pub fn parse_to_ast(expr: &str, mode: Mode, optimize: bool) -> Result<Box<ASTNode>, String> {
  let mut ast = ASTNode::Empty;
  let mut temp_val1: Option<ASTNode> = None;
  let mut temp_val2: Option<ASTNode> = None;
  let mut op_type: Option<OpType> = None;
  let mut not_type: Option<OpType> = None; // exclusive for NOT operation
  let mut can_put_not: bool = false; // can put not only when value occurs after NOT operation

  let mut chars_peekable = expr.chars().enumerate().peekable();
  while let Some((i, char)) = chars_peekable.next() {
    match char {
      '(' => { // brackets
        let mut layer: usize = 1;
        let mut temp_expr: String = String::from("");

        // bracket begin
        while let Some((j, inside_char)) = chars_peekable.next() {
          match inside_char {
            '(' => {
              layer += 1;
              temp_expr += "(";
            }
            ')' => {
              layer -= 1;
              if layer > 0 {
                temp_expr += ")";
                continue;
              }

              // bracket end
              let bracket_ast = match parse_to_ast(&temp_expr, mode, optimize) {
                Ok(parsed) => { Some(*parsed) }
                Err(e) => { return err_wrapper(&e, j); }
              };

              if temp_val1.is_none() {
                temp_val1 = bracket_ast;
              } else if temp_val2.is_none() {
                temp_val2 = bracket_ast;
              }
              if matches!(not_type, Some(OpType::Not(_))) { can_put_not = true; }
              break; // stop the bracket loop
            }
            _ => { temp_expr += &inside_char.to_string(); }
          }
        }
      }
      '0' | '1' => { // constant value
        let value_ast = Some(ASTNode::Value(char == '1'));

        if temp_val1.is_none() {
          temp_val1 = value_ast;
        } else if temp_val2.is_none() {
          temp_val2 = value_ast;
        }
        if matches!(not_type, Some(OpType::Not(_))) { can_put_not = true; }
      }
      NOT_SYM => {
        if let Some(OpType::Not(should_reverse)) = not_type {
          not_type = Some(OpType::Not(!should_reverse));
        } else {
          not_type = Some(OpType::Not(false));
        }
      }
      AND_SYM => {
        if op_type.is_some() {
          return err_wrapper("Expect a value, but found an AND operation.", i);
        }

        op_type = Some(OpType::And);
        if !ast.eq(&ASTNode::Empty) {
          temp_val1 = Some(ast.clone());
        }
      }
      OR_SYM => {
        if op_type.is_some() {
          return err_wrapper("Expect a value, but found an OR operation.", i);
        }

        if let Some((_, next_char)) = chars_peekable.peek() && *next_char == OR_SYM {
          op_type = Some(OpType::StrictOr);
          chars_peekable.next();
        } else {
          op_type = Some(OpType::Or);
        }

        if !ast.eq(&ASTNode::Empty) {
          temp_val1 = Some(ast.clone());
        }
      }
      IFTHEN_FORWARD_SYM => {
        if op_type.is_some() {
          return err_wrapper("Expect a value, but found an IFTHEN_FORWARD operation.", i);
        }

        op_type = Some(OpType::Ifthen(ImplicationType::Forward));
        if !ast.eq(&ASTNode::Empty) {
          temp_val1 = Some(ast.clone());
        }
      }
      IFTHEN_REVERSE_SYM => {
        if op_type.is_some() {
          return err_wrapper("Expect a value, but found an IFTHEN_REVERSE operation.", i);
        }

        op_type = Some(OpType::Ifthen(ImplicationType::Reverse));
        if !ast.eq(&ASTNode::Empty) {
          temp_val1 = Some(ast.clone());
        }
      }
      IFTHEN_BIDIRECTIONAL_SYM => {
        if op_type.is_some() {
          return err_wrapper("Expect a value, but found an IFTHEN_BIDIRECTIONAL operation.", i);
        }

        op_type = Some(OpType::Ifthen(ImplicationType::Bidirectional));
        if !ast.eq(&ASTNode::Empty) {
          temp_val1 = Some(ast.clone());
        }
      }
      _ => {
        match validate_var_name(&char.to_string()) {
          Err(_) => return err_wrapper("Unexpected character.", i),
          _ => {}
        }

        let var_ast = match mode {
          Mode::Default | Mode::Simplify => { // default mode
            let variables = Variables::get_instance().read().unwrap();
            let var_value = match variables.get_var(&char) {
              Some(val) => val,
              None => { return err_wrapper("No such variable.", i); }
            };

            Some(ASTNode::Value(var_value))
          }
          Mode::Table => Some(ASTNode::Var(char))
        };

        if temp_val1.is_none() {
          temp_val1 = var_ast;
        } else if temp_val2.is_none() {
          temp_val2 = var_ast;
        }
        if matches!(not_type, Some(OpType::Not(_))) { can_put_not = true; }
      }
    }

    // create NOT node
    if can_put_not
      && matches!(not_type, Some(OpType::Not(_)))
      && let Some(OpType::Not(should_reverse)) = not_type.take()
      && !should_reverse {
      if temp_val2.is_some() {
        temp_val2 = Some(ASTNode::Not(
          Box::new(temp_val2.take().unwrap())
        ));
      } else if temp_val1.is_some() {
        temp_val1 = Some(ASTNode::Not(
          Box::new(temp_val1.take().unwrap())
        ));
      }
      can_put_not = false;
    }

    // put the temp values (val1, val2) into ast
    if temp_val1.is_none() || temp_val2.is_none() { continue; }
    
    // put ifthen
    if matches!(op_type, Some(OpType::Ifthen(_)))
      && let Some(OpType::Ifthen(implication_type)) = op_type.take() {
      match put_ifthen_op_into_ast(
        &mut ast,
        implication_type,
        temp_val1.take().unwrap(),
        temp_val2.take().unwrap(),
      ) {
        Ok(()) => { temp_val1 = None; }
        Err(e) => { return err_wrapper(e, i); }
      }
      continue;
    }

    // put common
    if op_type.is_some() {
      match put_common_op_into_ast(
        &mut ast,
        op_type.take().unwrap(),
        temp_val1.take().unwrap(),
        temp_val2.take().unwrap(),
      ) {
        Ok(()) => { temp_val1 = None; }
        Err(e) => { return err_wrapper(e, i); }
      }
      continue;
    }
  }

  if temp_val1.is_some() {
    ast = temp_val1.take().unwrap();
  }

  match optimize {
    true => Ok(optimize_ast(Box::new(ast))),
    false => Ok(Box::new(ast))
  }
}
