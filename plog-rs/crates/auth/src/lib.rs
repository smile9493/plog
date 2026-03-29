//! Plog CMS Auth Module
//! 
//! 认证和授权模块

pub mod jwt;
pub mod password;
pub mod middleware;
pub mod service;

pub use jwt::{JwtService, Claims};
pub use password::PasswordHasher;
pub use middleware::{AuthState, AuthUser, AuthError, auth_middleware, require_role};
pub use service::{AuthService, LoginRequest, LoginResponse, UserInfo};
