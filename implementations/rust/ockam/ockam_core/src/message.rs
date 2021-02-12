use crate::lib::{Box, Vec};
use crate::{Result, Worker};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

// TODO: swap this for a non-heaped data structure
pub type Encoded = Vec<u8>;

/// A user defined message that can be serialised and deserialised
pub trait Message: Serialize + DeserializeOwned + Send + 'static {
    fn encode(&self) -> Result<Encoded> {
        Ok(bincode::serialize(self)?)
    }

    #[allow(clippy::ptr_arg)]
    fn decode(e: &Encoded) -> Result<Self> {
        Ok(bincode::deserialize(e)?)
    }
}

#[async_trait]
pub trait Handler<M: Message>: Worker {
    async fn handle(&mut self, _msg: M, _ctx: &mut Self::Context);
}

// TODO: see comment in Cargo.toml about this dependency
impl From<bincode::Error> for crate::Error {
    fn from(_: bincode::Error) -> Self {
        Self::new(1, "bincode")
    }
}
