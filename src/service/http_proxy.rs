use async_trait::async_trait;
use pingora_http::{RequestHeader, ResponseHeader};
use std::sync::Arc;
use tracing::info;

use pingora_core::server::Server;
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::Result;
use pingora_load_balancing::{selection::RoundRobin, LoadBalancer};
use pingora_proxy::{http_proxy_service, ProxyHttp, Session};

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self
            .0
            .select(b"", 256) // hash doesn't matter
            .unwrap();

        info!("upstream peer is: {:?}", upstream);

        let peer = Box::new(HttpPeer::new(upstream, true, "one.one.one.one".to_string()));
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        upstream_request
            .insert_header("Host", "one.one.one.one")
            .unwrap();
        Ok(())
    }

    async fn response_filter(
        &self,
        _session: &mut Session,
        upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()>
    where
        Self::CTX: Send + Sync,
    {
        // replace existing header if any
        upstream_response
            .insert_header("Server", "moxie")
            .unwrap();

        Ok(())
    }
        
}

pub struct HttpProxy {
    proxy_server: Server
}

impl HttpProxy {
    pub fn new() -> Self {
        let mut my_server = Server::new(None).unwrap();
        my_server.bootstrap();

        let upstreams = LoadBalancer::try_from_iter(["1.1.1.1:443", "1.0.0.1:443"]).unwrap();

        let mut lb = http_proxy_service(&my_server.configuration, LB(Arc::new(upstreams)));
        lb.add_tcp("0.0.0.0:6188");

        my_server.add_service(lb);

        HttpProxy { proxy_server: my_server}
    }

    pub fn serve(self) {
        self.proxy_server.run_forever();
    }
}
