
fn main() {
    println!("cargo:rerun-if-changed=src/lib.cpp");
    cc::Build::new()
        .cpp(true)
        .file("src/lib.cpp")
        .include("stdio.h")
        .include("src/rapidjson/include/rapidjson/document.h")
        .include("src/rapidjson/include/rapidjson/writer.h")
        .include("src/rapidjson/include/rapidjson/stringbuffer.h")
        .include("stdio.h")
        .include("string.h")
        .opt_level(3)
        .compile("foo");
}