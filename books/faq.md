# FAQ
[TOC]

## Rust使用C库时如何指定运行时库的路径
Rust调用一个C库 使用如下注解能够编译通过并且cargo run能够正常执行 #[link(name = "tst_so", kind = "dylib")] 但是单独ldd时或者手动执行可执行文件时报libtst_so.so => not found

### 问： 如何在编译的时候将参数-Wl,-rpath,./传递给Rust

### 答：通过RUSTFLAGS来指定，例如: RUSTFLAGS="-Clink-arg=-Wl,-rpath,./" cargo build --release