#![deny(unused_must_use)]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate rocket;

use std::sync::Arc;

use dotenv::dotenv;
use rocket::{Build, Rocket};

use routes::active_routes;
use utils::cors::CORS;

use crate::cache::manager::{create_cache_manager, RedisCacheManager};
use crate::routes::error_catchers;
use crate::utils::http_client::{setup_http_client, HttpClient};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

#[doc(hidden)]
#[macro_use]
pub mod macros;

#[doc(hidden)]
mod cache;
mod common;
#[doc(hidden)]
mod config;

#[doc(hidden)]
mod monitoring;
mod providers;

/// Collection of all endpoints all endpoints
pub mod routes;
#[doc(hidden)]
mod utils;

#[cfg(test)]
mod tests;
