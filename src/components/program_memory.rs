use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ProgramMemoryProp {
  pub executing_address: &'static str
}

fn acode_area_style(address: &str) -> &str {
  match address {
    "f"        => "width: 60%;  height: 100%; margin-left: 4px;",
    "a" | "c"  => "width: 60%;  height: 100%; margin-left: 2.8px;",
    _others    => "width: 60%;  height: 100%; margin-left: 2px;"
  }
}

fn mcode_area_id(address: &str, executing_address: &str) -> &'static str {
  if address == executing_address {
    "executing"
  } else {
    ""
  }
}

#[function_component(ProgramMemory)]
pub fn program_memory(prop: &ProgramMemoryProp) -> Html {
    let program_memory_addresses = [
        "0","1","2","3","4","5","6","7","8","9","a","b","c","d","e","f"
    ].iter();

    html!(
        <div style="text-align: center">
          <p style="margin: 6px;">{"Program"}</p>
          <ul style="margin: 0; padding: 0;">
            {program_memory_addresses.map(|address| html!{
              <li style="list-style: none;">
                {address}
                <input
                  class="assembly-code-area" spellcheck="false"
                  style={acode_area_style(address)}
                />
                <input
                  class="machine-code-area" id={mcode_area_id(address, prop.executing_address)}
                  disabled=true style="width: 15%; height: 100%;"
                />
              </li>
            }).collect::<Html>()}
          </ul>
        </div>
    )
}