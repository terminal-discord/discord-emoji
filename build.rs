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
    #[serde(rename = "diversityChildren")]
    pub diversity_children: Option<Vec<Emoji>>,
}

fn main() {
    println!("cargo:rerun-if-changed=discord_emojis.json");

    let mut file = BufWriter::new(
        File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("emojis.rs")).unwrap(),
    );

    writeln!(
        &mut file,
        "/// Compile time generated lookup table for emoji."
    )
    .unwrap();
    writeln!(&mut file, "/// ").unwrap();
    writeln!(&mut file, "/// Taken from the Discord client").unwrap();
    write!(
        &mut file,
        "pub static EMOJI: phf::Map<&'static str, &'static str> = "
    )
    .unwrap();

    let categories: HashMap<String, Vec<Emoji>> =
        serde_json::from_reader(File::open("discord_emojis.json").expect("to read emoji data"))
            .expect("emoji data to be valid");
    let mut m: Map<&str> = phf_codegen::Map::new();

    for emojis in categories.values() {
        for emoji in emojis {
            #[cfg(feature = "diversity")]
            for diversity_child in emoji.diversity_children.iter().flatten() {
                for name in &diversity_child.names {
                    m.entry(name, &format!("\"{}\"", diversity_child.surrogates));
                }
            }
            for name in &emoji.names {
                m.entry(name, &format!("\"{}\"", emoji.surrogates));
            }
        }
    }

    let m = m.build();
    writeln!(&mut file, "{};", m).unwrap();
}
