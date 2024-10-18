# Azure Learning Rust

This repository helps new Rust developers understand core Rust concepts through labs. Each lab has comments that explain different Rust concepts through examples. 

## Lab 2 topics
- [Ownership](https://github.com/robertschaedler3/Fe2O3/blob/17e71092e07c6fe9164f9125e2c4ba4954667001/learn/2-ownership/README.md)
- [Type system](https://github.com/robertschaedler3/Fe2O3/blob/8713776cda56194ecd108ff56ed99a73fc711b71/learn/3-type-system/README.md)


## Lab 2 instructions
- Uncomment the code block in `\src\module\mod.rs` to load modules from `/usr/lib/osconfig` and insert them into the `modules` HashMap.
  - Note the move of `path` variable contents to the `Module::init()` function.
  - Note the pass by reference of `name` variable to the `self.modules.get()` function.
  - Note the user of clone() in the `name` variable to the `self.modules.insert()` function.
- Execute `cargo run` at the base folder

## Dev note
Labs 1-4 are independent learning labs that do not provide fully end-to-end functionality. Each lab build on top of the previous lab, but they can be run in any order. The labs culminate in a functional Unix Domain Socket based service that dynamically loads and invokes OS modules from the OSConfig repository, used for locally managing Linux Edge devices. 

For dev environment setup, the README in `platform_lab1` branch has detailed instructions. The other labs assume the same dev environment setup.

The final solution can be run in the `platform_labcomplete` branch. 