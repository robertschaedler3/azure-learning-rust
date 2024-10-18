# Azure Learning Rust

This repository helps new Rust developers understand core Rust concepts through labs. Each lab has comments that explain different Rust concepts through examples. 

## Dev environment setup
- [Setup Ubuntu 22.04 in WSL2](https://github.com/ubuntu/WSL/blob/main/docs/guides/install-ubuntu-wsl2.md)
- [Setup WSL2 in VSCode](https://code.visualstudio.com/blogs/2019/09/03/wsl2)
- [Setup Rust & VSCode rust-analyzer extension inside WSL2](https://code.visualstudio.com/docs/languages/rust) (Note - Rust VSCode setup instructions are for Windows, but they will work in WSL2 Linux too)
- [Setup pre-requisites for Azure/azure-osconfig repo](https://github.com/Azure/azure-osconfig/tree/main?tab=readme-ov-file#prerequisites)
- Clone and configure the Azure/azure-osconfig repository
 
```
git clone https://github.com/Azure/azure-osconfig.git
pushd azure-osconfig
git submodule update --init --recursive
mkdir build && pushd build
cmake ../src -DCMAKE_BUILD_TYPE=Release -Duse_prov_client=ON -Dhsm_type_symm_key=ON -DBUILD_TESTS=OFF
```

- Install the azure-osconfig

`sudo cmake --build . --config Release  --target install`

## Lab 1 topics
- [What is Rust? / Rust tooling](https://github.com/robertschaedler3/Fe2O3/blob/258875b66cfc9e86be385147ce491db501600f18/learn/0-what-is-rust/README.md)
- [Hello World!](https://github.com/robertschaedler3/Fe2O3/blob/17e71092e07c6fe9164f9125e2c4ba4954667001/learn/1-hello-world/README.md)
- [Type system](https://github.com/robertschaedler3/Fe2O3/blob/6c5d7552c3ec7b3820e774bc67156c287892fd37/learn/3-type-system/strings.md)
- [Traits and structs](https://github.com/robertschaedler3/Fe2O3/blob/36508438ee1624fab746470a811ffe8a5e9a72c2/learn/4-traits-and-structs/README.md)

## Lab 1 instructions
- Uncomment the code block in `\src\module\mod.rs` to load modules from `/usr/lib/osconfig` and insert them into the `modules` HashMap.
  - Note the move of `path` variable contents to the `Module::init()` function.
  - Note the pass by reference of `name` variable to the `self.modules.get()` function.
  - Note the user of clone() in the `name` variable to the `self.modules.insert()` function.


## Dev note
Labs 1-4 are independent learning labs that do not provide fully end-to-end functionality. Each lab build on top of the previous lab, but they can be run in any order. The labs culminate in a functional Unix Domain Socket based service that dynamically loads and invokes OS modules from the OSConfig repository, used for locally managing Linux Edge devices. 

For dev environment setup, the README in `platform_lab1` branch has detailed instructions. The other labs assume the same dev environment setup.

The final solution can be run in the `platform_labcomplete` branch. 