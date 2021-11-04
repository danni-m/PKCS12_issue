use k8s_openapi::api::apps::v1::StatefulSet;
use kube::{
    api::{Api},
    Client,
};
use kube::core::params::ListParams;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,kube=debug");
    let client = Client::try_default().await?;
    let sts: Api<StatefulSet> = Api::default_namespaced(client.clone());
    let objects = sts.list(&ListParams::default()).await.unwrap();
    println!("sts: {:?}", objects.items);
    Ok(())
}