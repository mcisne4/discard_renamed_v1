//

pub enum Theme {
    DARK,
    Light,
}

pub struct SettingsDB {
    id: String,
    theme: Theme,
    show_display_settings: bool,
}
impl SettingsDB {
    pub fn default() -> Self {
        Self {
            id: String::from("main"),
            theme: Theme::DARK,
            show_display_settings: true,
        }
    }

    //
}
