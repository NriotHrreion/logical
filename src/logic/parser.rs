use crate::global::*;

#[derive(Clone, PartialEq)]
pub enum ImplicationType {
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

fn put_not_op_into_ast(ast: &mut ASTNode, val: ASTNode) -> Result<(), &str> {
  *ast = ASTNode::Not(Box::new(val));

  Ok(())
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
    OpType::Not => {
      return Err("Cannot add this NOT operation into the ast.");
    }
    OpType::Ifthen(_) => {
      return Err("Cannot add this IFTHEN operation into the ast.");
    }
  }

  Ok(())
}

pub fn parse_to_ast(expr: &str) -> Result<Box<ASTNode>, String> {
  let mut ast = ASTNode::Empty;
  let mut temp_val1: Option<ASTNode> = None;
  let mut temp_val2: Option<ASTNode> = None;
  let mut op_type: Option<OpType> = None;

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
              if temp_val1.is_none() {
                temp_val1 = match parse_to_ast(&temp_expr) {
                  Ok(bracket_ast) => { Some(*bracket_ast) }
                  Err(e) => { return err_wrapper(&e, j); }
                };
              } else if temp_val2.is_none() {
                temp_val2 = match parse_to_ast(&temp_expr) {
                  Ok(bracket_ast) => { Some(*bracket_ast) }
                  Err(e) => { return err_wrapper(&e, j); }
                };
              }
              break; // stop the bracket loop
            }
            _ => { temp_expr += &inside_char.to_string(); }
          }
        }
      }
      '0' | '1' => { // constant value
        if temp_val1.is_none() {
          temp_val1 = Some(ASTNode::Value(char == '1'));
        } else if temp_val2.is_none() {
          temp_val2 = Some(ASTNode::Value(char == '1'));
        }
      }
      NOT_SYM => { /* todo */
        if let Some(&(_, next_char)) = chars_peekable.peek() {
          if next_char != '0' && next_char != '1' && next_char != NOT_SYM {
            return err_wrapper("Expect a constant value.", i);
          }

          // To auto optimize expressions like `!!!!!1` to `!1`
          let mut should_reverse = false;
          while let Some(&(_, next_next)) = chars_peekable.peek() && next_next == NOT_SYM {
            should_reverse = !should_reverse;
            chars_peekable.next();
          }
          
          let comparator: char;
          if should_reverse { comparator = '0'; }
          else { comparator = '1'; }

          if let Some(&(_, new_next_char)) = chars_peekable.peek() {
            match put_not_op_into_ast(&mut ast, ASTNode::Value(new_next_char == comparator)) {
              Ok(()) => {}
              Err(e) => { return err_wrapper(e, i) }
            }
            chars_peekable.next();
          }
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

        op_type = Some(OpType::Or);
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
      _ => return err_wrapper("Unexpected character.", i),
    }

    if temp_val1.is_none() || temp_val2.is_none() { continue; }
    
    // put the temp values (val1, val2) into ast
    if matches!(op_type, Some(OpType::Ifthen(_))) {
      if let Some(OpType::Ifthen(implication_type)) = op_type.take() {
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
    }

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

  Ok(optimize_ast(Box::new(ast)))
}

/** todo */
fn optimize_ast(ast: Box<ASTNode>) -> Box<ASTNode> {
  ast
}
