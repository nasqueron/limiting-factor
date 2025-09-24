/*  -------------------------------------------------------------
    Limiting Factor :: axum :: App
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    Project:        Nasqueron
    License:        BSD-2-Clause
    -------------------------------------------------------------    */

use axum::Router;
use log::{error, info};
use tokio::net::TcpListener;

/*  -------------------------------------------------------------
    Re-exports from core
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

pub use limiting_factor_core::app::ServerConfig;

/*  -------------------------------------------------------------
    Main application server
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

pub struct App {
    pub config: ServerConfig,

    router: Router,
}

impl Default for App {
    fn default() -> Self {
        Self {
            config: ServerConfig::from_env(),
            router: Router::new(),
        }
    }
}

impl App {
    pub fn new (config: ServerConfig, router: Router) -> Self {
        Self {
            config,
            router,
        }
    }

    pub fn from_config(config: ServerConfig) -> Self {
        Self {
            config,
            router: Router::new(),
        }
    }

    pub fn with_config(mut self, config: ServerConfig) -> Self {
        self.config = config;

        self
    }

    fn resolve_router(&self) -> Router {
        if self.config.mount_point == "/" {
            return self.router.clone();
        }

        Router::new()
            .nest(&*self.config.mount_point, self.router.clone())
    }

    pub async fn run(self) -> bool {
        let app = self.resolve_router();
        let socket_address = self.config.get_socket_address();

        info!("🚀 Starting server");
        match TcpListener::bind(&socket_address).await {
            Ok(listener) => {
                info!("Listening to {}", socket_address);
                axum::serve(listener, app).await.unwrap();

                true
            }

            Err(error) => {
                error!("{}", error);

                false
            }
        }
    }
}
