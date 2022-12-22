use crate::ir::Type;

// handles type conflicts, comparisons
// and type declarations
pub struct Types {}

impl Types {
  // checks if a type is coercable to another type
  // returns true if coercable
  pub fn coerce_type(&self, from: Type, to: Type) -> bool {
    match (from, to) {
      (Type::Integer, Type::Integer) => true,
      (Type::Integer, Type::Floating) => true,
      (Type::Floating, Type::Floating) => true,
      _ => false,
    }
  }

  // checks if types are binary compatable
  pub fn binary_compatable_types(&self, left: Type, right: Type) -> bool {
    match (left, right) {
      (Type::Integer, Type::Integer) => true,
      (Type::Floating, Type::Floating) => true,
      _ => false,
    }
  }
}
