include!(concat!(env!("OUT_DIR"), "/emojis.rs"));

pub fn lookup(name: &str) -> Option<&str> {
    EMOJI.get(name).map(|x| *x)
}
