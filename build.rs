use std::env;
use std::fs;
use std::path::Path;
use reqwest::blocking::get;
use std::io::Write;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("svgo.browser.js");

    let response = get("https://unpkg.com/svgo@3.3.2/dist/svgo.browser.js")
        .expect("Failed to send request");
    if !response.status().is_success() {
        panic!("Failed to download svgo.browser.js");
    }

    let mut file = fs::File::create(&dest_path).expect("Failed to create file");
    file.write_all(&response.bytes().expect("Failed to read response"))
        .expect("Failed to write to file");
}
