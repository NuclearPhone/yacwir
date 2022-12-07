// %1 = ConstInteger(24)
// %2 = ConstInteger(25)
// %3 = Add(%1, %2)

pub enum Ir {
  Assign(u32),

  // constants
  ConstFloat(f64),
  ConstInteger(i64),

  // index into local temps
  Add(u32, u32),
  Subtract(u32, u32),
  Multiply(u32, u32),
  Divide(u32, u32),
}
