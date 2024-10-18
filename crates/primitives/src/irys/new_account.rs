use alloy_primitives::U256;
use alloy_rlp::{RlpDecodable, RlpEncodable};

use reth_codecs::{main_codec, Compact};

use super::{
    commitment::{Commitments, Stake},
    last_tx::LastTx,
};

// temporary arbitrary account diff logic
// remove me!
#[main_codec]
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, RlpEncodable, RlpDecodable)]
#[rlp(trailing)]
pub struct NewAccountState {
    /// Account balance.
    pub balance: Option<U256>,
    /// Account nonce.
    pub nonce: Option<u64>,
    /// code hash,
    // pub code_hash: B256,
    /// code: if None, `code_by_hash` will be used to fetch it if code needs to be loaded from
    /// inside of `revm`.
    // pub code: Option<Bytecode>,
    /// custom pledge addition
    pub stake: Option<WrappedStake>,
    pub commitments: Option<WrappedCommitments>,
    pub last_tx: Option<WrappedLastTx>,
    pub mining_permission: Option<bool>,
}
// weird how many problems can be solved with wrapper structs
#[main_codec]
#[derive(Debug, Clone, Eq, PartialEq, Hash, RlpEncodable, RlpDecodable)]
#[rlp(trailing)]
pub struct WrappedStake(pub Option<Stake>);

#[main_codec]
#[derive(Debug, Clone, Eq, PartialEq, Hash, RlpEncodable, RlpDecodable)]
#[rlp(trailing)]
pub struct WrappedCommitments(pub Option<Commitments>);
#[main_codec]
#[derive(Debug, Clone, Eq, PartialEq, Hash, RlpEncodable, RlpDecodable)]
#[rlp(trailing)]
pub struct WrappedLastTx(pub Option<LastTx>);
