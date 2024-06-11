# Rlox (Rust Lox)

An implementation of the Lox programming language base on
Robert Nystrom's book [Crafting Interpreters](https://craftinginterpreters.com) in Rust.

## Language Features Completed

- [x] Arithmetic
- [x] Variable declarations
- [x] Local scopes
- [x] Control flow
  - [x] If statements
  - [x] Loops
    - [x] For
    - [x] While
- [ ] Functions
  - [ ] Normal functions
  - [ ] Closures
- [ ] Classes
  - [ ] Data holding structure
  - [ ] Methods (instance and static)

## Implementation Differences

- The number 0 is considered falsy in this implementation rather than truthy as in the book's implementation.
- Only allows a block, a print statement, or an expression statement as the loop body rather than any statement.
