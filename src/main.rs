// Type: Sets of Values
// Functions: Maps of types: A -> B

// Traits: Sets of Types
// Traits (Associated Types):       Map: Type -> Type
// Traits (Associated Constants):   Map: Type -> Value

// Pointers: Map: value -> value

// FromStr: {u8, u16, f64, f32, ... }
// FromStr::Err {u8, u16, f64, f32 } -> { U8Error, U16Error, F64Error, }

// FnOnce (trait) - functions that you can only call once.
// FnMut (trait) - functions that
// Fn (trait) - functions that can inspect their environment.
// fn (trait, but also not) - pure functions.

// Exercise for the reader: Implement boolean logic using only the type system.
// Use the notes about Values, Types as sets of Values, Traits as sets of Types,
// assoc. types as maps upon Types, etc. for your implementation.

struct True;
struct False;
trait TRUE {}
trait FALSE {}

impl TRUE for True {}
impl FALSE for False {}

pub enum Operations {
    And,
    Or,
    Not,
    Nand,
    Nor,
}
mod sealed {
    pub trait Boolean {}
}
use sealed::Boolean;

impl Boolean for True {}
impl Boolean for False {}

// AND
trait And<Rhs: Boolean>: Boolean {
    // Rhs = Right hand side
    type Output: Boolean;
}

impl And<True> for True {
    type Output = True;
}

impl And<False> for True {
    type Output = False;
}

impl And<True> for False {
    type Output = False;
}

impl And<False> for False {
    type Output = False;
}

// OR
trait Or<Rhs: Boolean>: Boolean {
    // Rhs = Right hand side
    type Output: Boolean;
}

impl Or<True> for True {
    type Output = True;
}

impl Or<False> for True {
    type Output = True;
}

impl Or<True> for False {
    type Output = True;
}

impl Or<False> for False {
    type Output = False;
}

// NOT
trait Not: Boolean {
    // Rhs = Right hand side
    type Output: Boolean;
}

impl Not for True {
    type Output = False;
}

impl Not for False {
    type Output = True;
}

trait Implies<Rhs: Boolean>: Boolean {
    type Output: Boolean;
}

impl<L: Boolean, R: Boolean> Implies<R> for L
where
    L: Not,
    <L as Not>::Output: Or<R>, // What we're trying to write is "Not L or A = L -> A"
{
    type Output = <<L as Not>::Output as Or<R>>::Output;
}

trait Equality<Rhs: Boolean>: Boolean {
    // Rhs = Right hand side
    type Output: Boolean;
}

impl Equality<True> for True {
    type Output = True;
}

impl Equality<False> for True {
    type Output = False;
}

impl Equality<True> for False {
    type Output = False;
}

impl Equality<False> for False {
    type Output = True;
}

fn test<L: Boolean, R: Boolean>()
where
    <L as And<R>>::Output: TRUE,
    L: And<R>, // L implements R
{
}

fn main() {
    test::<True, False>();
}
