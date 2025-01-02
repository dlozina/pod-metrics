use anyhow::Result;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams},
    Client as KubeClient,
};
use crate::metrics::types::MetricsResponse;

#[derive(Clone)]
pub struct Client {
    inner: KubeClient,
    namespace: String,
}

impl Client {
    pub async fn new(namespace: String) -> Result<Self> {
        let client = KubeClient::try_default().await?;
        Ok(Self {
            inner: client,
            namespace,
        })
    }

    pub async fn list_pods(&self) -> Result<Vec<Pod>> {
        let pods: Api<Pod> = Api::namespaced(self.inner.clone(), &self.namespace);
        let pod_list = pods.list(&ListParams::default()).await?;
        Ok(pod_list.items)
    }

    pub async fn get_metrics(&self) -> Result<MetricsResponse> {
        let list_params = kube::api::ListParams::default().timeout(10);
        let metrics = self.inner
            .request::<MetricsResponse>(
                kube::api::Request::new(format!("/apis/metrics.k8s.io/v1beta1/namespaces/{}/pods", self.namespace))
                    .list(&list_params)?
            )
            .await?;
        Ok(metrics)
    }
}