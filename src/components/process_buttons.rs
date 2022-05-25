use web_sys::MouseEvent;
use yew::{function_component, html, Properties, Callback};

#[derive(Properties, PartialEq)]
pub struct ProcessCallbacks {
    pub handle_step: Callback<MouseEvent>,
    pub handle_go_through: Callback<MouseEvent>
}

#[function_component(ProcessButtons)]
pub fn process_buttons(prop: &ProcessCallbacks) -> Html {
    /*let buttons_container_style = "
        display: flex;
        flex-flow: column;
        margin-right: 3.5%;
        margin-left: auto;
    ";*/
    let buttons_container_style = "
        margin-right: 8px;
        margin-left: auto;
    ";
    let button_style = "
        width: 42px;
        height: 42px;
        border-color: white;
        border-radius: 20px;
    ";

    html!{
        <span style={buttons_container_style}>
          <button id="step-button"
            class="process-buttons" disabled=true
            style={button_style} onclick={prop.handle_step.clone()}
          >{"step"}</button>
          <button
            class="process-buttons" disabled=true
            style={button_style} onclick={prop.handle_go_through.clone()}
          >{"go"}</button>
        </span>
    }
}