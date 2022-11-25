use std::env;
use std::fmt::format;
use std::io::Cursor;

use http_req;
use http_req::request;

const DOWNLOAD_BASE_URL: &'static str = "https://raw.githubusercontent.com/ragegun/ragegun-assets/master/";

const LEXICA: [&'static str; 5] = [
    "age.csv",
    "distress.csv",
    "emolex-full.csv",
    "gender.csv",
    "perma.csv"
];

const TOKENIZER: [&'static str; 1] = [
    "en_tokenizer.bin"
];

pub fn download_asset(asset: &str, out_path: &str) -> Result<(), http_req::error::Error> {
    let mut response = Vec::new();
    let mut writer = Cursor::new(&mut response);

    let url = format!("{}{}", DOWNLOAD_BASE_URL, asset);

    request::get(&url, &mut writer)?;

    // write file to outbuf
    std::fs::write(out_path, response)?;

    Ok(())
}

fn main() {
    let dir = env::var("OUT_DIR").unwrap();

    let basename = format!("assets");
    for file in LEXICA.iter().chain(TOKENIZER.iter()) {
        let out_path = format!("{}/{}", &dir, file);

        // check if file exists
        if std::path::Path::new(&out_path).exists() {
            continue;
        }

        download_asset(&file, &out_path).unwrap();
    }

    println!("cargo:rustc-env=ASSET_DIR={}", &dir);
}