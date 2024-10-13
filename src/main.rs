// Type: Sets of Values
// Functions: Maps of types: A -> B

// Traits: Sets of Types
// Traits (Associated Types):       Map: Type -> Type
// Traits (Associated Constants):   Map: Type -> Value

// Pointers: Map: value -> value

// Exercise for the reader: Implement boolean logic using only the type system.
// Use the notes about Values, Types as sets of Values, Traits as sets of Types,
// assoc. types as maps upon Types, etc. for your implementation.
#![allow(nonstandard_style)]
use std::marker::PhantomData;

struct Ast;
struct Eval;
trait Mode {}
impl Mode for Ast {}
impl Mode for Eval {}

trait Boolean {}
trait Formula {
    type Output: Boolean + Formula;
}

enum True {}
impl Boolean for True {}
impl Formula for True {
    type Output = Self;
}

enum False {}
impl Boolean for False {}
impl Formula for False {
    type Output = Self;
}

// ast
struct And<L: Formula, R: Formula, M: Mode = Ast>(
    core::convert::Infallible,
    core::marker::PhantomData<(*const M, fn(L, R))>,
);

// eval
impl<L: Formula, R: Formula> Formula for And<L, R>
where
    And<L::Output, R::Output, Eval>: Formula,
{
    type Output = <And<L::Output, R::Output, Eval> as Formula>::Output;
}
impl Formula for And<True, True, Eval> {
    type Output = True;
}
impl Formula for And<False, True, Eval> {
    type Output = False;
}
impl Formula for And<True, False, Eval> {
    type Output = False;
}
impl Formula for And<False, False, Eval> {
    type Output = False;
}

// ast
struct Or<L: Formula, R: Formula, M: Mode = Ast>(
    core::convert::Infallible,
    core::marker::PhantomData<(*const M, fn(L, R))>,
);
// eval
impl<L: Formula, R: Formula> Formula for Or<L, R>
where
    Or<L::Output, R::Output, Eval>: Formula,
{
    type Output = <Or<L::Output, R::Output, Eval> as Formula>::Output;
}
impl Formula for Or<True, True, Eval> {
    type Output = True;
}
impl Formula for Or<False, True, Eval> {
    type Output = True;
}
impl Formula for Or<True, False, Eval> {
    type Output = True;
}
impl Formula for Or<False, False, Eval> {
    type Output = False;
}

struct Not<T: Formula, M: Mode = Ast>(
    core::convert::Infallible,
    core::marker::PhantomData<(M, fn(T))>,
);

impl<T: Formula> Formula for Not<T>
where
    Not<T::Output, Eval>: Formula,
{
    type Output = <Not<T::Output, Eval> as Formula>::Output;
}
impl Formula for Not<True, Eval> {
    type Output = False;
}
impl Formula for Not<False, Eval> {
    type Output = True;
}

struct Proof<F: Formula<Output = True>>(PhantomData<F>);

type _0 = Proof<Not<Not<Or<False, And<True, And<True, True>>>>>>;
