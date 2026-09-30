# Programming a Guessing Game

Check comments for indepth analysis of code

## Things to Know
- Variables are immutable by default, must pass mut when declaring to make it mutable
- References (&) are also immutable, so must pass &mut to make them mutable
- to use types that aren't in prelude, must use "use" keyword
- Something like read_line returns a Result, which is an enumeration
    - A type with multiple states called variants
- can use {var} or {} sort of like printf, and can pass variables into it

## Crates (For Dev)
- A crate is a collection of Rust Source Code Files
- Cargo can coordinate external crates to have programs interact with other source code / programs.

## Pattern Matching
- Has pattern matching similar to OCaml
- Rust has a strong static type system, but has type inference.

## Variables
- Can shadow variables to change their types
- parse method only works on characters that can logically be converted to a number

## Looping
- To create a basic loop,