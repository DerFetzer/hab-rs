#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

/// Types for errors produced by hab-rs
pub mod error;
/// Types for openHAB events and messages
pub mod event;
/// Types for item handling
pub mod item;
/// Provides rule trait and manager
pub mod rule;

/// Generated REST-API
pub use hab_rs_api_client::apis as rest_api;
/// Generated REST-API models
pub use hab_rs_api_client::models as rest_api_models;
