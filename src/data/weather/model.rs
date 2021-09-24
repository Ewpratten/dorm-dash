use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct WttrHour {
    #[serde(rename = "tempC")]
    pub temperature: i8,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct WttrDay {
    pub hourly: Vec<WttrHour>,
    pub date: String
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct WttrData {
    pub weather: Vec<WttrDay>,
}
