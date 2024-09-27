# Azure Learning Rust

This repository helps new Rust developers understand core Rust concepts through labs. Each lab has comments that explain different Rust concepts through examples. 

The labs culminate in a functional Unix Domain Socket based service that dynamically loads and invokes OS modules from the OSConfig repository, used for locally managing Linux Edge devices. 

To run the final lab E2E, here are the steps:
- Clone the Azure/azure-osconfig repository
 
`git clone https://github.com/Azure/azure-osconfig.git`

- Install the azure-osconfig

`sudo cmake --build . --config Release  --target install`
 
- Clone this repo and set the platform_labcomplete branch

`git clone https://github.com/robertschaedler3/azure-learning-rust.git -b platform_labcomplete`
   
- Build the final lab

`cargo build && sudo ./target/debug/osc-platform`

- In a separate terminal, run this to test

`sudo curl --unix-socket /tmp/osc-platform.sock -XGET http://localhost/DeviceInfo/osName/`
