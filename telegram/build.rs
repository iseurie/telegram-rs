extern crate telegram_codegen;

use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("schema.rs");
    telegram_codegen::translate_from_json_file("schema/schema.json", &dest_path).unwrap();

    let dest_path = Path::new(&out_dir).join("mtproto_schema.rs");
    telegram_codegen::translate_from_json_file("schema/mtproto-schema.json", &dest_path).unwrap();
}
