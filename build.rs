use phf_codegen::Map;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

#[derive(Deserialize)]

struct Emoji {
    pub names: Vec<String>,
    pub surrogates: String,
}

fn main() {
    println!("cargo:rerun-if-changed=discord_emojis.json");

    let mut file = BufWriter::new(
        File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("emojis.rs")).unwrap(),
    );

    write!(
        &mut file,
        "/// Compile time generated lookup table for emoji.\n"
    )
    .unwrap();
    write!(&mut file, "/// \n").unwrap();
    write!(&mut file, "/// Taken from the Discord client\n").unwrap();
    write!(
        &mut file,
        "pub static EMOJI: phf::Map<&'static str, &'static str> = "
    )
    .unwrap();

    let categories: HashMap<String, Vec<Emoji>> =
        serde_json::from_reader(File::open("discord_emojis.json").expect("to read emoji data"))
            .expect("emoji data to be valid");
    let mut m: Map<&str> = phf_codegen::Map::new();

    for (_, emojis) in &categories {
        for emoji in emojis {
            for name in &emoji.names {
                m.entry(name, &format!("\"{}\"", emoji.surrogates));
            }
        }
    }

    let m = m.build();
    write!(&mut file, "{};\n", m).unwrap();
}
