use yew::{function_component, html, Callback, Event, MouseEvent, use_state_eq};
use wasm_bindgen::{JsCast};
use web_sys::{HtmlInputElement};

mod components; use components::{
  mode_radio::ModeRadio,
  program_memory::ProgramMemory,
  data_memory::DataMemory,
  display::Display,
  program_counter::{ProgramCounter},
  register::{Register},
  process_buttons::ProcessButtons,
};

mod utils; use utils::{
  document,
  print_log_in_display,
  display_of,
  get_data_list,
  get_parsed_mcode_list,
  parse_address_to_index,
  switch_to_edit,
  switch_to_execute,

  exec_utils::{
    exec,
    go_through,
  }
};


#[function_component(App)]
fn app() -> Html {
  let executing_address = use_state_eq(|| "");
  let register_value = use_state_eq(|| 0_i64);
  let data_list = use_state_eq(|| [None; 8]);
  let mcode_list = use_state_eq(|| [(0_usize, 0_usize); 16]);


  /*let handle_reset = Callback::from(|_:MouseEvent| {
    window().expect("Window not set")
            .location()
            .reload().expect("Failed to reload");
  });*/
    
  let handle_mode_change = {
    let executing_address = executing_address.clone();
    let register_value = register_value.clone();
    let data_list = data_list.clone();
    let mcode_list = mcode_list.clone();
    let doc = document();
    Callback::from(move |_:Event| {
      let is_execute_mode = doc.get_element_by_id("execute-button")
                               .expect("Execute button doesn't exist")
                               .unchecked_into::<HtmlInputElement>()
                               .checked();
      if is_execute_mode {
        match get_data_list(&doc) {
          Err(msg) => {
            print_log_in_display(msg, &display_of(&doc));
            doc.get_element_by_id("edit-button")
               .expect("Edit button not found")
               .unchecked_into::<HtmlInputElement>()
               .click()
          },
          Ok(list) => {
            data_list.set(list);
            executing_address.set("0");
            switch_to_execute(&doc);
            // after machine codes get rendered by switch_to_execute() 
            mcode_list.set(get_parsed_mcode_list(&doc));
          }
        }
      } else {
        register_value.set(0);
        executing_address.set("");
        switch_to_edit(&doc);
      }
    })
  };  /* 選択されていない方のラジオボタンをクリックしてもなぜか選択が変わらない。
  - 見た目: 変わらない
  - onchange: 実行される
  - 選択: されない（したがって、複数回連続で押すと押した回数だけ onchange が実行される）
  Callback が move を伴わないようにすると、なぜかちゃんと選択が変わる。

  今回はひとまず、executing_address を、ラジオボタンたちを含むコンポーネントに
  prop として渡してしまい、その中身が
  - 空文字なら Edit mode,
  - そうでなければ Execute mode
  と判断してそれぞれ対応する方のラジオボタンに checked=true を指定した Html を返すように
  して妥協した。
  */ 

  let handle_step = {
    let executing_address = executing_address.clone();
    let mcode_list = mcode_list.clone();
    let register_value = register_value.clone();
    let data_list = data_list.clone();
    let doc = document();
    
    Callback::from(move |_:MouseEvent| {
      let result = exec(*executing_address,
        *mcode_list,
        *register_value,
        *data_list
      );
      match result {
        Ok((next_address, next_register_value, new_data_list)) => {
          executing_address.set(next_address);
          register_value.set(next_register_value);
          data_list.set(new_data_list);
        },
        Err(msg) => {
          let error_msg = format!("line {}: {}",
            parse_address_to_index(*executing_address), msg
          );
          print_log_in_display(error_msg, &display_of(&doc));

          register_value.set(0);
          executing_address.set("");
          switch_to_edit(&doc);
        }
      }
    })
  };

  let handle_go_through = {
    let executing_address = executing_address.clone();
    let register_value = register_value.clone();
    let data_list = data_list.clone();
    let doc = document();

    Callback::from(move |_:MouseEvent| {
      let result = go_through( *mcode_list,
        *executing_address,
        *register_value,
        *data_list
      );
      match result {
        Ok(last_datalist) => {
          data_list.set(last_datalist);
        },
        Err((msg, last_datalist)) => {
          data_list.set(last_datalist);
          print_log_in_display(msg, &display_of(&doc));
        }
      }
      register_value.set(0);
      executing_address.set("");
      switch_to_edit(&doc);
    })
  };
    
  
  html! {
    <>
      <header style="display: flex;">
        <ModeRadio
          {handle_mode_change} // {handle_reset}
          is_edit_mode={*executing_address==""}
        />
        <ProcessButtons {handle_step} {handle_go_through}/>
      </header>
      <div style="display: flex">
        <span style="flex-basis: 50%;">
          <ProgramMemory executing_address={*executing_address}/>
        </span>
        <span style="flex-basis: 50%; display: flex; flex-flow: column;">
          <span style="flex-basis: 60%">
            <DataMemory data_list={*data_list}/>
          </span>
          <span style="flex-basis: 40%">
            <Display/>
          </span>
        </span>
      </div>
      // <div style="display: flex; margin-top: 15px;">
        <div style="margin: 15px 0 0 0; display: flex;">
          <ProgramCounter address={*executing_address}/>
          <Register value={*register_value} is_edit_mode={*executing_address==""}/>
        </div>
        // <ProcessButtons {handle_step} {handle_go_through}/>
      // </div>
    </> 
  }
}

fn main() {
  yew::start_app::<App>();
}
