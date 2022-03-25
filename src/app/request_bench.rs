use comfort::comfort_client::ComfortClient;
use comfort::TemperatureRequest;
use std::time::Instant;
use tonic::{metadata::MetadataValue, transport::Channel, Request};

pub mod comfort {
    tonic::include_proto!("comfort");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;

    let token = MetadataValue::from_str("Bearer 123ABC")?;

    let mut client = ComfortClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let start = Instant::now();

    let total_iterations = 100000;

    for _ in 0..total_iterations {
        let request = tonic::Request::new(TemperatureRequest { temperature: 17.5 });

        let _ = client.set_desired_temperature(request).await?;
    }

    let duration_per_iteration = start.elapsed().as_micros() / total_iterations;

    println!(
        "performed {} iterations in {} microseconds per rpc",
        total_iterations, duration_per_iteration
    );

    Ok(())
}
