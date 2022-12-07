use crate::{context::CompilerContext, ir::IrUnit};

use super::OptimizerPass;

// performs dead code analysis on an entire unit
pub struct Pass<'a> {
  ctx: &'a CompilerContext,
  unit: IrUnit,
}

impl<'a> Pass<'a> {}

impl<'a> OptimizerPass<'a> for Pass<'a> {
  fn transform(ctx: &'a CompilerContext, unit: IrUnit) -> IrUnit {
    unimplemented!()
  }
}

/*

// determining what counts as dead code

- [ref-list] a list of instruction id's that are "live" (not dead)

// FOR RETURNS
- start at a return statement of any code block,
  and read what value it references
- go to that value, then check its references,
  continue this, adding values to a temporary ref-list
- if the tree leads back to the start of control flow,
  push the temp list to the main ref-list

// FOR PARAMETER-PASSED REFERENCES
- find any statements that mutate a reference,
  then build the tree backwards similar to the return value

// FOR ANY TREE-BUILDER
 - if control flow leads the checker to run into a node already on the ref-list,
    immediately mark every node on the temporary ref-list as live

// FINALLY
- discard any values not within the list, and reduce the block

*/
