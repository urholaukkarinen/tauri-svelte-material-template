use std::cell::RefCell;

use tauri::{plugin::Plugin, Webview};

pub(crate) struct ExamplePlugin {
    texts: RefCell<Vec<String>>
}

impl ExamplePlugin {
    pub fn new() -> Self {
        Self {
            texts: RefCell::new(vec![])
        }
    }
    
    fn add_text(&self, text: String) {
        println!("Adding text: {}", text);
        self.texts.borrow_mut().push(text);
    }

    fn send_texts_to_webview(&self, webview: &mut Webview) {
        let texts = self.texts.to_owned();
        println!("Sending texts to webview: {:?}", texts.borrow());
        let mut webview = webview.as_mut();
        tauri::event::emit(&mut webview, "texts".to_string(),
            Some(texts)).expect("failed to emit");
    }
}

impl Plugin for ExamplePlugin {
    /// Callback invoked when the webview is created.
    fn created(&self, webview: &mut Webview) {
        
    }

    /// Callback invoked when the webview is ready.
    fn ready(&self, webview: &mut Webview) {
        self.send_texts_to_webview(webview);
    }
    
    fn extend_api(&self, webview: &mut Webview, payload: &str) -> Result<bool, String> {
        match serde_json::from_str(payload) {
            Err(e) => {
                Err(e.to_string())
            }
            Ok(command) => {
                match command {
                    crate::cmd::Cmd::AddText { text } => {
                        self.add_text(text);
                        self.send_texts_to_webview(webview);
                    }
                }
                Ok(true)
            }
        }
    }
}