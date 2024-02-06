use std::{hash::Hash, marker::PhantomData};

use commit::{Commitment, Committable};
use hotshot_types::traits::{node_implementation::NodeType, BlockPayload};
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(bound = "")]
pub struct BlockMetadata<I: NodeType> {
    pub block_hash: BlockHash<I>,
    pub block_size: u64,
    pub offered_fee: u64,
    _phantom: PhantomData<I>,
}
