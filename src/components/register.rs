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
    <span style="flex-basis: 50%; display: flex; justify-content: center;">
      <span style="margin: auto 3px auto 0;">
        <p style="margin: 0:">{"Register"}</p>
      </span>
      <span style="position: relative; width: 40%;">
        <input id="register"
          value={register_content} disabled=true
          style="position: absolute; width: 100%;
                 top: 50%; transform: translateY(-50%);
          "
        />
      </span>
    </span>
  }
}