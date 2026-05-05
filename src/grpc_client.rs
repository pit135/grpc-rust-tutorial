pub mod services {
    tonic::include_proto!("services");
}

use services::{
    payment_service_client::PaymentServiceClient, PaymentRequest,
    transaction_service_client::TransactionServiceClient, TransactionRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Eksekusi Payment Service
    let mut payment_client = PaymentServiceClient::connect("http://[::1]:50051").await?;
    let pay_request = tonic::Request::new(PaymentRequest {
        user_id: "user_123".to_string(),
        amount: 100.0,
    });
    let pay_response = payment_client.process_payment(pay_request).await?;
    println!("RESPONSE={:?}", pay_response.into_inner());

    // 2. Eksekusi Transaction Service (Streaming)
    let mut trans_client = TransactionServiceClient::connect("http://[::1]:50051").await?;
    let trans_request = tonic::Request::new(TransactionRequest {
        user_id: "user_123".to_string(),
    });

    let mut stream = trans_client.get_transaction_history(trans_request).await?.into_inner();

    println!("--- Streaming Transaction History ---");
    while let Some(transaction) = stream.message().await? {
        println!("Transaction: {:?}", transaction);
    }

    Ok(())
}