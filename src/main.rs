#![windows_subsystem="windows"]
use std::process::Command;

fn main() {
    web_view::builder()
        .title("超音波通信デモ")
        .content(web_view::Content::Html(HTML))
        .size(640, 480)
        .user_data(())
        .invoke_handler(|_webview, arg| {
            if arg == "stream" {
                Command::new("python3")
                    .args(&["./assets/python/receive.py"])
                    .spawn().expect("failed to run this command.");
            }
            Ok(())
        })
        .run().expect("failed to create window.");

    /*
    Command::new("ls")
        .args(&["-a"])
        .spawn().expect("failed to run this command.");
    */
}

const HTML: &str = r#"<button onclick="external.invoke('stream')">stream</button>"#;

#[test]
fn nop() { assert_eq!(0, 0); }

