use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct DataMemoryProps {
  pub data_list: [Option<i64>; 8],
}

fn data_appearance(data: Option<i64>) -> String {
  if data == None {
    String::from("")
  } else {
    data.unwrap().to_string()
  }
}

#[function_component(DataMemory)]
pub fn data_memory(prop: &DataMemoryProps) -> Html {
  let data_memory_indexes = [
      0, 1, 2, 3, 4, 5, 6, 7
  ].iter();

  html!(
    <div style="text-align: center">
      <p style="margin-bottom: 2.5%">{"Data"}</p>
      <ul style="margin: 0; padding: 0;">
        {data_memory_indexes.map(|index| html!{
          <li style="list-style: none;">
            {&index}<input
              class="data-memory" spellcheck="false"
              style="margin-left: 2px; width: 82%;"
              value={data_appearance(prop.data_list[*index])}
            />
          </li>
        }).collect::<Html>()}
      </ul>
    </div>
  )
}

