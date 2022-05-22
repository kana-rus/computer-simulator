use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct DataMemoryProps {
  pub is_edit_mode: bool,
  pub contents: [i64; 8],
}

#[function_component(DataMemory)]
pub fn data_memory(prop: &DataMemoryProps) -> Html {
  let data_memory_indexes = [
      0, 1, 2, 3, 4, 5, 6, 7
  ].iter();

  if prop.is_edit_mode {
    html!(
      <div style="text-align: center">
        <p style="margin-bottom: 2.5%">{"Data"}</p>
        <ul style="margin: 0; padding: 0;">
          {data_memory_indexes.map(|index| html!{
            <li style="list-style: none;">
              {&index}<input
                class="data-memory" spellcheck="false"
                style="margin-left: 2px; width: 87%;"
              />
            </li>
          }).collect::<Html>()}
        </ul>
      </div>
    )
  } else {
    html!(
      <div style="text-align: center">
        <p style="margin-bottom: 2.5%">{"Data"}</p>
        <ul style="margin: 0; padding: 0;">
          {data_memory_indexes.map(|index| html!{
            <li style="list-style: none;">
              {&index}<input
                class="data-memory" spellcheck="false"
                style="margin-left: 2px; width: 87%;"
                value={prop.contents[*index].to_string()}
              />
            </li>
          }).collect::<Html>()}
        </ul>
      </div>
    )
  }
   
}

