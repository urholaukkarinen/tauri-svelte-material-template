use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  // note that rename_all = "camelCase": you need to use "addText" on JS
  AddText { text: String },
}
