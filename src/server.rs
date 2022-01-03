use tonic::Code;
use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default, Debug)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    #[tracing::instrument(skip(self, request))]
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", some_logic(&request.get_ref().name).await?),
        };

        tracing::info!("Done");

        Ok(Response::new(reply))
    }
}

async fn some_logic(name: &str) -> Result<String, Status> {
    tracing::info!("run some logic");
    match name {
        "foo" => {
            tracing::error!("Failed some_logic");
            Err(Status::new(Code::InvalidArgument, "who is foo"))
        }
        _ => Ok(name.to_string()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    tracing::info!("GreeterServer listening on {}", addr);

    Server::builder()
        .trace_fn(|_| tracing::info_span!("gRPC server"))
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
