use crate::{context::CompilerContext, ir::IrUnit};

use self::types::Types;

// mod binary_lowering;
// mod type_checking;
pub mod checker;
pub mod type_propogation;
mod types;

// order of sema/optimizer calls:
//  1. ast->ir | convert the AST into workable IR
//  2. dead-block removal | removes any dead blocks so unnessesary blocks dont get analyzed
//  3. type-propogation | propogate types through instructions and blocks
//  ...
//  n - 1: dead-code analysis | remove any useless instructions

// does not handle the typechecking itself, instead
//   calls upon different processes
pub struct SemaContext {
  pub types: Types,
}

impl SemaContext {
  pub fn new(ctx: &CompilerContext) -> Self {
    Self { types: Types {} }
  }
}
