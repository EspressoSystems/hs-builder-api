use async_trait::async_trait;
use hotshot_types::{
    data::VidCommitment,
    traits::{node_implementation::NodeType, signature_key::SignatureKey},
    utils::BuilderCommitment
};
use tagged_base64::TaggedBase64;

use crate::{
    block_info::{AvailableBlockInfo, AvailableBlockData},
    builder::BuildError,
};

#[async_trait]
pub trait BuilderDataSource<I>
where
    I: NodeType,
    <<I as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType:
        for<'a> TryFrom<&'a TaggedBase64> + Into<TaggedBase64>,
{
    async fn get_available_blocks(
        &self,
        for_parent: &VidCommitment,
    ) -> Result<Vec<AvailableBlockInfo<I>>, BuildError>;
    async fn claim_block(
        &self,
        block_hash: &BuilderCommitment,
        signature: &<<I as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockData<I>, BuildError>;
    async fn submit_txn(&self, txn: <I as NodeType>::Transaction) -> Result<(), BuildError>;
}
