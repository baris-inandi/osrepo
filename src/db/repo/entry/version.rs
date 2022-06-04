#[derive(serde::Deserialize, Debug)]

pub struct Version {
    url: String,
    arch: Option<String>,
    is_prerelease: Option<bool>,
}

/*
    impl download etc.
*/