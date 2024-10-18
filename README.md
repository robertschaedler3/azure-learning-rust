# Azure Learning Rust

This repository helps new Rust developers understand core Rust concepts through labs. Each lab has comments that explain different Rust concepts through examples. 

## Lab 3 topics
- [Authoring tests](https://github.com/robertschaedler3/Fe2O3/blob/258875b66cfc9e86be385147ce491db501600f18/learn/0-what-is-rust/README.md#L39)
- [Implementing Traits](https://github.com/robertschaedler3/Fe2O3/blob/36508438ee1624fab746470a811ffe8a5e9a72c2/learn/4-traits-and-structs/README.md#L134-L135)

## Lab 3 instructions
- In `src\module\mod.rs`
  - Uncomment to implement the `TestAdapter` struct.
  - Uncomment to Implement the `Adapter` trait for `TestAdapter`.
  - Uncomment to implement the `test_module` test.


## Dev note
Labs 1-4 are independent learning labs that do not provide fully end-to-end functionality. Each lab build on top of the previous lab, but they can be run in any order. The labs culminate in a functional Unix Domain Socket based service that dynamically loads and invokes OS modules from the OSConfig repository, used for locally managing Linux Edge devices. 

For dev environment setup, the README in `platform_lab1` branch has detailed instructions. The other labs assume the same dev environment setup.

The final solution can be run in the `platform_labcomplete` branch. 