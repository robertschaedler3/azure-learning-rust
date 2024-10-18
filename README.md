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
Labs 1-4 are learning labs that do not provide fully end-to-end functionality. The labs culminate in a functional Unix Domain Socket based service that dynamically loads and invokes OS modules from the OSConfig repository, used for locally managing Linux Edge devices. 

To run the final lab E2E, here are the steps (tested on Ubuntu 22.04 in WSL2):
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
 
- Clone this repo and set the platform_labcomplete branch

`git clone https://github.com/robertschaedler3/azure-learning-rust.git -b platform_labcomplete`
   
- Build the final lab

`cargo build && sudo ./target/debug/osc-platform`

- In a separate terminal, run this to test

`sudo curl --unix-socket /tmp/osc-platform.sock -XGET http://localhost/DeviceInfo/osName/`
