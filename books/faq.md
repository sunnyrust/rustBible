# FAQ
[TOC]

## Rust使用C库时如何指定运行时库的路径
Rust调用一个C库 使用如下注解能够编译通过并且cargo run能够正常执行 #[link(name = "tst_so", kind = "dylib")] 但是单独ldd时或者手动执行可执行文件时报libtst_so.so => not found

### 问： 如何在编译的时候将参数-Wl,-rpath,./传递给Rust

### 答：通过RUSTFLAGS来指定，例如: RUSTFLAGS="-Clink-arg=-Wl,-rpath,./" cargo build --release

> 1. export LD_LIBRARY_PATH=`具体路径`
>  将某具体路径添加到动态库路径环境变量，但仅当前终端有效
>  2. 将.so文件放到/lib或/usr/lib目录下（不推荐）
>  3. 将库文件绝对路径添加到/etc/ls.so.conf文件中，并用ldconfig命令重建ld.so.cache文件（不推荐）
>  4. 增加编译链接参数 （LDFLAGS = -Wl,--hash-style=sysv,-Bsymbolic,-rpath=./）让程序在当前目录寻找依赖的.so文件   （推荐）
编译链接可执行文件时，增加 -Wl,--rpath=选项，链接器在可执行文件头中记录动态库的路径，动态加载器运行时读取动态库路径，加载动态库


## 清除编译所占空间

Rust编译占得硬盘空间只能用恐怖来形容，我一共也就写了50多个程序，居然占了100G的空间，硬盘直接报警。然后，网友告诉了我一个很厉害的方法解决这个问题。

<https://crates.io/crates/cargo-clean-recursive> 使用这个工具来解决。
