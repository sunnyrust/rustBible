fn main() {
    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .flag("-Wall")
        .flag("-std=c++14")
        .flag("-c")
        .file("cpp_lib/greet.cpp")
        .compile("libgreet.a");    // greet.so
}