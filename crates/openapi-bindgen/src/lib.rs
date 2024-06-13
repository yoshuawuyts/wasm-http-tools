//! Bidirectional bindings between OpenAPI (Swagger) and WIT
//!
//! # Examples
//!
//! ```text
//! // tbi
//! ```

#![forbid(unsafe_code, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, future_incompatible, unreachable_pub)]

// use std::str::FromStr;

// use openapiv3::OpenAPI;

// struct Parser {
//     open_api: OpenAPI,
// }

// impl Parser {
//     /// Convert the parsed OpenAPI definition to WIT.
//     pub fn generate_wit(&self) -> Wit {
//         todo!();
//     }
// }

// impl FromStr for Parser {
//     type Err = std::io::Error;

//     /// Take a string of JSON and attempt to parse it as an OpenAPI definition
//     fn from_str(data: &str) -> Result<Self, Self::Err> {
//         let open_api: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");
//         Ok(Self { open_api })
//     }
// }

// /// The generated Wit document
// struct Wit {}
