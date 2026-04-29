//! Plog CMS Settings Module

pub mod entity;
pub mod repository;
pub mod service;

pub use entity::Model as Setting;
pub use repository::SettingsRepository;
pub use service::SettingsService;
