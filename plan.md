# Nov 29

Trying to work in TDD workflow

## Machine

* [x] Finalize (or partially finalize) operations execution (end-to-end execution of some commands)
* [x] `in`/`out` commands implementation
* [x] Hermetization cleanup
* [ ] Stack register
* [x] Properly handling PC updating (check jumps)
* [x] Proper error handling (thiserror/anyhow)
* [ ] Read program from stdin
* [ ] Command line arguments (structopt crate)

### Instruction processing:

1. Fetch instruction from PC address
2. Execute instruction
3. If execution didn't update PC
    1. increment PC

## Some theory

* [x] Generics
* [x] Ownership + borrowing model
* [x] Lifetimes
* [ ] Memory model

## Maybe

* [ ] Threading basics


# Future agenda

## Machine

* [ ] Memory handling
* [ ] Execute program from std input