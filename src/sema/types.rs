use crate::ir::{PrimType, Type};

// the main typechecking construct
// manages all of the individual processes required
// including declarations, conflicts, comparisons, etc
pub struct Types {}

/*

coercions vs casts
  coercions are implicit conversions between types, e.g.
    U8  y = 9
    U16 x = y;

  9 is a U8 num, but it gets implicitly cast to a U16

  casts are explicit conversions between types, e.g.
    F64 y = 24.3;
    F32 x = @cast(F32, y);

*/

impl Types {
  // checks if a type is coercable to another type
  // returns true if coercable
  pub fn coerce_type(&self, from: Type, to: Type) -> bool {
    // coercions between different levels
    // of indirection do not exist
    if from.ptr != to.ptr {
      return false;
    }

    match (from.prim, to.prim) {
      (PrimType::Integer(bw_l), PrimType::Integer(bw_r)) => bw_r >= bw_l,
      (PrimType::Integer(_), PrimType::Floating(_)) => true,
      (PrimType::Floating(bw_l), PrimType::Floating(bw_r)) => bw_r >= bw_l,

      (PrimType::ComptimeInt, PrimType::ComptimeInt) => true,
      (PrimType::ComptimeUnsigned, PrimType::ComptimeUnsigned) => true,
      (PrimType::ComptimeFloat, PrimType::ComptimeFloat) => true,

      _ => false,
    }
  }

  // checks if types are binary compatable
  pub fn binary_compatable_types(&self, left: Type, right: Type) -> bool {
    if left.ptr != right.ptr {
      return false;
    }

    if left.ptr != 0 {
      return false;
    }

    match (left.prim, right.prim) {
      (PrimType::Integer(bw_l), PrimType::Integer(bw_r)) => bw_r >= bw_l,
      (PrimType::Floating(_), PrimType::Floating(_)) => true,

      (PrimType::ComptimeInt, PrimType::ComptimeInt) => true,
      (PrimType::ComptimeUnsigned, PrimType::ComptimeUnsigned) => true,
      (PrimType::ComptimeFloat, PrimType::ComptimeFloat) => true,

      _ => false,
    }
  }
}
