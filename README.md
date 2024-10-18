# Azure Learning Rust

This repository helps new Rust developers understand core Rust concepts through labs. Each lab has comments that explain different Rust concepts through examples. 

## Final solution instructions
The labs culminate in this functional Unix Domain Socket based service that dynamically loads and invokes OS modules from the OSConfig repository, used for locally managing Linux Edge devices. 

- Build and run the final solution

`cargo build && sudo ./target/debug/osc-platform`

- In a separate terminal, run this to test

`sudo curl --unix-socket /tmp/osc-platform.sock -XGET http://localhost/DeviceInfo/osName/`
