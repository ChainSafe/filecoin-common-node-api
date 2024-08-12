#![allow(clippy::to_string_trait_impl, clippy::clone_on_copy)]
use super::TestFailure;
use serde::{Deserialize, Serialize};
#[allow(non_snake_case, unused)]
impl super::Ctx {
    pub fn Filecoin_ChainGetTipSetAfterHeight(
        &mut self,
        height: i64,
        tsk: Option<&Vec<Cid>>,
    ) -> Result<Tipset, TestFailure> {
        self.call("Filecoin.ChainGetTipSetAfterHeight", (height, tsk))
    }
    pub fn Filecoin_StateGetBeaconEntry(&mut self, epoch: i64) -> Result<BeaconEntry, TestFailure> {
        self.call("Filecoin.StateGetBeaconEntry", (epoch,))
    }
    pub fn Filecoin_StateSectorPreCommitInfo(
        &mut self,
        miner_address: &Address,
        sector_number: u64,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Option<SectorPreCommitOnChainInfo>, TestFailure> {
        self.call(
            "Filecoin.StateSectorPreCommitInfo",
            (miner_address, sector_number, tipset_key),
        )
    }
    pub fn Filecoin_StateMinerAllocated(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<BitField, TestFailure> {
        self.call("Filecoin.StateMinerAllocated", (address, tipset_key))
    }
    pub fn Filecoin_StateGetRandomnessDigestFromTickets(
        &mut self,
        rand_epoch: i64,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Base64String, TestFailure> {
        self.call(
            "Filecoin.StateGetRandomnessDigestFromTickets",
            (rand_epoch, tipset_key),
        )
    }
    pub fn Filecoin_StateGetRandomnessDigestFromBeacon(
        &mut self,
        rand_epoch: i64,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Base64String, TestFailure> {
        self.call(
            "Filecoin.StateGetRandomnessDigestFromBeacon",
            (rand_epoch, tipset_key),
        )
    }
    pub fn Filecoin_StateWaitMsg(
        &mut self,
        message_cid: &Cid,
        confidence: i64,
        look_back_limit: i64,
        allow_replaced: bool,
    ) -> Result<MessageLookup, TestFailure> {
        self.call(
            "Filecoin.StateWaitMsg",
            (message_cid, confidence, look_back_limit, allow_replaced),
        )
    }
    pub fn Filecoin_StateGetAllocationIdForPendingDeal(
        &mut self,
        deal_id: u64,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<u64, TestFailure> {
        self.call(
            "Filecoin.StateGetAllocationIdForPendingDeal",
            (deal_id, tipset_key),
        )
    }
    pub fn Filecoin_StateGetAllocationForPendingDeal(
        &mut self,
        deal_id: u64,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Option<Allocation>, TestFailure> {
        self.call(
            "Filecoin.StateGetAllocationForPendingDeal",
            (deal_id, tipset_key),
        )
    }
    pub fn Filecoin_StateLookupRobustAddress(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Address, TestFailure> {
        self.call("Filecoin.StateLookupRobustAddress", (address, tipset_key))
    }
}
#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Address"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Address(pub String);
impl std::ops::Deref for Address {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Address> for String {
    fn from(value: Address) -> Self {
        value.0
    }
}
impl From<&Address> for Address {
    fn from(value: &Address) -> Self {
        value.clone()
    }
}
impl From<String> for Address {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for Address {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for Address {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "Allocation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Client\","]
#[doc = "    \"Data\","]
#[doc = "    \"Expiration\","]
#[doc = "    \"Provider\","]
#[doc = "    \"Size\","]
#[doc = "    \"TermMax\","]
#[doc = "    \"TermMin\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Client\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Data\": {"]
#[doc = "      \"$ref\": \"#/definitions/Cid\""]
#[doc = "    },"]
#[doc = "    \"Expiration\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"Provider\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Size\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"TermMax\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"TermMin\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Allocation {
    #[serde(rename = "Client")]
    pub client: u64,
    #[serde(rename = "Data")]
    pub data: Cid,
    #[serde(rename = "Expiration")]
    pub expiration: i64,
    #[serde(rename = "Provider")]
    pub provider: u64,
    #[serde(rename = "Size")]
    pub size: u64,
    #[serde(rename = "TermMax")]
    pub term_max: i64,
    #[serde(rename = "TermMin")]
    pub term_min: i64,
}
impl From<&Allocation> for Allocation {
    fn from(value: &Allocation) -> Self {
        value.clone()
    }
}
#[doc = "Base64String"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"null\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Base64String(pub Option<String>);
impl std::ops::Deref for Base64String {
    type Target = Option<String>;
    fn deref(&self) -> &Option<String> {
        &self.0
    }
}
impl From<Base64String> for Option<String> {
    fn from(value: Base64String) -> Self {
        value.0
    }
}
impl From<&Base64String> for Base64String {
    fn from(value: &Base64String) -> Self {
        value.clone()
    }
}
impl From<Option<String>> for Base64String {
    fn from(value: Option<String>) -> Self {
        Self(value)
    }
}
#[doc = "BeaconEntry"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Data\","]
#[doc = "    \"Round\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Data\": {"]
#[doc = "      \"$ref\": \"#/definitions/Base64String\""]
#[doc = "    },"]
#[doc = "    \"Round\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BeaconEntry {
    #[serde(rename = "Data")]
    pub data: Base64String,
    #[serde(rename = "Round")]
    pub round: u64,
}
impl From<&BeaconEntry> for BeaconEntry {
    fn from(value: &BeaconEntry) -> Self {
        value.clone()
    }
}
#[doc = "BigInt"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BigInt(pub String);
impl std::ops::Deref for BigInt {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<BigInt> for String {
    fn from(value: BigInt) -> Self {
        value.0
    }
}
impl From<&BigInt> for BigInt {
    fn from(value: &BigInt) -> Self {
        value.clone()
    }
}
impl From<String> for BigInt {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for BigInt {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for BigInt {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "BitField"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"array\","]
#[doc = "    \"null\""]
#[doc = "  ],"]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"integer\","]
#[doc = "    \"format\": \"uint8\","]
#[doc = "    \"minimum\": 0.0"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitField(pub Option<Vec<u8>>);
impl std::ops::Deref for BitField {
    type Target = Option<Vec<u8>>;
    fn deref(&self) -> &Option<Vec<u8>> {
        &self.0
    }
}
impl From<BitField> for Option<Vec<u8>> {
    fn from(value: BitField) -> Self {
        value.0
    }
}
impl From<&BitField> for BitField {
    fn from(value: &BitField) -> Self {
        value.clone()
    }
}
impl From<Option<Vec<u8>>> for BitField {
    fn from(value: Option<Vec<u8>>) -> Self {
        Self(value)
    }
}
#[doc = "BlockHeader"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BeaconEntries\","]
#[doc = "    \"ForkSignaling\","]
#[doc = "    \"Height\","]
#[doc = "    \"Messages\","]
#[doc = "    \"Miner\","]
#[doc = "    \"ParentBaseFee\","]
#[doc = "    \"ParentMessageReceipts\","]
#[doc = "    \"ParentStateRoot\","]
#[doc = "    \"ParentWeight\","]
#[doc = "    \"Parents\","]
#[doc = "    \"Timestamp\","]
#[doc = "    \"WinPoStProof\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BLSAggregate\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Signature\""]
#[doc = "    },"]
#[doc = "    \"BeaconEntries\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_BeaconEntry\""]
#[doc = "    },"]
#[doc = "    \"BlockSig\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Signature\""]
#[doc = "    },"]
#[doc = "    \"ElectionProof\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_ElectionProof\""]
#[doc = "    },"]
#[doc = "    \"ForkSignaling\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Height\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"Messages\": {"]
#[doc = "      \"$ref\": \"#/definitions/Cid\""]
#[doc = "    },"]
#[doc = "    \"Miner\": {"]
#[doc = "      \"$ref\": \"#/definitions/Address\""]
#[doc = "    },"]
#[doc = "    \"ParentBaseFee\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"ParentMessageReceipts\": {"]
#[doc = "      \"$ref\": \"#/definitions/Cid\""]
#[doc = "    },"]
#[doc = "    \"ParentStateRoot\": {"]
#[doc = "      \"$ref\": \"#/definitions/Cid\""]
#[doc = "    },"]
#[doc = "    \"ParentWeight\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"Parents\": {"]
#[doc = "      \"$ref\": \"#/definitions/NonEmpty_Array_of_Cid\""]
#[doc = "    },"]
#[doc = "    \"Ticket\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Ticket\""]
#[doc = "    },"]
#[doc = "    \"Timestamp\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"WinPoStProof\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_PoStProof\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockHeader {
    #[serde(rename = "BeaconEntries")]
    pub beacon_entries: NullableArrayOfBeaconEntry,
    #[serde(rename = "BlockSig", default, skip_serializing_if = "Option::is_none")]
    pub block_sig: Option<NullableSignature>,
    #[serde(
        rename = "BLSAggregate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bls_aggregate: Option<NullableSignature>,
    #[serde(
        rename = "ElectionProof",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub election_proof: Option<NullableElectionProof>,
    #[serde(rename = "ForkSignaling")]
    pub fork_signaling: u64,
    #[serde(rename = "Height")]
    pub height: i64,
    #[serde(rename = "Messages")]
    pub messages: Cid,
    #[serde(rename = "Miner")]
    pub miner: Address,
    #[serde(rename = "ParentBaseFee")]
    pub parent_base_fee: BigInt,
    #[serde(rename = "ParentMessageReceipts")]
    pub parent_message_receipts: Cid,
    #[serde(rename = "ParentStateRoot")]
    pub parent_state_root: Cid,
    #[serde(rename = "ParentWeight")]
    pub parent_weight: BigInt,
    #[serde(rename = "Parents")]
    pub parents: NonEmptyArrayOfCid,
    #[serde(rename = "Ticket", default, skip_serializing_if = "Option::is_none")]
    pub ticket: Option<NullableTicket>,
    #[serde(rename = "Timestamp")]
    pub timestamp: u64,
    #[serde(rename = "WinPoStProof")]
    pub win_po_st_proof: NullableArrayOfPoStProof,
}
impl From<&BlockHeader> for BlockHeader {
    fn from(value: &BlockHeader) -> Self {
        value.clone()
    }
}
#[doc = "Cid"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"/\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"/\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cid {
    #[serde(rename = "/")]
    pub x: String,
}
impl From<&Cid> for Cid {
    fn from(value: &Cid) -> Self {
        value.clone()
    }
}
#[doc = "ElectionProof"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"VRFProof\","]
#[doc = "    \"WinCount\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"VRFProof\": {"]
#[doc = "      \"$ref\": \"#/definitions/Base64String\""]
#[doc = "    },"]
#[doc = "    \"WinCount\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ElectionProof {
    #[serde(rename = "VRFProof")]
    pub vrf_proof: Base64String,
    #[serde(rename = "WinCount")]
    pub win_count: i64,
}
impl From<&ElectionProof> for ElectionProof {
    fn from(value: &ElectionProof) -> Self {
        value.clone()
    }
}
#[doc = "Int64"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"int64\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Int64(pub i64);
impl std::ops::Deref for Int64 {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl From<Int64> for i64 {
    fn from(value: Int64) -> Self {
        value.0
    }
}
impl From<&Int64> for Int64 {
    fn from(value: &Int64) -> Self {
        value.clone()
    }
}
impl From<i64> for Int64 {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for Int64 {
    type Err = <i64 as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for Int64 {
    type Error = <i64 as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Int64 {
    type Error = <i64 as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Int64 {
    type Error = <i64 as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for Int64 {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "MessageLookup"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Height\","]
#[doc = "    \"Message\","]
#[doc = "    \"Receipt\","]
#[doc = "    \"ReturnDec\","]
#[doc = "    \"TipSet\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Height\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"Message\": {"]
#[doc = "      \"$ref\": \"#/definitions/Cid\""]
#[doc = "    },"]
#[doc = "    \"Receipt\": {"]
#[doc = "      \"$ref\": \"#/definitions/Receipt\""]
#[doc = "    },"]
#[doc = "    \"ReturnDec\": true,"]
#[doc = "    \"TipSet\": {"]
#[doc = "      \"$ref\": \"#/definitions/NonEmpty_Array_of_Cid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageLookup {
    #[serde(rename = "Height")]
    pub height: i64,
    #[serde(rename = "Message")]
    pub message: Cid,
    #[serde(rename = "Receipt")]
    pub receipt: Receipt,
    #[serde(rename = "ReturnDec")]
    pub return_dec: serde_json::Value,
    #[serde(rename = "TipSet")]
    pub tip_set: NonEmptyArrayOfCid,
}
impl From<&MessageLookup> for MessageLookup {
    fn from(value: &MessageLookup) -> Self {
        value.clone()
    }
}
#[doc = "NonEmptyArrayOfBlockHeader"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/definitions/BlockHeader\""]
#[doc = "  },"]
#[doc = "  \"minItems\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NonEmptyArrayOfBlockHeader(pub Vec<BlockHeader>);
impl std::ops::Deref for NonEmptyArrayOfBlockHeader {
    type Target = Vec<BlockHeader>;
    fn deref(&self) -> &Vec<BlockHeader> {
        &self.0
    }
}
impl From<NonEmptyArrayOfBlockHeader> for Vec<BlockHeader> {
    fn from(value: NonEmptyArrayOfBlockHeader) -> Self {
        value.0
    }
}
impl From<&NonEmptyArrayOfBlockHeader> for NonEmptyArrayOfBlockHeader {
    fn from(value: &NonEmptyArrayOfBlockHeader) -> Self {
        value.clone()
    }
}
impl From<Vec<BlockHeader>> for NonEmptyArrayOfBlockHeader {
    fn from(value: Vec<BlockHeader>) -> Self {
        Self(value)
    }
}
#[doc = "NonEmptyArrayOfCid"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/definitions/Cid\""]
#[doc = "  },"]
#[doc = "  \"minItems\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NonEmptyArrayOfCid(pub Vec<Cid>);
impl std::ops::Deref for NonEmptyArrayOfCid {
    type Target = Vec<Cid>;
    fn deref(&self) -> &Vec<Cid> {
        &self.0
    }
}
impl From<NonEmptyArrayOfCid> for Vec<Cid> {
    fn from(value: NonEmptyArrayOfCid) -> Self {
        value.0
    }
}
impl From<&NonEmptyArrayOfCid> for NonEmptyArrayOfCid {
    fn from(value: &NonEmptyArrayOfCid) -> Self {
        value.clone()
    }
}
impl From<Vec<Cid>> for NonEmptyArrayOfCid {
    fn from(value: Vec<Cid>) -> Self {
        Self(value)
    }
}
#[doc = "NullableArrayOfBeaconEntry"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"array\","]
#[doc = "    \"null\""]
#[doc = "  ],"]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/definitions/BeaconEntry\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableArrayOfBeaconEntry(pub Option<Vec<BeaconEntry>>);
impl std::ops::Deref for NullableArrayOfBeaconEntry {
    type Target = Option<Vec<BeaconEntry>>;
    fn deref(&self) -> &Option<Vec<BeaconEntry>> {
        &self.0
    }
}
impl From<NullableArrayOfBeaconEntry> for Option<Vec<BeaconEntry>> {
    fn from(value: NullableArrayOfBeaconEntry) -> Self {
        value.0
    }
}
impl From<&NullableArrayOfBeaconEntry> for NullableArrayOfBeaconEntry {
    fn from(value: &NullableArrayOfBeaconEntry) -> Self {
        value.clone()
    }
}
impl From<Option<Vec<BeaconEntry>>> for NullableArrayOfBeaconEntry {
    fn from(value: Option<Vec<BeaconEntry>>) -> Self {
        Self(value)
    }
}
#[doc = "NullableArrayOfPoStProof"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"array\","]
#[doc = "    \"null\""]
#[doc = "  ],"]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/definitions/PoStProof\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableArrayOfPoStProof(pub Option<Vec<PoStProof>>);
impl std::ops::Deref for NullableArrayOfPoStProof {
    type Target = Option<Vec<PoStProof>>;
    fn deref(&self) -> &Option<Vec<PoStProof>> {
        &self.0
    }
}
impl From<NullableArrayOfPoStProof> for Option<Vec<PoStProof>> {
    fn from(value: NullableArrayOfPoStProof) -> Self {
        value.0
    }
}
impl From<&NullableArrayOfPoStProof> for NullableArrayOfPoStProof {
    fn from(value: &NullableArrayOfPoStProof) -> Self {
        value.clone()
    }
}
impl From<Option<Vec<PoStProof>>> for NullableArrayOfPoStProof {
    fn from(value: Option<Vec<PoStProof>>) -> Self {
        Self(value)
    }
}
#[doc = "NullableArrayOfUint64"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"array\","]
#[doc = "    \"null\""]
#[doc = "  ],"]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"integer\","]
#[doc = "    \"format\": \"uint64\","]
#[doc = "    \"minimum\": 0.0"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableArrayOfUint64(pub Option<Vec<u64>>);
impl std::ops::Deref for NullableArrayOfUint64 {
    type Target = Option<Vec<u64>>;
    fn deref(&self) -> &Option<Vec<u64>> {
        &self.0
    }
}
impl From<NullableArrayOfUint64> for Option<Vec<u64>> {
    fn from(value: NullableArrayOfUint64) -> Self {
        value.0
    }
}
impl From<&NullableArrayOfUint64> for NullableArrayOfUint64 {
    fn from(value: &NullableArrayOfUint64) -> Self {
        value.clone()
    }
}
impl From<Option<Vec<u64>>> for NullableArrayOfUint64 {
    fn from(value: Option<Vec<u64>>) -> Self {
        Self(value)
    }
}
#[doc = "NullableCid"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Cid\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableCid(pub Option<Cid>);
impl std::ops::Deref for NullableCid {
    type Target = Option<Cid>;
    fn deref(&self) -> &Option<Cid> {
        &self.0
    }
}
impl From<NullableCid> for Option<Cid> {
    fn from(value: NullableCid) -> Self {
        value.0
    }
}
impl From<&NullableCid> for NullableCid {
    fn from(value: &NullableCid) -> Self {
        value.clone()
    }
}
impl From<Option<Cid>> for NullableCid {
    fn from(value: Option<Cid>) -> Self {
        Self(value)
    }
}
#[doc = "NullableElectionProof"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ElectionProof\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableElectionProof(pub Option<ElectionProof>);
impl std::ops::Deref for NullableElectionProof {
    type Target = Option<ElectionProof>;
    fn deref(&self) -> &Option<ElectionProof> {
        &self.0
    }
}
impl From<NullableElectionProof> for Option<ElectionProof> {
    fn from(value: NullableElectionProof) -> Self {
        value.0
    }
}
impl From<&NullableElectionProof> for NullableElectionProof {
    fn from(value: &NullableElectionProof) -> Self {
        value.clone()
    }
}
impl From<Option<ElectionProof>> for NullableElectionProof {
    fn from(value: Option<ElectionProof>) -> Self {
        Self(value)
    }
}
#[doc = "NullableSignature"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Signature\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableSignature(pub Option<Signature>);
impl std::ops::Deref for NullableSignature {
    type Target = Option<Signature>;
    fn deref(&self) -> &Option<Signature> {
        &self.0
    }
}
impl From<NullableSignature> for Option<Signature> {
    fn from(value: NullableSignature) -> Self {
        value.0
    }
}
impl From<&NullableSignature> for NullableSignature {
    fn from(value: &NullableSignature) -> Self {
        value.clone()
    }
}
impl From<Option<Signature>> for NullableSignature {
    fn from(value: Option<Signature>) -> Self {
        Self(value)
    }
}
#[doc = "NullableTicket"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Ticket\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableTicket(pub Option<Ticket>);
impl std::ops::Deref for NullableTicket {
    type Target = Option<Ticket>;
    fn deref(&self) -> &Option<Ticket> {
        &self.0
    }
}
impl From<NullableTicket> for Option<Ticket> {
    fn from(value: NullableTicket) -> Self {
        value.0
    }
}
impl From<&NullableTicket> for NullableTicket {
    fn from(value: &NullableTicket) -> Self {
        value.clone()
    }
}
impl From<Option<Ticket>> for NullableTicket {
    fn from(value: Option<Ticket>) -> Self {
        Self(value)
    }
}
#[doc = "PoStProof"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"PoStProof\","]
#[doc = "    \"ProofBytes\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"PoStProof\": {"]
#[doc = "      \"$ref\": \"#/definitions/int64\""]
#[doc = "    },"]
#[doc = "    \"ProofBytes\": {"]
#[doc = "      \"$ref\": \"#/definitions/Base64String\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PoStProof {
    #[serde(rename = "PoStProof")]
    pub po_st_proof: Int64,
    #[serde(rename = "ProofBytes")]
    pub proof_bytes: Base64String,
}
impl From<&PoStProof> for PoStProof {
    fn from(value: &PoStProof) -> Self {
        value.clone()
    }
}
#[doc = "Receipt"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ExitCode\","]
#[doc = "    \"GasUsed\","]
#[doc = "    \"Return\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"EventsRoot\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Cid\""]
#[doc = "    },"]
#[doc = "    \"ExitCode\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"GasUsed\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Return\": {"]
#[doc = "      \"$ref\": \"#/definitions/Base64String\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Receipt {
    #[serde(rename = "EventsRoot", default = "defaults::receipt_events_root")]
    pub events_root: NullableCid,
    #[serde(rename = "ExitCode")]
    pub exit_code: u32,
    #[serde(rename = "GasUsed")]
    pub gas_used: u64,
    #[serde(rename = "Return")]
    pub return_: Base64String,
}
impl From<&Receipt> for Receipt {
    fn from(value: &Receipt) -> Self {
        value.clone()
    }
}
#[doc = "SectorPreCommitInfo"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"DealIDs\","]
#[doc = "    \"Expiration\","]
#[doc = "    \"SealProof\","]
#[doc = "    \"SealRandEpoch\","]
#[doc = "    \"SealedCID\","]
#[doc = "    \"SectorNumber\","]
#[doc = "    \"UnsealedCid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"DealIDs\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_uint64\""]
#[doc = "    },"]
#[doc = "    \"Expiration\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"SealProof\": {"]
#[doc = "      \"$ref\": \"#/definitions/int64\""]
#[doc = "    },"]
#[doc = "    \"SealRandEpoch\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"SealedCID\": {"]
#[doc = "      \"$ref\": \"#/definitions/Cid\""]
#[doc = "    },"]
#[doc = "    \"SectorNumber\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"UnsealedCid\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Cid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SectorPreCommitInfo {
    #[serde(rename = "DealIDs")]
    pub deal_i_ds: NullableArrayOfUint64,
    #[serde(rename = "Expiration")]
    pub expiration: i64,
    #[serde(rename = "SealProof")]
    pub seal_proof: Int64,
    #[serde(rename = "SealRandEpoch")]
    pub seal_rand_epoch: i64,
    #[serde(rename = "SealedCID")]
    pub sealed_cid: Cid,
    #[serde(rename = "SectorNumber")]
    pub sector_number: u64,
    #[serde(rename = "UnsealedCid")]
    pub unsealed_cid: NullableCid,
}
impl From<&SectorPreCommitInfo> for SectorPreCommitInfo {
    fn from(value: &SectorPreCommitInfo) -> Self {
        value.clone()
    }
}
#[doc = "SectorPreCommitOnChainInfo"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Info\","]
#[doc = "    \"PreCommitDeposit\","]
#[doc = "    \"PreCommitEpoch\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Info\": {"]
#[doc = "      \"$ref\": \"#/definitions/SectorPreCommitInfo\""]
#[doc = "    },"]
#[doc = "    \"PreCommitDeposit\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"PreCommitEpoch\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SectorPreCommitOnChainInfo {
    #[serde(rename = "Info")]
    pub info: SectorPreCommitInfo,
    #[serde(rename = "PreCommitDeposit")]
    pub pre_commit_deposit: BigInt,
    #[serde(rename = "PreCommitEpoch")]
    pub pre_commit_epoch: i64,
}
impl From<&SectorPreCommitOnChainInfo> for SectorPreCommitOnChainInfo {
    fn from(value: &SectorPreCommitOnChainInfo) -> Self {
        value.clone()
    }
}
#[doc = "Signature"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Data\","]
#[doc = "    \"Type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Data\": {"]
#[doc = "      \"$ref\": \"#/definitions/Base64String\""]
#[doc = "    },"]
#[doc = "    \"Type\": {"]
#[doc = "      \"$ref\": \"#/definitions/SignatureTypeLotusJson\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Signature {
    #[serde(rename = "Data")]
    pub data: Base64String,
    #[serde(rename = "Type")]
    pub type_: SignatureTypeLotusJson,
}
impl From<&Signature> for Signature {
    fn from(value: &Signature) -> Self {
        value.clone()
    }
}
#[doc = "Signature variants for Filecoin signatures."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Signature variants for Filecoin signatures.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Secp256k1\","]
#[doc = "    \"Bls\","]
#[doc = "    \"Delegated\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SignatureType {
    Secp256k1,
    Bls,
    Delegated,
}
impl From<&SignatureType> for SignatureType {
    fn from(value: &SignatureType) -> Self {
        value.clone()
    }
}
impl ToString for SignatureType {
    fn to_string(&self) -> String {
        match *self {
            Self::Secp256k1 => "Secp256k1".to_string(),
            Self::Bls => "Bls".to_string(),
            Self::Delegated => "Delegated".to_string(),
        }
    }
}
impl std::str::FromStr for SignatureType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Secp256k1" => Ok(Self::Secp256k1),
            "Bls" => Ok(Self::Bls),
            "Delegated" => Ok(Self::Delegated),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for SignatureType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SignatureType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SignatureType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "SignatureTypeLotusJson"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint8\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/SignatureType\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SignatureTypeLotusJson {
    Variant0(u8),
    Variant1(SignatureType),
}
impl From<&SignatureTypeLotusJson> for SignatureTypeLotusJson {
    fn from(value: &SignatureTypeLotusJson) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for SignatureTypeLotusJson {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for SignatureTypeLotusJson {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SignatureTypeLotusJson {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SignatureTypeLotusJson {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ToString for SignatureTypeLotusJson {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<u8> for SignatureTypeLotusJson {
    fn from(value: u8) -> Self {
        Self::Variant0(value)
    }
}
impl From<SignatureType> for SignatureTypeLotusJson {
    fn from(value: SignatureType) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "Ticket"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"VRFProof\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"VRFProof\": {"]
#[doc = "      \"$ref\": \"#/definitions/Base64String\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ticket {
    #[serde(rename = "VRFProof")]
    pub vrf_proof: Base64String,
}
impl From<&Ticket> for Ticket {
    fn from(value: &Ticket) -> Self {
        value.clone()
    }
}
#[doc = "Tipset"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$ref\": \"#/definitions/TipsetInner\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tipset(pub TipsetInner);
impl std::ops::Deref for Tipset {
    type Target = TipsetInner;
    fn deref(&self) -> &TipsetInner {
        &self.0
    }
}
impl From<Tipset> for TipsetInner {
    fn from(value: Tipset) -> Self {
        value.0
    }
}
impl From<&Tipset> for Tipset {
    fn from(value: &Tipset) -> Self {
        value.clone()
    }
}
impl From<TipsetInner> for Tipset {
    fn from(value: TipsetInner) -> Self {
        Self(value)
    }
}
#[doc = "TipsetInner"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Blocks\","]
#[doc = "    \"Cids\","]
#[doc = "    \"Height\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Blocks\": {"]
#[doc = "      \"$ref\": \"#/definitions/NonEmpty_Array_of_BlockHeader\""]
#[doc = "    },"]
#[doc = "    \"Cids\": {"]
#[doc = "      \"$ref\": \"#/definitions/NonEmpty_Array_of_Cid\""]
#[doc = "    },"]
#[doc = "    \"Height\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TipsetInner {
    #[serde(rename = "Blocks")]
    pub blocks: NonEmptyArrayOfBlockHeader,
    #[serde(rename = "Cids")]
    pub cids: NonEmptyArrayOfCid,
    #[serde(rename = "Height")]
    pub height: i64,
}
impl From<&TipsetInner> for TipsetInner {
    fn from(value: &TipsetInner) -> Self {
        value.clone()
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn receipt_events_root() -> super::NullableCid {
        super::NullableCid(None)
    }
}
