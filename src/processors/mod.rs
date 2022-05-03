use reqwest::blocking::{Client, Response};
use serde_json::Value;

use crate::cli::MainCliArgs;

mod caches;
mod keys;
mod users;

pub enum ProcessorErrorStatus {
    NotMyCommand, // Processor gives control to next processor
    Error,        // Processor processed command but some errors ocurs. Stop a application.
}

pub trait Processor {
    fn process_args(
        &self,
        args: &MainCliArgs,
        client: &mut Client,
    ) -> Result<(), ProcessorErrorStatus>;
}

pub fn init_processors() -> Vec<Box<dyn Processor>> {
    vec![
        // Users
        Box::new(users::UserCreateProcessor {}),
        Box::new(users::UserViewProcessor {}),
        Box::new(users::UserChangeProcessor {}),
        // Keys
        Box::new(keys::KeysCreateProcessor {}),
        Box::new(keys::KeysViewProcessor {}),
        Box::new(keys::KeysRevokeProcessor {}),
        // Caches
        Box::new(caches::CacheCreateProcessor {}),
        Box::new(caches::CacheFindProcessor {}),
        Box::new(caches::CacheViewProcessor {}),
        Box::new(caches::CacheChangeProcessor {}),
        Box::new(caches::CacheDeleteProcessor {}),
        // MUST BE ALWAYS LAST
        Box::new(NotProcessedCommand {}),
    ]
}

pub struct NotProcessedCommand;
impl Processor for NotProcessedCommand {
    fn process_args(&self, _: &MainCliArgs, _: &mut Client) -> Result<(), ProcessorErrorStatus> {
        println!("No processor avaiable for command. Report a bug.");
        return Err(ProcessorErrorStatus::Error);
    }
}

pub fn print_json_value(json_value: &Value) {
    if json_value.is_object() {
        let json_obj = json_value.as_object().unwrap();
        let keys = json_obj.keys();

        for k in keys {
            println!("\t{}: {}", k, json_obj[k]);
        }
    } else {
        println!("\t{}", json_value)
    }
}

pub fn print_json_value_wo_error(json_value: &Value) {
    if json_value.is_object() {
        let json_obj = json_value.as_object().unwrap();
        let keys = json_obj.keys();

        for k in keys {
            if k.eq("error") {
                continue;
            }
            println!("\t{}: {}", k, json_obj[k]);
        }
    } else {
        println!("\t{}", json_value)
    }
}

pub fn check_server_error(json_value: &Value) -> Result<(), ProcessorErrorStatus> {
    if let Some(v) = json_value.get("error") {
        if v.is_boolean() && v.as_bool().unwrap() {
            // Error
            println!("Server returned a error!");
            print_json_value_wo_error(&json_value);

            return Err(ProcessorErrorStatus::Error);
        }
    }

    return Ok(());
}

pub fn basic_server_response_check(
    resp_res: Result<Response, reqwest::Error>,
    args: &MainCliArgs,
) -> Result<Value, ProcessorErrorStatus> {
    if let Err(e) = resp_res {
        println!("Error: {:?}", e);
        return Err(ProcessorErrorStatus::Error);
    }

    let response = resp_res.unwrap();

    let json_value_res = response.json::<Value>();
    if let Err(e) = json_value_res {
        println!("Server returned invalid JSON: {:?}", e);
        return Err(ProcessorErrorStatus::Error);
    }

    let json_value = json_value_res.unwrap();

    if args.verbose {
        println!("Server response: \n{:#?}", json_value);
    }

    check_server_error(&json_value)?;

    return Ok(json_value);
}
