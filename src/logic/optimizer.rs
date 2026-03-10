use crate::logic::parser::{ASTNode, ImplicationType};

fn is_ast_equal(ast1: &mut Box<ASTNode>, ast2: &mut Box<ASTNode>) -> bool {
  (*ast1).eq(&*ast2)
}

fn clone_node(node: &mut Box<ASTNode>) -> Box<ASTNode> {
  Box::new(*node.clone())
}

pub fn optimize_ast(mut ast: Box<ASTNode>) -> Box<ASTNode> {
  match &mut *ast {
    ASTNode::Not(node) => {
      match &mut **node {
        ASTNode::Not(sub_node) => {
          ast = optimize_ast(clone_node(sub_node));
        }
        ASTNode::And(node1, node2) => {
          ast = optimize_ast(Box::new(ASTNode::Or(
            optimize_ast(Box::new(ASTNode::Not(
              clone_node(node1)
            ))),
            optimize_ast(Box::new(ASTNode::Not(
              clone_node(node2)
            ))),
          )));
        }
        ASTNode::Or(node1, node2) => {
          ast = optimize_ast(Box::new(ASTNode::And(
            optimize_ast(Box::new(ASTNode::Not(
              clone_node(node1)
            ))),
            optimize_ast(Box::new(ASTNode::Not(
              clone_node(node2)
            ))),
          )));
        }
        _ => {}
      }
    }
    ASTNode::And(node1, node2) => {
      if let ASTNode::Value(x) = **node1 && !x {
        *ast = ASTNode::Value(false);
      } else if let ASTNode::Value(x) = **node2 && !x {
        *ast = ASTNode::Value(false);
      } else if is_ast_equal(node1, node2) {
        *ast = *node1.clone();
      } else if let ASTNode::Or(sub_node1, sub_node2) = &mut **node1 {
        *ast = ASTNode::Or(
          optimize_ast(Box::new(ASTNode::And(
            clone_node(sub_node1),
            clone_node(node2)
          ))),
          optimize_ast(Box::new(ASTNode::And(
            clone_node(sub_node2),
            clone_node(node2)
          ))),
        );
      } else if let ASTNode::Or(sub_node1, sub_node2) = &mut **node2 {
        *ast = ASTNode::Or(
          optimize_ast(Box::new(ASTNode::And(
            clone_node(sub_node1),
            clone_node(node1)
          ))),
          optimize_ast(Box::new(ASTNode::And(
            clone_node(sub_node2),
            clone_node(node1)
          ))),
        );
      }
    }
    ASTNode::Or(node1, node2) => {
      if let ASTNode::Value(x) = **node1 && x {
        *ast = ASTNode::Value(true);
      } else if let ASTNode::Value(x) = **node2 && x {
        *ast = ASTNode::Value(true);
      } else if is_ast_equal(node1, node2) {
        *ast = *node1.clone();
      } else if let ASTNode::And(sub_node1, sub_node2) = &mut **node1 {
        *ast = ASTNode::And(
          optimize_ast(Box::new(ASTNode::Or(
            clone_node(sub_node1),
            clone_node(node2)
          ))),
          optimize_ast(Box::new(ASTNode::Or(
            clone_node(sub_node2),
            clone_node(node2)
          ))),
        );
      } else if let ASTNode::And(sub_node1, sub_node2) = &mut **node2 {
        *ast = ASTNode::And(
          optimize_ast(Box::new(ASTNode::Or(
            clone_node(sub_node1),
            clone_node(node1)
          ))),
          optimize_ast(Box::new(ASTNode::Or(
            clone_node(sub_node2),
            clone_node(node1)
          ))),
        );
      }
    }
    ASTNode::Ifthen(implication_type, node1, node2) => {
      match implication_type {
        ImplicationType::Forward => {
          *ast = ASTNode::Or(
            optimize_ast(Box::new(ASTNode::Not(
              clone_node(node1)
            ))),
            optimize_ast(clone_node(node2))
          );
        }
        ImplicationType::Reverse => {
          *ast = ASTNode::Or(
            optimize_ast(clone_node(node1)),
            optimize_ast(Box::new(ASTNode::Not(
              clone_node(node2)
            )))
          );
        }
        ImplicationType::Bidirectional => {
          *ast = ASTNode::And(
            optimize_ast(Box::new(
              ASTNode::Ifthen(
                ImplicationType::Forward,
                optimize_ast(clone_node(node1)),
                optimize_ast(clone_node(node2))
              )
            )),
            optimize_ast(Box::new(
              ASTNode::Ifthen(
                ImplicationType::Reverse,
                optimize_ast(clone_node(node1)),
                optimize_ast(clone_node(node2))
              )
            ))
          );
        }
      }
    }
    _ => {}
  }

  ast
}
