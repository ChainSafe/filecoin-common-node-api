#![allow(clippy::to_string_trait_impl, clippy::clone_on_copy)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
#[allow(non_snake_case, unused)]
pub trait Api {
    type Error;
    fn call<T: DeserializeOwned>(
        &mut self,
        method: impl Into<String>,
        params: impl ez_jsonrpc::params::SerializePositional,
    ) -> Result<T, Self::Error>;
    fn Filecoin_ChainGetMessage(&mut self, msg_cid: &Cid) -> Result<Message, Self::Error> {
        self.call("Filecoin.ChainGetMessage", (msg_cid,))
    }
    fn Filecoin_ChainGetParentMessages(
        &mut self,
        block_cid: &Cid,
    ) -> Result<Option<Vec<ApiMessage>>, Self::Error> {
        self.call("Filecoin.ChainGetParentMessages", (block_cid,))
    }
    fn Filecoin_ChainGetParentReceipts(
        &mut self,
        block_cid: &Cid,
    ) -> Result<Option<Vec<ApiReceipt>>, Self::Error> {
        self.call("Filecoin.ChainGetParentReceipts", (block_cid,))
    }
    fn Filecoin_ChainReadObj(&mut self, cid: &Cid) -> Result<Base64String, Self::Error> {
        self.call("Filecoin.ChainReadObj", (cid,))
    }
    fn Filecoin_ChainHasObj(&mut self, cid: &Cid) -> Result<bool, Self::Error> {
        self.call("Filecoin.ChainHasObj", (cid,))
    }
    fn Filecoin_ChainGetBlockMessages(&mut self, cid: &Cid) -> Result<BlockMessages, Self::Error> {
        self.call("Filecoin.ChainGetBlockMessages", (cid,))
    }
    fn Filecoin_ChainGetPath(
        &mut self,
        from: &NonEmptyArrayOfCid,
        to: &NonEmptyArrayOfCid,
    ) -> Result<Option<Vec<PathChangeForTipset>>, Self::Error> {
        self.call("Filecoin.ChainGetPath", (from, to))
    }
    fn Filecoin_ChainGetTipSetByHeight(
        &mut self,
        height: i64,
        tsk: Option<&Vec<Cid>>,
    ) -> Result<Tipset, Self::Error> {
        self.call("Filecoin.ChainGetTipSetByHeight", (height, tsk))
    }
    fn Filecoin_ChainHead(&mut self) -> Result<Tipset, Self::Error> {
        self.call("Filecoin.ChainHead", ())
    }
    fn Filecoin_ChainGetBlock(&mut self, cid: &Cid) -> Result<BlockHeader, Self::Error> {
        self.call("Filecoin.ChainGetBlock", (cid,))
    }
    fn Filecoin_ChainGetTipSet(&mut self, tsk: Option<&Vec<Cid>>) -> Result<Tipset, Self::Error> {
        self.call("Filecoin.ChainGetTipSet", (tsk,))
    }
    fn Filecoin_ChainTipSetWeight(
        &mut self,
        tsk: Option<&Vec<Cid>>,
    ) -> Result<BigInt, Self::Error> {
        self.call("Filecoin.ChainTipSetWeight", (tsk,))
    }
    fn Filecoin_GasEstimateGasLimit(
        &mut self,
        msg: &Message,
        tsk: Option<&Vec<Cid>>,
    ) -> Result<i64, Self::Error> {
        self.call("Filecoin.GasEstimateGasLimit", (msg, tsk))
    }
    fn Filecoin_GasEstimateMessageGas(
        &mut self,
        msg: &Message,
        spec: Option<&MessageSendSpec>,
        tsk: Option<&Vec<Cid>>,
    ) -> Result<Message, Self::Error> {
        self.call("Filecoin.GasEstimateMessageGas", (msg, spec, tsk))
    }
    fn Filecoin_GasEstimateFeeCap(
        &mut self,
        message: &Message,
        max_queue_blocks: i64,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<String, Self::Error> {
        self.call(
            "Filecoin.GasEstimateFeeCap",
            (message, max_queue_blocks, tipset_key),
        )
    }
    fn Filecoin_GasEstimateGasPremium(
        &mut self,
        nblocksincl: u64,
        sender: &Address,
        gas_limit: i64,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<String, Self::Error> {
        self.call(
            "Filecoin.GasEstimateGasPremium",
            (nblocksincl, sender, gas_limit, tipset_key),
        )
    }
    fn Filecoin_MinerCreateBlock(
        &mut self,
        block_template: &BlockTemplate,
    ) -> Result<BlockMessage, Self::Error> {
        self.call("Filecoin.MinerCreateBlock", (block_template,))
    }
    fn Filecoin_MinerGetBaseInfo(
        &mut self,
        address: &Address,
        epoch: i64,
        tsk: Option<&Vec<Cid>>,
    ) -> Result<Option<MiningBaseInfo>, Self::Error> {
        self.call("Filecoin.MinerGetBaseInfo", (address, epoch, tsk))
    }
    fn Filecoin_MpoolGetNonce(&mut self, address: &Address) -> Result<u64, Self::Error> {
        self.call("Filecoin.MpoolGetNonce", (address,))
    }
    fn Filecoin_MpoolPending(
        &mut self,
        tsk: Option<&Vec<Cid>>,
    ) -> Result<Vec<SignedMessage>, Self::Error> {
        self.call("Filecoin.MpoolPending", (tsk,))
    }
    fn Filecoin_MpoolSelect(
        &mut self,
        tipset_key: Option<&Vec<Cid>>,
        ticket_quality: f64,
    ) -> Result<Option<Vec<SignedMessage>>, Self::Error> {
        self.call("Filecoin.MpoolSelect", (tipset_key, ticket_quality))
    }
    fn Filecoin_MpoolPush(&mut self, msg: &SignedMessage) -> Result<Cid, Self::Error> {
        self.call("Filecoin.MpoolPush", (msg,))
    }
    fn Filecoin_MpoolPushUntrusted(&mut self, msg: &SignedMessage) -> Result<Cid, Self::Error> {
        self.call("Filecoin.MpoolPushUntrusted", (msg,))
    }
    fn Filecoin_MpoolPushMessage(
        &mut self,
        usmg: &Message,
        spec: Option<&MessageSendSpec>,
    ) -> Result<SignedMessage, Self::Error> {
        self.call("Filecoin.MpoolPushMessage", (usmg, spec))
    }
    fn Filecoin_NetAddrsListen(&mut self) -> Result<AddrInfo, Self::Error> {
        self.call("Filecoin.NetAddrsListen", ())
    }
    fn Filecoin_NetPeers(&mut self) -> Result<Option<Vec<AddrInfo>>, Self::Error> {
        self.call("Filecoin.NetPeers", ())
    }
    fn Filecoin_NetConnect(&mut self, info: &AddrInfo) -> Result<(), Self::Error> {
        self.call("Filecoin.NetConnect", (info,))
    }
    fn Filecoin_NetDisconnect(&mut self, id: &str) -> Result<(), Self::Error> {
        self.call("Filecoin.NetDisconnect", (id,))
    }
    fn Filecoin_NetAgentVersion(&mut self, id: &str) -> Result<String, Self::Error> {
        self.call("Filecoin.NetAgentVersion", (id,))
    }
    fn Filecoin_NetProtectAdd(&mut self, acl: &str) -> Result<(), Self::Error> {
        self.call("Filecoin.NetProtectAdd", (acl,))
    }
    fn Filecoin_StateCall(
        &mut self,
        message: &Message,
        tsk: Option<&Vec<Cid>>,
    ) -> Result<ApiInvocResult, Self::Error> {
        self.call("Filecoin.StateCall", (message, tsk))
    }
    fn Filecoin_StateListMessages(
        &mut self,
        message_filter: &MessageFilter,
        tipset_key: Option<&Vec<Cid>>,
        max_height: i64,
    ) -> Result<Option<Vec<Cid>>, Self::Error> {
        self.call(
            "Filecoin.StateListMessages",
            (message_filter, tipset_key, max_height),
        )
    }
    fn Filecoin_StateGetNetworkParams(&mut self) -> Result<NetworkParams, Self::Error> {
        self.call("Filecoin.StateGetNetworkParams", ())
    }
    fn Filecoin_StateReplay(
        &mut self,
        tipset_key: Option<&Vec<Cid>>,
        message_cid: &Cid,
    ) -> Result<ApiInvocResult, Self::Error> {
        self.call("Filecoin.StateReplay", (tipset_key, message_cid))
    }
    fn Filecoin_StateSectorGetInfo(
        &mut self,
        miner_address: &Address,
        sector_number: u64,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Option<SectorOnChainInfo>, Self::Error> {
        self.call(
            "Filecoin.StateSectorGetInfo",
            (miner_address, sector_number, tipset_key),
        )
    }
    fn Filecoin_StateAccountKey(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Address, Self::Error> {
        self.call("Filecoin.StateAccountKey", (address, tipset_key))
    }
    fn Filecoin_StateLookupID(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Address, Self::Error> {
        self.call("Filecoin.StateLookupID", (address, tipset_key))
    }
    fn Filecoin_StateGetActor(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Option<ActorState>, Self::Error> {
        self.call("Filecoin.StateGetActor", (address, tipset_key))
    }
    fn Filecoin_StateMinerInfo(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<MinerInfo, Self::Error> {
        self.call("Filecoin.StateMinerInfo", (address, tipset_key))
    }
    fn Filecoin_StateMinerActiveSectors(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Option<Vec<SectorOnChainInfo>>, Self::Error> {
        self.call("Filecoin.StateMinerActiveSectors", (address, tipset_key))
    }
    fn Filecoin_StateMinerPartitions(
        &mut self,
        address: &Address,
        deadline_index: u64,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Option<Vec<MinerPartitions>>, Self::Error> {
        self.call(
            "Filecoin.StateMinerPartitions",
            (address, deadline_index, tipset_key),
        )
    }
    fn Filecoin_StateMinerSectors(
        &mut self,
        address: &Address,
        sectors: Option<&BitField>,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Option<Vec<SectorOnChainInfo>>, Self::Error> {
        self.call("Filecoin.StateMinerSectors", (address, sectors, tipset_key))
    }
    fn Filecoin_StateMinerSectorCount(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<MinerSectors, Self::Error> {
        self.call("Filecoin.StateMinerSectorCount", (address, tipset_key))
    }
    fn Filecoin_StateMinerSectorAllocated(
        &mut self,
        miner_address: &Address,
        sector_number: u64,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<bool, Self::Error> {
        self.call(
            "Filecoin.StateMinerSectorAllocated",
            (miner_address, sector_number, tipset_key),
        )
    }
    fn Filecoin_StateMinerPower(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<MinerPower, Self::Error> {
        self.call("Filecoin.StateMinerPower", (address, tipset_key))
    }
    fn Filecoin_StateMinerDeadlines(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Option<Vec<ApiDeadline>>, Self::Error> {
        self.call("Filecoin.StateMinerDeadlines", (address, tipset_key))
    }
    fn Filecoin_StateMinerProvingDeadline(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<ApiDeadlineInfo, Self::Error> {
        self.call("Filecoin.StateMinerProvingDeadline", (address, tipset_key))
    }
    fn Filecoin_StateMinerFaults(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<BitField, Self::Error> {
        self.call("Filecoin.StateMinerFaults", (address, tipset_key))
    }
    fn Filecoin_StateMinerRecoveries(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<BitField, Self::Error> {
        self.call("Filecoin.StateMinerRecoveries", (address, tipset_key))
    }
    fn Filecoin_StateMinerAvailableBalance(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<BigInt, Self::Error> {
        self.call("Filecoin.StateMinerAvailableBalance", (address, tipset_key))
    }
    fn Filecoin_StateMinerInitialPledgeCollateral(
        &mut self,
        address: &Address,
        sector_pre_commit_info: &SectorPreCommitInfo,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<BigInt, Self::Error> {
        self.call(
            "Filecoin.StateMinerInitialPledgeCollateral",
            (address, sector_pre_commit_info, tipset_key),
        )
    }
    fn Filecoin_StateGetRandomnessFromTickets(
        &mut self,
        personalization: i64,
        rand_epoch: i64,
        entropy: &Base64String,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Base64String, Self::Error> {
        self.call(
            "Filecoin.StateGetRandomnessFromTickets",
            (personalization, rand_epoch, entropy, tipset_key),
        )
    }
    fn Filecoin_StateGetRandomnessFromBeacon(
        &mut self,
        personalization: i64,
        rand_epoch: i64,
        entropy: &Base64String,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Base64String, Self::Error> {
        self.call(
            "Filecoin.StateGetRandomnessFromBeacon",
            (personalization, rand_epoch, entropy, tipset_key),
        )
    }
    fn Filecoin_StateReadState(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<ApiActorState, Self::Error> {
        self.call("Filecoin.StateReadState", (address, tipset_key))
    }
    fn Filecoin_StateCirculatingSupply(
        &mut self,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<BigInt, Self::Error> {
        self.call("Filecoin.StateCirculatingSupply", (tipset_key,))
    }
    fn Filecoin_StateVerifiedClientStatus(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Option<BigInt>, Self::Error> {
        self.call("Filecoin.StateVerifiedClientStatus", (address, tipset_key))
    }
    fn Filecoin_StateVMCirculatingSupplyInternal(
        &mut self,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<CirculatingSupply, Self::Error> {
        self.call("Filecoin.StateVMCirculatingSupplyInternal", (tipset_key,))
    }
    fn Filecoin_StateListMiners(
        &mut self,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Option<Vec<Address>>, Self::Error> {
        self.call("Filecoin.StateListMiners", (tipset_key,))
    }
    fn Filecoin_StateListActors(
        &mut self,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Option<Vec<Address>>, Self::Error> {
        self.call("Filecoin.StateListActors", (tipset_key,))
    }
    fn Filecoin_StateMarketBalance(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<MarketBalance, Self::Error> {
        self.call("Filecoin.StateMarketBalance", (address, tipset_key))
    }
    fn Filecoin_StateMarketParticipants(
        &mut self,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<std::collections::HashMap<String, MarketBalance>, Self::Error> {
        self.call("Filecoin.StateMarketParticipants", (tipset_key,))
    }
    fn Filecoin_StateMarketDeals(
        &mut self,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<std::collections::HashMap<String, ApiMarketDeal>, Self::Error> {
        self.call("Filecoin.StateMarketDeals", (tipset_key,))
    }
    fn Filecoin_StateDealProviderCollateralBounds(
        &mut self,
        size: u64,
        verified: bool,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<DealCollateralBounds, Self::Error> {
        self.call(
            "Filecoin.StateDealProviderCollateralBounds",
            (size, verified, tipset_key),
        )
    }
    fn Filecoin_StateMarketStorageDeal(
        &mut self,
        deal_id: u64,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<ApiMarketDeal, Self::Error> {
        self.call("Filecoin.StateMarketStorageDeal", (deal_id, tipset_key))
    }
    fn Filecoin_StateSearchMsg(&mut self, message_cid: &Cid) -> Result<MessageLookup, Self::Error> {
        self.call("Filecoin.StateSearchMsg", (message_cid,))
    }
    fn Filecoin_StateMinerPreCommitDepositForPower(
        &mut self,
        address: &Address,
        sector_pre_commit_info: &SectorPreCommitInfo,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<BigInt, Self::Error> {
        self.call(
            "Filecoin.StateMinerPreCommitDepositForPower",
            (address, sector_pre_commit_info, tipset_key),
        )
    }
    fn Filecoin_StateVerifiedRegistryRootKey(
        &mut self,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Address, Self::Error> {
        self.call("Filecoin.StateVerifiedRegistryRootKey", (tipset_key,))
    }
    fn Filecoin_StateVerifierStatus(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Option<BigInt>, Self::Error> {
        self.call("Filecoin.StateVerifierStatus", (address, tipset_key))
    }
    fn Filecoin_StateGetClaim(
        &mut self,
        address: &Address,
        claim_id: u64,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Option<ClaimLotusJson>, Self::Error> {
        self.call("Filecoin.StateGetClaim", (address, claim_id, tipset_key))
    }
    fn Filecoin_StateGetClaims(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<std::collections::HashMap<String, ClaimLotusJson>, Self::Error> {
        self.call("Filecoin.StateGetClaims", (address, tipset_key))
    }
    fn Filecoin_StateGetAllClaims(
        &mut self,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<std::collections::HashMap<String, ClaimLotusJson>, Self::Error> {
        self.call("Filecoin.StateGetAllClaims", (tipset_key,))
    }
    fn Filecoin_StateGetAllocation(
        &mut self,
        address: &Address,
        allocation_id: u64,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<Option<Allocation>, Self::Error> {
        self.call(
            "Filecoin.StateGetAllocation",
            (address, allocation_id, tipset_key),
        )
    }
    fn Filecoin_StateGetAllocations(
        &mut self,
        address: &Address,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<std::collections::HashMap<String, Allocation>, Self::Error> {
        self.call("Filecoin.StateGetAllocations", (address, tipset_key))
    }
    fn Filecoin_StateGetAllAllocations(
        &mut self,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<std::collections::HashMap<String, Allocation>, Self::Error> {
        self.call("Filecoin.StateGetAllAllocations", (tipset_key,))
    }
    fn Filecoin_StateSectorExpiration(
        &mut self,
        miner_address: &Address,
        sector_number: u64,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<SectorExpiration, Self::Error> {
        self.call(
            "Filecoin.StateSectorExpiration",
            (miner_address, sector_number, tipset_key),
        )
    }
    fn Filecoin_StateSectorPartition(
        &mut self,
        miner_address: &Address,
        sector_number: u64,
        tipset_key: Option<&Vec<Cid>>,
    ) -> Result<SectorLocation, Self::Error> {
        self.call(
            "Filecoin.StateSectorPartition",
            (miner_address, sector_number, tipset_key),
        )
    }
    fn Filecoin_WalletBalance(&mut self, address: &Address) -> Result<BigInt, Self::Error> {
        self.call("Filecoin.WalletBalance", (address,))
    }
    fn Filecoin_WalletHas(&mut self, address: &Address) -> Result<bool, Self::Error> {
        self.call("Filecoin.WalletHas", (address,))
    }
    fn Filecoin_WalletList(&mut self) -> Result<Option<Vec<Address>>, Self::Error> {
        self.call("Filecoin.WalletList", ())
    }
    fn Filecoin_WalletSign(
        &mut self,
        address: &Address,
        message: &Base64String,
    ) -> Result<Signature, Self::Error> {
        self.call("Filecoin.WalletSign", (address, message))
    }
    fn Filecoin_WalletSignMessage(
        &mut self,
        address: &Address,
        message: &Message,
    ) -> Result<SignedMessage, Self::Error> {
        self.call("Filecoin.WalletSignMessage", (address, message))
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
#[doc = "ActorState"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Balance\","]
#[doc = "    \"Code\","]
#[doc = "    \"Head\","]
#[doc = "    \"Nonce\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Address\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Address\""]
#[doc = "    },"]
#[doc = "    \"Balance\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"Code\": {"]
#[doc = "      \"$ref\": \"#/definitions/Cid\""]
#[doc = "    },"]
#[doc = "    \"Head\": {"]
#[doc = "      \"$ref\": \"#/definitions/Cid\""]
#[doc = "    },"]
#[doc = "    \"Nonce\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActorState {
    #[serde(rename = "Address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<NullableAddress>,
    #[serde(rename = "Balance")]
    pub balance: BigInt,
    #[serde(rename = "Code")]
    pub code: Cid,
    #[serde(rename = "Head")]
    pub head: Cid,
    #[serde(rename = "Nonce")]
    pub nonce: u64,
}
impl From<&ActorState> for ActorState {
    fn from(value: &ActorState) -> Self {
        value.clone()
    }
}
#[doc = "ActorTrace"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Id\","]
#[doc = "    \"State\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Id\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"State\": {"]
#[doc = "      \"$ref\": \"#/definitions/ActorState\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActorTrace {
    #[serde(rename = "Id")]
    pub id: u64,
    #[serde(rename = "State")]
    pub state: ActorState,
}
impl From<&ActorTrace> for ActorTrace {
    fn from(value: &ActorTrace) -> Self {
        value.clone()
    }
}
#[doc = "AddrInfo"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Addrs\","]
#[doc = "    \"ID\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Addrs\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"ID\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddrInfo {
    #[serde(rename = "Addrs")]
    pub addrs: Vec<String>,
    #[serde(rename = "ID")]
    pub id: String,
}
impl From<&AddrInfo> for AddrInfo {
    fn from(value: &AddrInfo) -> Self {
        value.clone()
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
#[doc = "ApiActorState"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Balance\","]
#[doc = "    \"Code\","]
#[doc = "    \"State\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Balance\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"Code\": {"]
#[doc = "      \"$ref\": \"#/definitions/Cid\""]
#[doc = "    },"]
#[doc = "    \"State\": {"]
#[doc = "      \"$ref\": \"#/definitions/ApiState\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiActorState {
    #[serde(rename = "Balance")]
    pub balance: BigInt,
    #[serde(rename = "Code")]
    pub code: Cid,
    #[serde(rename = "State")]
    pub state: ApiState,
}
impl From<&ApiActorState> for ApiActorState {
    fn from(value: &ApiActorState) -> Self {
        value.clone()
    }
}
#[doc = "ApiDeadline"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"DisputableProofCount\","]
#[doc = "    \"PostSubmissions\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"DisputableProofCount\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"PostSubmissions\": {"]
#[doc = "      \"$ref\": \"#/definitions/BitField\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiDeadline {
    #[serde(rename = "DisputableProofCount")]
    pub disputable_proof_count: u64,
    #[serde(rename = "PostSubmissions")]
    pub post_submissions: BitField,
}
impl From<&ApiDeadline> for ApiDeadline {
    fn from(value: &ApiDeadline) -> Self {
        value.clone()
    }
}
#[doc = "ApiDeadlineInfo"]
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
pub struct ApiDeadlineInfo(pub String);
impl std::ops::Deref for ApiDeadlineInfo {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ApiDeadlineInfo> for String {
    fn from(value: ApiDeadlineInfo) -> Self {
        value.0
    }
}
impl From<&ApiDeadlineInfo> for ApiDeadlineInfo {
    fn from(value: &ApiDeadlineInfo) -> Self {
        value.clone()
    }
}
impl From<String> for ApiDeadlineInfo {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for ApiDeadlineInfo {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for ApiDeadlineInfo {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "ApiDealProposal"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Client\","]
#[doc = "    \"ClientCollateral\","]
#[doc = "    \"EndEpoch\","]
#[doc = "    \"Label\","]
#[doc = "    \"PieceCID\","]
#[doc = "    \"PieceSize\","]
#[doc = "    \"Provider\","]
#[doc = "    \"ProviderCollateral\","]
#[doc = "    \"StartEpoch\","]
#[doc = "    \"StoragePricePerEpoch\","]
#[doc = "    \"VerifiedDeal\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Client\": {"]
#[doc = "      \"$ref\": \"#/definitions/Address\""]
#[doc = "    },"]
#[doc = "    \"ClientCollateral\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"EndEpoch\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"Label\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"PieceCID\": {"]
#[doc = "      \"$ref\": \"#/definitions/Cid\""]
#[doc = "    },"]
#[doc = "    \"PieceSize\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Provider\": {"]
#[doc = "      \"$ref\": \"#/definitions/Address\""]
#[doc = "    },"]
#[doc = "    \"ProviderCollateral\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"StartEpoch\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"StoragePricePerEpoch\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"VerifiedDeal\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiDealProposal {
    #[serde(rename = "Client")]
    pub client: Address,
    #[serde(rename = "ClientCollateral")]
    pub client_collateral: BigInt,
    #[serde(rename = "EndEpoch")]
    pub end_epoch: i64,
    #[serde(rename = "Label")]
    pub label: String,
    #[serde(rename = "PieceCID")]
    pub piece_cid: Cid,
    #[serde(rename = "PieceSize")]
    pub piece_size: u64,
    #[serde(rename = "Provider")]
    pub provider: Address,
    #[serde(rename = "ProviderCollateral")]
    pub provider_collateral: BigInt,
    #[serde(rename = "StartEpoch")]
    pub start_epoch: i64,
    #[serde(rename = "StoragePricePerEpoch")]
    pub storage_price_per_epoch: BigInt,
    #[serde(rename = "VerifiedDeal")]
    pub verified_deal: bool,
}
impl From<&ApiDealProposal> for ApiDealProposal {
    fn from(value: &ApiDealProposal) -> Self {
        value.clone()
    }
}
#[doc = "ApiDealState"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"LastUpdatedEpoch\","]
#[doc = "    \"SectorStartEpoch\","]
#[doc = "    \"SlashEpoch\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"LastUpdatedEpoch\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"SectorStartEpoch\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"SlashEpoch\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiDealState {
    #[serde(rename = "LastUpdatedEpoch")]
    pub last_updated_epoch: i64,
    #[serde(rename = "SectorStartEpoch")]
    pub sector_start_epoch: i64,
    #[serde(rename = "SlashEpoch")]
    pub slash_epoch: i64,
}
impl From<&ApiDealState> for ApiDealState {
    fn from(value: &ApiDealState) -> Self {
        value.clone()
    }
}
#[doc = "ApiInvocResult"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Duration\","]
#[doc = "    \"Error\","]
#[doc = "    \"GasCost\","]
#[doc = "    \"Msg\","]
#[doc = "    \"MsgCid\","]
#[doc = "    \"MsgRct\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Duration\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Error\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ExecutionTrace\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ExecutionTrace\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"GasCost\": {"]
#[doc = "      \"$ref\": \"#/definitions/MessageGasCost\""]
#[doc = "    },"]
#[doc = "    \"Msg\": {"]
#[doc = "      \"$ref\": \"#/definitions/Message\""]
#[doc = "    },"]
#[doc = "    \"MsgCid\": {"]
#[doc = "      \"$ref\": \"#/definitions/Cid\""]
#[doc = "    },"]
#[doc = "    \"MsgRct\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Receipt\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiInvocResult {
    #[serde(rename = "Duration")]
    pub duration: u64,
    #[serde(rename = "Error")]
    pub error: String,
    #[serde(
        rename = "ExecutionTrace",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_trace: Option<ExecutionTrace>,
    #[serde(rename = "GasCost")]
    pub gas_cost: MessageGasCost,
    #[serde(rename = "Msg")]
    pub msg: Message,
    #[serde(rename = "MsgCid")]
    pub msg_cid: Cid,
    #[serde(rename = "MsgRct")]
    pub msg_rct: NullableReceipt,
}
impl From<&ApiInvocResult> for ApiInvocResult {
    fn from(value: &ApiInvocResult) -> Self {
        value.clone()
    }
}
#[doc = "ApiMarketDeal"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Proposal\","]
#[doc = "    \"State\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Proposal\": {"]
#[doc = "      \"$ref\": \"#/definitions/ApiDealProposal\""]
#[doc = "    },"]
#[doc = "    \"State\": {"]
#[doc = "      \"$ref\": \"#/definitions/ApiDealState\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiMarketDeal {
    #[serde(rename = "Proposal")]
    pub proposal: ApiDealProposal,
    #[serde(rename = "State")]
    pub state: ApiDealState,
}
impl From<&ApiMarketDeal> for ApiMarketDeal {
    fn from(value: &ApiMarketDeal) -> Self {
        value.clone()
    }
}
#[doc = "ApiMessage"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Cid\","]
#[doc = "    \"Message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Cid\": {"]
#[doc = "      \"$ref\": \"#/definitions/Cid\""]
#[doc = "    },"]
#[doc = "    \"Message\": {"]
#[doc = "      \"$ref\": \"#/definitions/Message\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiMessage {
    #[serde(rename = "Cid")]
    pub cid: Cid,
    #[serde(rename = "Message")]
    pub message: Message,
}
impl From<&ApiMessage> for ApiMessage {
    fn from(value: &ApiMessage) -> Self {
        value.clone()
    }
}
#[doc = "ApiReceipt"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"EventsRoot\","]
#[doc = "    \"ExitCode\","]
#[doc = "    \"GasUsed\","]
#[doc = "    \"Return\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"EventsRoot\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Cid\""]
#[doc = "    },"]
#[doc = "    \"ExitCode\": {"]
#[doc = "      \"$ref\": \"#/definitions/ExitCode\""]
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
pub struct ApiReceipt {
    #[serde(rename = "EventsRoot")]
    pub events_root: NullableCid,
    #[serde(rename = "ExitCode")]
    pub exit_code: ExitCode,
    #[serde(rename = "GasUsed")]
    pub gas_used: u64,
    #[serde(rename = "Return")]
    pub return_: Base64String,
}
impl From<&ApiReceipt> for ApiReceipt {
    fn from(value: &ApiReceipt) -> Self {
        value.clone()
    }
}
#[doc = "ApiState"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BuiltinActors\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BuiltinActors\": true"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiState {
    #[serde(rename = "BuiltinActors")]
    pub builtin_actors: serde_json::Value,
}
impl From<&ApiState> for ApiState {
    fn from(value: &ApiState) -> Self {
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
#[doc = "BeneficiaryTerm"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Expiration\","]
#[doc = "    \"Quota\","]
#[doc = "    \"UsedQuota\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Expiration\": {"]
#[doc = "      \"description\": \"The epoch at which the beneficiary's rights expire and revert to the owner\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"Quota\": {"]
#[doc = "      \"description\": \"The total amount the current beneficiary can withdraw. Monotonic, but reset when beneficiary changes.\","]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"UsedQuota\": {"]
#[doc = "      \"description\": \"The amount of quota the current beneficiary has already withdrawn\","]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BeneficiaryTerm {
    #[doc = "The epoch at which the beneficiary's rights expire and revert to the owner"]
    #[serde(rename = "Expiration")]
    pub expiration: i64,
    #[doc = "The total amount the current beneficiary can withdraw. Monotonic, but reset when beneficiary changes."]
    #[serde(rename = "Quota")]
    pub quota: BigInt,
    #[doc = "The amount of quota the current beneficiary has already withdrawn"]
    #[serde(rename = "UsedQuota")]
    pub used_quota: BigInt,
}
impl From<&BeneficiaryTerm> for BeneficiaryTerm {
    fn from(value: &BeneficiaryTerm) -> Self {
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
#[doc = "BlockMessage"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BlsMessages\","]
#[doc = "    \"Header\","]
#[doc = "    \"SecpkMessages\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BlsMessages\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_Cid\""]
#[doc = "    },"]
#[doc = "    \"Header\": {"]
#[doc = "      \"$ref\": \"#/definitions/BlockHeader\""]
#[doc = "    },"]
#[doc = "    \"SecpkMessages\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_Cid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockMessage {
    #[serde(rename = "BlsMessages")]
    pub bls_messages: NullableArrayOfCid,
    #[serde(rename = "Header")]
    pub header: BlockHeader,
    #[serde(rename = "SecpkMessages")]
    pub secpk_messages: NullableArrayOfCid,
}
impl From<&BlockMessage> for BlockMessage {
    fn from(value: &BlockMessage) -> Self {
        value.clone()
    }
}
#[doc = "BlockMessages"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BlsMessages\","]
#[doc = "    \"Cids\","]
#[doc = "    \"SecpkMessages\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BlsMessages\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_Message\""]
#[doc = "    },"]
#[doc = "    \"Cids\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_Cid\""]
#[doc = "    },"]
#[doc = "    \"SecpkMessages\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_SignedMessage\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockMessages {
    #[serde(rename = "BlsMessages")]
    pub bls_messages: NullableArrayOfMessage,
    #[serde(rename = "Cids")]
    pub cids: NullableArrayOfCid,
    #[serde(rename = "SecpkMessages")]
    pub secpk_messages: NullableArrayOfSignedMessage,
}
impl From<&BlockMessages> for BlockMessages {
    fn from(value: &BlockMessages) -> Self {
        value.clone()
    }
}
#[doc = "BlockTemplate"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BeaconValues\","]
#[doc = "    \"Epoch\","]
#[doc = "    \"Eproof\","]
#[doc = "    \"Messages\","]
#[doc = "    \"Miner\","]
#[doc = "    \"Parents\","]
#[doc = "    \"Ticket\","]
#[doc = "    \"Timestamp\","]
#[doc = "    \"WinningPoStProof\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BeaconValues\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_BeaconEntry\""]
#[doc = "    },"]
#[doc = "    \"Epoch\": {"]
#[doc = "      \"$ref\": \"#/definitions/int64\""]
#[doc = "    },"]
#[doc = "    \"Eproof\": {"]
#[doc = "      \"$ref\": \"#/definitions/ElectionProof\""]
#[doc = "    },"]
#[doc = "    \"Messages\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_SignedMessage\""]
#[doc = "    },"]
#[doc = "    \"Miner\": {"]
#[doc = "      \"$ref\": \"#/definitions/Address\""]
#[doc = "    },"]
#[doc = "    \"Parents\": {"]
#[doc = "      \"$ref\": \"#/definitions/NonEmpty_Array_of_Cid\""]
#[doc = "    },"]
#[doc = "    \"Ticket\": {"]
#[doc = "      \"$ref\": \"#/definitions/Ticket\""]
#[doc = "    },"]
#[doc = "    \"Timestamp\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"WinningPoStProof\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_PoStProof\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockTemplate {
    #[serde(rename = "BeaconValues")]
    pub beacon_values: NullableArrayOfBeaconEntry,
    #[serde(rename = "Epoch")]
    pub epoch: Int64,
    #[serde(rename = "Eproof")]
    pub eproof: ElectionProof,
    #[serde(rename = "Messages")]
    pub messages: NullableArrayOfSignedMessage,
    #[serde(rename = "Miner")]
    pub miner: Address,
    #[serde(rename = "Parents")]
    pub parents: NonEmptyArrayOfCid,
    #[serde(rename = "Ticket")]
    pub ticket: Ticket,
    #[serde(rename = "Timestamp")]
    pub timestamp: u64,
    #[serde(rename = "WinningPoStProof")]
    pub winning_po_st_proof: NullableArrayOfPoStProof,
}
impl From<&BlockTemplate> for BlockTemplate {
    fn from(value: &BlockTemplate) -> Self {
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
#[doc = "CirculatingSupply"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"FilBurnt\","]
#[doc = "    \"FilCirculating\","]
#[doc = "    \"FilLocked\","]
#[doc = "    \"FilMined\","]
#[doc = "    \"FilReserveDisbursed\","]
#[doc = "    \"FilVested\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"FilBurnt\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"FilCirculating\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"FilLocked\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"FilMined\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"FilReserveDisbursed\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"FilVested\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CirculatingSupply {
    #[serde(rename = "FilBurnt")]
    pub fil_burnt: BigInt,
    #[serde(rename = "FilCirculating")]
    pub fil_circulating: BigInt,
    #[serde(rename = "FilLocked")]
    pub fil_locked: BigInt,
    #[serde(rename = "FilMined")]
    pub fil_mined: BigInt,
    #[serde(rename = "FilReserveDisbursed")]
    pub fil_reserve_disbursed: BigInt,
    #[serde(rename = "FilVested")]
    pub fil_vested: BigInt,
}
impl From<&CirculatingSupply> for CirculatingSupply {
    fn from(value: &CirculatingSupply) -> Self {
        value.clone()
    }
}
#[doc = "Claim"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"QualityAdjPower\","]
#[doc = "    \"RawBytePower\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"QualityAdjPower\": {"]
#[doc = "      \"description\": \"Sum of quality adjusted power for a miner's sectors.\","]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"RawBytePower\": {"]
#[doc = "      \"description\": \"Sum of raw byte power for a miner's sectors.\","]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Claim {
    #[doc = "Sum of quality adjusted power for a miner's sectors."]
    #[serde(rename = "QualityAdjPower")]
    pub quality_adj_power: BigInt,
    #[doc = "Sum of raw byte power for a miner's sectors."]
    #[serde(rename = "RawBytePower")]
    pub raw_byte_power: BigInt,
}
impl From<&Claim> for Claim {
    fn from(value: &Claim) -> Self {
        value.clone()
    }
}
#[doc = "ClaimLotusJson"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Client\","]
#[doc = "    \"Data\","]
#[doc = "    \"Provider\","]
#[doc = "    \"Sector\","]
#[doc = "    \"Size\","]
#[doc = "    \"TermMax\","]
#[doc = "    \"TermMin\","]
#[doc = "    \"TermStart\""]
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
#[doc = "    \"Provider\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Sector\": {"]
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
#[doc = "    },"]
#[doc = "    \"TermStart\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClaimLotusJson {
    #[serde(rename = "Client")]
    pub client: u64,
    #[serde(rename = "Data")]
    pub data: Cid,
    #[serde(rename = "Provider")]
    pub provider: u64,
    #[serde(rename = "Sector")]
    pub sector: u64,
    #[serde(rename = "Size")]
    pub size: u64,
    #[serde(rename = "TermMax")]
    pub term_max: i64,
    #[serde(rename = "TermMin")]
    pub term_min: i64,
    #[serde(rename = "TermStart")]
    pub term_start: i64,
}
impl From<&ClaimLotusJson> for ClaimLotusJson {
    fn from(value: &ClaimLotusJson) -> Self {
        value.clone()
    }
}
#[doc = "DealCollateralBounds"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Max\","]
#[doc = "    \"Min\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Max\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"Min\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DealCollateralBounds {
    #[serde(rename = "Max")]
    pub max: BigInt,
    #[serde(rename = "Min")]
    pub min: BigInt,
}
impl From<&DealCollateralBounds> for DealCollateralBounds {
    fn from(value: &DealCollateralBounds) -> Self {
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
#[doc = "ExecutionTrace"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"GasCharges\","]
#[doc = "    \"Msg\","]
#[doc = "    \"MsgRct\","]
#[doc = "    \"Subcalls\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"GasCharges\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/GasTrace\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"InvokedActor\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ActorTrace\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"Msg\": {"]
#[doc = "      \"$ref\": \"#/definitions/MessageTrace\""]
#[doc = "    },"]
#[doc = "    \"MsgRct\": {"]
#[doc = "      \"$ref\": \"#/definitions/ReturnTrace\""]
#[doc = "    },"]
#[doc = "    \"Subcalls\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_ExecutionTrace\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExecutionTrace {
    #[serde(rename = "GasCharges")]
    pub gas_charges: Vec<GasTrace>,
    #[serde(
        rename = "InvokedActor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub invoked_actor: Option<ActorTrace>,
    #[serde(rename = "Msg")]
    pub msg: MessageTrace,
    #[serde(rename = "MsgRct")]
    pub msg_rct: ReturnTrace,
    #[serde(rename = "Subcalls")]
    pub subcalls: NullableArrayOfExecutionTrace,
}
impl From<&ExecutionTrace> for ExecutionTrace {
    fn from(value: &ExecutionTrace) -> Self {
        value.clone()
    }
}
#[doc = "`Newtype` wrapper for the FVM `ExitCode`.\n\n# Examples ``` # use forest_filecoin::doctest_private::ExitCode; let fvm2_success = fvm_shared2::error::ExitCode::new(0); let fvm3_success = fvm_shared3::error::ExitCode::new(0);\n\nlet shim_from_v2 = ExitCode::from(fvm2_success); let shim_from_v3 = ExitCode::from(fvm3_success);\n\nassert_eq!(shim_from_v2, shim_from_v3); assert_eq!(shim_from_v2, fvm2_success.into()); assert_eq!(shim_from_v3, fvm3_success.into()); ```"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"`Newtype` wrapper for the FVM `ExitCode`.\\n\\n# Examples ``` # use forest_filecoin::doctest_private::ExitCode; let fvm2_success = fvm_shared2::error::ExitCode::new(0); let fvm3_success = fvm_shared3::error::ExitCode::new(0);\\n\\nlet shim_from_v2 = ExitCode::from(fvm2_success); let shim_from_v3 = ExitCode::from(fvm3_success);\\n\\nassert_eq!(shim_from_v2, shim_from_v3); assert_eq!(shim_from_v2, fvm2_success.into()); assert_eq!(shim_from_v3, fvm3_success.into()); ```\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"uint32\","]
#[doc = "  \"minimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExitCode(pub u32);
impl std::ops::Deref for ExitCode {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}
impl From<ExitCode> for u32 {
    fn from(value: ExitCode) -> Self {
        value.0
    }
}
impl From<&ExitCode> for ExitCode {
    fn from(value: &ExitCode) -> Self {
        value.clone()
    }
}
impl From<u32> for ExitCode {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for ExitCode {
    type Err = <u32 as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for ExitCode {
    type Error = <u32 as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ExitCode {
    type Error = <u32 as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ExitCode {
    type Error = <u32 as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for ExitCode {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "ForkUpgradeParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BreezeGasTampingDuration\","]
#[doc = "    \"UpgradeAssemblyHeight\","]
#[doc = "    \"UpgradeBreezeHeight\","]
#[doc = "    \"UpgradeCalicoHeight\","]
#[doc = "    \"UpgradeChocolateHeight\","]
#[doc = "    \"UpgradeClausHeight\","]
#[doc = "    \"UpgradeDragonHeight\","]
#[doc = "    \"UpgradeHyggeHeight\","]
#[doc = "    \"UpgradeHyperdriveHeight\","]
#[doc = "    \"UpgradeIgnitionHeight\","]
#[doc = "    \"UpgradeKumquatHeight\","]
#[doc = "    \"UpgradeLiftoffHeight\","]
#[doc = "    \"UpgradeLightningHeight\","]
#[doc = "    \"UpgradeNorwegianHeight\","]
#[doc = "    \"UpgradeOhSnapHeight\","]
#[doc = "    \"UpgradeOrangeHeight\","]
#[doc = "    \"UpgradePersianHeight\","]
#[doc = "    \"UpgradePhoenixHeight\","]
#[doc = "    \"UpgradeRefuelHeight\","]
#[doc = "    \"UpgradeSharkHeight\","]
#[doc = "    \"UpgradeSkyrHeight\","]
#[doc = "    \"UpgradeSmokeHeight\","]
#[doc = "    \"UpgradeTapeHeight\","]
#[doc = "    \"UpgradeThunderHeight\","]
#[doc = "    \"UpgradeTrustHeight\","]
#[doc = "    \"UpgradeTurboHeight\","]
#[doc = "    \"UpgradeWaffleHeight\","]
#[doc = "    \"UpgradeWatermelonHeight\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BreezeGasTampingDuration\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeAssemblyHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeBreezeHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeCalicoHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeChocolateHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeClausHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeDragonHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeHyggeHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeHyperdriveHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeIgnitionHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeKumquatHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeLiftoffHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeLightningHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeNorwegianHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeOhSnapHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeOrangeHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradePersianHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradePhoenixHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeRefuelHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeSharkHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeSkyrHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeSmokeHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeTapeHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeThunderHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeTrustHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeTurboHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeWaffleHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"UpgradeWatermelonHeight\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ForkUpgradeParams {
    #[serde(rename = "BreezeGasTampingDuration")]
    pub breeze_gas_tamping_duration: i64,
    #[serde(rename = "UpgradeAssemblyHeight")]
    pub upgrade_assembly_height: i64,
    #[serde(rename = "UpgradeBreezeHeight")]
    pub upgrade_breeze_height: i64,
    #[serde(rename = "UpgradeCalicoHeight")]
    pub upgrade_calico_height: i64,
    #[serde(rename = "UpgradeChocolateHeight")]
    pub upgrade_chocolate_height: i64,
    #[serde(rename = "UpgradeClausHeight")]
    pub upgrade_claus_height: i64,
    #[serde(rename = "UpgradeDragonHeight")]
    pub upgrade_dragon_height: i64,
    #[serde(rename = "UpgradeHyggeHeight")]
    pub upgrade_hygge_height: i64,
    #[serde(rename = "UpgradeHyperdriveHeight")]
    pub upgrade_hyperdrive_height: i64,
    #[serde(rename = "UpgradeIgnitionHeight")]
    pub upgrade_ignition_height: i64,
    #[serde(rename = "UpgradeKumquatHeight")]
    pub upgrade_kumquat_height: i64,
    #[serde(rename = "UpgradeLiftoffHeight")]
    pub upgrade_liftoff_height: i64,
    #[serde(rename = "UpgradeLightningHeight")]
    pub upgrade_lightning_height: i64,
    #[serde(rename = "UpgradeNorwegianHeight")]
    pub upgrade_norwegian_height: i64,
    #[serde(rename = "UpgradeOhSnapHeight")]
    pub upgrade_oh_snap_height: i64,
    #[serde(rename = "UpgradeOrangeHeight")]
    pub upgrade_orange_height: i64,
    #[serde(rename = "UpgradePersianHeight")]
    pub upgrade_persian_height: i64,
    #[serde(rename = "UpgradePhoenixHeight")]
    pub upgrade_phoenix_height: i64,
    #[serde(rename = "UpgradeRefuelHeight")]
    pub upgrade_refuel_height: i64,
    #[serde(rename = "UpgradeSharkHeight")]
    pub upgrade_shark_height: i64,
    #[serde(rename = "UpgradeSkyrHeight")]
    pub upgrade_skyr_height: i64,
    #[serde(rename = "UpgradeSmokeHeight")]
    pub upgrade_smoke_height: i64,
    #[serde(rename = "UpgradeTapeHeight")]
    pub upgrade_tape_height: i64,
    #[serde(rename = "UpgradeThunderHeight")]
    pub upgrade_thunder_height: i64,
    #[serde(rename = "UpgradeTrustHeight")]
    pub upgrade_trust_height: i64,
    #[serde(rename = "UpgradeTurboHeight")]
    pub upgrade_turbo_height: i64,
    #[serde(rename = "UpgradeWaffleHeight")]
    pub upgrade_waffle_height: i64,
    #[serde(rename = "UpgradeWatermelonHeight")]
    pub upgrade_watermelon_height: i64,
}
impl From<&ForkUpgradeParams> for ForkUpgradeParams {
    fn from(value: &ForkUpgradeParams) -> Self {
        value.clone()
    }
}
#[doc = "GasTrace"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Name\","]
#[doc = "    \"cg\","]
#[doc = "    \"sg\","]
#[doc = "    \"tg\","]
#[doc = "    \"tt\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"cg\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"sg\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"tg\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"tt\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GasTrace {
    pub cg: u64,
    #[serde(rename = "Name")]
    pub name: String,
    pub sg: u64,
    pub tg: u64,
    pub tt: u64,
}
impl From<&GasTrace> for GasTrace {
    fn from(value: &GasTrace) -> Self {
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
#[doc = "External format for returning market balance from state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"External format for returning market balance from state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Escrow\","]
#[doc = "    \"Locked\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Escrow\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"Locked\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketBalance {
    #[serde(rename = "Escrow")]
    pub escrow: BigInt,
    #[serde(rename = "Locked")]
    pub locked: BigInt,
}
impl From<&MarketBalance> for MarketBalance {
    fn from(value: &MarketBalance) -> Self {
        value.clone()
    }
}
#[doc = "Message"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"From\","]
#[doc = "    \"To\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"From\": {"]
#[doc = "      \"$ref\": \"#/definitions/Address\""]
#[doc = "    },"]
#[doc = "    \"GasFeeCap\": {"]
#[doc = "      \"default\": \"0\","]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"GasLimit\": {"]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"GasPremium\": {"]
#[doc = "      \"default\": \"0\","]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"Method\": {"]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Nonce\": {"]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Params\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Base64String\""]
#[doc = "    },"]
#[doc = "    \"To\": {"]
#[doc = "      \"$ref\": \"#/definitions/Address\""]
#[doc = "    },"]
#[doc = "    \"Value\": {"]
#[doc = "      \"default\": \"0\","]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"Version\": {"]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    #[serde(rename = "From")]
    pub from: Address,
    #[serde(rename = "GasFeeCap", default = "defaults::message_gas_fee_cap")]
    pub gas_fee_cap: BigInt,
    #[serde(rename = "GasLimit", default)]
    pub gas_limit: u64,
    #[serde(rename = "GasPremium", default = "defaults::message_gas_premium")]
    pub gas_premium: BigInt,
    #[serde(rename = "Method", default)]
    pub method: u64,
    #[serde(rename = "Nonce", default)]
    pub nonce: u64,
    #[serde(rename = "Params", default, skip_serializing_if = "Option::is_none")]
    pub params: Option<NullableBase64String>,
    #[serde(rename = "To")]
    pub to: Address,
    #[serde(rename = "Value", default = "defaults::message_value")]
    pub value: BigInt,
    #[serde(rename = "Version", default)]
    pub version: u64,
}
impl From<&Message> for Message {
    fn from(value: &Message) -> Self {
        value.clone()
    }
}
#[doc = "MessageFilter"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"From\","]
#[doc = "    \"To\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"From\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Address\""]
#[doc = "    },"]
#[doc = "    \"To\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Address\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageFilter {
    #[serde(rename = "From")]
    pub from: NullableAddress,
    #[serde(rename = "To")]
    pub to: NullableAddress,
}
impl From<&MessageFilter> for MessageFilter {
    fn from(value: &MessageFilter) -> Self {
        value.clone()
    }
}
#[doc = "MessageGasCost"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BaseFeeBurn\","]
#[doc = "    \"GasUsed\","]
#[doc = "    \"Message\","]
#[doc = "    \"MinerPenalty\","]
#[doc = "    \"MinerTip\","]
#[doc = "    \"OverEstimationBurn\","]
#[doc = "    \"Refund\","]
#[doc = "    \"TotalCost\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BaseFeeBurn\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"GasUsed\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"Message\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Cid\""]
#[doc = "    },"]
#[doc = "    \"MinerPenalty\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"MinerTip\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"OverEstimationBurn\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"Refund\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"TotalCost\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageGasCost {
    #[serde(rename = "BaseFeeBurn")]
    pub base_fee_burn: BigInt,
    #[serde(rename = "GasUsed")]
    pub gas_used: BigInt,
    #[serde(rename = "Message")]
    pub message: NullableCid,
    #[serde(rename = "MinerPenalty")]
    pub miner_penalty: BigInt,
    #[serde(rename = "MinerTip")]
    pub miner_tip: BigInt,
    #[serde(rename = "OverEstimationBurn")]
    pub over_estimation_burn: BigInt,
    #[serde(rename = "Refund")]
    pub refund: BigInt,
    #[serde(rename = "TotalCost")]
    pub total_cost: BigInt,
}
impl From<&MessageGasCost> for MessageGasCost {
    fn from(value: &MessageGasCost) -> Self {
        value.clone()
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
#[doc = "MessageSendSpec"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"MaxFee\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"MaxFee\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageSendSpec {
    #[serde(rename = "MaxFee")]
    pub max_fee: BigInt,
}
impl From<&MessageSendSpec> for MessageSendSpec {
    fn from(value: &MessageSendSpec) -> Self {
        value.clone()
    }
}
#[doc = "MessageTrace"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"From\","]
#[doc = "    \"Method\","]
#[doc = "    \"Params\","]
#[doc = "    \"ParamsCodec\","]
#[doc = "    \"To\","]
#[doc = "    \"Value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"From\": {"]
#[doc = "      \"$ref\": \"#/definitions/Address\""]
#[doc = "    },"]
#[doc = "    \"GasLimit\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Method\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Params\": {"]
#[doc = "      \"$ref\": \"#/definitions/Base64String\""]
#[doc = "    },"]
#[doc = "    \"ParamsCodec\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"ReadOnly\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"To\": {"]
#[doc = "      \"$ref\": \"#/definitions/Address\""]
#[doc = "    },"]
#[doc = "    \"Value\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageTrace {
    #[serde(rename = "From")]
    pub from: Address,
    #[serde(rename = "GasLimit", default, skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<u64>,
    #[serde(rename = "Method")]
    pub method: u64,
    #[serde(rename = "Params")]
    pub params: Base64String,
    #[serde(rename = "ParamsCodec")]
    pub params_codec: u64,
    #[serde(rename = "ReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "To")]
    pub to: Address,
    #[serde(rename = "Value")]
    pub value: BigInt,
}
impl From<&MessageTrace> for MessageTrace {
    fn from(value: &MessageTrace) -> Self {
        value.clone()
    }
}
#[doc = "MinerInfo"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Beneficiary\","]
#[doc = "    \"BeneficiaryTerm\","]
#[doc = "    \"ConsensusFaultElapsed\","]
#[doc = "    \"ControlAddresses\","]
#[doc = "    \"Multiaddrs\","]
#[doc = "    \"NewWorker\","]
#[doc = "    \"Owner\","]
#[doc = "    \"PeerId\","]
#[doc = "    \"PendingBeneficiaryTerm\","]
#[doc = "    \"PendingOwnerAddress\","]
#[doc = "    \"SectorSize\","]
#[doc = "    \"WindowPoStPartitionSectors\","]
#[doc = "    \"WindowPoStProofType\","]
#[doc = "    \"Worker\","]
#[doc = "    \"WorkerChangeEpoch\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Beneficiary\": {"]
#[doc = "      \"$ref\": \"#/definitions/Address\""]
#[doc = "    },"]
#[doc = "    \"BeneficiaryTerm\": {"]
#[doc = "      \"$ref\": \"#/definitions/BeneficiaryTerm\""]
#[doc = "    },"]
#[doc = "    \"ConsensusFaultElapsed\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"ControlAddresses\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_Address\""]
#[doc = "    },"]
#[doc = "    \"Multiaddrs\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_Base64String\""]
#[doc = "    },"]
#[doc = "    \"NewWorker\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Address\""]
#[doc = "    },"]
#[doc = "    \"Owner\": {"]
#[doc = "      \"$ref\": \"#/definitions/Address\""]
#[doc = "    },"]
#[doc = "    \"PeerId\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_String\""]
#[doc = "    },"]
#[doc = "    \"PendingBeneficiaryTerm\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_PendingBeneficiaryChange\""]
#[doc = "    },"]
#[doc = "    \"PendingOwnerAddress\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Address\""]
#[doc = "    },"]
#[doc = "    \"SectorSize\": {"]
#[doc = "      \"$ref\": \"#/definitions/SectorSize\""]
#[doc = "    },"]
#[doc = "    \"WindowPoStPartitionSectors\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"WindowPoStProofType\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Worker\": {"]
#[doc = "      \"$ref\": \"#/definitions/Address\""]
#[doc = "    },"]
#[doc = "    \"WorkerChangeEpoch\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MinerInfo {
    #[serde(rename = "Beneficiary")]
    pub beneficiary: Address,
    #[serde(rename = "BeneficiaryTerm")]
    pub beneficiary_term: BeneficiaryTerm,
    #[serde(rename = "ConsensusFaultElapsed")]
    pub consensus_fault_elapsed: i64,
    #[serde(rename = "ControlAddresses")]
    pub control_addresses: NullableArrayOfAddress,
    #[serde(rename = "Multiaddrs")]
    pub multiaddrs: NullableArrayOfBase64String,
    #[serde(rename = "NewWorker")]
    pub new_worker: NullableAddress,
    #[serde(rename = "Owner")]
    pub owner: Address,
    #[serde(rename = "PeerId")]
    pub peer_id: NullableString,
    #[serde(rename = "PendingBeneficiaryTerm")]
    pub pending_beneficiary_term: NullablePendingBeneficiaryChange,
    #[serde(rename = "PendingOwnerAddress")]
    pub pending_owner_address: NullableAddress,
    #[serde(rename = "SectorSize")]
    pub sector_size: SectorSize,
    #[serde(rename = "WindowPoStPartitionSectors")]
    pub window_po_st_partition_sectors: u64,
    #[serde(rename = "WindowPoStProofType")]
    pub window_po_st_proof_type: String,
    #[serde(rename = "Worker")]
    pub worker: Address,
    #[serde(rename = "WorkerChangeEpoch")]
    pub worker_change_epoch: i64,
}
impl From<&MinerInfo> for MinerInfo {
    fn from(value: &MinerInfo) -> Self {
        value.clone()
    }
}
#[doc = "MinerPartitions"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ActiveSectors\","]
#[doc = "    \"AllSectors\","]
#[doc = "    \"FaultySectors\","]
#[doc = "    \"LiveSectors\","]
#[doc = "    \"RecoveringSectors\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ActiveSectors\": {"]
#[doc = "      \"$ref\": \"#/definitions/BitField\""]
#[doc = "    },"]
#[doc = "    \"AllSectors\": {"]
#[doc = "      \"$ref\": \"#/definitions/BitField\""]
#[doc = "    },"]
#[doc = "    \"FaultySectors\": {"]
#[doc = "      \"$ref\": \"#/definitions/BitField\""]
#[doc = "    },"]
#[doc = "    \"LiveSectors\": {"]
#[doc = "      \"$ref\": \"#/definitions/BitField\""]
#[doc = "    },"]
#[doc = "    \"RecoveringSectors\": {"]
#[doc = "      \"$ref\": \"#/definitions/BitField\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MinerPartitions {
    #[serde(rename = "ActiveSectors")]
    pub active_sectors: BitField,
    #[serde(rename = "AllSectors")]
    pub all_sectors: BitField,
    #[serde(rename = "FaultySectors")]
    pub faulty_sectors: BitField,
    #[serde(rename = "LiveSectors")]
    pub live_sectors: BitField,
    #[serde(rename = "RecoveringSectors")]
    pub recovering_sectors: BitField,
}
impl From<&MinerPartitions> for MinerPartitions {
    fn from(value: &MinerPartitions) -> Self {
        value.clone()
    }
}
#[doc = "MinerPower"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"HasMinPower\","]
#[doc = "    \"MinerPower\","]
#[doc = "    \"TotalPower\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"HasMinPower\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"MinerPower\": {"]
#[doc = "      \"$ref\": \"#/definitions/Claim\""]
#[doc = "    },"]
#[doc = "    \"TotalPower\": {"]
#[doc = "      \"$ref\": \"#/definitions/Claim\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MinerPower {
    #[serde(rename = "HasMinPower")]
    pub has_min_power: bool,
    #[serde(rename = "MinerPower")]
    pub miner_power: Claim,
    #[serde(rename = "TotalPower")]
    pub total_power: Claim,
}
impl From<&MinerPower> for MinerPower {
    fn from(value: &MinerPower) -> Self {
        value.clone()
    }
}
#[doc = "MinerSectors"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Active\","]
#[doc = "    \"Faulty\","]
#[doc = "    \"Live\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Active\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Faulty\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Live\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MinerSectors {
    #[serde(rename = "Active")]
    pub active: u64,
    #[serde(rename = "Faulty")]
    pub faulty: u64,
    #[serde(rename = "Live")]
    pub live: u64,
}
impl From<&MinerSectors> for MinerSectors {
    fn from(value: &MinerSectors) -> Self {
        value.clone()
    }
}
#[doc = "MiningBaseInfo"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BeaconEntries\","]
#[doc = "    \"EligibleForMining\","]
#[doc = "    \"MinerPower\","]
#[doc = "    \"NetworkPower\","]
#[doc = "    \"PrevBeaconEntry\","]
#[doc = "    \"SectorSize\","]
#[doc = "    \"Sectors\","]
#[doc = "    \"WorkerKey\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BeaconEntries\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_BeaconEntry\""]
#[doc = "    },"]
#[doc = "    \"EligibleForMining\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"MinerPower\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"NetworkPower\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"PrevBeaconEntry\": {"]
#[doc = "      \"$ref\": \"#/definitions/BeaconEntry\""]
#[doc = "    },"]
#[doc = "    \"SectorSize\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Sectors\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_SectorInfo\""]
#[doc = "    },"]
#[doc = "    \"WorkerKey\": {"]
#[doc = "      \"$ref\": \"#/definitions/Address\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MiningBaseInfo {
    #[serde(rename = "BeaconEntries")]
    pub beacon_entries: NullableArrayOfBeaconEntry,
    #[serde(rename = "EligibleForMining")]
    pub eligible_for_mining: bool,
    #[serde(rename = "MinerPower")]
    pub miner_power: BigInt,
    #[serde(rename = "NetworkPower")]
    pub network_power: BigInt,
    #[serde(rename = "PrevBeaconEntry")]
    pub prev_beacon_entry: BeaconEntry,
    #[serde(rename = "SectorSize")]
    pub sector_size: u64,
    #[serde(rename = "Sectors")]
    pub sectors: NullableArrayOfSectorInfo,
    #[serde(rename = "WorkerKey")]
    pub worker_key: Address,
}
impl From<&MiningBaseInfo> for MiningBaseInfo {
    fn from(value: &MiningBaseInfo) -> Self {
        value.clone()
    }
}
#[doc = "NetworkParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BlockDelaySecs\","]
#[doc = "    \"ConsensusMinerMinPower\","]
#[doc = "    \"Eip155ChainID\","]
#[doc = "    \"ForkUpgradeParams\","]
#[doc = "    \"NetworkName\","]
#[doc = "    \"PreCommitChallengeDelay\","]
#[doc = "    \"SupportedProofTypes\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BlockDelaySecs\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"ConsensusMinerMinPower\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"Eip155ChainID\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"ForkUpgradeParams\": {"]
#[doc = "      \"$ref\": \"#/definitions/ForkUpgradeParams\""]
#[doc = "    },"]
#[doc = "    \"NetworkName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"PreCommitChallengeDelay\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"SupportedProofTypes\": {"]
#[doc = "      \"$ref\": \"#/definitions/int64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NetworkParams {
    #[serde(rename = "BlockDelaySecs")]
    pub block_delay_secs: u64,
    #[serde(rename = "ConsensusMinerMinPower")]
    pub consensus_miner_min_power: BigInt,
    #[serde(rename = "Eip155ChainID")]
    pub eip155_chain_id: u64,
    #[serde(rename = "ForkUpgradeParams")]
    pub fork_upgrade_params: ForkUpgradeParams,
    #[serde(rename = "NetworkName")]
    pub network_name: String,
    #[serde(rename = "PreCommitChallengeDelay")]
    pub pre_commit_challenge_delay: i64,
    #[serde(rename = "SupportedProofTypes")]
    pub supported_proof_types: Int64,
}
impl From<&NetworkParams> for NetworkParams {
    fn from(value: &NetworkParams) -> Self {
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
#[doc = "NullableAddress"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Address\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableAddress(pub Option<Address>);
impl std::ops::Deref for NullableAddress {
    type Target = Option<Address>;
    fn deref(&self) -> &Option<Address> {
        &self.0
    }
}
impl From<NullableAddress> for Option<Address> {
    fn from(value: NullableAddress) -> Self {
        value.0
    }
}
impl From<&NullableAddress> for NullableAddress {
    fn from(value: &NullableAddress) -> Self {
        value.clone()
    }
}
impl From<Option<Address>> for NullableAddress {
    fn from(value: Option<Address>) -> Self {
        Self(value)
    }
}
#[doc = "NullableArrayOfAddress"]
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
#[doc = "    \"$ref\": \"#/definitions/Address\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableArrayOfAddress(pub Option<Vec<Address>>);
impl std::ops::Deref for NullableArrayOfAddress {
    type Target = Option<Vec<Address>>;
    fn deref(&self) -> &Option<Vec<Address>> {
        &self.0
    }
}
impl From<NullableArrayOfAddress> for Option<Vec<Address>> {
    fn from(value: NullableArrayOfAddress) -> Self {
        value.0
    }
}
impl From<&NullableArrayOfAddress> for NullableArrayOfAddress {
    fn from(value: &NullableArrayOfAddress) -> Self {
        value.clone()
    }
}
impl From<Option<Vec<Address>>> for NullableArrayOfAddress {
    fn from(value: Option<Vec<Address>>) -> Self {
        Self(value)
    }
}
#[doc = "NullableArrayOfBase64String"]
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
#[doc = "    \"$ref\": \"#/definitions/Base64String\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableArrayOfBase64String(pub Option<Vec<Base64String>>);
impl std::ops::Deref for NullableArrayOfBase64String {
    type Target = Option<Vec<Base64String>>;
    fn deref(&self) -> &Option<Vec<Base64String>> {
        &self.0
    }
}
impl From<NullableArrayOfBase64String> for Option<Vec<Base64String>> {
    fn from(value: NullableArrayOfBase64String) -> Self {
        value.0
    }
}
impl From<&NullableArrayOfBase64String> for NullableArrayOfBase64String {
    fn from(value: &NullableArrayOfBase64String) -> Self {
        value.clone()
    }
}
impl From<Option<Vec<Base64String>>> for NullableArrayOfBase64String {
    fn from(value: Option<Vec<Base64String>>) -> Self {
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
#[doc = "NullableArrayOfCid"]
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
#[doc = "    \"$ref\": \"#/definitions/Cid\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableArrayOfCid(pub Option<Vec<Cid>>);
impl std::ops::Deref for NullableArrayOfCid {
    type Target = Option<Vec<Cid>>;
    fn deref(&self) -> &Option<Vec<Cid>> {
        &self.0
    }
}
impl From<NullableArrayOfCid> for Option<Vec<Cid>> {
    fn from(value: NullableArrayOfCid) -> Self {
        value.0
    }
}
impl From<&NullableArrayOfCid> for NullableArrayOfCid {
    fn from(value: &NullableArrayOfCid) -> Self {
        value.clone()
    }
}
impl From<Option<Vec<Cid>>> for NullableArrayOfCid {
    fn from(value: Option<Vec<Cid>>) -> Self {
        Self(value)
    }
}
#[doc = "NullableArrayOfExecutionTrace"]
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
#[doc = "    \"$ref\": \"#/definitions/ExecutionTrace\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableArrayOfExecutionTrace(pub Option<Vec<ExecutionTrace>>);
impl std::ops::Deref for NullableArrayOfExecutionTrace {
    type Target = Option<Vec<ExecutionTrace>>;
    fn deref(&self) -> &Option<Vec<ExecutionTrace>> {
        &self.0
    }
}
impl From<NullableArrayOfExecutionTrace> for Option<Vec<ExecutionTrace>> {
    fn from(value: NullableArrayOfExecutionTrace) -> Self {
        value.0
    }
}
impl From<&NullableArrayOfExecutionTrace> for NullableArrayOfExecutionTrace {
    fn from(value: &NullableArrayOfExecutionTrace) -> Self {
        value.clone()
    }
}
impl From<Option<Vec<ExecutionTrace>>> for NullableArrayOfExecutionTrace {
    fn from(value: Option<Vec<ExecutionTrace>>) -> Self {
        Self(value)
    }
}
#[doc = "NullableArrayOfMessage"]
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
#[doc = "    \"$ref\": \"#/definitions/Message\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableArrayOfMessage(pub Option<Vec<Message>>);
impl std::ops::Deref for NullableArrayOfMessage {
    type Target = Option<Vec<Message>>;
    fn deref(&self) -> &Option<Vec<Message>> {
        &self.0
    }
}
impl From<NullableArrayOfMessage> for Option<Vec<Message>> {
    fn from(value: NullableArrayOfMessage) -> Self {
        value.0
    }
}
impl From<&NullableArrayOfMessage> for NullableArrayOfMessage {
    fn from(value: &NullableArrayOfMessage) -> Self {
        value.clone()
    }
}
impl From<Option<Vec<Message>>> for NullableArrayOfMessage {
    fn from(value: Option<Vec<Message>>) -> Self {
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
#[doc = "NullableArrayOfSectorInfo"]
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
#[doc = "    \"$ref\": \"#/definitions/SectorInfo\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableArrayOfSectorInfo(pub Option<Vec<SectorInfo>>);
impl std::ops::Deref for NullableArrayOfSectorInfo {
    type Target = Option<Vec<SectorInfo>>;
    fn deref(&self) -> &Option<Vec<SectorInfo>> {
        &self.0
    }
}
impl From<NullableArrayOfSectorInfo> for Option<Vec<SectorInfo>> {
    fn from(value: NullableArrayOfSectorInfo) -> Self {
        value.0
    }
}
impl From<&NullableArrayOfSectorInfo> for NullableArrayOfSectorInfo {
    fn from(value: &NullableArrayOfSectorInfo) -> Self {
        value.clone()
    }
}
impl From<Option<Vec<SectorInfo>>> for NullableArrayOfSectorInfo {
    fn from(value: Option<Vec<SectorInfo>>) -> Self {
        Self(value)
    }
}
#[doc = "NullableArrayOfSignedMessage"]
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
#[doc = "    \"$ref\": \"#/definitions/SignedMessage\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableArrayOfSignedMessage(pub Option<Vec<SignedMessage>>);
impl std::ops::Deref for NullableArrayOfSignedMessage {
    type Target = Option<Vec<SignedMessage>>;
    fn deref(&self) -> &Option<Vec<SignedMessage>> {
        &self.0
    }
}
impl From<NullableArrayOfSignedMessage> for Option<Vec<SignedMessage>> {
    fn from(value: NullableArrayOfSignedMessage) -> Self {
        value.0
    }
}
impl From<&NullableArrayOfSignedMessage> for NullableArrayOfSignedMessage {
    fn from(value: &NullableArrayOfSignedMessage) -> Self {
        value.clone()
    }
}
impl From<Option<Vec<SignedMessage>>> for NullableArrayOfSignedMessage {
    fn from(value: Option<Vec<SignedMessage>>) -> Self {
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
#[doc = "NullableBase64String"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Base64String\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableBase64String(pub Option<Base64String>);
impl std::ops::Deref for NullableBase64String {
    type Target = Option<Base64String>;
    fn deref(&self) -> &Option<Base64String> {
        &self.0
    }
}
impl From<NullableBase64String> for Option<Base64String> {
    fn from(value: NullableBase64String) -> Self {
        value.0
    }
}
impl From<&NullableBase64String> for NullableBase64String {
    fn from(value: &NullableBase64String) -> Self {
        value.clone()
    }
}
impl From<Option<Base64String>> for NullableBase64String {
    fn from(value: Option<Base64String>) -> Self {
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
#[doc = "NullablePendingBeneficiaryChange"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/PendingBeneficiaryChange\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullablePendingBeneficiaryChange(pub Option<PendingBeneficiaryChange>);
impl std::ops::Deref for NullablePendingBeneficiaryChange {
    type Target = Option<PendingBeneficiaryChange>;
    fn deref(&self) -> &Option<PendingBeneficiaryChange> {
        &self.0
    }
}
impl From<NullablePendingBeneficiaryChange> for Option<PendingBeneficiaryChange> {
    fn from(value: NullablePendingBeneficiaryChange) -> Self {
        value.0
    }
}
impl From<&NullablePendingBeneficiaryChange> for NullablePendingBeneficiaryChange {
    fn from(value: &NullablePendingBeneficiaryChange) -> Self {
        value.clone()
    }
}
impl From<Option<PendingBeneficiaryChange>> for NullablePendingBeneficiaryChange {
    fn from(value: Option<PendingBeneficiaryChange>) -> Self {
        Self(value)
    }
}
#[doc = "NullableReceipt"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Receipt\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"null\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullableReceipt(pub Option<Receipt>);
impl std::ops::Deref for NullableReceipt {
    type Target = Option<Receipt>;
    fn deref(&self) -> &Option<Receipt> {
        &self.0
    }
}
impl From<NullableReceipt> for Option<Receipt> {
    fn from(value: NullableReceipt) -> Self {
        value.0
    }
}
impl From<&NullableReceipt> for NullableReceipt {
    fn from(value: &NullableReceipt) -> Self {
        value.clone()
    }
}
impl From<Option<Receipt>> for NullableReceipt {
    fn from(value: Option<Receipt>) -> Self {
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
#[doc = "NullableString"]
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
pub struct NullableString(pub Option<String>);
impl std::ops::Deref for NullableString {
    type Target = Option<String>;
    fn deref(&self) -> &Option<String> {
        &self.0
    }
}
impl From<NullableString> for Option<String> {
    fn from(value: NullableString) -> Self {
        value.0
    }
}
impl From<&NullableString> for NullableString {
    fn from(value: &NullableString) -> Self {
        value.clone()
    }
}
impl From<Option<String>> for NullableString {
    fn from(value: Option<String>) -> Self {
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
#[doc = "PathChangeForTipset"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"Type\","]
#[doc = "        \"Val\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"Type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"revert\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"Val\": {"]
#[doc = "          \"$ref\": \"#/definitions/Tipset\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"Type\","]
#[doc = "        \"Val\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"Type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"apply\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"Val\": {"]
#[doc = "          \"$ref\": \"#/definitions/Tipset\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "Type", content = "Val")]
pub enum PathChangeForTipset {
    #[serde(rename = "revert")]
    Revert(Tipset),
    #[serde(rename = "apply")]
    Apply(Tipset),
}
impl From<&PathChangeForTipset> for PathChangeForTipset {
    fn from(value: &PathChangeForTipset) -> Self {
        value.clone()
    }
}
#[doc = "PendingBeneficiaryChange"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ApprovedByBeneficiary\","]
#[doc = "    \"ApprovedByNominee\","]
#[doc = "    \"NewBeneficiary\","]
#[doc = "    \"NewExpiration\","]
#[doc = "    \"NewQuota\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ApprovedByBeneficiary\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"ApprovedByNominee\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"NewBeneficiary\": {"]
#[doc = "      \"$ref\": \"#/definitions/Address\""]
#[doc = "    },"]
#[doc = "    \"NewExpiration\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"NewQuota\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PendingBeneficiaryChange {
    #[serde(rename = "ApprovedByBeneficiary")]
    pub approved_by_beneficiary: bool,
    #[serde(rename = "ApprovedByNominee")]
    pub approved_by_nominee: bool,
    #[serde(rename = "NewBeneficiary")]
    pub new_beneficiary: Address,
    #[serde(rename = "NewExpiration")]
    pub new_expiration: i64,
    #[serde(rename = "NewQuota")]
    pub new_quota: BigInt,
}
impl From<&PendingBeneficiaryChange> for PendingBeneficiaryChange {
    fn from(value: &PendingBeneficiaryChange) -> Self {
        value.clone()
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
#[doc = "ReturnTrace"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ExitCode\","]
#[doc = "    \"Return\","]
#[doc = "    \"ReturnCodec\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ExitCode\": {"]
#[doc = "      \"$ref\": \"#/definitions/ExitCode\""]
#[doc = "    },"]
#[doc = "    \"Return\": {"]
#[doc = "      \"$ref\": \"#/definitions/Base64String\""]
#[doc = "    },"]
#[doc = "    \"ReturnCodec\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReturnTrace {
    #[serde(rename = "ExitCode")]
    pub exit_code: ExitCode,
    #[serde(rename = "Return")]
    pub return_: Base64String,
    #[serde(rename = "ReturnCodec")]
    pub return_codec: u64,
}
impl From<&ReturnTrace> for ReturnTrace {
    fn from(value: &ReturnTrace) -> Self {
        value.clone()
    }
}
#[doc = "SectorExpiration"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Early\","]
#[doc = "    \"OnTime\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Early\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"OnTime\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SectorExpiration {
    #[serde(rename = "Early")]
    pub early: i64,
    #[serde(rename = "OnTime")]
    pub on_time: i64,
}
impl From<&SectorExpiration> for SectorExpiration {
    fn from(value: &SectorExpiration) -> Self {
        value.clone()
    }
}
#[doc = "SectorInfo"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"SealProof\","]
#[doc = "    \"SealedCID\","]
#[doc = "    \"SectorNumber\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"SealProof\": {"]
#[doc = "      \"$ref\": \"#/definitions/int64\""]
#[doc = "    },"]
#[doc = "    \"SealedCID\": {"]
#[doc = "      \"$ref\": \"#/definitions/Cid\""]
#[doc = "    },"]
#[doc = "    \"SectorNumber\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SectorInfo {
    #[serde(rename = "SealProof")]
    pub seal_proof: Int64,
    #[serde(rename = "SealedCID")]
    pub sealed_cid: Cid,
    #[serde(rename = "SectorNumber")]
    pub sector_number: u64,
}
impl From<&SectorInfo> for SectorInfo {
    fn from(value: &SectorInfo) -> Self {
        value.clone()
    }
}
#[doc = "SectorLocation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Deadline\","]
#[doc = "    \"Partition\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Deadline\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Partition\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SectorLocation {
    #[serde(rename = "Deadline")]
    pub deadline: u64,
    #[serde(rename = "Partition")]
    pub partition: u64,
}
impl From<&SectorLocation> for SectorLocation {
    fn from(value: &SectorLocation) -> Self {
        value.clone()
    }
}
#[doc = "SectorOnChainInfo"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Activation\","]
#[doc = "    \"DealIDs\","]
#[doc = "    \"DealWeight\","]
#[doc = "    \"ExpectedDayReward\","]
#[doc = "    \"ExpectedStoragePledge\","]
#[doc = "    \"Expiration\","]
#[doc = "    \"Flags\","]
#[doc = "    \"InitialPledge\","]
#[doc = "    \"PowerBaseEpoch\","]
#[doc = "    \"ReplacedDayReward\","]
#[doc = "    \"SealProof\","]
#[doc = "    \"SealedCID\","]
#[doc = "    \"SectorKeyCID\","]
#[doc = "    \"SectorNumber\","]
#[doc = "    \"VerifiedDealWeight\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Activation\": {"]
#[doc = "      \"description\": \"Epoch during which the sector proof was accepted\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"DealIDs\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Array_of_uint64\""]
#[doc = "    },"]
#[doc = "    \"DealWeight\": {"]
#[doc = "      \"description\": \"Integral of active deals over sector lifetime\","]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"ExpectedDayReward\": {"]
#[doc = "      \"description\": \"Expected one day projection of reward for sector computed at activation time\","]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"ExpectedStoragePledge\": {"]
#[doc = "      \"description\": \"Expected twenty day projection of reward for sector computed at activation time\","]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"Expiration\": {"]
#[doc = "      \"description\": \"Epoch during which the sector expires\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"Flags\": {"]
#[doc = "      \"description\": \"Additional flags, see [`fil_actor_miner_state::v12::SectorOnChainInfoFlags`]\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"InitialPledge\": {"]
#[doc = "      \"description\": \"Pledge collected to commit this sector\","]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"PowerBaseEpoch\": {"]
#[doc = "      \"description\": \"Epoch at which this sector's power was most recently updated\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"ReplacedDayReward\": {"]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    },"]
#[doc = "    \"SealProof\": {"]
#[doc = "      \"description\": \"The seal proof type implies the PoSt proofs\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"SealedCID\": {"]
#[doc = "      \"description\": \"`CommR`\","]
#[doc = "      \"$ref\": \"#/definitions/Cid\""]
#[doc = "    },"]
#[doc = "    \"SectorKeyCID\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Cid\""]
#[doc = "    },"]
#[doc = "    \"SectorNumber\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"VerifiedDealWeight\": {"]
#[doc = "      \"description\": \"Integral of active verified deals over sector lifetime\","]
#[doc = "      \"$ref\": \"#/definitions/BigInt\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SectorOnChainInfo {
    #[doc = "Epoch during which the sector proof was accepted"]
    #[serde(rename = "Activation")]
    pub activation: i64,
    #[serde(rename = "DealIDs")]
    pub deal_i_ds: NullableArrayOfUint64,
    #[doc = "Integral of active deals over sector lifetime"]
    #[serde(rename = "DealWeight")]
    pub deal_weight: BigInt,
    #[doc = "Expected one day projection of reward for sector computed at activation time"]
    #[serde(rename = "ExpectedDayReward")]
    pub expected_day_reward: BigInt,
    #[doc = "Expected twenty day projection of reward for sector computed at activation time"]
    #[serde(rename = "ExpectedStoragePledge")]
    pub expected_storage_pledge: BigInt,
    #[doc = "Epoch during which the sector expires"]
    #[serde(rename = "Expiration")]
    pub expiration: i64,
    #[doc = "Additional flags, see [`fil_actor_miner_state::v12::SectorOnChainInfoFlags`]"]
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[doc = "Pledge collected to commit this sector"]
    #[serde(rename = "InitialPledge")]
    pub initial_pledge: BigInt,
    #[doc = "Epoch at which this sector's power was most recently updated"]
    #[serde(rename = "PowerBaseEpoch")]
    pub power_base_epoch: i64,
    #[serde(rename = "ReplacedDayReward")]
    pub replaced_day_reward: BigInt,
    #[doc = "The seal proof type implies the PoSt proofs"]
    #[serde(rename = "SealProof")]
    pub seal_proof: i64,
    #[doc = "`CommR`"]
    #[serde(rename = "SealedCID")]
    pub sealed_cid: Cid,
    #[serde(rename = "SectorKeyCID")]
    pub sector_key_cid: NullableCid,
    #[serde(rename = "SectorNumber")]
    pub sector_number: u64,
    #[doc = "Integral of active verified deals over sector lifetime"]
    #[serde(rename = "VerifiedDealWeight")]
    pub verified_deal_weight: BigInt,
}
impl From<&SectorOnChainInfo> for SectorOnChainInfo {
    fn from(value: &SectorOnChainInfo) -> Self {
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
#[doc = "SectorSize"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"uint64\","]
#[doc = "  \"minimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SectorSize(pub u64);
impl std::ops::Deref for SectorSize {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}
impl From<SectorSize> for u64 {
    fn from(value: SectorSize) -> Self {
        value.0
    }
}
impl From<&SectorSize> for SectorSize {
    fn from(value: &SectorSize) -> Self {
        value.clone()
    }
}
impl From<u64> for SectorSize {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for SectorSize {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for SectorSize {
    type Error = <u64 as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SectorSize {
    type Error = <u64 as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SectorSize {
    type Error = <u64 as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for SectorSize {
    fn to_string(&self) -> String {
        self.0.to_string()
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
#[doc = "SignedMessage"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Message\","]
#[doc = "    \"Signature\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"CID\": {"]
#[doc = "      \"$ref\": \"#/definitions/Nullable_Cid\""]
#[doc = "    },"]
#[doc = "    \"Message\": {"]
#[doc = "      \"$ref\": \"#/definitions/Message\""]
#[doc = "    },"]
#[doc = "    \"Signature\": {"]
#[doc = "      \"$ref\": \"#/definitions/Signature\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignedMessage {
    #[serde(rename = "CID", default, skip_serializing_if = "Option::is_none")]
    pub cid: Option<NullableCid>,
    #[serde(rename = "Message")]
    pub message: Message,
    #[serde(rename = "Signature")]
    pub signature: Signature,
}
impl From<&SignedMessage> for SignedMessage {
    fn from(value: &SignedMessage) -> Self {
        value.clone()
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
    pub(super) fn message_gas_fee_cap() -> super::BigInt {
        super::BigInt("0".to_string())
    }
    pub(super) fn message_gas_premium() -> super::BigInt {
        super::BigInt("0".to_string())
    }
    pub(super) fn message_value() -> super::BigInt {
        super::BigInt("0".to_string())
    }
    pub(super) fn receipt_events_root() -> super::NullableCid {
        super::NullableCid(None)
    }
}
