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
        .push(Router::with_path("mpv/set-volume").get(mpv::handle_set_volume_api))
        .push(Router::with_path("mpv/volume-up").get(mpv::handle_volume_up_api))
        .push(Router::with_path("mpv/volume-down").get(mpv::handle_volume_down_api))
        .push(Router::with_path("mpv/skip-backward").get(mpv::handle_skip_backward_api))
        .push(Router::with_path("mpv/skip-forward").get(mpv::handle_skip_forward_api))
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
