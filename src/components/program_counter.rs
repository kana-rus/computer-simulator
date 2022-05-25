use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct ProgramCounterProp {
  pub address: &'static str
}

#[function_component(ProgramCounter)]
pub fn program_counter(prop: &ProgramCounterProp) -> Html {
  html!{
    <span style="flex-basis: 50%; display: flex; justify-content: center;">
      <span style="margin: auto 3px auto 0;">
        <p style="margin: 0;">{"Program"}</p>
        <p style="margin: 0;">{"Counter"}</p>
      </span>
      <span style="position: relative; width: 40%;">
        <input id="program-counter"
          value={prop.address}  disabled=true
          style="position: absolute; width: 100%;
                 top: 50%; transform: translateY(-50%);"
        />
      </span>
    </span>
  }
}