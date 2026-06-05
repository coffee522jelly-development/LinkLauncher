fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/core/link_launcher_core.cpp")
        .compile("link_launcher_core");

    tauri_build::build()
}
