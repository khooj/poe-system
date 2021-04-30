#![recursion_limit="256"]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod domain;
mod pob;
mod actors;
pub mod ports;
pub mod implementations;
pub mod schema;