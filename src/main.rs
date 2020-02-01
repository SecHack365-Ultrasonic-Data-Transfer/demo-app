#![windows_subsystem="windows"]
use std::process::Command;

/*
struct UserData {
    text: String,
}
*/

fn main() {
    web_view::builder()
        .title("UDTP Demo")
        .content(web_view::Content::Html(HTML))
        .size(640, 480)
        // .user_data( UserData { text: String::new() } )
        .user_data(())
        .invoke_handler(|_webview, arg| {
            match arg {
                "stream" => {
                    Command::new("python3")
                        .args(&["./assets/python/receive.py"])
                        .spawn().expect("failed to run this command.");
                },
                "send" => {
                    println!("-> {:#?}", arg);
                    Command::new("python3")
                        .args(&["./assets/python/play_sond_connect_with_header.py", "hoge"])
                        .spawn().expect("failed to run this command.");
                },
                _ => { }
            };

            Ok(())
        })
        .run().expect("failed to create window.");
}

const HTML: &str = r#"
<head>
    <link rel="stylesheet" href="./assets/css/style.css">
    <link href="https://fonts.googleapis.com/css?family=Montserrat:100,300&display=swap" rel="stylesheet">
</head>
<body style="background: #181E29;">
    <!--
    <h1 style="
        font-family: 'Montserrat', sans-serif;
        text-align: center;
        font-size: 30px;

        position: relative;
        top: 70px;

        background: linear-gradient(110deg, #601FC7, #3182E0);
        -webkit-background-clip: text;
        -webkit-text-fill-color: rgba(255,255,255,0.0);
        color: #ff0000;
    ">Ultrasonic Data Trans</h1>
    -->

    <input value="hoge" onkeydown="external.invoke('send', 'test')" placeholder="enter message" style="
        border-radius: 100px;
        background: #f0f0f0;
        border: none;

        transform: translate(-50%, -50%);
        position: absolute;
        left: 50%;
        top: 50%;

        height: 50px;
        width: 350px;

        font-family: 'Montserrat', sans-serif;
        text-align: center;
        font-size: 20px;
        color: #242424;
    ">

    <!-- for start streaming -->
    <!--
    <button onclick="external.invoke('stream')" style="
        background: none;
        border: none;

        font-family: 'Montserrat', sans-serif;
        text-align: center;
        font-size: 24px;
        color: #F4F4F4;

        position: absolute;
        left: 50%;
        top: 50%;

        transform: translate(-50%, -50%);
        cursor: pointer;
    ">stream</button>
    -->

    <!-- graphics -->
    <!-- <div style="
        border-radius: 1000px;
        height: 90px;
        width: 90px;

        position: absolute;
        right: 130px;
        top: 120px;

        background: #3182e0;
        background: -moz-linear-gradient(left, #3182e0 0%, #5a29c4 100%);
        background: -webkit-linear-gradient(left, #3182e0 0%,#5a29c4 100%);
        background: linear-gradient(to right, #3182e0 0%,#5a29c4 100%);
        filter: progid:DXImageTransform.Microsoft.gradient(startColorstr='#3182e0', endColorstr='#5a29c4',GradientType=1);
    "></div>
    -->

    <script>
        document.querySelector('input').addEventListener('keydown', e => {
            if (e.keyCode == 13) {
                // hoge
            }
        });
    </script>
</body>
"#;

#[test]
fn nop() { assert_eq!(0, 0); }

