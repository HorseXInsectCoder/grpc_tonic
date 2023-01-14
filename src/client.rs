use tonic::Request;
use records::recorder_client::RecorderClient;
use records::RecordRequest;

pub mod records {
    // tonic::include_proto!("records");
    include!("../proto/records.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RecorderClient::connect("http://[::1]:5050").await?;
    let request = Request::new(RecordRequest {
        user_name: "voyager".to_string(),
        user_age: 18,
    });

    let response = client.send_message(request).await?;
    println!("{:#?}", response);

    // 只返回元数据，即 headers 那些
    println!("Meta: {:#?}", response.metadata());

    // 只返回请求的数据(结构体)，不包含 headers
    println!("message: {:#?}", response.get_ref());

    // 没有结构体包裹的信息
    println!("message: {:#?}", response.get_ref().message);

    Ok(())
}