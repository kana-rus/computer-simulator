use yew::{function_component, html, Properties, Callback, Event, MouseEvent};

#[derive(Properties, PartialEq)]
pub struct ModeRadioProp {
  pub handle_mode_change: Callback<Event>,
  pub handle_reset: Callback<MouseEvent>,

  pub is_edit_mode: bool,
}

#[function_component(ModeRadio)]
pub fn mode_radio(prop: &ModeRadioProp) -> Html {
  let reset_button_style = "
    position: absolute;
    right: 8px;
    margin: 3px auto;
    border-color: white;
    width: 45px;
    height: 27px;
  ";

  html! {
    <div class="mode-radio-container" style="position: relative;">
      <input checked={prop.is_edit_mode}
        type="radio" name="mode_radio" id="edit-button"
        onchange={prop.handle_mode_change.clone()}
      />{"Edit"}
      <input checked={!prop.is_edit_mode}
        type="radio" name="mode_radio" id="execute-button"
        style="margin-left: 6%;" onchange={prop.handle_mode_change.clone()}
      />{"Execute"}
      <button style={reset_button_style} onclick={prop.handle_reset.clone()}>{"reset"}</button>
    </div>
  }
}