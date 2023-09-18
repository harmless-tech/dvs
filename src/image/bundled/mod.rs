use phf::phf_map;

static BUNDLED_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "rust" => include_str!("./rust.toml")
};

pub fn get_bundled(item: &str) -> Option<&&'static str> {
    BUNDLED_MAP.get(item)
}
