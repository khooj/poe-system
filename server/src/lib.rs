#![recursion_limit="256"]
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

mod domain;
pub mod actors;
pub mod ports;
pub mod implementations;
pub mod schema;
pub mod application;