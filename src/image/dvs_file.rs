use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DvsData {}
impl From<DvsFile> for DvsData {
    fn from(_value: DvsFile) -> Self {
        let mut _dvs = DvsData::default();
        todo!()
    }
}
impl Default for DvsData {
    fn default() -> Self {
        todo!()
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ValBoolOrString {
    Bool(bool),
    String(String),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "version")]
enum DvsFile {
    #[serde(rename = "1")]
    V1(DvsFileV1),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DvsFileV1 {
    id: String,
    author: Option<String>,
    repo: Option<String>,
    license: Option<String>,
    desc: Option<String>,
    container: Option<DvsFileContainerV1>,
    image: DvsFileImageV1,
    tag: Option<HashMap<String, DvsFileTagV1>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DvsFileContainerV1 {
    workspace: Option<String>,
    mount_pwd: Option<String>,
    mounts: Option<Vec<String>>,
    env: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DvsFileImageV1 {
    default: Option<String>,
    tags: Option<Vec<String>>,
    platforms: Option<Vec<String>>,
    dockerfile: Option<String>,
    image: Option<String>,
    pull: Option<ValBoolOrString>,
    entrypoint: Option<String>,
    cmd: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DvsFileTagV1 {
    platforms: Option<Vec<String>>,
    dockerfile: Option<String>,
    image: Option<String>,
    entrypoint: Option<String>,
    cmd: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::image::dvs_file::DvsFile;

    #[test]
    #[allow(non_snake_case)]
    fn dvs_file__abc() {
        let test: DvsFile = toml::from_str(include_str!("bundled/_abc.toml")).unwrap();
        let _test = test;
    }

    #[test]
    fn dvs_file_rust() {
        let test: DvsFile = toml::from_str(include_str!("./bundled/rust.toml")).unwrap();
        let _test = test;
    }
}
