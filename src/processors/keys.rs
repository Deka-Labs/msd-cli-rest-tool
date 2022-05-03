use reqwest::blocking::Client;

use crate::{cli::*, processors::basic_server_response_check};

use super::{Processor, ProcessorErrorStatus};

pub struct KeysCreateProcessor;
impl Processor for KeysCreateProcessor {
    fn process_args(
        &self,
        args: &crate::cli::MainCliArgs,
        client: &mut Client,
    ) -> Result<(), ProcessorErrorStatus> {
        if let Command::User(user_args) = &args.command {
            if let UserCommand::Keys(key_args) = &user_args.command {
                if let UserKeysCommand::Generate(cmd_args) = &key_args.command {
                    let api_path = args.get_api_base();
                    let req_url = format!("{}/user/{}/keys", api_path, cmd_args.id);

                    let res = client.post(req_url).send();

                    let json = basic_server_response_check(res, args)?;

                    println!("Key created");
                    println!(
                        "Use your new API key: {}",
                        json.get("key").unwrap().as_str().unwrap()
                    );
                    return Ok(());
                }
            }
        }

        Err(ProcessorErrorStatus::NotMyCommand)
    }
}

pub struct KeysViewProcessor;
impl Processor for KeysViewProcessor {
    fn process_args(
        &self,
        args: &crate::cli::MainCliArgs,
        client: &mut Client,
    ) -> Result<(), ProcessorErrorStatus> {
        if let Command::User(user_args) = &args.command {
            if let UserCommand::Keys(key_args) = &user_args.command {
                if let UserKeysCommand::View(cmd_args) = &key_args.command {
                    let api_path = args.get_api_base();
                    let mut req_url = format!("{}/user/{}/keys", api_path, cmd_args.id);
                    let mut one_view = false;

                    if let Some(nmb) = cmd_args.nmb {
                        req_url = format!("{}/{}", req_url, nmb);
                        one_view = true;
                    }

                    let res = client.get(req_url).send();
                    let json = basic_server_response_check(res, args)?;

                    println!("Key found");
                    if one_view {
                        println!("API key: {}", json.get("key").unwrap().as_str().unwrap());
                    } else {
                        let keys_value = json
                            .get("keys")
                            .expect("Server error: keys not found")
                            .as_array()
                            .expect("Server error: Keys is not array");

                        for json_key in keys_value {
                            println!(
                                "Key #{}: {}",
                                json_key.get("nmb").unwrap().as_u64().unwrap(),
                                json_key.get("api_key").unwrap().as_str().unwrap(),
                            )
                        }
                    }

                    return Ok(());
                }
            }
        }

        Err(ProcessorErrorStatus::NotMyCommand)
    }
}

pub struct KeysRevokeProcessor;
impl Processor for KeysRevokeProcessor {
    fn process_args(
        &self,
        args: &crate::cli::MainCliArgs,
        client: &mut Client,
    ) -> Result<(), ProcessorErrorStatus> {
        if let Command::User(user_args) = &args.command {
            if let UserCommand::Keys(key_args) = &user_args.command {
                if let UserKeysCommand::Revoke(cmd_args) = &key_args.command {
                    let api_path = args.get_api_base();
                    let req_url =
                        format!("{}/user/{}/keys/{}", api_path, cmd_args.id, cmd_args.nmb);

                    let res = client.delete(req_url).send();
                    let _ = basic_server_response_check(res, args)?;

                    println!("Key deleted");

                    return Ok(());
                }
            }
        }

        Err(ProcessorErrorStatus::NotMyCommand)
    }
}
