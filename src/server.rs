use std::net::SocketAddr;
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use records::recorder_server::{Recorder, RecorderServer};
use records::{RecordRequest, RecordResponse};


// 定义一个mod，然后引入proto文件
pub mod records {
    // tonic::include_proto!("records");
    // 修改路径之后需使用include!宏来导入.rs文件
    include!("../proto/records.rs");
}

#[derive(Debug, Default)]
pub struct RecorderService {}

#[tonic::async_trait]
impl Recorder for RecorderService {
    // 参数那些，是相对固定的写法
    async fn send_message(
        &self,
        request: Request<RecordRequest>,
    ) -> Result<Response<RecordResponse>, Status> {
        println!("request: {:#?}", request);
        let req = request.into_inner();
        let response = RecordResponse {
            successful: true,
            message: format!("User {} is {} years old", req.user_name, req.user_age).into()
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:5050".parse()?;

    let recorder = RecorderService::default();
    println!("Recorder listening on {}", addr);

    Server::builder()
        .add_service(RecorderServer::new(recorder))
        .serve(addr)
        .await?;

    println!("hello server");
    Ok(())
}