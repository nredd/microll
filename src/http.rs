extern crate reqwest;

pub fn get_text(url: &String) -> Result<String, reqwest::Error> {
    // Get all response texts with utf-8 encoding, as required by imgui
    let body = reqwest::get(url)?.text_with_charset("utf-8")?;
    Ok(body)
}
