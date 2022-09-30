## Crusty Crab
*Created By Alex Schmith, Chandler Hake, Robbie Heine*

## Overview
This is a Command and Control framework developed in rust having in mind to have a fully functional api to create mallicious applications. 


## Developing 

To build the project using cargo
```
cargo build 
```

To run the binary 
```
cargo run --bin 
```
and select either crusty\_crab, listener, or implant


## Cross Compiling


First check to see your toolchains
```
rustup toolchain list
```

Then check all available toolchains you can install
```
rustup target list
```

First install the architecture platform that you need with the command

```
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-unknown-linux-gnu
```

## Testing 

Running cargo test will be used for integration testing inside the test


