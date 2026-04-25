use std::fs;

use crate::interpreter::Value;

pub enum BuiltIn{
    ReadFile,
    WriteFile,
}

pub fn call_builtin(name: &str, args: &[Value]) -> Result<Option<Value>, String>{
        match name {
            "read_file" => {
                expect_arity("read_file", args.len(), 1)?;
                let path = expect_str_arg("read_file", args, 0)?;
                Ok(Some(read_file(path.as_str())?))
            },
            // "write_file" => {
                // TODO: Would start doing in a minute
            // },
            _ => Ok(None)
        }
}



// got is length of args recieved and expected should be self-explanatory...
fn expect_arity(func_name: &str, got: usize, expected: usize ) -> Result<(), String>{
    if got == expected{
        Ok(())
    } else {
        Err(format!("{func_name} expects {expected} argument(s), got {got}"))
    }
}

fn expect_str_arg(func_name: &str, args: &[Value], arg_idx: usize) -> Result<String, String>{
    match args.get(arg_idx){
        Some(Value::StringVal(inner)) => Ok(inner.clone()),
        Some(_) => Err(format!("{func_name} argument {} must be String", arg_idx + 1)),
        _ => Err(format!("{func_name} argument {} must be String", arg_idx + 1))
    }
}

fn read_file(file: &str) -> Result<Value, String> {
    let contents = fs::read_to_string(file)
    .map_err(|e| format!("Failed to read file {file}: {e}"))?;

    Ok(Value::StringVal(contents))
}

