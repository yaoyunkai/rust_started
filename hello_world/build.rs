fn main() {
    // 告诉 Cargo 编译 my_c_code.c，并将其打包成一个名为 libmy_c_code.a 的静态库链接到 Rust 中
    cc::Build::new().file("c_add.c").compile("c_add");
}
