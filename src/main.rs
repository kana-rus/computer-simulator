use yew::{function_component, html, Callback, Event, MouseEvent, use_state, use_state_eq};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, Document, HtmlInputElement, HtmlTextAreaElement, console};

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
  get_display,
  disable_elms_by_class,
  enable_elms_by_class,
  // print_in_display,
  log_in_display,
  print_machine_code,
  clear_exec_items,
  assemble,
  get_data_list,
  get_parsed_mcode_list,
  exec, print_in_display,
};


fn assembe_program(doc: &Document) -> bool {
  let program = doc.get_elements_by_class_name("assembly-code-area");
  let machine_code_area_list = doc.get_elements_by_class_name("machine-code-area");
  let display = doc.get_element_by_id("display")
                   .expect("display doesm't exist")
                   .unchecked_into::<HtmlTextAreaElement>();
  
  let mut assembled_successfully = true;
  for address in 0..16 {
    let assembly_code = program.item(address)
                               .expect("failed to get code from program")
                               .unchecked_into::<HtmlInputElement>()
                               .value();
    match assemble(assembly_code) {
      Err(msg) => {
        log_in_display(format!("line {}: AssembleError", address), &display);
        log_in_display(msg, &display);
        assembled_successfully = false;
        break;
      },
      Ok(machine_code) => {
        print_machine_code(machine_code, address, &machine_code_area_list);
      }
    }
  }
  if assembled_successfully {
    log_in_display(String::from("assembled successfully"), &display);
  }
  assembled_successfully
}
fn switch_to_edit(doc: &Document) {
  clear_exec_items(doc);
  disable_elms_by_class("process-buttons", doc);
  enable_elms_by_class("assembly-code-area", doc);
  enable_elms_by_class("data-memory", doc);
}
fn switch_to_execute(doc: &Document) {
  disable_elms_by_class("assembly-code-area", doc);
  disable_elms_by_class("data-memory", doc);
  if assembe_program(doc) {
    enable_elms_by_class("process-buttons", &doc);
  }
}


#[function_component(App)]
fn app() -> Html {
  let executing_address = use_state(|| "");
  let register_value = use_state_eq(|| 0_i64);
  let data_list = use_state_eq(|| [0_i64; 8]);
  let mcode_list = use_state_eq(|| [(0_usize, 0_usize); 16]);


  let handle_reset = Callback::from(|_:MouseEvent| {
    window().expect("Window not set")
            .location()
            .reload().expect("Failed to reload");
  });
    
  let handle_mode_change = {
    let executing_address = executing_address.clone();
    let data_list = data_list.clone();
    let mcode_list = mcode_list.clone();
    Callback::from(move |_:Event| {
      let doc = document();
      let is_execute_mode = doc.get_element_by_id("execute-button")
                               .expect("Execute button doesn't exist")
                               .unchecked_into::<HtmlInputElement>()
                               .checked();
      if is_execute_mode {
        match get_data_list(&doc) {
          Err(msg) => {
            log_in_display(msg, &get_display(&doc));
            doc.get_element_by_id("edit-button")
               .expect("Edit button not found")
               .unchecked_into::<HtmlInputElement>()
               .click()
          },
          Ok(list) => {
            data_list.set(list);
            executing_address.set("0");
            switch_to_execute(&doc);

            mcode_list.set(get_parsed_mcode_list(&doc));
            for mcode in *mcode_list {
              print_in_display(format!("{} {}", mcode.0, mcode.1), &get_display(&doc))
            }
          }
        }
      } else {
        executing_address.set("");
        switch_to_edit(&doc);
      }
    })
  };  /* 選択されていない方のラジオボタンをクリックしてもなぜか選択が変わらない。
  - 見た目: 変わらない
  - onchange: 実行される
  - 選択: されない（したがって、複数回押すと押した回数だけ onchange が実行される）
  Callback が move を伴わないようにすると、なぜかちゃんと変わる。

  今回はひとまず、executing_address を、ラジオボタンたちを含むコンポーネントに
  prop.mode_ref として渡してしまい、その中身が
  - 空文字なら Edit mode,
  - そうでなければ Execute mode
  と判断してそれぞれ対応する方のラジオボタンに checked=true を指定した Html を返すように
  して妥協した。
  */ 

  let handle_step = {
    let executing_address = executing_address.clone();
    let register_value = register_value.clone();
    let data_list = data_list.clone();
    // let mcode_list = mcode_list.clone();
    Callback::from(move |_:MouseEvent| {
      let doc = document();

    //  mcode_list.set(get_parsed_mcode_list(&doc));
    //  for mcode in *mcode_list {
    //    print_in_display(format!("{} {}", mcode.0, mcode.1), &get_display(&doc))
    //  }

      let result = exec(*executing_address, *mcode_list, *register_value, *data_list);
      match result {
        Ok((next_address, next_register_value, new_data_list)) => {
          console::log_2(
            &JsValue::from_str(next_address),
            &JsValue::from(next_register_value)
          );
          executing_address.set(next_address);
          register_value.set(next_register_value);
          data_list.set(new_data_list);
        },
        Err(msg) => {
          log_in_display(String::from(msg), &get_display(&doc));
          doc.get_element_by_id("edit-button").expect("Edit button not found")
             .unchecked_into::<HtmlInputElement>()
             .click();
          executing_address.set("");
          switch_to_edit(&doc)
        }
      }
    })
  };
  
  html! {
    <>
      <ModeRadio
        {handle_mode_change} {handle_reset}
        is_edit_mode={*executing_address==""}
      />
      <div style="display: flex">
        <span style="flex-basis: 50%;">
          <ProgramMemory executing_address={*executing_address}/>
        </span>
        <span style="flex-basis: 50%; display: flex; flex-flow: column;">
          <span style="flex-basis: 60%">
            <DataMemory is_edit_mode={*executing_address==""} contents={*data_list}/>
          </span>
          <span style="flex-basis: 40%">
            <Display/>
          </span>
        </span>
      </div>
      <div style="display: flex; margin-top: 40px;">
        <span>
          <ProgramCounter address={*executing_address}/>
          <Register value={*register_value} is_edit_mode={*executing_address==""}/>
        </span>
        <ProcessButtons {handle_step}/>
      </div>
    </> 
  }
}

fn main() {
  yew::start_app::<App>();
}
