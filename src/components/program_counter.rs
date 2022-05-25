use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct ProgramCounterProp {
  pub address: &'static str
}

#[function_component(ProgramCounter)]
pub fn program_counter(prop: &ProgramCounterProp) -> Html {
  html!{
    <span style="display: flex;">
      <span style="margin: 0;">
        <p style="margin: 0; width: 90px;">{"Program"}</p>
        <p style="margin: 0; width: 90px;">{"Counter"}</p>
      </span>
      <span style="position: relative; width: 75%;">
        <input id="program-counter"
          value={prop.address}  disabled=true
          style="position: absolute; width: 75%; top: 50%; transform: translateY(-50%);"
        />
      </span>
    </span>
  }
}