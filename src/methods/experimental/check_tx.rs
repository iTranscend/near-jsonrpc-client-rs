//! Checks a transaction on the network
//!
//! ## Example
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_jsonrpc_primitives::types::{query::QueryResponseKind, transactions};
//! use near_primitives::types::{AccountId, BlockReference};
//! use near_primitives::transaction::{Action, Transaction, FunctionCallAction};
//! use near_crypto::SecretKey;
//! use core::str::FromStr;
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");
//!
//! let signer_account_id = "fido.testnet".parse::<AccountId>()?;
//! let signer_secret_key = SecretKey::from_str("ed25519:12dhevYshfiRqFSu8DSfxA27pTkmGRv6C5qQWTJYTcBEoB7MSTyidghi5NWXzWqrxCKgxVx97bpXPYQxYN5dieU")?;    // Replace secret_key with valid signer_secret_key
//!
//! let signer = near_crypto::InMemorySigner::from_secret_key(signer_account_id, signer_secret_key);
//! println!("{}, {}", signer.account_id, signer.public_key);
//!
//! let access_key_query_response = client
//!     .call(methods::query::RpcQueryRequest {
//!         block_reference: BlockReference::latest(),
//!         request: near_primitives::views::QueryRequest::ViewAccessKey {
//!             account_id: signer.account_id.clone(),
//!             public_key: signer.public_key.clone(),
//!         },
//!     })
//!     .await?;
//!
//! let current_nonce = match access_key_query_response.kind {
//!     QueryResponseKind::AccessKey(access_key) => access_key.nonce,
//!     _ => Err("failed to extract current nonce")?,
//!  };
//!
//! let other_account = "rpc_docs.testnet".parse::<AccountId>()?;
//! let rating = "4.7".parse::<f32>()?;
//!
//! let transaction = Transaction {
//!     signer_id: signer.account_id.clone(),
//!     public_key: signer.public_key.clone(),
//!     nonce: current_nonce + 1,
//!     receiver_id: "nosedive.testnet".parse::<AccountId>()?,
//!     block_hash: access_key_query_response.block_hash,
//!     actions: vec![Action::FunctionCall(FunctionCallAction {
//!         method_name: "rate".to_string(),
//!         args: json!({
//!             "account_id": other_account,
//!             "rating": rating,
//!         })
//!         .to_string()
//!         .into_bytes(),
//!         gas: 100_000_000_000_000, // 100 TeraGas
//!         deposit: 0,
//!     })],
//! };
//!
//! let request = methods::EXPERIMENTAL_check_tx::RpcCheckTxRequest {
//!     signed_transaction: transaction.sign(&signer)
//! };
//!
//! let response = client.call(request).await;
//!
//! assert!(matches!(
//!     response,
//!     Ok(methods::EXPERIMENTAL_check_tx::RpcBroadcastTxSyncResponse { .. })
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::transactions::{
    RpcBroadcastTxSyncResponse, RpcTransactionError,
};
pub use near_primitives::transaction::SignedTransaction;

#[derive(Debug)]
pub struct RpcCheckTxRequest {
    pub signed_transaction: SignedTransaction,
}

impl From<RpcCheckTxRequest>
    for near_jsonrpc_primitives::types::transactions::RpcBroadcastTransactionRequest
{
    fn from(this: RpcCheckTxRequest) -> Self {
        Self {
            signed_transaction: this.signed_transaction,
        }
    }
}

impl RpcMethod for RpcCheckTxRequest {
    type Response = RpcBroadcastTxSyncResponse;
    type Error = RpcTransactionError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_check_tx"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!([common::serialize_signed_transaction(
            &self.signed_transaction
        )?]))
    }
}

impl private::Sealed for RpcCheckTxRequest {}
