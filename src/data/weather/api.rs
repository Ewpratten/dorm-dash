use log::info;

use super::model::WttrData;


/// Get the current weather in ANSI escape format for direct display
pub async fn get_ansi_weather() -> Result<String, reqwest::Error> {
    info!("Fetching ANSI weather");
    Ok(reqwest::get(
        // "http://v2.wttr.in/?FAT"
        "http://wttr.in/?QnmAF"
    ).await?.text().await?)
}

/// Get the current and future weather in programmatic format
pub async fn get_programmatic_weather() -> Result<WttrData, reqwest::Error> {
    info!("Fetching programmatic weather");
    Ok(reqwest::get("http://wttr.in/?format=j1").await?.json().await?)
}