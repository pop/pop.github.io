use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub projects: Vec<Game>,
    pub quotes: Vec<Quote>,
    pub desktop: DesktopConfig,
    pub taskbar: TaskbarConfig,
    pub start_menu: StartMenu,
    pub clippy: ClippyConfig,
    pub webamp: Option<WebampConfig>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Game {
    pub id: String,
    pub title: String,
    pub description: String,
    pub contributors: Vec<String>,
    pub tech: Vec<Tech>,
    pub icon: String,
    pub demo: Option<String>,
    pub launch_url: String,
    pub launch_type: String,
    pub launch_label: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Tech {
    pub name: String,
    pub icon: String,
    pub url: Option<String>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Quote {
    pub text: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct DesktopConfig {
    pub background_color: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct TaskbarConfig {
    pub start_icon: String,
    pub start_label: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct StartMenuItem {
    pub icon: String,
    pub title: String,
    pub url: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct StartMenu {
    pub items: Vec<StartMenuItem>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct WebampConfig {
    #[serde(default)]
    pub tracks: Vec<WebampTrack>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct WebampTrack {
    pub url: String,
    pub artist: String,
    pub title: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct ClippyConfig {
    pub icon: String,
    pub brand_title: String,
    pub logo: String,
    pub modal_title: String,
    pub modal_ok_label: String,
    pub modal_paragraphs: Vec<String>,
}

pub fn load_config() -> Config {
    let raw = include_str!("../portfolios.toml");
    toml::from_str(raw).expect("failed to parse portfolios.toml")
}
