use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct RegisterProps {
  pub value: i64,
  pub is_edit_mode: bool
}

#[function_component(Register)]
pub fn register(prop: &RegisterProps) -> Html {
  let register_content
    = if prop.is_edit_mode {
      String::from("")
    } else {
      prop.value.to_string()
    };
  html!(
    <div style="margin-top: 10px;">
      {"Register"}<input value={register_content}
        id="register" disabled=true style="margin-left: 4px;"
      />
    </div>
  )
}