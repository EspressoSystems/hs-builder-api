use std::sync::Arc;

use async_trait::async_trait;
use hotshot_types::{data::VidCommitment, traits::{node_implementation::NodeType, signature_key::SignatureKey}};

use crate::block_metadata::{BlockHash, BlockMetadata};

#[async_trait]
pub trait BuilderDataSource<I: NodeType> {
    async fn get_available_blocks(&self, for_parent: &VidCommitment) -> Vec<BlockMetadata<I>>;
    async fn claim_block(&self, block_hash: BlockHash, signature: <<I as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType) -> Arc<Vec<u8>>;
    async fn submit_txn(&self, txn: <I as NodeType>::Transaction);
}
