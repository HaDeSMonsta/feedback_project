use crate::{AUTH, SERVER_ADDR};
use anyhow::{Context, Result};
use comm::{communication_client::CommunicationClient, MsgRequest};

mod comm {
    tonic::include_proto!("comm");
}

pub(crate) async fn send_msg(msg: &str) -> Result<()> {
    let request = tonic::Request::new(
        MsgRequest {
            auth: AUTH.clone(),
            msg: msg.to_string(),
        }
    );

    CommunicationClient::connect(
        SERVER_ADDR.clone()
    ).await
        .with_context(|| {
            format!("Unable to create client to connect to server with addr {}", *SERVER_ADDR)
        })?
        .send_msg(request)
        .await
        .with_context(|| {
            format!("Unable to send msg to server with addr {}", *SERVER_ADDR)
        })?;

    Ok(())
}
