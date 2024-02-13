use std::{hash::Hash, marker::PhantomData};

use commit::{Commitment, Committable};
use hotshot_types::{
    traits::{node_implementation::NodeType, signature_key::SignatureKey, BlockPayload},
    utils::BuilderCommitment,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub struct HashableBlock<I: NodeType>(
    <I as NodeType>::BlockPayload,
    <<I as NodeType>::BlockPayload as BlockPayload>::Metadata,
);
pub type BlockHash<I> = Commitment<HashableBlock<I>>;
impl<I: NodeType> Default for HashableBlock<I> {
    fn default() -> Self {
        let (bp, bm) = <I as NodeType>::BlockPayload::from_transactions(Vec::new())
            .unwrap_or_else(|_| <I as NodeType>::BlockPayload::genesis());
        Self(bp, bm)
    }
}

impl<I: NodeType> Committable for HashableBlock<I> {
    fn commit(&self) -> Commitment<Self> {
        let builder = commit::RawCommitmentBuilder::new("Hashable Block Payload");
        let mut hasher = Sha256::new();
        let encoded = if let Ok(encoder) = self.0.encode() {
            encoder.collect()
        } else {
            Vec::new()
        };
        hasher.update(&encoded);
        let generic_array = hasher.finalize();
        builder.generic_byte_array(&generic_array).finalize()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(bound = "")]
pub struct AvailableBlockInfo<I: NodeType> {
    pub block_hash: BuilderCommitment,
    pub block_size: u64,
    pub offered_fee: u64,
    pub signature: <<I as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    pub sender: <I as NodeType>::SignatureKey,
    pub _phantom: PhantomData<I>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(bound = "")]
pub struct AvailableBlockData<I: NodeType> {
    pub block_payload: <I as NodeType>::BlockPayload,
    pub signature: <<I as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    pub sender: <I as NodeType>::SignatureKey,
    pub _phantom: PhantomData<I>,
}