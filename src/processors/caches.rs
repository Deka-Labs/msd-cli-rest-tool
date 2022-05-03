use reqwest::blocking::Client;

use crate::{
    cli::*,
    processors::{print_json_value, print_json_value_wo_error},
};

use super::{basic_server_response_check, Processor, ProcessorErrorStatus};

pub struct CacheCreateProcessor;
impl Processor for CacheCreateProcessor {
    fn process_args(
        &self,
        args: &crate::cli::MainCliArgs,
        client: &mut Client,
    ) -> Result<(), ProcessorErrorStatus> {
        if let Command::Cache(cache_args) = &args.command {
            if let CacheCommand::Create(cmd_args) = &cache_args.command {
                let api_path = args.get_api_base();
                let req_url = format!("{}/cache/", api_path);

                let res = client
                    .post(req_url)
                    .json(&json!(
                        {
                            "lat": cmd_args.lat,
                            "long": cmd_args.long,

                            "descrip": cmd_args.descrip,
                            "hint": cmd_args.hint,
                        }
                    ))
                    .send();

                let json = basic_server_response_check(res, args)?;

                println!("Cache created:");
                print_json_value_wo_error(&json);

                return Ok(());
            }
        }

        Err(ProcessorErrorStatus::NotMyCommand)
    }
}

pub struct CacheFindProcessor;
impl Processor for CacheFindProcessor {
    fn process_args(
        &self,
        args: &crate::cli::MainCliArgs,
        client: &mut Client,
    ) -> Result<(), ProcessorErrorStatus> {
        if let Command::Cache(cache_args) = &args.command {
            if let CacheCommand::Find(cmd_args) = &cache_args.command {
                let api_path = args.get_api_base();
                let req_url = format!("{}/cache/", api_path);

                let res = client
                    .get(req_url)
                    .query(&CacheFindArgsServer::new(cmd_args))
                    .send();

                let json = basic_server_response_check(res, args)?;

                let caches_array = json
                    .get("caches")
                    .expect("Server error: Field caches not found")
                    .as_array()
                    .expect("Server error: Field caches is not array");

                println!("Cache find result:");
                if caches_array.is_empty() {
                    println!("\tNo caches");
                } else {
                    for c in caches_array {
                        println!("Cache {}", c.get("id").unwrap().as_u64().unwrap());
                        print_json_value(&c);
                    }
                }

                return Ok(());
            }
        }

        Err(ProcessorErrorStatus::NotMyCommand)
    }
}

pub struct CacheViewProcessor;
impl Processor for CacheViewProcessor {
    fn process_args(
        &self,
        args: &crate::cli::MainCliArgs,
        client: &mut Client,
    ) -> Result<(), ProcessorErrorStatus> {
        if let Command::Cache(cache_args) = &args.command {
            if let CacheCommand::View(cmd_args) = &cache_args.command {
                let api_path = args.get_api_base();
                let req_url = format!("{}/cache/{}", api_path, cmd_args.id);

                let res = client.get(req_url).send();

                let json = basic_server_response_check(res, args)?;

                let cache = json
                    .get("caches")
                    .expect("Server error: Field caches not found");

                println!("Cache view:");
                print_json_value(cache);

                return Ok(());
            }
        }

        Err(ProcessorErrorStatus::NotMyCommand)
    }
}

pub struct CacheChangeProcessor;
impl Processor for CacheChangeProcessor {
    fn process_args(
        &self,
        args: &crate::cli::MainCliArgs,
        client: &mut Client,
    ) -> Result<(), ProcessorErrorStatus> {
        if let Command::Cache(cache_args) = &args.command {
            if let CacheCommand::Change(cmd_args) = &cache_args.command {
                let api_path = args.get_api_base();
                let req_url = format!("{}/cache/{}", api_path, cmd_args.id);

                let res = client.put(req_url).json(&cmd_args).send();

                let _ = basic_server_response_check(res, args)?;
                println!("Cache edited");
                return Ok(());
            }
        }

        Err(ProcessorErrorStatus::NotMyCommand)
    }
}

pub struct CacheDeleteProcessor;
impl Processor for CacheDeleteProcessor {
    fn process_args(
        &self,
        args: &crate::cli::MainCliArgs,
        client: &mut Client,
    ) -> Result<(), ProcessorErrorStatus> {
        if let Command::Cache(cache_args) = &args.command {
            if let CacheCommand::Delete(cmd_args) = &cache_args.command {
                let api_path = args.get_api_base();
                let req_url = format!("{}/cache/{}", api_path, cmd_args.id);

                let res = client.delete(req_url).send();

                let _ = basic_server_response_check(res, args)?;
                println!("Cache deleted");
                return Ok(());
            }
        }

        Err(ProcessorErrorStatus::NotMyCommand)
    }
}
