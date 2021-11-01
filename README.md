# Course project: assembly interpreter

The project of the course woul be very naive assembly language. Every operation
would be a single line formatted as:

```
mnemonic arg1 arg2 arg3
```

The interpreter would contain:

* 14 general purpose registers (0..=13)
* a program counter (14)
* a stack pseudo-register (15)
* four address registers (16..=19)
* four memory pseudo-registers (20..=23)

The GPR registers behaves trivially - value written to them sets whatever is in
there for future reads.

A program counter register is a register pointing to currently executed instruction.
Instructions are indexed in auto-increment pattern. Reading from this register gives
the index of currently executed instruction. Writhing into this register is effectively
a jump.

The stack pseudo-register is tricky - writing to it pushes a value to internal stack,
and reading from it pops value from a stack.

Four address registers works exactly the same as as GPR registers, but have
special semantics.

Four memory registers are using address registers as an address in global memory
of 1k fp numbers. So if for eg register `16` has value of `42`, writing into
20 register should write into `mem[42]`, and reading should read from the same
memory cell.

Natively all values are `i64` values.

All registers and whole memory is zero-initialized (and stack is empty-initialized)

## Instruction set

* `load imm reg` - Loads `imm` value into register. `imm` is immediate `i64`
  value. Eg. `load -15 2` loads value of `-15` to 3rd GPR.
* `mov reg0 reg1` - Copies a value from reg0 to reg1. Eg. `load 14 21` pops
  a value from stack and stores it in memory address pointed by register `17`
  (as `21` is memory register addressed by `17` register)

* `add reg0 reg1 reg2` - Adds values in `reg0` and `reg1`, and stores the result
  in `reg2`. Eg. `add 16 2 16` offsets address in the `16` register by value
  in register `2`.
* `neg reg0 reg1` - Negates value in register `reg0` and stores result in
  `reg1`
* `mul reg0 reg1 reg1` and `rev reg0 reg1` - Analogous of `add` and `neg`:
  multiplication of two numbers, and inversion of a number
* `lea reg0 imm reg1 reg2` - Calculates `reg0 + imm * reg1` and stores
  the result in `reg2`.

* `movgz reg0 reg1 reg2` - Conditional mov: if value in `reg0` is greater than
  `0`, copies a value from `reg1` to `reg2`
* `movz reg0 reg1 reg2` - Conditional mov: if value in `reg0` is equal to `0`,
  copies a value from `reg1` to `reg2`
