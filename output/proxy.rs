// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct NftUpgradeProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for NftUpgradeProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = NftUpgradeProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        NftUpgradeProxyMethods { wrapped_tx: tx }
    }
}

pub struct NftUpgradeProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> NftUpgradeProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> NftUpgradeProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> NftUpgradeProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    /// Initialize a Test NFT with level 1 in attributes, plus some more info to match current EMR NFTs. 
    /// This will make an NFT similar to the current EMR NFTs. 
    pub fn initialize(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("initialize")
            .original_result()
    }

    /// Upgrade an NFT to the same level but with more data in attributes. 
    pub fn upgrade_nft(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("upgradeNft")
            .original_result()
    }

    /// Increase the level of an NFT by 1. 
    pub fn increase_level(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("increaseLevel")
            .original_result()
    }

    /// Decrease the level of an NFT by 1. 
    pub fn decrease_level(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("decreaseLevel")
            .original_result()
    }

    pub fn is_sc_paused(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getIsScPaused")
            .original_result()
    }

    pub fn allowed_addresses(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAllowedAddresses")
            .original_result()
    }

    pub fn pause_sc(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("pauseSc")
            .original_result()
    }

    pub fn resume_sc(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("resumeSc")
            .original_result()
    }

    pub fn add_allowed_addresses<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        addresses: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("addAllowedAddresses")
            .argument(&addresses)
            .original_result()
    }

    pub fn get_tags(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedBuffer<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTags")
            .original_result()
    }

    pub fn get_nft_identifier(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getNftIdentifier")
            .original_result()
    }

    pub fn get_nft_attributes<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
    >(
        self,
        owner: Arg0,
        token_identifier: Arg1,
        token_nonce: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedBuffer<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getNftAttributes")
            .argument(&owner)
            .argument(&token_identifier)
            .argument(&token_nonce)
            .original_result()
    }

    pub fn get_nft_uris<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
    >(
        self,
        owner: Arg0,
        token_identifier: Arg1,
        token_nonce: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, ManagedBuffer<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getNftUris")
            .argument(&owner)
            .argument(&token_identifier)
            .argument(&token_nonce)
            .original_result()
    }

    pub fn get_nft_uri_json<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
    >(
        self,
        owner: Arg0,
        token_identifier: Arg1,
        token_nonce: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedBuffer<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getNftUriJson")
            .argument(&owner)
            .argument(&token_identifier)
            .argument(&token_nonce)
            .original_result()
    }

    pub fn get_nft_attributes_level_before_upgrade<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
    >(
        self,
        owner: Arg0,
        token_identifier: Arg1,
        token_nonce: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedBuffer<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getNftAttributesLevelBeforeUpgrade")
            .argument(&owner)
            .argument(&token_identifier)
            .argument(&token_nonce)
            .original_result()
    }

    pub fn get_nft_attributes_level_after_upgrade<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
    >(
        self,
        owner: Arg0,
        token_identifier: Arg1,
        token_nonce: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedBuffer<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getNftAttributesLevelAfterUpgrade")
            .argument(&owner)
            .argument(&token_identifier)
            .argument(&token_nonce)
            .original_result()
    }
}
