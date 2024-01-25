use std::marker::PhantomData;

use hotshot_types::traits::node_implementation::NodeType;
use serde::{Deserialize, Serialize};
use sha2::digest::{generic_array::GenericArray, typenum};

pub type BlockHash = GenericArray<u8, typenum::consts::U32>;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(bound = "")]
pub struct BlockMetadata<I: NodeType> {
    block_hash: BlockHash,
    block_size: u64,
    offered_fee: u64,
    _phantom: PhantomData<I>,
}

