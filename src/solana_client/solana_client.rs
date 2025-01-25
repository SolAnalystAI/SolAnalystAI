use solana_client::{
    client_error::ClientError,
    nonblocking::rpc_client::RpcClient,
    rpc_request::TokenAccountsFilter,
    rpc_response::{RpcConfirmedTransactionStatusWithSignature, RpcKeyedAccount},
};
use solana_sdk::{account::Account, pubkey::Pubkey};
use std::env;

pub struct SolanaClient {
    client: RpcClient,
}