# Rust扭转乾坤

[TOC]

## 前言
今天是牛年第一个工作日（2021-02-18），今年我准备写一下Rust的工具书。希望能够利用一年的时间完成此书，我也能够在写书的过程中提高本人Rust的水平。

![logo](./logo.png)
![logo](./logo.jpeg)
![logo_rust](./logo_rust.png)
有人说这个图标是一个螃蟹，我听到的说法是这个一个铁锈。

## 名词解释
|序号|缩略语|英文全称|中文含义|
|--:|:--|:--|:--|
|1|API|Application Programming Interface|应用程序编程接口|
|2|FFI|Foreign Function Interface|语言交互接口|
|3|ABI|Application Binary Interfac|应用程序二进制接口|

### ABI与API的区别

&nbsp;&nbsp; &nbsp; &nbsp; &nbsp; &nbsp;  应用程序接口（Application Programming Interface，API），又称为应用编程接口，就是软件系统不同组成部分衔接的约定。由于近年来软件的规模日益庞大，常常需要把复杂的系统划分成小的组成部分，编程接口的设计十分重要。程序设计的实践中，编程接口的设计首先要使软件系统的职责得到合理划分。良好的接口设计可以降低系统各部分的相互依赖，提高组成单元的内聚性，降低组成单元间的耦合程度，从而提高系统的维护性和扩展性。

&nbsp;&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; ABI不同于API ，API定义了源代码和库之间的接口，因此同样的代码可以在支持这个API的任何系统中编译 ，然而ABI允许编译好的目标代码在使用兼容ABI的系统中无需改动就能运行。 ABI掩盖了各种细节，例如:调用约定控制着函数的参数如何传送以及如何接受返回值；系统调用的编码和一个应用如何向操作系统进行系统调用；以及在一个完整的操作系统ABI中，对象文件的二进制格式、程序库等等。一个完整的ABI，像 Intel二进制兼容标准 (iBCS) ，允许支持它的操作系统上的程序不经修改在其他支持此ABI的操作系统上运行。其他的 ABI 标准化细节包括C++ name decoration和同一个平台上的编译器之间的调用约定，但是不包括跨平台的兼容性。在Unix的操作系统中，存在很多运行在同一件平台上互相相关但是不兼容的操作系统（尤其是80386兼容系统）。有一些努力尝试标准化A I，以减少销售商将程序移植到其他系统时所需的工作。然而，还没有很成功的例子，虽然LSB正在为Linux做这方面的努力。

 &nbsp;&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; 它描述了应用程序与OS之间的底层接口。ABI涉及了程序的各个方面，比如：目标文件格式、数据类型、数据对齐、函数调用约定以及函数如何传递参数、如何返回值、系统调用号、如何实现系统调用等。

  &nbsp;&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; 一套完整的ABI（比如：Intel Binary Compatibility Standard (iBCS)），可以让程序在所有支持该ABI的系统上运行，而无需对程序进行修改。

## 提纲
## 一、勿在浮沙筑高台——Rust语言基础 
### 1）[Rust语言简介](books/01.md) 

### 2）初识Rust
#### [2.1 Hello Rust 牛转乾坤！](books/02.md) 

#### [2.2 Rust IDE简介](books/03.md) 

#### [2.3 Hello, Cargo!](books/04.md) 

#### [2.4 Rust 注释](books/05.md) 

#### [2.5 简单程序——我是一个锈菌](books/06.md) 

## 二、斜阳草树，寻常巷陌，人道寄奴曾住。

### [3.1 Rust的常见编程概念](books/3.1.md)

### [3.2 变量和可变性](books/3.2.md)

### [3.3 基本数据类型](books/3.3.md)

### [3.4 复合型数据类型](books/3.4.md)

### [3.5 函数](books/3.5.md)

### [3.6 字符串](books/3.6.md)

### [3.7 枚举类型](books/3.7.md)

### [3.8 struct](books/3.8.md)

## 三、没有规矩不成方圆

### [4.1 if](books/4.1.md)

### [4.2 for](books/4.2.md)

### [4.3 while](books/4.3.md)

### [4.4 loop](books/4.4.md)

### [4.5 所有权](books/4.5.md)

### [4.6 引用和借用](books/4.6.md)

### [4.7 match](books/4.7.md)

## 四、屠龙绝艺岂世用，仪凤至业非公专。——出自韩维《招景仁饮》

### [5.1 struct 高阶教程](books/5.1.md)

### [5.2 Rust 工程管理](books/5.2.md)

### [5.3 Rust 泛型](books/5.3.md)

### [5.4 特性(trait)](books/5.4.md)

### [5.5 闭包(Closure)](books/5.5.md)

### [5.6  宏](books/5.6.md)


### [5.7  单元测试用例](books/5.7.md)


### [5.8  错误处理](books/5.8.md)

### [5.9  FFI](books/5.9.md)

### [5. 指定泛型实现特定的 trait](books/5..md)

## 静观田野际，多少辍耕人

### [6.1 多线程初步](books/6.1.md)

### [6.2 多线程——channel](books/6.2.md)

### [6.3 多线程——future](books/6.3.md)

## 书上得来终觉浅 绝知此事要躬行 —— 工程实践

### [opencv rust](books/project_opencv-rust.md)

### [web 开发一](books/rust-web01.md)

### [Rust里面时间相关](books/rust-time.md)

### [Rustaceans 准则](books/rule.md)

### [FAQ](books/faq.md)