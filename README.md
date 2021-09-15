# rusthor


## Prerequisites

### Install Rust:  
- Linux:  
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- Windows:  
Download `rustup`: https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe

## Usage

### Compilation

1. Clone the repo:

    ```
    git clone ssh://keranova@keranova-git:2222/bwatelet/rusthor.git
    ```
   
2. Build:

    ```
    cd rusthor
    cargo build # for debug
    cargo build --release # for release
    ```

### Run rusthor

1. Print help information:
    ```
    $ ./target/debug/rusthor --help
    rusthor 0.1.0
    bwatelet
    Rhothor clone in Rust.
    
    USAGE:
        rusthor <IP_ADDRESS>
    
    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
    
    ARGS:
        <IP_ADDRESS>    Scanners IP address, e.g. 192.168.0.6
    ```
