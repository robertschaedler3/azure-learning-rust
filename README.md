# Azure Learning Rust

This repository helps new Rust developers understand core Rust concepts through labs. Each lab has comments that explain different Rust concepts through examples. 

## Lab 4 topics
- [`?` operator](https://github.com/robertschaedler3/Fe2O3/blob/8713776cda56194ecd108ff56ed99a73fc711b71/learn/3-type-system/README.md#L113)
- [Error handling](https://github.com/robertschaedler3/Fe2O3/blob/aa0d99f17361821c71d6038065100227e61cf14a/learn/5-error-handling/README.md)

## Lab 4 instructions
- In `src\error.rs`
  - Uncomment to extend the error enum to include a new variant for the `ComponentNotFound` error.
- In `src\handlers.rs`
  - Uncomment to add the Error::ComponentNotFound variant to the match arm.
- In `src\module\mod.rs`
  - Uncomment the lines to return the ComponentNotFound error if the component is not found in the `get()` method
  - Uncomment the line to return the ComponentNotFound error if the component is not found in the `set()` method
- In `src\main.rs`
  - Add a `?` operator to the call to `init_logger()` to propagate any errors that may occur
  - Uncomment the following code block to enable logging to a rolling file appender (implementation does not need changing)
  - Introduce a return type that can capture all errors originating from this function
  - Change previously unused `_encoder` variable to `encoder`


## Dev note
Labs 1-4 are independent learning labs that do not provide fully end-to-end functionality. Each lab build on top of the previous lab, but they can be run in any order. The labs culminate in a functional Unix Domain Socket based service that dynamically loads and invokes OS modules from the OSConfig repository, used for locally managing Linux Edge devices. 

For dev environment setup, the README in `platform_lab1` branch has detailed instructions. The other labs assume the same dev environment setup.

The final solution can be run in the `platform_labcomplete` branch. 
