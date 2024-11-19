// #![allow(deprecated)]
// #![allow(dead_code)]

// use axum::async_trait;
// use config::{AsyncSource, Config, ConfigError, Environment, File, Map};
// use std::{env, error::Error};

// // use crate::utils::tikv_db_grpc;

// pub fn get_config() -> Config {
//     let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
//     let env_settings = Environment::new();

//     Config::builder()
//         // Start off by merging in the "default" configuration file
//         .add_source(File::with_name("config/config"))
//         // Add in the current environment file
//         // Default to 'development' env
//         // Note that this file is _optional_
//         .add_source(File::with_name(&format!("config/config-{}", run_mode)).required(false))
//         // Add in a local configuration file
//         // This file shouldn't be checked in to git
//         .add_source(env_settings.prefix("app").separator("_"))
//         // You may also programmatically change settings
//         .build()
//         .unwrap()

//     // Now that we're done, let's access our configuration

//     // You can deserialize (and thus freeze) the entire configuration as
// }

// pub async fn get_async_config() -> Result<Config, Box<dyn Error>> {
//     let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
//     let env_settings = Environment::new();
//     // uncomment UserConfig if you are running in local environment

//     let s: Config = Config::builder()
//         .add_source(File::with_name("config/config"))
//         .add_source(File::with_name(&format!("config/config-{}", run_mode)).required(false))
//         .add_async_source(ServiceConfig { env_key: run_mode.to_string() })
//         .add_async_source(LookupConfig { env_key: run_mode.to_string() })
//         // This file shouldn't be checked in to git
//         .add_source(env_settings.prefix("app").separator("_"))
//         .build()
//         .await?;
//     Ok(s)
// }

// #[derive(Debug)]
// struct ServiceConfig {
//     env_key: String,
// }

// // #[async_trait]
// // // impl AsyncSource for ServiceConfig {
// // //     async fn collect(&self) -> Result<Map<String, config::Value>, ConfigError> {
// // //         let current_env = &self.env_key;
// // //         let project_name = match current_env.as_str() {
// // //             "production" => "DCS".to_string(),
// // //             "nextalpha" => "NXA_DCS".to_string(),
// // //             &_ => "DCS".to_string(),
// // //         };
// // //         println!("{:?}", project_name);
// // //         //based on ENV load config from tikv
// // //         let config_doc_key = format!("DCS-API-CONFIG-{}", current_env);
// // //         println!("{:?}", config_doc_key);
// // //         // Get CONGIF doc from tikv.
// // //         let response = tikv_db_grpc::get_single_record(config_doc_key, Some(project_name)).await;
// // //         match response {
// // //             Ok(result) => {
// // //                 let config_json: Map<String, config::Value> = serde_json::from_value(result).unwrap();
// // //                 Ok(config_json)
// // //             }
// // //             Err(error) => Err(ConfigError::NotFound(error)),
// // //         }
// // //     }
// // }

// #[derive(Debug)]
// struct LookupConfig {
//     env_key: String,
// }

// // #[async_trait]
// // impl AsyncSource for LookupConfig {
// //     async fn collect(&self) -> Result<Map<String, config::Value>, ConfigError> {
// //         let current_env = &self.env_key;
// //         let project_name = match current_env.as_str() {
// //             "production" => "DCS".to_string(),
// //             "nextalpha" => "NXA_DCS".to_string(),
// //             &_ => "DCS".to_string(),
// //         };
// //         // Get LOOKUP doc from tikv.
// //         println!("{:?}", project_name);
// //         let response = tikv_db_grpc::get_single_record("LOOKUP".to_string(), Some(project_name)).await;
// //         match response {
// //             Ok(result) => {
// //                 let config_json: Map<String, config::Value> = serde_json::from_value(result).unwrap();
// //                 Ok(config_json)
// //             }
// //             Err(error) => Err(ConfigError::NotFound(error)),
// //         }
// //     }
// // }
