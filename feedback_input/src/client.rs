use crate::{AUTH, SERVER_ADDR};
use anyhow::Result;
use comm::{communication_client::CommunicationClient, MsgRequest};

mod comm {
    tonic::include_proto!("comm");
}

pub(crate) async fn send_msg(msg: &str)
    -> Result<()> {

    let mut client = CommunicationClient::connect(
        SERVER_ADDR.clone()
    ).await?;

    let request = tonic::Request::new(
        MsgRequest {
            auth: AUTH.clone(),
            msg: msg.to_string(),
        }
    );

    client.send_msg(request).await?;

    Ok(())
}
