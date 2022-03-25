use comfort::comfort_client::ComfortClient;
use comfort::TemperatureStreamRequest;
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

    let request = tonic::Request::new(TemperatureStreamRequest { period: 500});

    let mut stream = client
        .get_cabin_temperature(request)
        .await?
        .into_inner();

    while let Some(temperature) = stream.message().await? {
        println!("Temperature Update = {:?}", temperature);
    }

    Ok(())
}
