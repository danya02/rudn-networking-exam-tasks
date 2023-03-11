use trust_dns_client::rr::RecordType;

mod querying;

#[tokio::main]
async fn main() {
    println!("{}", querying::perform_query("1.1.1.1:53".parse().unwrap(), ".", RecordType::NS).await.unwrap().into_inner());
}