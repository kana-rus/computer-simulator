use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct ProgramCounterProp {
  pub address: &'static str
}

#[function_component(ProgramCounter)]
pub fn program_counter(prop: &ProgramCounterProp) -> Html {
    html!{
        <div style="display: flex;">
          <div style="margin-right: 2px;">
            <p style="margin: 0">{"Program"}</p>
            <p style="margin: 0">{"Counter"}</p>
          </div>
          <div>
            <input
              id="program-counter" value={prop.address}
              disabled=true style="margin-top: 12.5%;"
            />
          </div>
        </div>
    }
}