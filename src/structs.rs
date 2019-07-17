pub struct State {
    pub show_app_main_menu_bar: bool,
    pub file_menu: FileMenuState,
    pub main_body_text: String,
    pub url_to_get: String,
}

impl Default for State {
    fn default() -> Self {
        State {
            show_app_main_menu_bar: true,
            file_menu: Default::default(),
            main_body_text: String::from("Here: Have some sample text!"),
            // Changed this to a random wikipedia page for testing
             url_to_get: String::from("https://www.york.ac.uk/teaching/cws/wws/webpage1.html"),
            //url_to_get: String::from("https://en.wikipedia.org/wiki/Special:Random"),
        }
    }
}

pub struct FileMenuState {
    pub test_enabled: bool,
    pub can_search: bool,
}

impl Default for FileMenuState {
    fn default() -> Self {
        FileMenuState {
            test_enabled: true,
            can_search: true,
        }
    }
}
