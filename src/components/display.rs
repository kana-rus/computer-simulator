use yew::{function_component, html};

#[function_component(Display)]
pub fn display() -> Html {
  html!{
    <div style="text-align: center; height: 100%;">
      <p style="margin: 0 0 6px 0">{"Display"}</p>
      <textarea
        id="display" disabled=true
        // styled by css
      />
    </div>
  }
}