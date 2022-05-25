use yew::{function_component, html};

#[function_component(Display)]
pub fn display() -> Html {
  let display_style = "
  width: 85%;
    height: calc(100% - 30px);
    padding: 0;
    margin-left: 5px;
  "; // <p></p> 's height: 24px and margin: 6px

  html!{
      <div style="text-align: center; height: 100%;">
        <p style="margin: 0 0 6px 0">{"Display"}</p>
        <textarea
          id="display" disabled=true
          style={display_style}
        />
      </div>
  }
}