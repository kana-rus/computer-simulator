use web_sys::MouseEvent;
use yew::{function_component, html, Properties, Callback};

#[derive(Properties, PartialEq)]
pub struct ProcessCallbacks {
    pub handle_step: Callback<MouseEvent>
}

#[function_component(ProcessButtons)]
pub fn process_buttons(prop: &ProcessCallbacks) -> Html {
    let buttons_container_style = "
        display: flex;
        flex-flow: column;
        margin-right: 2.5%;
        margin-left: auto;
    ";
    let button_style = "
        width: 80px;
        height: 41px;
        border-color: white;
    ";

    html!{
        <span style={buttons_container_style}>
          <button class="process-buttons" disabled=true style={button_style} onclick={prop.handle_step.clone()}>{"step"}</button>
          <button class="process-buttons" disabled=true style={button_style}>{"go"}</button>
        </span>
    }
}