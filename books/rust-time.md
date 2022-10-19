# Rust里面时间相关
[TOC]

## 概述
有关时间是开发程序经常需要的模块。在这里我吧我认为重要的知识在这里和大家分享一下。
## 时间

## 时区概述
<https://en.wikipedia.org/wiki/List_of_tz_database_time_zones>

这里面有详细的各个时区的描述，尤其是EST、EDT，这些美国时间，如果你不开发美国软件，这些很没有用，如果你一旦开发，就非常有用。因为美国各个州有的有夏令时，有的没有，痛苦。

|时区|UTC offset|UTC DST|Notes|
|---|---|---|---|
|EST|-5:00|-5:00|Choose a zone that currently observes EST without daylight saving time, such as America/Cancun.|
|EDT|-5:00|-4:00|Choose a zone that observes EST with United States daylight saving time rules, such as America/New_York.|
## 时区

获取时区，我从<https://crates.io/>上搜索到一个`crate`——iana-time-zone。这个工具可以读取Linux、Windows、Mac上设置的本机时区。

具体写法如下：

```rust
extern crate iana_time_zone;
println!("current: {}", iana_time_zone::get_timezone().unwrap());
```

## 根据经纬度获得时区
我还发现一个有趣的工具，通过经纬度获得时区。<https://crates.io/> 上的——zone-detect。

具体写法如下：

```rust
use clap::AppSettings;
use std::{path::PathBuf, process::exit};
use structopt::StructOpt;
use zone_detect::{Database, Location};

#[derive(StructOpt, Debug)]
#[structopt(name = "demo", global_settings(&[AppSettings::AllowNegativeNumbers]))]
struct Opt {
    database_path: PathBuf,
    latitude: f32,
    longitude: f32,
}

fn lookup(opt: &Opt) -> Result<(), zone_detect::Error> {
    let database = Database::open(&opt.database_path)?;
    let result = database.lookup(Location {
        latitude: opt.latitude,
        longitude: opt.longitude,
    });
    for (index, zone) in result.matches.iter().enumerate() {
        println!("zone {}: {:#?}", index, zone);
    }
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    if let Err(err) = lookup(&opt) {
        eprintln!("error: {}", err);
        exit(1);
    }
}

```

