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
    
  html!{
    <span style="display: flex; margin-top: 6px;">
      <span style="margin: 0;">
        <p style="margin: 0; width: 90px;">{"Register"}</p>
      </span>
      <span>
        <input id="register"
          value={register_content} disabled=true
          style="width: 75%;"
        />
      </span>
    </span>
  }
}