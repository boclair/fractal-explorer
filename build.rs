fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_style("cupertino-dark".into());

    slint_build::compile_with_config("ui/main-window.slint", config).unwrap();
}