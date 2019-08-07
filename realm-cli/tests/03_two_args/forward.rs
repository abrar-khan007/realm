pub fn magic(req: realm::Request) -> realm::Result {
    let mut input = realm::request_config::RequestConfig::new(&req)?;
    match input.path.as_str() {
        "/" => {
            let i = input.get("i", false)?;
            let s = input.get("s", false)?;
            crate::routes::index::layout(&req, i, s)
        },
        _ => unimplemented!(),
    }
}