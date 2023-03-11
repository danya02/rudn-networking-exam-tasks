use trust_dns_client::client::{Client, ClientConnection, SyncClient, AsyncClient, ClientHandle};
use trust_dns_client::tcp::TcpClientStream;
use trust_dns_client::udp::{UdpClientConnection, UdpClientStream};
use std::net::{Ipv4Addr, SocketAddr};
use std::str::FromStr;
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};

#[derive(Debug)]
pub enum QueryError {
    Timeout,
    Unexpected(String),
}

pub async fn perform_query(server: SocketAddr, name: &str, query_type: RecordType) -> Result<DnsResponse, QueryError>  {
    let conn = UdpClientConnection::new(server).unwrap(); // infallible
    let client = SyncClient::new(conn);
    let (mut client, bg) = client.new_future().await.map_err(|e| QueryError::Unexpected(format!("{:?}", e)) )?;
    let bg_task = tokio::spawn(bg);
    let name = Name::from_str(name).unwrap();

    let response: DnsResponse = match client.query(name, DNSClass::IN, query_type).await {
        Ok(resp) => resp,
        Err(error) => {
            let response = match error.kind() {
                // trust_dns_client::error::ClientErrorKind::Message(_) => todo!(),
                // trust_dns_client::error::ClientErrorKind::Msg(_) => todo!(),
                // trust_dns_client::error::ClientErrorKind::DnsSec(_) => todo!(),
                // trust_dns_client::error::ClientErrorKind::Io(_) => todo!(),
                // trust_dns_client::error::ClientErrorKind::Proto(_) => todo!(),
                // trust_dns_client::error::ClientErrorKind::SendError(_) => todo!(),
                trust_dns_client::error::ClientErrorKind::Timeout => Err(QueryError::Timeout),
                _ => Err(QueryError::Unexpected(format!("{:?}", error))),
            };
            bg_task.abort();
            return response;
        }
    };
    
    // Messages are the packets sent between client and server in DNS.
    //  there are many fields to a Message, DnsResponse can be dereferenced into
    //  a Message. It's beyond the scope of these examples
    //  to explain all the details of a Message. See trust_dns_client::op::message::Message for more details.
    //  generally we will be interested in the Message::answers
    bg_task.abort();
    Ok(response)
}
