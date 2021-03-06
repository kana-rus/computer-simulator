use std::{
  str::SplitAsciiWhitespace,
};
use wasm_bindgen::JsCast;
use web_sys::{
    window,
    Document,
    HtmlInputElement,
    HtmlTextAreaElement,
    HtmlCollection
};

pub mod exec_utils;


pub fn document() -> Document {
  window().expect("Window is not set")
          .document()
          .expect("Document is not set")
}
pub fn disable_elms_by_class(class_name: &str, doc: &Document) {
  let target_elms = doc.get_elements_by_class_name(class_name);
  for i in 0..target_elms.length() {
    target_elms.item(i).expect("no such element")
               .set_attribute("disabled", "disabled")
               .expect("failed to set disabled attribute");
  }
}
pub fn enable_elms_by_class(class_name: &str, doc: &Document) {
  let target_elms = doc.get_elements_by_class_name(class_name);
  for i in 0..target_elms.length() {
    target_elms.item(i).expect("No such element")
               .remove_attribute("disabled")
               .expect("Failed to remove attribute \"disabled\"")
  }
}
pub fn parse_address_to_index(address: &str) -> usize {
  match address {
    "0" => 0,
    "1" => 1,
    "2" => 2,
    "3" => 3,
    "4" => 4,
    "5" => 5,
    "6" => 6,
    "7" => 7,
    "8" => 8,
    "9" => 9,
    "a" => 10,
    "b" => 11,
    "c" => 12,
    "d" => 13,
    "e" => 14,
    "f" => 15,
    // other => address.parse().expect(&format!("parse error at {}", other)) // 0〜9
    _other => 10000000
  }
}
pub fn parse_index_to_address(index: usize) -> &'static str {
  assert!(index < 16);
  match index {
    0  => "0",
    1  => "1",
    2  => "2",
    3  => "3",
    4  => "4",
    5  => "5",
    6  => "6",
    7  => "7",
    8  => "8",
    9  => "9",
    10 => "a",
    11 => "b",
    12 => "c",
    13 => "d",
    14 => "e",
    15 => "f",
    _other => "impossible pattern"
  }
}
pub fn get_data_list(doc: &Document) -> Result<[Option<i64>; 8], String> {
  let data_memory = doc.get_elements_by_class_name("data-memory");
  let data_list = {
    let mut dl = [None; 8];
    for i in 0..8 {
      let data_str = data_memory.item(i as u32).expect("NO such data-memory item")
                                .unchecked_into::<HtmlInputElement>()
                                .value();
      if !data_str.is_empty() {
        match data_str.parse::<i64>() {
          Ok(data) => dl[i] = Some(data),
          Err(err) => return Err(err.to_string())
        }
      }
    }
    dl
  };
  Ok(data_list)
}
pub fn get_parsed_mcode_list(doc: &Document) -> [(usize, usize); 16] {
  let mcode_areas = doc.get_elements_by_class_name("machine-code-area");
  let mut parsed_mcode_list = [(0, 0); 16];
  for i in 0..16 {
    let mcode = mcode_areas.item(i as u32).expect("No such machine-code-area item")
                           .unchecked_into::<HtmlInputElement>()
                           .value();
    // certainly to be form of '¥ ¥' beacuse this has passed assembling
    let parsed_mcode = mcode.split_at(1);
    parsed_mcode_list[i] = (parse_address_to_index(parsed_mcode.0), parse_address_to_index(parsed_mcode.1.trim_start()));
  }
  parsed_mcode_list
}
pub fn display_of(doc: &Document) -> HtmlTextAreaElement {
  doc.get_element_by_id("display")
     .expect("Display doesn't exist")
     .unchecked_into::<HtmlTextAreaElement>()
}
fn display() -> HtmlTextAreaElement {
  document().get_element_by_id("display")
            .expect("display not found")
            .unchecked_into::<HtmlTextAreaElement>()
}
pub fn clear_exec_items(doc: &Document) {
  let mcode_ares = doc.get_elements_by_class_name("machine-code-area");
  for i in 0..16 {
    let mcode_area = mcode_ares.item(i).expect("No such machine-code-area")
                               .unchecked_into::<HtmlInputElement>();
    mcode_area.set_value("");
    mcode_area.remove_attribute("id").expect("Failed to remove attribute \"id\"");
  }
}
fn is_direct_data_address(arg: &str) -> bool {
  match arg {
    "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7" => true,
    _other                          => false
  }
}
fn is_indirect_data_address(arg: &str) -> bool {
  match arg {
    "*0"|"*1"|"*2"|"*3"|"*4"|"*5"|"*6"|"*7" => true,
    _other                                  => false
  }
}
fn is_program_address(arg: &str) -> bool {
  match arg {
    "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"
    |"8"|"9"|"a"|"b"|"c"|"d"|"e"|"f" => true,
    _other                           => false
  }
}
fn assemble_with_program_address(
  mut args: SplitAsciiWhitespace, operation_id: &str
) -> Result<String, String> {
  if let Some(arg) = args.next() {
    if is_program_address(arg) && args.next()==None {
      Ok(format!("{} {}", operation_id, arg))
    } else {
      Err(String::from("Arg format is wrong. 1 arg (data address) is required"))
    }
  } else {
    Err(String::from("1 arg (data address) is required"))
  }
}
fn assemble_with_data_address(
  mut args: SplitAsciiWhitespace, operation_id: &str
) -> Result<String, String> {
  if let Some(arg) = args.next() {
    if is_direct_data_address(arg) {
      Ok(format!("{} {}", operation_id, arg))
    } else if is_indirect_data_address(arg) {
      let machine_code_arg = arg.split_at(1).1.parse::<u8>().unwrap() + 8;
      Ok(format!("{} {}", operation_id, machine_code_arg))
    } else {
      Err(String::from("Arg format is wrong. 1 arg (data address) is required"))
    }
  } else {
    Err(String::from("1 arg (data address) is required"))
  }
}
fn assemble_with_none(
  mut args: SplitAsciiWhitespace, operation_id: &str
) -> Result<String, String> {
  if args.next() == None {
    Ok(format!("{} 0", operation_id))
  } else {
    Err(String::from("This operation can't have any args"))
  }
}
fn assemble(assembly_code: String) -> Result<String, String> {
  let mut assembly_code = assembly_code.split_ascii_whitespace();
  let operation = assembly_code.next();
  if operation == None {
      Ok(String::from("0 0")) // deal with as case where arg == ""
  } else {
    match operation.unwrap() {
      "" | "nop" => { assemble_with_none(assembly_code, "0") },
      "print"    => { assemble_with_none(assembly_code, "b") },
      "halt"     => { assemble_with_none(assembly_code, "f") },

      "load"   => { assemble_with_data_address(assembly_code, "1") },
      "store"  => { assemble_with_data_address(assembly_code, "2") },
      "add"    => { assemble_with_data_address(assembly_code, "3") },
      "sub"    => { assemble_with_data_address(assembly_code, "4") },
      "mul"    => { assemble_with_data_address(assembly_code, "5") },
      "div"    => { assemble_with_data_address(assembly_code, "6") },
      "aprint" => { assemble_with_data_address(assembly_code, "c") },
      "clear"  => { assemble_with_data_address(assembly_code, "d") },
      "inc"    => { assemble_with_data_address(assembly_code, "e") },

      "jump"     => { assemble_with_program_address(assembly_code, "7") },
      "jumpzero" => { assemble_with_program_address(assembly_code, "8") },
      "jumpgr"   => { assemble_with_program_address(assembly_code, "9") },
      "jumpge"   => { assemble_with_program_address(assembly_code, "a") },

      other => { Err(format!("operation \"{}\" is invalid", other)) }
    }
  }
}
pub fn assembe_program(doc: &Document) -> bool {
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
        print_log_in_display(format!("line {}: AssembleError", address), &display);
        print_log_in_display(msg, &display);
        assembled_successfully = false;
        break;
      },
      Ok(machine_code) => {
        print_machine_code(machine_code, address, &machine_code_area_list);
      }
    }
  }
  if assembled_successfully {
    print_log_in_display(String::from("assembled successfully"), &display);
  }
  assembled_successfully
}

pub fn print_in_display(msg: String, display: &HtmlTextAreaElement) {
  display.set_value(format!("{}{}\n", display.value(), msg).as_str());
  display.set_scroll_top(display.scroll_height());
}
pub fn print_log_in_display(msg: String, display: &HtmlTextAreaElement) {
  print_in_display(format!("[log] {}", msg), display)
}
fn print_machine_code(mcode: String, address: u32, mcodearea_list: &HtmlCollection) {
  mcodearea_list.item(address)
                .expect("No such machine_code_area")
                .unchecked_into::<HtmlInputElement>()
                .set_value(mcode.as_str())
}

pub fn switch_to_edit(doc: &Document) {
  clear_exec_items(doc);
  disable_elms_by_class("process-buttons", doc);
  enable_elms_by_class("assembly-code-area", doc);
  enable_elms_by_class("data-memory", doc);
}
pub fn switch_to_execute(doc: &Document) {
  if assembe_program(doc) {
    enable_elms_by_class("process-buttons", doc);
    disable_elms_by_class("assembly-code-area", doc);
    disable_elms_by_class("data-memory", doc);
  }
}