use reqwest::blocking::Client;
use serde_json::json;

use crate::{
    cli::*,
    processors::{basic_server_response_check, print_json_value_wo_error},
};

use super::{Processor, ProcessorErrorStatus};

pub struct UserCreateProcessor;
impl Processor for UserCreateProcessor {
    fn process_args(
        &self,
        args: &crate::cli::MainCliArgs,
        client: &mut Client,
    ) -> Result<(), ProcessorErrorStatus> {
        if let Command::User(user_args) = &args.command {
            if let UserCommand::Create(cmd_args) = &user_args.command {
                let api_path = args.get_api_base();
                let req_url = format!("{}/user/", api_path);

                let res = client
                    .post(req_url)
                    .json(&json!(
                        {
                            "login": cmd_args.name,
                            "email": cmd_args.email,
                            "password": cmd_args.password,
                        }
                    ))
                    .send();

                let json = basic_server_response_check(res, args)?;

                println!("User created");
                println!(
                    "Use your default API key: {}",
                    json.get("api_key").unwrap().as_str().unwrap()
                );
                return Ok(());
            }
        }

        Err(ProcessorErrorStatus::NotMyCommand)
    }
}

pub struct UserViewProcessor;
impl Processor for UserViewProcessor {
    fn process_args(
        &self,
        args: &crate::cli::MainCliArgs,
        client: &mut Client,
    ) -> Result<(), ProcessorErrorStatus> {
        if let Command::User(user_args) = &args.command {
            if let UserCommand::View(cmd_args) = &user_args.command {
                let api_path = args.get_api_base();
                let req_url = format!("{}/user/{}", api_path, cmd_args.id);

                let res = client.get(req_url).send();

                let json = basic_server_response_check(res, args)?;

                println!("User founded!");
                print_json_value_wo_error(&json);
                return Ok(());
            }
        }

        Err(ProcessorErrorStatus::NotMyCommand)
    }
}

pub struct UserChangeProcessor;
impl Processor for UserChangeProcessor {
    fn process_args(
        &self,
        args: &crate::cli::MainCliArgs,
        client: &mut Client,
    ) -> Result<(), ProcessorErrorStatus> {
        if let Command::User(user_args) = &args.command {
            if let UserCommand::Change(cmd_args) = &user_args.command {
                let api_path = args.get_api_base();
                let req_url = format!("{}/user/{}", api_path, cmd_args.id);

                let res = client.put(req_url).json(cmd_args).send();

                let _ = basic_server_response_check(res, args)?;

                println!("User changed");
                return Ok(());
            }
        }

        Err(ProcessorErrorStatus::NotMyCommand)
    }
}
