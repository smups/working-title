/*
  Copyright (C) 2022 Raúl Wolters
  
  This file is part of srvr.
  
  srvr is free software: you can redistribute it and/or modify it under the
  terms of the European Union Public License (EUPL), provided that you publish
  your modifications under the terms of the EUPL or another compatible license
  as specified by the EUPL v1.2 or higher.

  As the copyright holder is a citizen of the Kingdom of the Netherlands, this
  license agreement shall be governed by dutch law, as specified in clause 15
  of the EUPL v1.2.

  srvr is distributed in the hope that it will be useful, but WITHOUT ANY
  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
  A PARTICULAR PURPOSE.  See the European Union Public License for more details.
  
  You should have received a copy of the European Union Public License in a
  official language of the European Union along with srvr. If not, see
  <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12> for the full
  text of the license in any official language of the European Union.
*/

use std::{
  error::Error,
  fmt::{self, Display, Formatter}
};

use tokio::sync::{oneshot, mpsc};

/*(Note to future self)
  This request is issued by clients and follows a strict Request-Response
  pattern:
    [Client] --(requests something)--> [Receiver] --(answer)--> [Client]
  When the client sends a request, it attaches a oneshot channel to the request
  to which it will then listen until the Receiver responds.

  To be specific:
    - The [ClientRequest] struct is sent from the Client to the Receiver
    - The message sent by the Client is an [CReqMsg] enum variant
    - The message received by the Client is a result: it receives either an
      [CReqDenied] Error containing the reason why the request was denied, or
      a [CReqRsp] enum variant containing the response.
  
  This struct is used for Request-Response style communication between the client
  and either the server manager OR the world that the client is connected to
*/

#[derive(Debug)]
pub struct ClientRequest {
  msg: CReqMsg,
  conn: oneshot::Sender<Result<CReqRsp, CReqDenied>>
}

impl ClientRequest {
  pub async fn send(msg: CReqMsg, destination: mpsc::Sender<ClientRequest>)
    -> Result<CReqRsp, CReqDenied>
  {
    //(1) Create the oneshot communication channel
    let (tx, rx) = oneshot::channel();

    //(2) Create the ClientRequest and send it off
    let req = ClientRequest {msg: msg, conn: tx};
    destination.send(req).await?;

    //(3) Await the result
    rx.await?
  }

  pub fn open(self) -> (CReqMsg, oneshot::Sender<Result<CReqRsp, CReqDenied>>) {
    (self.msg, self.conn)
  }
}

#[derive(Debug)]
pub enum CReqMsg {
  
}

#[derive(Debug)]
pub enum CReqRsp {

}

#[derive(Debug)]
pub struct CReqDenied(String);
impl Display for CReqDenied {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    write!(f, "Request denied. Reason: \"{}\"", self.0)
  }
}
impl Error for CReqDenied {}

impl<T> From<mpsc::error::SendError<T>> for CReqDenied {
  fn from(err: mpsc::error::SendError<T>) -> Self {
    CReqDenied(format!("Internal Error: {err}"))
  }
}

impl From<oneshot::error::RecvError> for CReqDenied {
  fn from(err: oneshot::error::RecvError) -> Self {
    CReqDenied(format!("Internal Error: {err}"))
  }
}