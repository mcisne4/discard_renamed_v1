pub struct Settings {
    pub theme: String,
    pub show_welcome_screen: bool,
}
impl Settings {
    pub fn default() -> Self {
        Self {
            theme: String::from("dark"),
            show_welcome_screen: true,
        }
    }

    pub fn as_insert_statement(&self) -> String {
        format!(
            "INSERT or REPLACE INTO settings (
          id,
          theme,
          show_welcome_screen
        ) VALUES (
          'main',
          '{}',
          '{}'
        );",
            self.theme,
            match self.show_welcome_screen {
                true => 1,
                false => 0,
            }
        )
    }
}

fn tw() {
    let x = Settings::default();
}
