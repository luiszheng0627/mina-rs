// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Mina ExternalTransition

use crate::types::*;
use crate::verifiable::Verifiable;
use mina_serialization_types::{json::*, v1::ExternalTransitionV1, *};
use mina_serialization_types_macros::AutoFrom;
use proof_systems::mina_signer::Signer;
use versioned::*;

/// This structure represents a mina block
#[derive(Clone, Debug, Eq, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::external_transition::ExternalTransition)]
/// This structure represents a mina block received from an external block producer
pub struct ExternalTransition {
    /// The blockchain state, including consensus and the ledger
    pub protocol_state: ProtocolStateLegacy,
    /// Proof that the protocol state and entire history of the chain is valid
    pub protocol_state_proof: ProtocolStateProof,
    /// Diff of the proposed next state of the blockchain
    pub staged_ledger_diff: StagedLedgerDiff,
    /// Proof that the block was produced within the allotted slot time
    pub delta_transition_chain_proof: DeltaTransitionChainProof,
    /// Current protocol version
    pub current_protocol_version: ProtocolVersion,
    /// Proposed protocol version
    pub proposed_protocol_version_opt: Option<ProtocolVersion>,
    /// Validation callback
    pub validation_callback: (),
}

impl_from_with_proxy!(
    ExternalTransition,
    ExternalTransitionV1,
    ExternalTransitionJson
);

impl BinProtSerializationType<'_> for ExternalTransition {
    type T = ExternalTransitionV1;
}

impl JsonSerializationType<'_> for ExternalTransition {
    type T = ExternalTransitionJson;
}

impl<CTX> Verifiable<CTX> for ExternalTransition
where
    CTX: Signer<SignedCommandPayload>,
{
    // ExternalTransition is considered valid if:
    // - Its staged ledger diff is valid
    // - TODO
    fn verify(&self, ctx: &mut CTX) -> bool {
        self.staged_ledger_diff.verify(ctx)
    }
}
