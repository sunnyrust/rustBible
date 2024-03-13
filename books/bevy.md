## 错误提示

```shell
Package alsa was not found in the pkg-config search path.
  Perhaps you should add the directory containing `alsa.pc'
  to the PKG_CONFIG_PATH environment variable
  No package 'alsa' found

```

### 解决办法

```shell
$ sudo apt install libudev*
$ sudo apt install librust-alsa-sys-dev 
```





## 代码检查

```shell
$ cargo fmt --all -- --check
$ cargo fmt
$ cargo clippy --workspace --all-targets --all-features -- -D warnings -A clippy::type_complexity -A clippy::manual-strip

```

