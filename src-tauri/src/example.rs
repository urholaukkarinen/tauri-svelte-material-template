use tauri::{plugin::Plugin, Webview};
use crate::cmd::Cmd::Hello;

pub(crate) struct ExamplePlugin;

impl ExamplePlugin {
    pub fn new() -> Self {
        Self {}
    }

    fn say_hello_to_webview(&self, webview: &mut Webview, name: &str) {
        let mut webview = webview.as_mut();
        tauri::event::emit(&mut webview, "hello_from_rust".to_string(),
            Some(format!("Hello {}", name))).expect("failed to emit");
    }
}

impl Plugin for ExamplePlugin {
    /// Callback invoked when the webview is created.
    fn created(&self, _webview: &mut Webview) {
        
    }

    /// Callback invoked when the webview is ready.
    fn ready(&self, _webview: &mut Webview) {

    }
    
    fn extend_api(&self, webview: &mut Webview, payload: &str) -> Result<bool, String> {
        match serde_json::from_str(payload) {
            Ok(command) => {
                match command {
                    Hello { name } => {
                        println!("{} said hello!", name);
                        self.say_hello_to_webview(webview, &name);
                    }
                }
                Ok(true)
            },
            Err(e) => {
                Err(e.to_string())
            }
        }
    }
}