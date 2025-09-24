/*  -------------------------------------------------------------
    Limiting Factor :: Core :: App
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    Project:        Nasqueron
    License:        BSD-2-Clause
    -------------------------------------------------------------    */

//! # Create a web server application

use std::default::Default;
use std::env;

/*  -------------------------------------------------------------
    Base server configuration
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

/// Base configuration for a server
pub struct ServerConfig {
    /// The address to attach the listener to
    pub address: String,

    /// The port to serve
    pub port: u16,

    /// The mount point of every request URL
    /// "/" is a good default to let proxy sort this
    pub mount_point: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            address: "0.0.0.0".to_string(),
            port: 8080,
            mount_point: "/".to_string(),
        }
    }
}

impl ServerConfig {
    pub fn from_env() -> Self {
        Self::from_env_or(ServerConfig::default())
    }

    pub fn from_env_or(default_config: ServerConfig) -> Self {
        let address = env::var("APP_ADDRESS")
            .unwrap_or_else(|_| default_config.address);

        let port = read_port_from_environment_or(default_config.port);

        let mount_point = env::var("APP_MOUNT_POINT")
            .unwrap_or_else(|_| default_config.mount_point);

        Self {
            address,
            port,
            mount_point,
        }
    }

    pub fn get_socket_address(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

/*  -------------------------------------------------------------
    Helper methods
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

fn read_port_from_environment_or(default_port: u16) -> u16 {
    match env::var("APP_PORT") {
        Ok(port) => port.parse().unwrap_or(default_port),

        Err(_) => default_port,
    }
}
