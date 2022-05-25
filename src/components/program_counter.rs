use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct ProgramCounterProp {
  pub address: &'static str
}

#[function_component(ProgramCounter)]
pub fn program_counter(prop: &ProgramCounterProp) -> Html {
  html!{
    <span style="display: flex;">
      <span style="margin-right: 2.2px;">
        <p style="margin: 0">{"Program"}</p>
        <p style="margin: 0">{"Counter"}</p>
      </span>
      <span>
        <input id="program-counter"
          value={prop.address}  disabled=true
          style="margin-top: 8px; width: 75%;"
        />
      </span>
    </span>
  }
}