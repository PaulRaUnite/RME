# RME
Register Machine Environment

## Installation

Clone the repository, run `cargo build --release` and launch `main` executable.

## About register machine dialect

In theory, there are infinite amount of registers that can store some integer values and integers belong to the interval of `(0, +infinity)`. Of course, the interpreter use finite set of registers and a program listing cannot contain integers bigger than `u64`, so there cannot be more than `u64` registers and the program cannot operate with numbers bigger than this.

I guess it's enough for education purposes, don't think that somebody will use it for something complicated seriously because of its inefficiency.

Types of instructions:

Instruction | Meaning
------------|---------
Z(n)        | zeroing of `n`-th register value
I(n)        | increasing by 1 `n`-th register value
T(n,m)      | copying `n`-th register value to `m`-th register
J(n,m,l)    | if `n` == `m` then jump to `l` line else next line after the instruction

Enumeration of instruction starts from 1.

Let `N` is amount of instructions in the program. A program finishes if current number of instruction is bigger than `N`.