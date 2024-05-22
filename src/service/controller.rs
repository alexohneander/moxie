use futures::TryStreamExt;
use k8s_openapi::api:: networking::v1::Ingress;
use kube::{runtime::{watcher, WatchStreamExt}, Api, Client, ResourceExt};
use tracing::{error, info};

const INGRESS_CLASS_NAME: &'static str = "moxie";

pub struct IngressController {}

impl IngressController {
    pub fn new() -> IngressController {
        return IngressController {};
    }

    pub async fn start_controller(&mut self) {
        info!("starting controller");
        let result = self.watch_ingress_resources().await;
        if result.is_err() {
            error!("failed to start ingress controller: {:?}", result);
        }

        info!("result: {:?}", result);
    }

    async fn watch_ingress_resources(&self) -> Result<(), watcher::Error> {
        info!(INGRESS_CLASS_NAME, "watching ingress resources with: ");
        let client = Client::try_default().await.unwrap();
        let ingresses: Api<Ingress> = Api::all(client);
    
        // After changes are applied, we want to print the ingress name and reload the proxy/gateway.
        watcher(ingresses, watcher::Config::default()).applied_objects()
            .try_for_each(|i| async move {
                info!("Got Ingress: {}", i.name_any());
                Ok(())
            })
            .await?;
       Ok(())
    }
}
