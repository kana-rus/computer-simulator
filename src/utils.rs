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
pub fn get_data_list(doc: &Document) -> Result<[i64; 8], String> {
  let data_memory = doc.get_elements_by_class_name("data-memory");
  let data_list = {
    let mut dl = [0_i64; 8];
    for i in 0..8 {
      let data_str = data_memory.item(i as u32).expect("NO such data-memory item")
                                .unchecked_into::<HtmlInputElement>()
                                .value();
      if !data_str.is_empty() {
        match data_str.parse::<i64>() {
          Ok(data) => dl[i] = data,
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
/*
pub fn get_data_at(
  address: &str, data_memory: HtmlCollection
) -> Result<i64, ParseIntError> {
  let index = address.parse::<u32>().unwrap();
  data_memory
    .item(index).expect("No such data")
    .unchecked_into::<HtmlInputElement>()
    .value()
    .parse::<i64>()
}
pub fn get_program_counter(doc: &Document) -> HtmlInputElement {
  doc.get_element_by_id("program-counter")
     .expect("ProgramCounter doesn't exist")
     .unchecked_into::<HtmlInputElement>()
}
*/
pub fn get_register(doc: &Document) -> HtmlInputElement {
  doc.get_element_by_id("register")
     .expect("Register doesn't exist")
     .unchecked_into::<HtmlInputElement>()
}
pub fn get_display(doc: &Document) -> HtmlTextAreaElement {
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
  //get_program_counter(doc).set_value("");
  get_register(doc).set_value("");

  let mcode_ares = doc.get_elements_by_class_name("machine-code-area");
  for i in 0..16 {
    let mcode_area = mcode_ares.item(i).expect("No such machine-code-area")
                               .unchecked_into::<HtmlInputElement>();
    mcode_area.set_value("");
    mcode_area.remove_attribute("id").expect("Failed to remove attribute \"id\"");
  }
}
fn is_data_address(arg: &str) -> bool {
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
fn assert_arg_by( validate_func: fn(&str) -> bool,
  mut args: SplitAsciiWhitespace, operation_id: &str, err_msg: &str
) -> Result<String, String> {
  if let Some(arg) = args.next() {
    if validate_func(arg) && args.next()==None {
      Ok(format!("{} {}", operation_id, arg))
    } else {
      Err(format!("Arg format is wrong. {}", err_msg))
    }
  } else {
    Err(String::from(err_msg))
  }
}
pub fn assert_arg_data_address(
  args: SplitAsciiWhitespace, operation_id: &str
) -> Result<String, String> {
  fn validate_func(arg: &str) -> bool {
    is_data_address(arg) || is_indirect_data_address(arg)
  }
  assert_arg_by(
    validate_func, args, operation_id, "1 arg (data address) is required"
  )
}
pub fn assert_arg_program_address(
  args: SplitAsciiWhitespace, operation_id: &str
) -> Result<String, String> {
  assert_arg_by(
    is_program_address, args, operation_id, "1 arg (program address) is required"
  )
}
pub fn assert_arg_none(
  mut args: SplitAsciiWhitespace, operation_id: &str
) -> Result<String, String> {
  if args.next() == None {
    Ok(format!("{} 0", operation_id))
  } else {
    Err(String::from("This operation can't have any args"))
  }
}
pub fn assemble(code: String) -> Result<String, String> {
  let mut code = code.split_ascii_whitespace();
  let operation = code.next();
  if operation == None {
      // deal with as case where arg == ""
      Ok(String::from("0 0"))
  } else {
    match operation.unwrap() {
      // no args
      "" | "nop" => { assert_arg_none(code, "0") },
      "print"    => { assert_arg_none(code, "b") },
      "halt"     => { assert_arg_none(code, "f") },

      // arg: data_address
      "load"   => { assert_arg_data_address(code, "1") },
      "store"  => { assert_arg_data_address(code, "2") },
      "add"    => { assert_arg_data_address(code, "3") },
      "sub"    => { assert_arg_data_address(code, "4") },
      "mul"    => { assert_arg_data_address(code, "5") },
      "div"    => { assert_arg_data_address(code, "6") },
      "aprint" => { assert_arg_data_address(code, "c") },
      "clear"  => { assert_arg_data_address(code, "d") },
      "inc"    => { assert_arg_data_address(code, "e") },

      // arg: program_address
      "jump"     => { assert_arg_program_address(code, "7") },
      "jumpzero" => { assert_arg_program_address(code, "8") },
      "jumpgr"   => { assert_arg_program_address(code, "9") },
      "jumpge"   => { assert_arg_program_address(code, "a") },

      other => { Err(format!("operation \"{}\" is invalid", other)) }
    }
  }
}
pub fn print_in_display(msg: String, display: &HtmlTextAreaElement) {
  display.set_value(format!("{}{}\n", display.value(), msg).as_str());
  display.set_scroll_top(display.scroll_height());
}
pub fn log_in_display(msg: String, display: &HtmlTextAreaElement) {
  print_in_display(format!("[log] {}", msg), display)
}
pub fn print_machine_code(mcode: String, address: u32, mcodearea_list: &HtmlCollection) {
  mcodearea_list.item(address)
                .expect("No such machine_code_area")
                .unchecked_into::<HtmlInputElement>()
                .set_value(mcode.as_str())
}

pub fn exec(
  exec_address: &str, parsed_mcode_list: [(usize,usize); 16],
  regis_val: i64, data_list: [i64; 8]
) -> Result<(&'static str, i64, [i64; 8]), &'static str> {
    /*
     Ok(next_exec_address, new_regis_val, new_data_list),
     Err(error_message)
    */
  let (operation_id, arg) = parsed_mcode_list[parse_address_to_index(exec_address)];
  match operation_id {
    0  => { nop(     exec_address, regis_val, data_list     ) }
    1  => { load(    exec_address,            data_list, arg) }
    2  => { store(   exec_address, regis_val, data_list, arg) }
    3  => { add(     exec_address, regis_val, data_list, arg) }
    4  => { sub(     exec_address, regis_val, data_list, arg) }
    5  => { mul(     exec_address, regis_val, data_list, arg) }
    6  => { div(     exec_address, regis_val, data_list, arg) }
    7  => { jump(                  regis_val, data_list, arg) }
    8  => { jumpzero(exec_address, regis_val, data_list, arg) }
    9  => { jumpgr(  exec_address, regis_val, data_list, arg) }
    10 => { jumpge(  exec_address, regis_val, data_list, arg) }
    11 => { print(   exec_address, regis_val, data_list     ) }
    12 => { aprint(  exec_address, regis_val, data_list, arg) }
    13 => { clear(   exec_address, regis_val, data_list, arg) }
    14 => { inc(     exec_address, regis_val, data_list, arg) }
    15 => { halt() }
    _other => Err("OperationIdError")
  }
}

fn nop(
  exec_address: &str, regis_val: i64, data_list: [i64; 8]
) -> Result<(&'static str, i64, [i64; 8]), &'static str> {
  let next_index = parse_address_to_index(exec_address) + 1;
  if next_index < 16 {
    Ok((parse_index_to_address(next_index), regis_val, data_list))
  } else {
    Err("ProgramCountError")
  }
}
fn load(
  exec_address: &str, data_list: [i64; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [i64; 8]), &'static str> {
  let new_regis_val = data_list[data_index];
  let next_index = parse_address_to_index(exec_address) + 1;
  if next_index < 16 {
    Ok((parse_index_to_address(next_index), new_regis_val, data_list))
  } else {
    Err("ProgramcountError")
  }
}
fn store(
  exec_address: &str, regis_val: i64, data_list: [i64; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [i64; 8]), &'static str> {
  let mut new_data_list = data_list;
  new_data_list[data_index] = regis_val;
  let next_index = parse_address_to_index(exec_address) + 1;
  if next_index < 16 {
    Ok((parse_index_to_address(next_index), regis_val, new_data_list))
  } else {
    Err("ProgramcountError")
  }
}
fn add(
  exec_address: &str, regis_val: i64, data_list: [i64; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [i64; 8]), &'static str> {
  let new_regis_val = regis_val + data_list[data_index];
  let next_index = parse_address_to_index(exec_address) + 1;
  if next_index < 16 {
    Ok((parse_index_to_address(next_index), new_regis_val, data_list))
  } else {
    Err("ProgramcountError")
  }
}
fn sub(
  exec_address: &str, regis_val: i64, data_list: [i64; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [i64; 8]), &'static str> {
  let new_regis_val = regis_val - data_list[data_index];
  let next_index = parse_address_to_index(exec_address) + 1;
  if next_index < 16 {
    Ok((parse_index_to_address(next_index), new_regis_val, data_list))
  } else {
    Err("ProgramcountError")
  }
}
fn mul(
  exec_address: &str, regis_val: i64, data_list: [i64; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [i64; 8]), &'static str> {
  let new_regis_val = regis_val * data_list[data_index];
  let next_index = parse_address_to_index(exec_address) + 1;
  if next_index < 16 {
    Ok((parse_index_to_address(next_index), new_regis_val, data_list))
  } else {
    Err("ProgramcountError")
  }
}
fn div(
  exec_address: &str, regis_val: i64, data_list: [i64; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [i64; 8]), &'static str> {
  let divider = data_list[data_index];
  if divider == 0 {
    return Err("ZeroDividingError")
  }
  let new_regis_val = regis_val / divider;
  let next_index = parse_address_to_index(exec_address) + 1;
  if next_index < 16 {
    Ok((parse_index_to_address(next_index), new_regis_val, data_list))
  } else {
    Err("ProgramcountError")
  }
}
fn jump(
  regis_val: i64, data_list: [i64; 8],
  /* arg */next_index: usize
) -> Result<(&'static str, i64, [i64; 8]), &'static str> {
  Ok((parse_index_to_address(next_index), regis_val, data_list))
}
fn jumpzero(
  exec_address: &str, regis_val: i64, data_list: [i64; 8],
  /* arg */next_index: usize
) -> Result<(&'static str, i64, [i64; 8]), &'static str> {
  if regis_val == 0 {
    Ok((parse_index_to_address(next_index), regis_val, data_list))
  } else {
    let next_index = parse_address_to_index(exec_address) + 1;
    if next_index < 16 {
      Ok((parse_index_to_address(next_index), regis_val, data_list))
    } else {
      Err("ProgramcountError")
    }
  }
}
fn jumpgr(
  exec_address: &str, regis_val: i64, data_list: [i64; 8],
  /* arg */next_index: usize
) -> Result<(&'static str, i64, [i64; 8]), &'static str> {
  if regis_val > 0 {
    Ok((parse_index_to_address(next_index), regis_val, data_list))
  } else {
    let next_index = parse_address_to_index(exec_address) + 1;
    if next_index < 16 {
      Ok((parse_index_to_address(next_index), regis_val, data_list))
    } else {
      Err("ProgramcountError")
    }
  }
}
fn jumpge(
  exec_address: &str, regis_val: i64, data_list: [i64; 8],
  /* arg */next_index: usize
) -> Result<(&'static str, i64, [i64; 8]), &'static str> {
  if regis_val >= 0 {
    Ok((parse_index_to_address(next_index), regis_val, data_list))
  } else {
    let next_index = parse_address_to_index(exec_address) + 1;
    if next_index < 16 {
      Ok((parse_index_to_address(next_index), regis_val, data_list))
    } else {
      Err("ProgramcountError")
    }
  }
}
fn print(
  exec_address: &str, regis_val: i64, data_list: [i64; 8]
) -> Result<(&'static str, i64, [i64; 8]), &'static str> {
  print_in_display(regis_val.to_string(), &display());
  let next_index = parse_address_to_index(exec_address) + 1;
  if next_index < 16 {
    Ok((parse_index_to_address(next_index), regis_val, data_list))
  } else {
    Err("ProgramcountError")
  }
}
fn aprint(
  exec_address: &str, regis_val: i64, data_list: [i64; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [i64; 8]), &'static str> {
  let print_content = data_list[data_index];
  print_in_display(print_content.to_string(), &display());
  let next_index = parse_address_to_index(exec_address) + 1;
  if next_index < 16 {
    Ok((parse_index_to_address(next_index), regis_val, data_list))
  } else {
    Err("ProgramcountError")
  }
}
fn clear(
  exec_address: &str, regis_val: i64, data_list: [i64; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [i64; 8]), &'static str> {
  let mut new_data_list = data_list;
  new_data_list[data_index] = 0;
  let next_index = parse_address_to_index(exec_address) + 1;
  if next_index < 16 {
    Ok((parse_index_to_address(next_index), regis_val, new_data_list))
  } else {
    Err("ProgramcountError")
  }
}
fn inc(
  exec_address: &str, regis_val: i64, data_list: [i64; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [i64; 8]), &'static str> {
  let mut new_data_list = data_list;
  new_data_list[data_index] += 1;
  let next_index = parse_address_to_index(exec_address) + 1;
  if next_index < 16 {
    Ok((parse_index_to_address(next_index), regis_val, new_data_list))
  } else {
    Err("ProgramcountError")
  }
}
fn halt() -> Result<(&'static str, i64, [i64; 8]), &'static str> {
  Err("Halted")
}