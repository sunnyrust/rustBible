# rust工程opencv-rust

## 安装



```shell
$ sudo apt install clang -y
$ sudo apt install libclang-dev -y

// 首先安装好opencv
$ opencv_version
4.5.1
```



## 写一个例子

Cargo.toml

```toml
opencv = {version = "0.52", default-features = false, features = ["opencv-34", "buildtime-bindgen"]}
```



main.rs

```rust

```

运行不通过，需要安装一个微软的工具——[vcpkg](https://github.com/microsoft/vcpkg)

```she
$ git clone https://github.com/microsoft/vcpkg
$ ./vcpkg/bootstrap-vcpkg.sh
//如果出现cmake版本过低的问题，去https://cmake.org/download/下载最新版安装
$ sudo cp vcpkg /usr/bin
$  vcpkg install boost:x64-linux
$ export VCPKG_ROOT=/usr/local/vcpkg
$ export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$CUDAROOT/lib64:/usr/lib/x86_64-linux-gnu:/usr/local/lib
$ export export PKG_CONFIG_PATH=/usr/local/vcpkg/installed/x64-linux/lib/pkgconfig/:/usr/share/pkgconfig/:/usr/local/lib/pkgconfig/
```



遇到一个奇葩问题

```
Error: "Failed to find OpenCV package using probes: environment, pkg_config, vcpkg_cmake, vcpkg, cmake"  
```

找遍了google、baidu、bing都没有找到原因，后来应验我常说的俗语“当你搜索不到有关这个问题的任何记录的时候，就是你自己的问题。”



我试着执行了一下下面的语句



```shell
$ pkg-config --modversion opencv
3.2.0
```



大家还记得我在Cargo.toml里面是怎么定义的吧？

```toml
opencv = {version = "0.52", default-features = false, features = ["opencv-34", "buildtime-bindgen"]}
```

我改成

```toml
opencv = {version = "0.52", default-features = false, features = ["opencv-32", "buildtime-bindgen"]}
```

这个问题解决了，但是依然编译不过，这个时候出现的都是各种语法错误，看来我得把版本升到3.4.0。



后来发现是乌龙了，使用`opencv-32` 没有问题，是因为原来程序里面需要一个图片，我没有放在当前路径下。



最后编译的过程：

```shell
$ RUST_BACKTRACE=full cargo build -vv
```



但是video依然不行，我还得努力升级opencv，必须是3.4才可以。

```shell
$ sudo apt-get purge libopencv* python-opencv
$ sudo apt-get install libopencv* python-opencv
$ sudo apt-get install python3-pip
$ pip3 uninstall opencv-python-headless
```



安装opencv_3.4.14

```shell
$ sudo apt-get install build-essential
$ sudo apt-get install cmake git libgtk2.0-dev pkg-config libavcodec-dev libavformat-dev libswscale-dev
$ sudo apt-get install python-dev python-numpy libtbb2 libtbb-dev libjpeg-dev libpng-dev libtiff-dev libjasper-dev libdc1394-22-dev
$ git clone https://github.com/opencv/opencv.git
$ cd opencv
$ mkdir build
$ cd build

$ sudo cmake -D WITH_TBB=ON -D BUILD_NEW_PYTHON_SUPPORT=ON -D WITH_V4L=ON -D INSTALL_C_EXAMPLES=ON -D INSTALL_PYTHON_EXAMPLES=ON -D BUILD_EXAMPLES=ON -D WITH_QT=ON -D WITH_OPENGL=ON -D CMAKE_LIBRARY_PATH=/usr/local/cuda/lib64/stubs -D CUDA_CUDA_LIBRARY=/usr/local/cuda/lib64/stubs/libcuda.so ..
//增强版，指定了安装目录和增加了opencv4.pc
$ sudo cmake -D WITH_TBB=ON -D BUILD_NEW_PYTHON_SUPPORT=ON -D WITH_V4L=ON -D INSTALL_C_EXAMPLES=ON -D CMAKE_INSTALL_PREFIX=/usr/local/opencv4 -D  INSTALL_PYTHON_EXAMPLES=ON -D BUILD_EXAMPLES=ON -D WITH_QT=ON -D WITH_OPENGL=ON -D CMAKE_LIBRARY_PATH=/usr/local/cuda/lib64/stubs -D OPENCV_GENERATE_PKGCONFIG=YES -D CUDA_CUDA_LIBRARY=/usr/local/cuda/lib64/stubs/libcuda.so ..


$ sudo make -j8

$ sudo make install

```

<https://opencv.org/releases/>

```shell
https://github.com/opencv/opencv/tree/3.4.14  去这里下载一个zip包

安装方式同上

最后这个成功了
```

:tada:tada:tada:tada:tada:tada:tada:tada:tada:tada:tada:::::::::::



下面是一个简单的例子

Cargo.toml

```toml
opencv = {version = "0.52", default-features = false, features = ["opencv-34", "buildtime-bindgen"]}
```



```rust
use opencv::{
	highgui,
	imgcodecs,
	Result,
};

fn main() -> Result<()> {
	let image = imgcodecs::imread("lena.jpg",imgcodecs::IMREAD_ANYCOLOR)?;
	highgui::named_window("hello opencv!", 0)?;
	highgui::imshow("hello opencv!", &image)?;
	highgui::wait_key(10000)?;
	Ok(())
}

```

```shell
$ RUST_BACKTRACE=full cargo build -vv
```

大成功。



这个工程虽然成功了，但是还有很多可以做的，比如现在程序是默认的读取`lena.jpg`这张图片，我们是不是可以通过传参数打开图片更友好一些？

我们接下来引入clap[^1]

```toml
[dependencies]
clap = "2.33.3"
```



```rust
extern crate clap;
use clap::App;
```



这个时候，我想把Go里面常用的添加各种git信息保存在程序里面。



首先在Cargo.toml里面增加

```toml
[package]

description="888888"
```



制作一个Makefile



```make
BUILD_NAME      := imgshow
GIT_VERSION      := $(shell git rev-parse HEAD )

all:
	sed -i -E 's/(description=").*(")/\1$(GIT_VERSION)\2/g' Cargo.toml 
	RUST_BACKTRACE=full cargo build -vv --release
```

修改main.rs程序



```rust
#[macro_use]
extern crate clap;
use opencv::{
	highgui,
	imgcodecs,
	Result,
};

use clap::App;
fn show_pic(filename:&str) -> Result<()> {
	let image = imgcodecs::imread(filename,imgcodecs::IMREAD_ANYCOLOR)?;
	highgui::named_window("hello opencv!", 0)?;
	highgui::imshow("hello opencv!", &image)?;
	highgui::wait_key(10000)?;
	Ok(())
}
#[allow(dead_code)]
fn flag()-> (){
	let s = "\r\nVersion:".to_owned()
	+ &crate_version!().to_owned()
	+ "\r\ngit:"
	+ &crate_description!().to_owned();
	let matches = App::new("imgshow")
	.version(&*s)
	.author(crate_authors!())
	.about(
		"查看图片的工具.
	具体看帮助:./imgshow --help
	",
	)
	.args_from_usage("-i,--img=[IMG] '图片文件名'")
	.get_matches();

	if matches.is_present("version") {
		use std::process;
        process::exit(0x0100);
	}
	if matches.is_present("img"){
        let filename = matches.value_of("img").unwrap_or("");
        // println!{"env:{:#?}",env}
		if filename.len()==0{
			println!("文件名不能为空");
		}else{
			let _r=show_pic(filename);
		}
      
    }
}
fn main() {
	flag();
}
```

## tx2 安装tensorrt

```shell
$ sudo apt-get update
$ apt-cache search tensorrt
$ sudo apt install libnvinfer*
```

## 解释

[^1]: Rust的命令行参数解析器。