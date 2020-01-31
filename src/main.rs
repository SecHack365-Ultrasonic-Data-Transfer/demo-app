#![windows_subsystem="windows"]
use std::process::Command;

fn main() {
    web_view::builder()
        .title("超音波通信デモ")
        .content(web_view::Content::Html(HTML))
        .size(640, 480)
        .user_data(())
        .invoke_handler(|_, _| Ok(()))
        .run().expect("failed to create window.");

    Command::new("ls")
        .args(&["-a"])
        .spawn().expect("failed to run this command.");
}

const HTML: &str = r#"udtp"#;

