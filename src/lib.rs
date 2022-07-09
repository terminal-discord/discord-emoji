include!(concat!(env!("OUT_DIR"), "/emojis.rs"));

pub fn lookup(name: &str) -> Option<&str> {
    println!("{}", EMOJI.len());
    EMOJI.get(name).copied()
}

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}
