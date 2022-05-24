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
    <span style="display: flex;">
      <span style="margin-right: 4px;">
        <p style="margin: 0">{"Register"}</p>
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