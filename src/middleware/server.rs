use comfort::comfort_server::{Comfort, ComfortServer};
use comfort::{TemperatureReply, TemperatureRequest, TemperatureStreamRequest};
use rand::thread_rng;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tokio_stream::wrappers::ReceiverStream;
use tonic::metadata::MetadataValue;
use tonic::{transport::Server, Request, Response, Status};

pub mod comfort {
    tonic::include_proto!("comfort"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct SClassComfort {}

#[tonic::async_trait]
impl Comfort for SClassComfort {
    async fn set_desired_temperature(
        &self,
        request: Request<TemperatureRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<TemperatureReply>, Status> {
        // Return an instance of type HelloReply

        let temperature = request.into_inner().temperature;

        // println!("Setting desired interior temperature to {:?}", temperature);

        let reply = comfort::TemperatureReply { temperature };

        Ok(Response::new(reply))
    }

    type GetCabinTemperatureStream = ReceiverStream<Result<TemperatureReply, Status>>;

    async fn get_cabin_temperature(
        &self,
        request: Request<TemperatureStreamRequest>,
    ) -> Result<Response<Self::GetCabinTemperatureStream>, Status> {
        let (tx, rx) = mpsc::channel(4);

        let period = request.into_inner().period;

        tokio::spawn(async move {
            loop {
                // In real implementation would fetch temperature from in vehicle twin.

                let temperature_reply = comfort::TemperatureReply { temperature: 17.5 };
                tx.send(Ok(temperature_reply)).await.unwrap();

                sleep(Duration::from_millis(period)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token = MetadataValue::from_str("Bearer 123ABC").unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let server = SClassComfort::default();

    let svc = ComfortServer::with_interceptor(server, check_auth);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
