use moxie::{
    config::load_configuration,
    service::{controller::IngressController, http_proxy::HttpProxy},
    telemetry::init_logger,
};

#[tokio::main]
async fn main() {
    // Initialization
    init_logger();
    load_configuration();

    // Start the IngressController asynchronously
    let ingress_handle = tokio::spawn(async {
        let mut ingress_controller = IngressController::new();
        ingress_controller.start_controller().await;
    });

    // Start the HttpProxy in a blocking thread
    let proxy_handle = tokio::task::spawn_blocking(move || {
        let mut http_proxy = HttpProxy::new();
        http_proxy.serve(); // Assuming serve is a blocking function
    });

    // Await both tasks
    let _ = tokio::try_join!(ingress_handle, proxy_handle);
}
