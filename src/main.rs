use tonic::{transport::Server, Request, Response, Status};

pub mod tutorial {
    tonic::include_proto!("tutorial");
}

use tutorial::greeter_server::{Greeter, GreeterServer};
use tutorial::{HelloReply, HelloRequest};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {

        let name = request.into_inner().name;

        let reply = HelloReply {
            message: format!("Hello {}", name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr = "127.0.0.1:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("Server running on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}