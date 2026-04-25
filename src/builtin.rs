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
            "write_file" => {
                expect_arity("write_file", args.len(), 2)?;
                let path = expect_str_arg("write_file", args, 0)?;
                let contents = expect_str_arg("write_file", args, 1)?;
                Ok(Some(write_file(path.as_str(), contents.as_str())?))
            },
            "fetch" => {
                expect_arity("fetch", args.len(), 1)?;
                let url = expect_str_arg("fetch", args, 0)?;
                Ok(Some(fetch(url.as_str())?))
            }
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

fn write_file(path: &str, contents: &str) -> Result<Value, String> {
    fs::write(path, contents).map_err(|e| format!("Failed to write {path}: {e}"))?;

    Ok(Value::Nil)
}

fn fetch(url: &str) -> Result<Value, String> {
    let mut response = ureq::get(url)
    .header("User-Agent", "tinylang/0.67")
    .call()
    .map_err(|e| format!("Failed to send request: {e}"))?;

    let body = response
    .body_mut()
    .read_to_string()
    .map_err(|e| format!("Failed to read response body: {e}"))?;

    Ok(Value::StringVal(body))
}