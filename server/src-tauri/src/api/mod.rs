pub mod mpv;

use salvo::prelude::*;
use salvo::server::ServerHandle;
use salvo::Router;
use std::sync::{Arc, Mutex};

use crate::settings;

pub async fn init() {
    let listener = TcpListener::new(format!("0.0.0.0:{}", settings::load_settings().port))
        .bind()
        .await;
    tracing::debug!("Server started on: {}", listener.local_addr().unwrap());

    let server = Server::new(listener);
    let handle = server.handle();

    let router = Router::new()
        .hoop(AddShutdownHandle::new(Arc::new(Mutex::new(handle))))
        .push(Router::with_path("ping").get(ping))
        .push(Router::with_path("stop").get(stop))
        .push(Router::with_path("restart").get(restart))
        .push(Router::with_path("mpv/status").get(mpv::handle_status_api))
        .push(Router::with_path("mpv/volume-up").get(mpv::handle_volume_up_api))
        .push(Router::with_path("mpv/volume-down").get(mpv::handle_volume_down_api))
        .push(Router::with_path("mpv/subtitle").get(mpv::handle_set_subtitle))
        .push(Router::with_path("mpv/pause").get(mpv::handle_pause_api));

    server.serve(router).await;
}

#[handler]
async fn ping(_req: &mut Request, res: &mut Response, _depot: &mut Depot) {
    res.render(Text::Plain("pong"));
}

#[handler]
async fn stop(_req: &mut Request, _res: &mut Response, depot: &mut Depot) {
    tracing::debug!("Stopping server...");
    let server_handle = depot
        .get::<Arc<Mutex<ServerHandle>>>("shutdown_handle")
        .expect("State not found");
    server_handle.lock().unwrap().stop_forcible();
}

#[handler]
async fn restart(_req: &mut Request, _res: &mut Response, depot: &mut Depot) {
    let server_handle = depot
        .get::<Arc<Mutex<ServerHandle>>>("shutdown_handle")
        .expect("State not found")
        .clone();

    // Start the server again
    tokio::spawn(async move {
        tracing::debug!("Stopping server for restart...");
        server_handle.lock().unwrap().stop_forcible();

        // Wait for the server to shut down
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        tracing::debug!("Restarting server...");
        init().await;
    });
}

#[derive(Clone)]
struct AddShutdownHandle {
    server_handle: Arc<Mutex<ServerHandle>>,
}

impl AddShutdownHandle {
    fn new(server_handle: Arc<Mutex<ServerHandle>>) -> Self {
        AddShutdownHandle { server_handle }
    }
}

#[async_trait]
impl Handler for AddShutdownHandle {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        depot.insert("shutdown_handle", self.server_handle.clone());
        ctrl.call_next(req, depot, res).await;
    }
}

// pub mod mpv;
//
// use std::{sync::Arc, thread::sleep};
//
// use crate::settings::load_settings;
// use actix_web::{dev::ServerHandle, get, middleware, web::Data, App, HttpResponse, HttpServer};
// use parking_lot::Mutex;
// use tauri::AppHandle;
//
// #[get("/hello")]
// async fn hello(app_state: Data<TauriAppState>) -> &'static str {
//     "Hello world!"
// }
//
// #[get("/stop")]
// async fn stop(app_state: Data<TauriAppState>) -> HttpResponse {
//     app_state.stop(false).await;
//     HttpResponse::NoContent().finish()
// }
//
// #[get("/restart")]
// async fn restart(app_state: Data<TauriAppState>) -> HttpResponse {
//     app_state.stop(false).await;
//     sleep(std::time::Duration::from_secs(3));
//     println!("Restarting server...");
//     app_state.start_with_retry(5).await;
//     HttpResponse::NoContent().finish()
// }
//
// pub async fn init(app: AppHandle) -> std::io::Result<()> {
//     let app_state = Data::new(TauriAppState {
//         app: Arc::new(Mutex::new(app)),
//         server_handle: Arc::new(Mutex::new(None)),
//     });
//
//     app_state.start().await?;
//     Ok(())
// }
//
// #[derive(Clone)]
// struct TauriAppState {
//     app: Arc<Mutex<AppHandle>>,
//     server_handle: Arc<Mutex<Option<ServerHandle>>>,
// }
//
// impl TauriAppState {
//     /// Sets the server handle to stop.
//     pub(crate) fn register(&self, handle: ServerHandle) {
//         let mut server_handle = self.server_handle.lock();
//         *server_handle = Some(handle);
//         println!("Server handle registered");
//     }
//
//     /// Sends stop signal through contained server handle.
//     pub(crate) async fn stop(&self, graceful: bool) {
//         if let Some(server) = self.server_handle.lock().take() {
//             println!("Stopping server");
//             server.stop(false).await;
//             println!("Server stopped");
//         } else {
//             println!("Server not running");
//         }
//
//         sleep(std::time::Duration::from_secs(1));
//     }
//
//     /// Starts the server by loading settings and initializing the server.
//     pub(crate) async fn start(&self) -> std::io::Result<()> {
//         let settings = load_settings();
//
//         println!("Starting server: {}", settings.port);
//
//         let app_state = self.clone();
//
//         let server = HttpServer::new(move || {
//             App::new()
//                 .app_data(Data::new(app_state.clone()))
//                 .wrap(middleware::Logger::default())
//                 .service(hello)
//                 .service(mpv::handle_pause_api)
//                 .service(mpv::handle_volume_up_api)
//                 .service(mpv::handle_volume_down_api)
//                 .service(mpv::handle_status_api)
//                 .service(stop)
//                 .service(restart)
//         })
//         .bind(("0.0.0.0", settings.port))?
//         .shutdown_timeout(0)
//         .run();
//
//         let handle = server.handle();
//         self.register(handle);
//
//         tokio::spawn(async move {
//             server.await.unwrap();
//         });
//         Ok(())
//     }
//
//     /// Starts the server with retry logic in case of address in use error.
//     pub(crate) async fn start_with_retry(&self, retries: usize) {
//         let mut attempts = 0;
//         loop {
//             match self.start().await {
//                 Ok(_) => return,
//                 Err(err) => {
//                     if attempts >= retries {
//                         println!(
//                             "Failed to start server after {} attempts: {:?}",
//                             attempts, err
//                         );
//                         return;
//                     }
//                     if err.kind() == std::io::ErrorKind::AddrInUse {
//                         // Address in use
//                         attempts += 1;
//                         println!("Address in use, retrying... attempt {}", attempts);
//                         sleep(std::time::Duration::from_secs(1));
//                     } else {
//                         println!("Unexpected error: {:?}", err);
//                         return;
//                     }
//                 }
//             }
//         }
//     }
// }
