use std::net::SocketAddr;
use std::str::FromStr;
use trust_dns_client::client::{Client, ClientHandle, SyncClient};
use trust_dns_client::op::{DnsResponse, ResponseCode};
use trust_dns_client::rr::{DNSClass, Name, RecordType};
use trust_dns_client::udp::UdpClientConnection;

#[derive(Debug)]
pub enum QueryError {
    Timeout,
    Unexpected(String),
}

pub async fn perform_query(
    server: SocketAddr,
    name: &str,
    query_type: RecordType,
) -> Result<DnsResponse, QueryError> {
    let conn = UdpClientConnection::new(server).unwrap(); // infallible
    let client = SyncClient::new(conn);
    let (mut client, bg) = client
        .new_future()
        .await
        .map_err(|e| QueryError::Unexpected(format!("{e:?}")))?;
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
                _ => Err(QueryError::Unexpected(format!("{error:?}"))),
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

/// Check whether a NS server is recursive
/// by querying it for several domains across multiple zones,
/// and seeing if it gives answers for all of them.
pub async fn is_recursive_server(server: SocketAddr) -> bool {
    let names = vec![
        "example.com",
        "example.net",
        "example.org",
        "iana.org",
        "cctld.ru",
        "bit.ly",
        "nih.gov",
        "github.io",
    ];
    let successes_needed = 4; // this or more responses with answer sections are needed

    let mut tasks = vec![];

    for name in names {
        tasks.push(tokio::spawn(perform_query(server, name, RecordType::A)));
    }
    let mut successes = 0;
    for task in tasks {
        let resp = task.await; // Result representing joining the task
        if resp.is_err() {
            continue;
        }
        let resp = resp.unwrap(); // Result representing querying.
        if resp.is_err() {
            continue;
        }
        let resp = resp.unwrap(); // the actual response
        if resp.response_code() == ResponseCode::NoError && resp.contains_answer() {
            successes += 1;
        }
        if successes >= successes_needed {
            return true;
        }
    }

    false
}
