use alloy_json_rpc::RpcObject;
use alloy_transports::{RpcCall, RpcClient, Transport};

pub trait Network {
    type Transaction: Transaction;
    type Receipt: RpcObject;
}

pub trait Transaction: alloy_rlp::Encodable + alloy_rlp::Decodable + RpcObject {}

pub trait Eip1559Transaction: Transaction {}

pub trait Middleware<N: Network> {
    type Transport: Transport;
    type Inner: Middleware<N, Transport = Self::Transport>;

    fn client(&self) -> &RpcClient<Self::Transport>;

    fn inner(&self) -> &Self::Inner;

    fn send_transaction(
        &self,
        tx: &N::Transaction,
    ) -> RpcCall<Self::Transport, N::Transaction, N::Receipt> {
        self.inner().send_transaction(tx)
    }
}

impl<N: Network, T: Transport> Middleware<N> for RpcClient<T> {
    type Transport = T;

    type Inner = Self;

    fn client(&self) -> &RpcClient<Self::Transport> {
        self
    }

    fn inner(&self) -> &Self::Inner {
        panic!("called inner on RpcClient")
    }

    fn send_transaction(
        &self,
        tx: &N::Transaction,
    ) -> RpcCall<Self::Transport, N::Transaction, N::Receipt> {
        self.prepare("eth_sendTransaction", tx)
    }
}
