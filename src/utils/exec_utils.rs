use super::{
    parse_address_to_index,
    parse_index_to_address,
    print_in_display,
    display,
};

pub fn go_through( mcode_list: [(usize,usize); 16],
  mut exec_address: &'static str,
  mut regis_val: i64,
  mut datalist: [Option<i64>; 8]
)// -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str>
 // /* last (exec_address, regis_val, datalist) */
  -> Result<[Option<i64>; 8], (String, [Option<i64>; 8])>
{
  for i in 0..3000 {
    if i == 2999 {
      let msg = format!("line {}: {}", parse_address_to_index(exec_address), "InfiniteLoopError");
      return Err((msg, datalist))
    }

    let result = exec(exec_address, mcode_list, regis_val, datalist);
    match result {
      Err(msg) => {
        let msg = format!("line {}: {}", parse_address_to_index(exec_address), msg);
        return Err((msg, datalist)); 
      },
      Ok((next_address, next_register_value, new_data_list)) => {
        exec_address = next_address;
        regis_val = next_register_value;
        datalist = new_data_list;
      }
    }
  }
  Ok(datalist)
}

pub fn exec(
  exec_address: &str, parsed_mcode_list: [(usize,usize); 16],
  regis_val: i64, data_list: [Option<i64>; 8]
) -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
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
  exec_address: &str, regis_val: i64, data_list: [Option<i64>; 8]
) -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
  let next_index = parse_address_to_index(exec_address) + 1;
  if next_index < 16 {
    Ok((parse_index_to_address(next_index), regis_val, data_list))
  } else {
    Err("ProgramCountError")
  }
}
fn load(
  exec_address: &str, data_list: [Option<i64>; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
  if data_index < 8 { // direct_data_index
    match data_list[data_index] {
      Some(new_regis_val) => {
        let next_index = parse_address_to_index(exec_address) + 1;
        if next_index < 16 {
          Ok((parse_index_to_address(next_index), new_regis_val, data_list))
        } else {
          Err("ProgramcountError")
        }
      },
      None => Err("InvalidDataAccessError")
    }
  } else { // indirect_data_index
    match data_list[data_index - 8] {
      Some(direct_data_index) => {
        if direct_data_index < 0 {
          Err("NegativeIndexError")
        } else {
          load(exec_address, data_list, direct_data_index as usize)
        }
      },
      None => Err("InvalidDataAccessError")
    }
  }
}
fn store(
  exec_address: &str, regis_val: i64, data_list: [Option<i64>; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
  if data_index < 8 { // direct_data_index
    let mut new_data_list = data_list;
    new_data_list[data_index] = Some(regis_val);
    let next_index = parse_address_to_index(exec_address) + 1;
    if next_index < 16 {
      Ok((parse_index_to_address(next_index), regis_val, new_data_list))
    } else {
      Err("ProgramcountError")
    }
  } else { // indirect_data_index
    match data_list[data_index - 8] {
      Some(direct_data_index) => {
        if direct_data_index < 0 {
          Err("NegativeIndexError")
        } else {
          store(exec_address, regis_val, data_list, direct_data_index as usize)
        }
      },
      None => Err("InvalidDataAccessError")
    }
  }
}
fn add(
  exec_address: &str, regis_val: i64, data_list: [Option<i64>; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
  if data_index < 8 { // direct_data_index
    match data_list[data_index] {
      Some(data) => {
        let new_regis_val = regis_val + data;
        let next_index = parse_address_to_index(exec_address) + 1;
        if next_index < 16 {
          Ok((parse_index_to_address(next_index), new_regis_val, data_list))
        } else {
          Err("ProgramcountError")
        }
      },
      None => Err("InvalidDataAccessError")
    }
  } else { // indirect_data_index
    match data_list[data_index - 8] {
      Some(direct_data_index) => {
        if direct_data_index < 0 {
          Err("NegativeIndexError")
        } else {
          add(exec_address, regis_val, data_list, direct_data_index as usize)
        }
      },
      None => Err("InvalidDataAccessError")
    }
  }
}
fn sub(
  exec_address: &str, regis_val: i64, data_list: [Option<i64>; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
  if data_index < 8 { // direct_data_index
    match data_list[data_index] {
      Some(data) => {
        let new_regis_val = regis_val - data;
        let next_index = parse_address_to_index(exec_address) + 1;
        if next_index < 16 {
          Ok((parse_index_to_address(next_index), new_regis_val, data_list))
        } else {
          Err("ProgramcountError")
        }
      },
      None => Err("InvalidDataAccessError")
    }
  } else { // indirect_data_index
    match data_list[data_index - 8] {
      Some(direct_data_index) => {
        if direct_data_index < 0 {
          Err("NegativeIndexError")
        } else {
          sub(exec_address, regis_val, data_list, direct_data_index as usize)
        }
      },
      None => Err("InvalidDataAccessError")
    }
  }
}
fn mul(
  exec_address: &str, regis_val: i64, data_list: [Option<i64>; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
  if data_index < 8 { // direct_data_index
    match data_list[data_index] {
      Some(data) => {
        let new_regis_val = regis_val * data;
        let next_index = parse_address_to_index(exec_address) + 1;
        if next_index < 16 {
          Ok((parse_index_to_address(next_index), new_regis_val, data_list))
        } else {
          Err("ProgramcountError")
        }
      },
      None => Err("InvalidDataAccessError")
    }
  } else { // indirect_data_index
    match data_list[data_index - 8] {
      Some(direct_data_index) => {
        if direct_data_index < 0 {
          Err("NegativeIndexError")
        } else {
          mul(exec_address, regis_val, data_list, direct_data_index as usize)
        }
      },
      None => Err("InvalidDataAccessError")
    }
  }
}
fn div(
  exec_address: &str, regis_val: i64, data_list: [Option<i64>; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
  if data_index < 8 { // direct_data_index
    match data_list[data_index] {
      Some(divider) => {
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
      },
      None => Err("InvalidDataAccessError")
    }
  } else { // indirect_data_index
    match data_list[data_index - 8] {
      Some(direct_data_index) => {
        if direct_data_index < 0 {
          Err("NegativeIndexError")
        } else {
          div(exec_address, regis_val, data_list, direct_data_index as usize)
        }
      },
      None => Err("InvalidDataAccessError")
    }
  }
}
fn jump(
  regis_val: i64, data_list: [Option<i64>; 8],
  /* arg */next_index: usize
) -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
  Ok((parse_index_to_address(next_index), regis_val, data_list))
}
fn jumpzero(
  exec_address: &str, regis_val: i64, data_list: [Option<i64>; 8],
  /* arg */next_index: usize
) -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
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
  exec_address: &str, regis_val: i64, data_list: [Option<i64>; 8],
  /* arg */next_index: usize
) -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
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
  exec_address: &str, regis_val: i64, data_list: [Option<i64>; 8],
  /* arg */next_index: usize
) -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
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
  exec_address: &str, regis_val: i64, data_list: [Option<i64>; 8]
) -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
  print_in_display(regis_val.to_string(), &display());
  let next_index = parse_address_to_index(exec_address) + 1;
  if next_index < 16 {
    Ok((parse_index_to_address(next_index), regis_val, data_list))
  } else {
    Err("ProgramcountError")
  }
}
fn aprint(
  exec_address: &str, regis_val: i64, data_list: [Option<i64>; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
  if data_index < 8 { // direct_data_index
    match data_list[data_index] {
      Some(data) => {
        print_in_display(data.to_string(), &display());
        let next_index = parse_address_to_index(exec_address) + 1;
        if next_index < 16 {
          Ok((parse_index_to_address(next_index), regis_val, data_list))
        } else {
          Err("ProgramcountError")
        }
      },
      None => Err("InvalidDataAccessError")
    }
  } else { // indirect_data_index
    match data_list[data_index - 8] {
      Some(direct_data_index) => {
        if direct_data_index < 0 {
          Err("NegativeIndexError")
        } else {
          aprint(exec_address, regis_val, data_list, direct_data_index as usize)
        }
      },
      None => Err("InvalidDataAccessError")
    }
  }
}
fn clear(
  exec_address: &str, regis_val: i64, data_list: [Option<i64>; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
  if data_index < 8 { // direct_data_index
    let mut new_data_list = data_list;
    new_data_list[data_index] = Some(0);
    let next_index = parse_address_to_index(exec_address) + 1;
    if next_index < 16 {
      Ok((parse_index_to_address(next_index), regis_val, new_data_list))
    } else {
      Err("ProgramcountError")
    }
  } else { // indirect_data_index
    match data_list[data_index - 8] {
      Some(direct_data_index) => {
        if direct_data_index < 0 {
          Err("NegativeIndexError")
        } else {
          clear(exec_address, regis_val, data_list, direct_data_index as usize)
        }
      },
      None => Err("InvalidDataAccessError")
    }
  }
}
fn inc(
  exec_address: &str, regis_val: i64, data_list: [Option<i64>; 8],
  /* arg */data_index: usize
) -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
  if data_index < 8 { // direct_data_index
    let mut new_data_list = data_list;
    match new_data_list[data_index] {
      Some(old_data) => {
        new_data_list[data_index] = Some(old_data + 1);
        let next_index = parse_address_to_index(exec_address) + 1;
        if next_index < 16 {
          Ok((parse_index_to_address(next_index), regis_val, new_data_list))
        } else {
          Err("ProgramcountError")
        }
      },
      None => Err("InvalidDataAccessError")
    }
  } else { // indirect_data_index
    match data_list[data_index - 8] {
      Some(direct_data_index) => {
        if direct_data_index < 0 {
          Err("NegativeIndexError")
        } else {
          inc(exec_address, regis_val, data_list, direct_data_index as usize)
        }
      },
      None => Err("InvalidDataAccessError")
    }
  }
}
fn halt() -> Result<(&'static str, i64, [Option<i64>; 8]), &'static str> {
  Err("halted")
}