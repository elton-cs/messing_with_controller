#[derive()]
pub struct CrosswordleCrosswordle<A: starknet::accounts::ConnectedAccount + Sync> {
    pub address: starknet::core::types::Felt,
    pub account: A,
    pub block_id: starknet::core::types::BlockId,
}
impl<A: starknet::accounts::ConnectedAccount + Sync> CrosswordleCrosswordle<A> {
    pub fn new(address: starknet::core::types::Felt, account: A) -> Self {
        Self {
            address,
            account,
            block_id: starknet::core::types::BlockId::Tag(starknet::core::types::BlockTag::Pending),
        }
    }
    pub fn set_contract_address(&mut self, address: starknet::core::types::Felt) {
        self.address = address;
    }
    pub fn provider(&self) -> &A::Provider {
        self.account.provider()
    }
    pub fn set_block(&mut self, block_id: starknet::core::types::BlockId) {
        self.block_id = block_id;
    }
    pub fn with_block(self, block_id: starknet::core::types::BlockId) -> Self {
        Self { block_id, ..self }
    }
}
#[derive()]
pub struct CrosswordleCrosswordleReader<P: starknet::providers::Provider + Sync> {
    pub address: starknet::core::types::Felt,
    pub provider: P,
    pub block_id: starknet::core::types::BlockId,
}
impl<P: starknet::providers::Provider + Sync> CrosswordleCrosswordleReader<P> {
    pub fn new(address: starknet::core::types::Felt, provider: P) -> Self {
        Self {
            address,
            provider,
            block_id: starknet::core::types::BlockId::Tag(starknet::core::types::BlockTag::Pending),
        }
    }
    pub fn set_contract_address(&mut self, address: starknet::core::types::Felt) {
        self.address = address;
    }
    pub fn provider(&self) -> &P {
        &self.provider
    }
    pub fn set_block(&mut self, block_id: starknet::core::types::BlockId) {
        self.block_id = block_id;
    }
    pub fn with_block(self, block_id: starknet::core::types::BlockId) -> Self {
        Self { block_id, ..self }
    }
}
#[derive()]
pub struct IWorldDispatcher {
    pub contract_address: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for IWorldDispatcher {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size +=
            cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.contract_address);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.contract_address,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let contract_address =
            cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&contract_address);
        Ok(IWorldDispatcher { contract_address })
    }
}
#[derive()]
pub struct Upgraded {
    pub class_hash: cainome::cairo_serde::ClassHash,
}
impl cainome::cairo_serde::CairoSerde for Upgraded {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::ClassHash::cairo_serialized_size(&__rust.class_hash);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ClassHash::cairo_serialize(
            &__rust.class_hash,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let class_hash = cainome::cairo_serde::ClassHash::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
        Ok(Upgraded { class_hash })
    }
}
impl Upgraded {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("Upgraded").unwrap()
    }
    pub fn event_name() -> &'static str {
        "Upgraded"
    }
}
#[derive()]
pub enum Event {}
impl cainome::cairo_serde::CairoSerde for Event {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "Event"
                )))
            }
        }
    }
}
impl TryFrom<&starknet::core::types::EmittedEvent> for Event {
    type Error = String;
    fn try_from(event: &starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
impl TryFrom<&starknet::core::types::Event> for Event {
    type Error = String;
    fn try_from(event: &starknet::core::types::Event) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}

impl<A: starknet::accounts::ConnectedAccount + Sync> CrosswordleCrosswordle<A> {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn dojo_init(&self) -> cainome::cairo_serde::call::FCall<A::Provider, ()> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("dojo_init"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn dojo_name(
        &self,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, cainome::cairo_serde::ByteArray> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("dojo_name"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn world_dispatcher(
        &self,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, IWorldDispatcher> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("world_dispatcher"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn start_game_getcall(&self) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("start_game"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn start_game(&self) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("start_game"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn submit_guess_getcall(
        &self,
        word_byte_array: &cainome::cairo_serde::ByteArray,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(
            word_byte_array,
        ));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("submit_guess"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn submit_guess(
        &self,
        word_byte_array: &cainome::cairo_serde::ByteArray,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(
            word_byte_array,
        ));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("submit_guess"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn upgrade_getcall(
        &self,
        new_class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(
            new_class_hash,
        ));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("upgrade"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn upgrade(
        &self,
        new_class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(
            new_class_hash,
        ));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("upgrade"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
}
impl<P: starknet::providers::Provider + Sync> CrosswordleCrosswordleReader<P> {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn dojo_init(&self) -> cainome::cairo_serde::call::FCall<P, ()> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("dojo_init"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn dojo_name(
        &self,
    ) -> cainome::cairo_serde::call::FCall<P, cainome::cairo_serde::ByteArray> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("dojo_name"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn world_dispatcher(&self) -> cainome::cairo_serde::call::FCall<P, IWorldDispatcher> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("world_dispatcher"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
}
