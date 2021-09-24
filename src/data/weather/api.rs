use super::model::WttrData;


/// Get the current weather in ANSII escape format for direct display
pub async fn get_ansii_weather() -> Result<String, reqwest::Error> {
    Ok(reqwest::get("http://wttr.in/?QnmAF").await?.text().await?)
}

/// Get the current and future weather in programmatic format
pub async fn get_programmatic_weather() -> Result<WttrData, reqwest::Error> {
    Ok(reqwest::get("http://wttr.in/?format=j1").await?.json().await?)
}