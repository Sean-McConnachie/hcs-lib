use crate::data;

#[cfg(feature = "client")]
pub mod client {
    use super::data;
    /// Specifies the protocol used by the client to communicate with the server.
    /// This trait specifies the points at which a the runtime will be passed data, from a connection,
    /// and where the runtime can return data to the connection.
    #[async_trait::async_trait]
    pub trait HCSProtocol<T>
    where
        T: data::Data,
    {
        /// `greet` is the entry point of any connection between the client and server.
        /// The client sends a `Greet` payload.
        async fn greet(&self) -> data::Greeting;

        /// The runtime that implements the HCSProtocol must be able to distinguish what the data_uid
        /// is. For the TCP runtime, this is done by reading the first 16 bytes of a payload.
        /// `receive_payload` is where the client receives the data sent by the server in `send_payload`.
        async fn receive_payload(&self, data_uid: u16, payload: T);

        /// Here, the payload (i.e. serialize `T`) is sent with the data_uid. This would then be received
        /// by the server in `receive_payload`.
        async fn send_payload(&self) -> T;
    }
}

#[cfg(feature = "server")]
pub mod server {
    use super::data;
    /// Specifies the protocol used by the client to communicate with the server.
    /// This trait specifies the points at which a the runtime will be passed data, from a connection,
    /// and where the runtime can return data to the connection.
    #[async_trait::async_trait]
    pub trait HCSProtocol<T>
    where
        T: data::Data,
    {
        /// `greet` is the entry point of any connection between the client and server.
        /// The client has sent a `Greet` payload and the server processes the query accordingly. For
        /// example, if the client is outdated, an error will be returned.
        async fn greet(&self, payload: data::Greeting) -> T;

        /// The runtime that implements the HCSProtocol must be able to distinguish what the data_uid
        /// is. For the TCP runtime, this is done by reading the first 16 bytes of a payload.
        /// `receive_payload` is where the server receives the data sent by the client in `send_payload`.
        async fn receive_payload(&self, data_uid: u16, payload: T);

        /// Here, the payload (i.e. serialize `T`) is sent with the data_uid. This would then be received
        /// by the client in `receive_payload`.
        async fn send_payload(&self) -> T;
    }
}
