
pub fn load_string(file_name: &str) -> anyhow::Result<String> {

    // @todo: Replace this when we get a proper build system
    let path = std::path::Path::new("")
    // let path = std::path::Path::new(&std::env::var("OUT_DIR").unwrap())
        .join("assets")
        .join(file_name);

    let text = std::fs::read_to_string(path)?;

    Ok(text)
}

pub fn load_binary(file_name: &str) -> anyhow::Result<Vec<u8>> {

    // @todo: Replace this when we get a proper build system
    let path = std::path::Path::new("")
    // let path = std::path::Path::new(&std::env::var("OUT_DIR").unwrap())
        .join("assets")
        .join(file_name);

    let data = std::fs::read(path)?;

    Ok(data)
}


pub trait Resource: Sized {
    fn load(file_name: &str) -> anyhow::Result<Self>;
}