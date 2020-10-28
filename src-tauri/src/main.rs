mod cmd;
mod example;

use example::ExamplePlugin;

fn main() {
  tauri::AppBuilder::new()
    .plugin(ExamplePlugin::new())
    .build()
    .run();
}
