// ir typechecker,
// should be applied before any optimizations
// as to allow the optimizer passes to make assumptions
// and do things that it would otherwise have to check

// performs rudementary typechecking (e.g. erroring on incompatable types)
// also fleshes out auto-types

// order of processes
//  - type propogation
//        flesh out the types of the IR from constants
//        %0 = ConstInt(1) : Integer
//        %1 = ConstInt(2) : Integer
//        %2 = Add(%0, %1) : Undecided -> Integer
//
//  - binary lowering
//        converts non-intrinsic binary operations (Integers and Floats) into their equivalent
//        function form, and replaces all instances of the binary ops with function calls
//
//        relies upon type-propogation, as binary calls do not intrinsically have a type
//
//        type Xyz {... implements Add(lhs, rhs) -> {...}}
//
//        %0 = Call("xyzConstructor", ...)
//        %1 = Call("xyzConstructor", ...)
//        %2 = Add(%0, %1) -> Call("xyzImplementsAdd", [%0, %1])
//
//  - expression-checks
//        run through the entire instruction listing of a block
//        and check that every instruction performs a type-correct operation
//        - relies upon type propogation

/*

// LIST OF THINGS TODO

- context dependent typechecking
  e.g. type x can be of type y because user has defined it


*/
