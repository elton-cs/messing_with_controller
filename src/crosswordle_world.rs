#[derive()]
pub struct CrosswordleWorld<A: starknet::accounts::ConnectedAccount + Sync> {
    pub address: starknet::core::types::Felt,
    pub account: A,
    pub block_id: starknet::core::types::BlockId,
}
impl<A: starknet::accounts::ConnectedAccount + Sync> CrosswordleWorld<A> {
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
pub struct CrosswordleWorldReader<P: starknet::providers::Provider + Sync> {
    pub address: starknet::core::types::Felt,
    pub provider: P,
    pub block_id: starknet::core::types::BlockId,
}
impl<P: starknet::providers::Provider + Sync> CrosswordleWorldReader<P> {
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
pub struct ContractInitialized {
    pub selector: starknet::core::types::Felt,
    pub init_calldata: Vec<starknet::core::types::Felt>,
}
impl cainome::cairo_serde::CairoSerde for ContractInitialized {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.selector);
        __size += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&__rust.init_calldata);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.selector,
        ));
        __out.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            &__rust.init_calldata,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let selector = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
        let init_calldata =
            Vec::<starknet::core::types::Felt>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&init_calldata);
        Ok(ContractInitialized {
            selector,
            init_calldata,
        })
    }
}
impl ContractInitialized {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("ContractInitialized").unwrap()
    }
    pub fn event_name() -> &'static str {
        "ContractInitialized"
    }
}
#[derive()]
pub struct ContractRegistered {
    pub name: cainome::cairo_serde::ByteArray,
    pub namespace: cainome::cairo_serde::ByteArray,
    pub address: cainome::cairo_serde::ContractAddress,
    pub class_hash: cainome::cairo_serde::ClassHash,
    pub salt: starknet::core::types::Felt,
}
impl cainome::cairo_serde::CairoSerde for ContractRegistered {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::ByteArray::cairo_serialized_size(&__rust.name);
        __size += cainome::cairo_serde::ByteArray::cairo_serialized_size(&__rust.namespace);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.address);
        __size += cainome::cairo_serde::ClassHash::cairo_serialized_size(&__rust.class_hash);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.salt);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ByteArray::cairo_serialize(
            &__rust.name,
        ));
        __out.extend(cainome::cairo_serde::ByteArray::cairo_serialize(
            &__rust.namespace,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.address,
        ));
        __out.extend(cainome::cairo_serde::ClassHash::cairo_serialize(
            &__rust.class_hash,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(&__rust.salt));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let name = cainome::cairo_serde::ByteArray::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&name);
        let namespace = cainome::cairo_serde::ByteArray::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&namespace);
        let address = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
        let class_hash = cainome::cairo_serde::ClassHash::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
        let salt = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&salt);
        Ok(ContractRegistered {
            name,
            namespace,
            address,
            class_hash,
            salt,
        })
    }
}
impl ContractRegistered {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("ContractRegistered").unwrap()
    }
    pub fn event_name() -> &'static str {
        "ContractRegistered"
    }
}
#[derive()]
pub struct ContractUpgraded {
    pub selector: starknet::core::types::Felt,
    pub class_hash: cainome::cairo_serde::ClassHash,
}
impl cainome::cairo_serde::CairoSerde for ContractUpgraded {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.selector);
        __size += cainome::cairo_serde::ClassHash::cairo_serialized_size(&__rust.class_hash);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.selector,
        ));
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
        let selector = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
        let class_hash = cainome::cairo_serde::ClassHash::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
        Ok(ContractUpgraded {
            selector,
            class_hash,
        })
    }
}
impl ContractUpgraded {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("ContractUpgraded").unwrap()
    }
    pub fn event_name() -> &'static str {
        "ContractUpgraded"
    }
}
#[derive()]
pub struct EventEmitted {
    pub selector: starknet::core::types::Felt,
    pub system_address: cainome::cairo_serde::ContractAddress,
    pub keys: Vec<starknet::core::types::Felt>,
    pub values: Vec<starknet::core::types::Felt>,
}
impl cainome::cairo_serde::CairoSerde for EventEmitted {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.selector);
        __size +=
            cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.system_address);
        __size += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&__rust.keys);
        __size += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&__rust.values);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.selector,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.system_address,
        ));
        __out.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            &__rust.keys,
        ));
        __out.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            &__rust.values,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let selector = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
        let system_address =
            cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&system_address);
        let keys = Vec::<starknet::core::types::Felt>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&keys);
        let values = Vec::<starknet::core::types::Felt>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&values);
        Ok(EventEmitted {
            selector,
            system_address,
            keys,
            values,
        })
    }
}
impl EventEmitted {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("EventEmitted").unwrap()
    }
    pub fn event_name() -> &'static str {
        "EventEmitted"
    }
}
#[derive()]
pub struct EventRegistered {
    pub name: cainome::cairo_serde::ByteArray,
    pub namespace: cainome::cairo_serde::ByteArray,
    pub class_hash: cainome::cairo_serde::ClassHash,
    pub address: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for EventRegistered {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::ByteArray::cairo_serialized_size(&__rust.name);
        __size += cainome::cairo_serde::ByteArray::cairo_serialized_size(&__rust.namespace);
        __size += cainome::cairo_serde::ClassHash::cairo_serialized_size(&__rust.class_hash);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.address);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ByteArray::cairo_serialize(
            &__rust.name,
        ));
        __out.extend(cainome::cairo_serde::ByteArray::cairo_serialize(
            &__rust.namespace,
        ));
        __out.extend(cainome::cairo_serde::ClassHash::cairo_serialize(
            &__rust.class_hash,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.address,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let name = cainome::cairo_serde::ByteArray::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&name);
        let namespace = cainome::cairo_serde::ByteArray::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&namespace);
        let class_hash = cainome::cairo_serde::ClassHash::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
        let address = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
        Ok(EventRegistered {
            name,
            namespace,
            class_hash,
            address,
        })
    }
}
impl EventRegistered {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("EventRegistered").unwrap()
    }
    pub fn event_name() -> &'static str {
        "EventRegistered"
    }
}
#[derive()]
pub struct EventUpgraded {
    pub selector: starknet::core::types::Felt,
    pub class_hash: cainome::cairo_serde::ClassHash,
    pub address: cainome::cairo_serde::ContractAddress,
    pub prev_address: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for EventUpgraded {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.selector);
        __size += cainome::cairo_serde::ClassHash::cairo_serialized_size(&__rust.class_hash);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.address);
        __size +=
            cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.prev_address);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.selector,
        ));
        __out.extend(cainome::cairo_serde::ClassHash::cairo_serialize(
            &__rust.class_hash,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.address,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.prev_address,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let selector = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
        let class_hash = cainome::cairo_serde::ClassHash::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
        let address = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
        let prev_address =
            cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&prev_address);
        Ok(EventUpgraded {
            selector,
            class_hash,
            address,
            prev_address,
        })
    }
}
impl EventUpgraded {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("EventUpgraded").unwrap()
    }
    pub fn event_name() -> &'static str {
        "EventUpgraded"
    }
}
#[derive()]
pub struct FieldLayout {
    pub selector: starknet::core::types::Felt,
    pub layout: Layout,
}
impl cainome::cairo_serde::CairoSerde for FieldLayout {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.selector);
        __size += Layout::cairo_serialized_size(&__rust.layout);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.selector,
        ));
        __out.extend(Layout::cairo_serialize(&__rust.layout));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let selector = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
        let layout = Layout::cairo_deserialize(__felts, __offset)?;
        __offset += Layout::cairo_serialized_size(&layout);
        Ok(FieldLayout { selector, layout })
    }
}
#[derive()]
pub struct MetadataUpdate {
    pub resource: starknet::core::types::Felt,
    pub uri: cainome::cairo_serde::ByteArray,
    pub hash: starknet::core::types::Felt,
}
impl cainome::cairo_serde::CairoSerde for MetadataUpdate {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.resource);
        __size += cainome::cairo_serde::ByteArray::cairo_serialized_size(&__rust.uri);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.hash);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.resource,
        ));
        __out.extend(cainome::cairo_serde::ByteArray::cairo_serialize(
            &__rust.uri,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(&__rust.hash));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let resource = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&resource);
        let uri = cainome::cairo_serde::ByteArray::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&uri);
        let hash = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&hash);
        Ok(MetadataUpdate {
            resource,
            uri,
            hash,
        })
    }
}
impl MetadataUpdate {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("MetadataUpdate").unwrap()
    }
    pub fn event_name() -> &'static str {
        "MetadataUpdate"
    }
}
#[derive()]
pub struct ModelRegistered {
    pub name: cainome::cairo_serde::ByteArray,
    pub namespace: cainome::cairo_serde::ByteArray,
    pub class_hash: cainome::cairo_serde::ClassHash,
    pub address: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for ModelRegistered {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::ByteArray::cairo_serialized_size(&__rust.name);
        __size += cainome::cairo_serde::ByteArray::cairo_serialized_size(&__rust.namespace);
        __size += cainome::cairo_serde::ClassHash::cairo_serialized_size(&__rust.class_hash);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.address);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ByteArray::cairo_serialize(
            &__rust.name,
        ));
        __out.extend(cainome::cairo_serde::ByteArray::cairo_serialize(
            &__rust.namespace,
        ));
        __out.extend(cainome::cairo_serde::ClassHash::cairo_serialize(
            &__rust.class_hash,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.address,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let name = cainome::cairo_serde::ByteArray::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&name);
        let namespace = cainome::cairo_serde::ByteArray::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&namespace);
        let class_hash = cainome::cairo_serde::ClassHash::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
        let address = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
        Ok(ModelRegistered {
            name,
            namespace,
            class_hash,
            address,
        })
    }
}
impl ModelRegistered {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("ModelRegistered").unwrap()
    }
    pub fn event_name() -> &'static str {
        "ModelRegistered"
    }
}
#[derive()]
pub struct ModelUpgraded {
    pub selector: starknet::core::types::Felt,
    pub class_hash: cainome::cairo_serde::ClassHash,
    pub address: cainome::cairo_serde::ContractAddress,
    pub prev_address: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for ModelUpgraded {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.selector);
        __size += cainome::cairo_serde::ClassHash::cairo_serialized_size(&__rust.class_hash);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.address);
        __size +=
            cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.prev_address);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.selector,
        ));
        __out.extend(cainome::cairo_serde::ClassHash::cairo_serialize(
            &__rust.class_hash,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.address,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.prev_address,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let selector = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
        let class_hash = cainome::cairo_serde::ClassHash::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
        let address = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
        let prev_address =
            cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&prev_address);
        Ok(ModelUpgraded {
            selector,
            class_hash,
            address,
            prev_address,
        })
    }
}
impl ModelUpgraded {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("ModelUpgraded").unwrap()
    }
    pub fn event_name() -> &'static str {
        "ModelUpgraded"
    }
}
#[derive()]
pub struct NamespaceRegistered {
    pub namespace: cainome::cairo_serde::ByteArray,
    pub hash: starknet::core::types::Felt,
}
impl cainome::cairo_serde::CairoSerde for NamespaceRegistered {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::ByteArray::cairo_serialized_size(&__rust.namespace);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.hash);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ByteArray::cairo_serialize(
            &__rust.namespace,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(&__rust.hash));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let namespace = cainome::cairo_serde::ByteArray::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&namespace);
        let hash = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&hash);
        Ok(NamespaceRegistered { namespace, hash })
    }
}
impl NamespaceRegistered {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("NamespaceRegistered").unwrap()
    }
    pub fn event_name() -> &'static str {
        "NamespaceRegistered"
    }
}
#[derive()]
pub struct OwnerUpdated {
    pub resource: starknet::core::types::Felt,
    pub contract: cainome::cairo_serde::ContractAddress,
    pub value: bool,
}
impl cainome::cairo_serde::CairoSerde for OwnerUpdated {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.resource);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.contract);
        __size += bool::cairo_serialized_size(&__rust.value);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.resource,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.contract,
        ));
        __out.extend(bool::cairo_serialize(&__rust.value));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let resource = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&resource);
        let contract = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&contract);
        let value = bool::cairo_deserialize(__felts, __offset)?;
        __offset += bool::cairo_serialized_size(&value);
        Ok(OwnerUpdated {
            resource,
            contract,
            value,
        })
    }
}
impl OwnerUpdated {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("OwnerUpdated").unwrap()
    }
    pub fn event_name() -> &'static str {
        "OwnerUpdated"
    }
}
#[derive()]
pub struct ResourceMetadata {
    pub resource_id: starknet::core::types::Felt,
    pub metadata_uri: cainome::cairo_serde::ByteArray,
    pub metadata_hash: starknet::core::types::Felt,
}
impl cainome::cairo_serde::CairoSerde for ResourceMetadata {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.resource_id);
        __size += cainome::cairo_serde::ByteArray::cairo_serialized_size(&__rust.metadata_uri);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.metadata_hash);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.resource_id,
        ));
        __out.extend(cainome::cairo_serde::ByteArray::cairo_serialize(
            &__rust.metadata_uri,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.metadata_hash,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let resource_id = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&resource_id);
        let metadata_uri = cainome::cairo_serde::ByteArray::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&metadata_uri);
        let metadata_hash = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&metadata_hash);
        Ok(ResourceMetadata {
            resource_id,
            metadata_uri,
            metadata_hash,
        })
    }
}
#[derive()]
pub struct StoreDelRecord {
    pub selector: starknet::core::types::Felt,
    pub entity_id: starknet::core::types::Felt,
}
impl cainome::cairo_serde::CairoSerde for StoreDelRecord {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.selector);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.entity_id);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.selector,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.entity_id,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let selector = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
        let entity_id = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&entity_id);
        Ok(StoreDelRecord {
            selector,
            entity_id,
        })
    }
}
impl StoreDelRecord {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("StoreDelRecord").unwrap()
    }
    pub fn event_name() -> &'static str {
        "StoreDelRecord"
    }
}
#[derive()]
pub struct StoreSetRecord {
    pub selector: starknet::core::types::Felt,
    pub entity_id: starknet::core::types::Felt,
    pub keys: Vec<starknet::core::types::Felt>,
    pub values: Vec<starknet::core::types::Felt>,
}
impl cainome::cairo_serde::CairoSerde for StoreSetRecord {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.selector);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.entity_id);
        __size += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&__rust.keys);
        __size += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&__rust.values);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.selector,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.entity_id,
        ));
        __out.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            &__rust.keys,
        ));
        __out.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            &__rust.values,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let selector = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
        let entity_id = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&entity_id);
        let keys = Vec::<starknet::core::types::Felt>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&keys);
        let values = Vec::<starknet::core::types::Felt>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&values);
        Ok(StoreSetRecord {
            selector,
            entity_id,
            keys,
            values,
        })
    }
}
impl StoreSetRecord {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("StoreSetRecord").unwrap()
    }
    pub fn event_name() -> &'static str {
        "StoreSetRecord"
    }
}
#[derive()]
pub struct StoreUpdateMember {
    pub selector: starknet::core::types::Felt,
    pub entity_id: starknet::core::types::Felt,
    pub member_selector: starknet::core::types::Felt,
    pub values: Vec<starknet::core::types::Felt>,
}
impl cainome::cairo_serde::CairoSerde for StoreUpdateMember {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.selector);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.entity_id);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.member_selector);
        __size += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&__rust.values);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.selector,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.entity_id,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.member_selector,
        ));
        __out.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            &__rust.values,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let selector = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
        let entity_id = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&entity_id);
        let member_selector = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&member_selector);
        let values = Vec::<starknet::core::types::Felt>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&values);
        Ok(StoreUpdateMember {
            selector,
            entity_id,
            member_selector,
            values,
        })
    }
}
impl StoreUpdateMember {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("StoreUpdateMember").unwrap()
    }
    pub fn event_name() -> &'static str {
        "StoreUpdateMember"
    }
}
#[derive()]
pub struct StoreUpdateRecord {
    pub selector: starknet::core::types::Felt,
    pub entity_id: starknet::core::types::Felt,
    pub values: Vec<starknet::core::types::Felt>,
}
impl cainome::cairo_serde::CairoSerde for StoreUpdateRecord {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.selector);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.entity_id);
        __size += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&__rust.values);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.selector,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.entity_id,
        ));
        __out.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            &__rust.values,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let selector = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
        let entity_id = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&entity_id);
        let values = Vec::<starknet::core::types::Felt>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&values);
        Ok(StoreUpdateRecord {
            selector,
            entity_id,
            values,
        })
    }
}
impl StoreUpdateRecord {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("StoreUpdateRecord").unwrap()
    }
    pub fn event_name() -> &'static str {
        "StoreUpdateRecord"
    }
}
#[derive()]
pub struct WorldSpawned {
    pub creator: cainome::cairo_serde::ContractAddress,
    pub class_hash: cainome::cairo_serde::ClassHash,
}
impl cainome::cairo_serde::CairoSerde for WorldSpawned {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.creator);
        __size += cainome::cairo_serde::ClassHash::cairo_serialized_size(&__rust.class_hash);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.creator,
        ));
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
        let creator = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&creator);
        let class_hash = cainome::cairo_serde::ClassHash::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
        Ok(WorldSpawned {
            creator,
            class_hash,
        })
    }
}
impl WorldSpawned {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("WorldSpawned").unwrap()
    }
    pub fn event_name() -> &'static str {
        "WorldSpawned"
    }
}
#[derive()]
pub struct WorldUpgraded {
    pub class_hash: cainome::cairo_serde::ClassHash,
}
impl cainome::cairo_serde::CairoSerde for WorldUpgraded {
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
        Ok(WorldUpgraded { class_hash })
    }
}
impl WorldUpgraded {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("WorldUpgraded").unwrap()
    }
    pub fn event_name() -> &'static str {
        "WorldUpgraded"
    }
}
#[derive()]
pub struct WriterUpdated {
    pub resource: starknet::core::types::Felt,
    pub contract: cainome::cairo_serde::ContractAddress,
    pub value: bool,
}
impl cainome::cairo_serde::CairoSerde for WriterUpdated {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.resource);
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.contract);
        __size += bool::cairo_serialized_size(&__rust.value);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.resource,
        ));
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.contract,
        ));
        __out.extend(bool::cairo_serialize(&__rust.value));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let resource = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&resource);
        let contract = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&contract);
        let value = bool::cairo_deserialize(__felts, __offset)?;
        __offset += bool::cairo_serialized_size(&value);
        Ok(WriterUpdated {
            resource,
            contract,
            value,
        })
    }
}
impl WriterUpdated {
    pub fn event_selector() -> starknet::core::types::Felt {
        starknet::core::utils::get_selector_from_name("WriterUpdated").unwrap()
    }
    pub fn event_name() -> &'static str {
        "WriterUpdated"
    }
}
#[derive()]
pub enum Event {
    WorldSpawned(WorldSpawned),
    WorldUpgraded(WorldUpgraded),
    NamespaceRegistered(NamespaceRegistered),
    ModelRegistered(ModelRegistered),
    EventRegistered(EventRegistered),
    ContractRegistered(ContractRegistered),
    ModelUpgraded(ModelUpgraded),
    EventUpgraded(EventUpgraded),
    ContractUpgraded(ContractUpgraded),
    ContractInitialized(ContractInitialized),
    EventEmitted(EventEmitted),
    MetadataUpdate(MetadataUpdate),
    StoreSetRecord(StoreSetRecord),
    StoreUpdateRecord(StoreUpdateRecord),
    StoreUpdateMember(StoreUpdateMember),
    StoreDelRecord(StoreDelRecord),
    WriterUpdated(WriterUpdated),
    OwnerUpdated(OwnerUpdated),
}
impl cainome::cairo_serde::CairoSerde for Event {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            Event::WorldSpawned(val) => WorldSpawned::cairo_serialized_size(val) + 1,
            Event::WorldUpgraded(val) => WorldUpgraded::cairo_serialized_size(val) + 1,
            Event::NamespaceRegistered(val) => NamespaceRegistered::cairo_serialized_size(val) + 1,
            Event::ModelRegistered(val) => ModelRegistered::cairo_serialized_size(val) + 1,
            Event::EventRegistered(val) => EventRegistered::cairo_serialized_size(val) + 1,
            Event::ContractRegistered(val) => ContractRegistered::cairo_serialized_size(val) + 1,
            Event::ModelUpgraded(val) => ModelUpgraded::cairo_serialized_size(val) + 1,
            Event::EventUpgraded(val) => EventUpgraded::cairo_serialized_size(val) + 1,
            Event::ContractUpgraded(val) => ContractUpgraded::cairo_serialized_size(val) + 1,
            Event::ContractInitialized(val) => ContractInitialized::cairo_serialized_size(val) + 1,
            Event::EventEmitted(val) => EventEmitted::cairo_serialized_size(val) + 1,
            Event::MetadataUpdate(val) => MetadataUpdate::cairo_serialized_size(val) + 1,
            Event::StoreSetRecord(val) => StoreSetRecord::cairo_serialized_size(val) + 1,
            Event::StoreUpdateRecord(val) => StoreUpdateRecord::cairo_serialized_size(val) + 1,
            Event::StoreUpdateMember(val) => StoreUpdateMember::cairo_serialized_size(val) + 1,
            Event::StoreDelRecord(val) => StoreDelRecord::cairo_serialized_size(val) + 1,
            Event::WriterUpdated(val) => WriterUpdated::cairo_serialized_size(val) + 1,
            Event::OwnerUpdated(val) => OwnerUpdated::cairo_serialized_size(val) + 1,
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            Event::WorldSpawned(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(WorldSpawned::cairo_serialize(val));
                temp
            }
            Event::WorldUpgraded(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(WorldUpgraded::cairo_serialize(val));
                temp
            }
            Event::NamespaceRegistered(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&2usize));
                temp.extend(NamespaceRegistered::cairo_serialize(val));
                temp
            }
            Event::ModelRegistered(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&3usize));
                temp.extend(ModelRegistered::cairo_serialize(val));
                temp
            }
            Event::EventRegistered(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&4usize));
                temp.extend(EventRegistered::cairo_serialize(val));
                temp
            }
            Event::ContractRegistered(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&5usize));
                temp.extend(ContractRegistered::cairo_serialize(val));
                temp
            }
            Event::ModelUpgraded(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&6usize));
                temp.extend(ModelUpgraded::cairo_serialize(val));
                temp
            }
            Event::EventUpgraded(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&7usize));
                temp.extend(EventUpgraded::cairo_serialize(val));
                temp
            }
            Event::ContractUpgraded(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&8usize));
                temp.extend(ContractUpgraded::cairo_serialize(val));
                temp
            }
            Event::ContractInitialized(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&9usize));
                temp.extend(ContractInitialized::cairo_serialize(val));
                temp
            }
            Event::EventEmitted(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&10usize));
                temp.extend(EventEmitted::cairo_serialize(val));
                temp
            }
            Event::MetadataUpdate(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&11usize));
                temp.extend(MetadataUpdate::cairo_serialize(val));
                temp
            }
            Event::StoreSetRecord(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&12usize));
                temp.extend(StoreSetRecord::cairo_serialize(val));
                temp
            }
            Event::StoreUpdateRecord(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&13usize));
                temp.extend(StoreUpdateRecord::cairo_serialize(val));
                temp
            }
            Event::StoreUpdateMember(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&14usize));
                temp.extend(StoreUpdateMember::cairo_serialize(val));
                temp
            }
            Event::StoreDelRecord(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&15usize));
                temp.extend(StoreDelRecord::cairo_serialize(val));
                temp
            }
            Event::WriterUpdated(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&16usize));
                temp.extend(WriterUpdated::cairo_serialize(val));
                temp
            }
            Event::OwnerUpdated(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&17usize));
                temp.extend(OwnerUpdated::cairo_serialize(val));
                temp
            }
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
            0usize => Ok(Event::WorldSpawned(WorldSpawned::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            1usize => Ok(Event::WorldUpgraded(WorldUpgraded::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            2usize => Ok(Event::NamespaceRegistered(
                NamespaceRegistered::cairo_deserialize(__felts, __offset + 1)?,
            )),
            3usize => Ok(Event::ModelRegistered(ModelRegistered::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            4usize => Ok(Event::EventRegistered(EventRegistered::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            5usize => Ok(Event::ContractRegistered(
                ContractRegistered::cairo_deserialize(__felts, __offset + 1)?,
            )),
            6usize => Ok(Event::ModelUpgraded(ModelUpgraded::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            7usize => Ok(Event::EventUpgraded(EventUpgraded::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            8usize => Ok(Event::ContractUpgraded(
                ContractUpgraded::cairo_deserialize(__felts, __offset + 1)?,
            )),
            9usize => Ok(Event::ContractInitialized(
                ContractInitialized::cairo_deserialize(__felts, __offset + 1)?,
            )),
            10usize => Ok(Event::EventEmitted(EventEmitted::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            11usize => Ok(Event::MetadataUpdate(MetadataUpdate::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            12usize => Ok(Event::StoreSetRecord(StoreSetRecord::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            13usize => Ok(Event::StoreUpdateRecord(
                StoreUpdateRecord::cairo_deserialize(__felts, __offset + 1)?,
            )),
            14usize => Ok(Event::StoreUpdateMember(
                StoreUpdateMember::cairo_deserialize(__felts, __offset + 1)?,
            )),
            15usize => Ok(Event::StoreDelRecord(StoreDelRecord::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            16usize => Ok(Event::WriterUpdated(WriterUpdated::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            17usize => Ok(Event::OwnerUpdated(OwnerUpdated::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
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
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("WorldSpawned")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "WorldSpawned"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let creator = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "creator", "WorldSpawned", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&creator);
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "WorldSpawned", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            return Ok(Event::WorldSpawned(WorldSpawned {
                creator,
                class_hash,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("WorldUpgraded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "WorldUpgraded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "WorldUpgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            return Ok(Event::WorldUpgraded(WorldUpgraded { class_hash }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("NamespaceRegistered")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "NamespaceRegistered"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let namespace =
                match cainome::cairo_serde::ByteArray::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "namespace", "NamespaceRegistered", e
                        ))
                    }
                };
            key_offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&namespace);
            let hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "hash", "NamespaceRegistered", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&hash);
            return Ok(Event::NamespaceRegistered(NamespaceRegistered {
                namespace,
                hash,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ModelRegistered")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ModelRegistered"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let name =
                match cainome::cairo_serde::ByteArray::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "name", "ModelRegistered", e
                        ))
                    }
                };
            key_offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&name);
            let namespace =
                match cainome::cairo_serde::ByteArray::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "namespace", "ModelRegistered", e
                        ))
                    }
                };
            key_offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&namespace);
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "ModelRegistered", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            let address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "address", "ModelRegistered", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
            return Ok(Event::ModelRegistered(ModelRegistered {
                name,
                namespace,
                class_hash,
                address,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("EventRegistered")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "EventRegistered"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let name =
                match cainome::cairo_serde::ByteArray::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "name", "EventRegistered", e
                        ))
                    }
                };
            key_offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&name);
            let namespace =
                match cainome::cairo_serde::ByteArray::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "namespace", "EventRegistered", e
                        ))
                    }
                };
            key_offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&namespace);
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "EventRegistered", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            let address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "address", "EventRegistered", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
            return Ok(Event::EventRegistered(EventRegistered {
                name,
                namespace,
                class_hash,
                address,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ContractRegistered")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ContractRegistered"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let name =
                match cainome::cairo_serde::ByteArray::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "name", "ContractRegistered", e
                        ))
                    }
                };
            key_offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&name);
            let namespace =
                match cainome::cairo_serde::ByteArray::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "namespace", "ContractRegistered", e
                        ))
                    }
                };
            key_offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&namespace);
            let address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "address", "ContractRegistered", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "ContractRegistered", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            let salt =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "salt", "ContractRegistered", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&salt);
            return Ok(Event::ContractRegistered(ContractRegistered {
                name,
                namespace,
                address,
                class_hash,
                salt,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ModelUpgraded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ModelUpgraded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "ModelUpgraded", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "ModelUpgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            let address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "address", "ModelUpgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
            let prev_address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "prev_address", "ModelUpgraded", e
                    ))
                }
            };
            data_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&prev_address);
            return Ok(Event::ModelUpgraded(ModelUpgraded {
                selector,
                class_hash,
                address,
                prev_address,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("EventUpgraded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "EventUpgraded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "EventUpgraded", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "EventUpgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            let address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "address", "EventUpgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
            let prev_address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "prev_address", "EventUpgraded", e
                    ))
                }
            };
            data_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&prev_address);
            return Ok(Event::EventUpgraded(EventUpgraded {
                selector,
                class_hash,
                address,
                prev_address,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ContractUpgraded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ContractUpgraded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "ContractUpgraded", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "ContractUpgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            return Ok(Event::ContractUpgraded(ContractUpgraded {
                selector,
                class_hash,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ContractInitialized")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ContractInitialized"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "ContractInitialized", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let init_calldata = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "init_calldata", "ContractInitialized", e
                    ))
                }
            };
            data_offset +=
                Vec::<starknet::core::types::Felt>::cairo_serialized_size(&init_calldata);
            return Ok(Event::ContractInitialized(ContractInitialized {
                selector,
                init_calldata,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("EventEmitted")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "EventEmitted"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "EventEmitted", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let system_address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "system_address", "EventEmitted", e
                    ))
                }
            };
            key_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&system_address);
            let keys = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "keys", "EventEmitted", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&keys);
            let values = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "values", "EventEmitted", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&values);
            return Ok(Event::EventEmitted(EventEmitted {
                selector,
                system_address,
                keys,
                values,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MetadataUpdate")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MetadataUpdate"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let resource =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "resource", "MetadataUpdate", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&resource);
            let uri = match cainome::cairo_serde::ByteArray::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "uri", "MetadataUpdate", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&uri);
            let hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "hash", "MetadataUpdate", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&hash);
            return Ok(Event::MetadataUpdate(MetadataUpdate {
                resource,
                uri,
                hash,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("StoreSetRecord")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "StoreSetRecord"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "StoreSetRecord", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let entity_id =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "entity_id", "StoreSetRecord", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&entity_id);
            let keys = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "keys", "StoreSetRecord", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&keys);
            let values = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "values", "StoreSetRecord", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&values);
            return Ok(Event::StoreSetRecord(StoreSetRecord {
                selector,
                entity_id,
                keys,
                values,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("StoreUpdateRecord")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "StoreUpdateRecord"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "StoreUpdateRecord", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let entity_id =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "entity_id", "StoreUpdateRecord", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&entity_id);
            let values = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "values", "StoreUpdateRecord", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&values);
            return Ok(Event::StoreUpdateRecord(StoreUpdateRecord {
                selector,
                entity_id,
                values,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("StoreUpdateMember")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "StoreUpdateMember"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "StoreUpdateMember", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let entity_id =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "entity_id", "StoreUpdateMember", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&entity_id);
            let member_selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "member_selector", "StoreUpdateMember", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&member_selector);
            let values = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "values", "StoreUpdateMember", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&values);
            return Ok(Event::StoreUpdateMember(StoreUpdateMember {
                selector,
                entity_id,
                member_selector,
                values,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("StoreDelRecord")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "StoreDelRecord"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "StoreDelRecord", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let entity_id =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "entity_id", "StoreDelRecord", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&entity_id);
            return Ok(Event::StoreDelRecord(StoreDelRecord {
                selector,
                entity_id,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("WriterUpdated")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "WriterUpdated"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let resource =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "resource", "WriterUpdated", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&resource);
            let contract = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "contract", "WriterUpdated", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&contract);
            let value = match bool::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "value", "WriterUpdated", e
                    ))
                }
            };
            data_offset += bool::cairo_serialized_size(&value);
            return Ok(Event::WriterUpdated(WriterUpdated {
                resource,
                contract,
                value,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnerUpdated")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "OwnerUpdated"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let resource =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "resource", "OwnerUpdated", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&resource);
            let contract = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "contract", "OwnerUpdated", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&contract);
            let value = match bool::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "value", "OwnerUpdated", e
                    ))
                }
            };
            data_offset += bool::cairo_serialized_size(&value);
            return Ok(Event::OwnerUpdated(OwnerUpdated {
                resource,
                contract,
                value,
            }));
        };
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
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("WorldSpawned")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "WorldSpawned"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let creator = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "creator", "WorldSpawned", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&creator);
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "WorldSpawned", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            return Ok(Event::WorldSpawned(WorldSpawned {
                creator,
                class_hash,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("WorldUpgraded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "WorldUpgraded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "WorldUpgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            return Ok(Event::WorldUpgraded(WorldUpgraded { class_hash }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("NamespaceRegistered")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "NamespaceRegistered"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let namespace =
                match cainome::cairo_serde::ByteArray::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "namespace", "NamespaceRegistered", e
                        ))
                    }
                };
            key_offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&namespace);
            let hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "hash", "NamespaceRegistered", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&hash);
            return Ok(Event::NamespaceRegistered(NamespaceRegistered {
                namespace,
                hash,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ModelRegistered")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ModelRegistered"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let name =
                match cainome::cairo_serde::ByteArray::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "name", "ModelRegistered", e
                        ))
                    }
                };
            key_offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&name);
            let namespace =
                match cainome::cairo_serde::ByteArray::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "namespace", "ModelRegistered", e
                        ))
                    }
                };
            key_offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&namespace);
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "ModelRegistered", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            let address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "address", "ModelRegistered", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
            return Ok(Event::ModelRegistered(ModelRegistered {
                name,
                namespace,
                class_hash,
                address,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("EventRegistered")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "EventRegistered"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let name =
                match cainome::cairo_serde::ByteArray::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "name", "EventRegistered", e
                        ))
                    }
                };
            key_offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&name);
            let namespace =
                match cainome::cairo_serde::ByteArray::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "namespace", "EventRegistered", e
                        ))
                    }
                };
            key_offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&namespace);
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "EventRegistered", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            let address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "address", "EventRegistered", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
            return Ok(Event::EventRegistered(EventRegistered {
                name,
                namespace,
                class_hash,
                address,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ContractRegistered")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ContractRegistered"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let name =
                match cainome::cairo_serde::ByteArray::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "name", "ContractRegistered", e
                        ))
                    }
                };
            key_offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&name);
            let namespace =
                match cainome::cairo_serde::ByteArray::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "namespace", "ContractRegistered", e
                        ))
                    }
                };
            key_offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&namespace);
            let address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "address", "ContractRegistered", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "ContractRegistered", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            let salt =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "salt", "ContractRegistered", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&salt);
            return Ok(Event::ContractRegistered(ContractRegistered {
                name,
                namespace,
                address,
                class_hash,
                salt,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ModelUpgraded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ModelUpgraded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "ModelUpgraded", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "ModelUpgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            let address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "address", "ModelUpgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
            let prev_address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "prev_address", "ModelUpgraded", e
                    ))
                }
            };
            data_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&prev_address);
            return Ok(Event::ModelUpgraded(ModelUpgraded {
                selector,
                class_hash,
                address,
                prev_address,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("EventUpgraded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "EventUpgraded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "EventUpgraded", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "EventUpgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            let address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "address", "EventUpgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
            let prev_address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "prev_address", "EventUpgraded", e
                    ))
                }
            };
            data_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&prev_address);
            return Ok(Event::EventUpgraded(EventUpgraded {
                selector,
                class_hash,
                address,
                prev_address,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ContractUpgraded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ContractUpgraded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "ContractUpgraded", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "ContractUpgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            return Ok(Event::ContractUpgraded(ContractUpgraded {
                selector,
                class_hash,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ContractInitialized")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ContractInitialized"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "ContractInitialized", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let init_calldata = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "init_calldata", "ContractInitialized", e
                    ))
                }
            };
            data_offset +=
                Vec::<starknet::core::types::Felt>::cairo_serialized_size(&init_calldata);
            return Ok(Event::ContractInitialized(ContractInitialized {
                selector,
                init_calldata,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("EventEmitted")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "EventEmitted"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "EventEmitted", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let system_address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "system_address", "EventEmitted", e
                    ))
                }
            };
            key_offset +=
                cainome::cairo_serde::ContractAddress::cairo_serialized_size(&system_address);
            let keys = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "keys", "EventEmitted", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&keys);
            let values = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "values", "EventEmitted", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&values);
            return Ok(Event::EventEmitted(EventEmitted {
                selector,
                system_address,
                keys,
                values,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("MetadataUpdate")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "MetadataUpdate"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let resource =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "resource", "MetadataUpdate", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&resource);
            let uri = match cainome::cairo_serde::ByteArray::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "uri", "MetadataUpdate", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&uri);
            let hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "hash", "MetadataUpdate", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&hash);
            return Ok(Event::MetadataUpdate(MetadataUpdate {
                resource,
                uri,
                hash,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("StoreSetRecord")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "StoreSetRecord"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "StoreSetRecord", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let entity_id =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "entity_id", "StoreSetRecord", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&entity_id);
            let keys = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "keys", "StoreSetRecord", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&keys);
            let values = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "values", "StoreSetRecord", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&values);
            return Ok(Event::StoreSetRecord(StoreSetRecord {
                selector,
                entity_id,
                keys,
                values,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("StoreUpdateRecord")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "StoreUpdateRecord"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "StoreUpdateRecord", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let entity_id =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "entity_id", "StoreUpdateRecord", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&entity_id);
            let values = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "values", "StoreUpdateRecord", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&values);
            return Ok(Event::StoreUpdateRecord(StoreUpdateRecord {
                selector,
                entity_id,
                values,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("StoreUpdateMember")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "StoreUpdateMember"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "StoreUpdateMember", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let entity_id =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "entity_id", "StoreUpdateMember", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&entity_id);
            let member_selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "member_selector", "StoreUpdateMember", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&member_selector);
            let values = match Vec::<starknet::core::types::Felt>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "values", "StoreUpdateMember", e
                    ))
                }
            };
            data_offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&values);
            return Ok(Event::StoreUpdateMember(StoreUpdateMember {
                selector,
                entity_id,
                member_selector,
                values,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("StoreDelRecord")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "StoreDelRecord"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let selector =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "selector", "StoreDelRecord", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
            let entity_id =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "entity_id", "StoreDelRecord", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&entity_id);
            return Ok(Event::StoreDelRecord(StoreDelRecord {
                selector,
                entity_id,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("WriterUpdated")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "WriterUpdated"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let resource =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "resource", "WriterUpdated", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&resource);
            let contract = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "contract", "WriterUpdated", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&contract);
            let value = match bool::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "value", "WriterUpdated", e
                    ))
                }
            };
            data_offset += bool::cairo_serialized_size(&value);
            return Ok(Event::WriterUpdated(WriterUpdated {
                resource,
                contract,
                value,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnerUpdated")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "OwnerUpdated"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let resource =
                match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "resource", "OwnerUpdated", e
                        ))
                    }
                };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&resource);
            let contract = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "contract", "OwnerUpdated", e
                    ))
                }
            };
            key_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&contract);
            let value = match bool::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "value", "OwnerUpdated", e
                    ))
                }
            };
            data_offset += bool::cairo_serialized_size(&value);
            return Ok(Event::OwnerUpdated(OwnerUpdated {
                resource,
                contract,
                value,
            }));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive()]
pub enum Layout {
    Fixed(Vec<u8>),
    Struct(Vec<FieldLayout>),
    Tuple(Vec<Layout>),
    Array(Vec<Layout>),
    ByteArray,
    Enum(Vec<FieldLayout>),
}
impl cainome::cairo_serde::CairoSerde for Layout {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            Layout::Fixed(val) => Vec::<u8>::cairo_serialized_size(val) + 1,
            Layout::Struct(val) => Vec::<FieldLayout>::cairo_serialized_size(val) + 1,
            Layout::Tuple(val) => Vec::<Layout>::cairo_serialized_size(val) + 1,
            Layout::Array(val) => Vec::<Layout>::cairo_serialized_size(val) + 1,
            Layout::ByteArray => 1,
            Layout::Enum(val) => Vec::<FieldLayout>::cairo_serialized_size(val) + 1,
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            Layout::Fixed(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(Vec::<u8>::cairo_serialize(val));
                temp
            }
            Layout::Struct(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(Vec::<FieldLayout>::cairo_serialize(val));
                temp
            }
            Layout::Tuple(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&2usize));
                temp.extend(Vec::<Layout>::cairo_serialize(val));
                temp
            }
            Layout::Array(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&3usize));
                temp.extend(Vec::<Layout>::cairo_serialize(val));
                temp
            }
            Layout::ByteArray => usize::cairo_serialize(&4usize),
            Layout::Enum(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&5usize));
                temp.extend(Vec::<FieldLayout>::cairo_serialize(val));
                temp
            }
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
            0usize => Ok(Layout::Fixed(Vec::<u8>::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            1usize => Ok(Layout::Struct(Vec::<FieldLayout>::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            2usize => Ok(Layout::Tuple(Vec::<Layout>::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            3usize => Ok(Layout::Array(Vec::<Layout>::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            4usize => Ok(Layout::ByteArray),
            5usize => Ok(Layout::Enum(Vec::<FieldLayout>::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "Layout"
                )))
            }
        }
    }
}
#[derive()]
pub enum ModelIndex {
    Keys(Vec<starknet::core::types::Felt>),
    Id(starknet::core::types::Felt),
    MemberId((starknet::core::types::Felt, starknet::core::types::Felt)),
}
impl cainome::cairo_serde::CairoSerde for ModelIndex {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            ModelIndex::Keys(val) => {
                Vec::<starknet::core::types::Felt>::cairo_serialized_size(val) + 1
            }
            ModelIndex::Id(val) => starknet::core::types::Felt::cairo_serialized_size(val) + 1,
            ModelIndex::MemberId(val) => {
                <(starknet::core::types::Felt, starknet::core::types::Felt)>::cairo_serialized_size(
                    val,
                ) + 1
            }
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            ModelIndex::Keys(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(val));
                temp
            }
            ModelIndex::Id(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(starknet::core::types::Felt::cairo_serialize(val));
                temp
            }
            ModelIndex::MemberId(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&2usize));
                temp.extend(
                    <(starknet::core::types::Felt, starknet::core::types::Felt)>::cairo_serialize(
                        val,
                    ),
                );
                temp
            }
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
            0usize => Ok(ModelIndex::Keys(
                Vec::<starknet::core::types::Felt>::cairo_deserialize(__felts, __offset + 1)?,
            )),
            1usize => Ok(ModelIndex::Id(
                starknet::core::types::Felt::cairo_deserialize(__felts, __offset + 1)?,
            )),
            2usize => Ok(ModelIndex::MemberId(<(
                starknet::core::types::Felt,
                starknet::core::types::Felt,
            )>::cairo_deserialize(
                __felts, __offset + 1
            )?)),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "ModelIndex"
                )))
            }
        }
    }
}
#[derive()]
pub enum Resource {
    Model(
        (
            cainome::cairo_serde::ContractAddress,
            starknet::core::types::Felt,
        ),
    ),
    Event(
        (
            cainome::cairo_serde::ContractAddress,
            starknet::core::types::Felt,
        ),
    ),
    Contract(
        (
            cainome::cairo_serde::ContractAddress,
            starknet::core::types::Felt,
        ),
    ),
    Namespace(cainome::cairo_serde::ByteArray),
    World,
    Unregistered,
}
impl cainome::cairo_serde::CairoSerde for Resource {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            Resource::Model(val) => {
                <(
                    cainome::cairo_serde::ContractAddress,
                    starknet::core::types::Felt,
                )>::cairo_serialized_size(val)
                    + 1
            }
            Resource::Event(val) => {
                <(
                    cainome::cairo_serde::ContractAddress,
                    starknet::core::types::Felt,
                )>::cairo_serialized_size(val)
                    + 1
            }
            Resource::Contract(val) => {
                <(
                    cainome::cairo_serde::ContractAddress,
                    starknet::core::types::Felt,
                )>::cairo_serialized_size(val)
                    + 1
            }
            Resource::Namespace(val) => {
                cainome::cairo_serde::ByteArray::cairo_serialized_size(val) + 1
            }
            Resource::World => 1,
            Resource::Unregistered => 1,
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            Resource::Model(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(<(
                    cainome::cairo_serde::ContractAddress,
                    starknet::core::types::Felt,
                )>::cairo_serialize(val));
                temp
            }
            Resource::Event(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(<(
                    cainome::cairo_serde::ContractAddress,
                    starknet::core::types::Felt,
                )>::cairo_serialize(val));
                temp
            }
            Resource::Contract(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&2usize));
                temp.extend(<(
                    cainome::cairo_serde::ContractAddress,
                    starknet::core::types::Felt,
                )>::cairo_serialize(val));
                temp
            }
            Resource::Namespace(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&3usize));
                temp.extend(cainome::cairo_serde::ByteArray::cairo_serialize(val));
                temp
            }
            Resource::World => usize::cairo_serialize(&4usize),
            Resource::Unregistered => usize::cairo_serialize(&5usize),
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
            0usize => Ok(Resource::Model(<(
                cainome::cairo_serde::ContractAddress,
                starknet::core::types::Felt,
            )>::cairo_deserialize(
                __felts, __offset + 1
            )?)),
            1usize => Ok(Resource::Event(<(
                cainome::cairo_serde::ContractAddress,
                starknet::core::types::Felt,
            )>::cairo_deserialize(
                __felts, __offset + 1
            )?)),
            2usize => Ok(Resource::Contract(<(
                cainome::cairo_serde::ContractAddress,
                starknet::core::types::Felt,
            )>::cairo_deserialize(
                __felts, __offset + 1
            )?)),
            3usize => Ok(Resource::Namespace(
                cainome::cairo_serde::ByteArray::cairo_deserialize(__felts, __offset + 1)?,
            )),
            4usize => Ok(Resource::World),
            5usize => Ok(Resource::Unregistered),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "Resource"
                )))
            }
        }
    }
}
impl<A: starknet::accounts::ConnectedAccount + Sync> CrosswordleWorld<A> {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn entities(
        &self,
        model_selector: &starknet::core::types::Felt,
        indexes: &Vec<ModelIndex>,
        layout: &Layout,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, Vec<Vec<starknet::core::types::Felt>>> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(model_selector));
        __calldata.extend(Vec::<ModelIndex>::cairo_serialize(indexes));
        __calldata.extend(Layout::cairo_serialize(layout));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("entities"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn entity(
        &self,
        model_selector: &starknet::core::types::Felt,
        index: &ModelIndex,
        layout: &Layout,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, Vec<starknet::core::types::Felt>> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(model_selector));
        __calldata.extend(ModelIndex::cairo_serialize(index));
        __calldata.extend(Layout::cairo_serialize(layout));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("entity"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_owner(
        &self,
        resource: &starknet::core::types::Felt,
        address: &cainome::cairo_serde::ContractAddress,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(resource));
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            address,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_owner"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_writer(
        &self,
        resource: &starknet::core::types::Felt,
        contract: &cainome::cairo_serde::ContractAddress,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(resource));
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            contract,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_writer"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn metadata(
        &self,
        resource_selector: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, ResourceMetadata> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(
            resource_selector,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("metadata"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn resource(
        &self,
        selector: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, Resource> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(selector));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("resource"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn delete_entities_getcall(
        &self,
        model_selector: &starknet::core::types::Felt,
        indexes: &Vec<ModelIndex>,
        layout: &Layout,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(model_selector));
        __calldata.extend(Vec::<ModelIndex>::cairo_serialize(indexes));
        __calldata.extend(Layout::cairo_serialize(layout));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("delete_entities"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn delete_entities(
        &self,
        model_selector: &starknet::core::types::Felt,
        indexes: &Vec<ModelIndex>,
        layout: &Layout,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(model_selector));
        __calldata.extend(Vec::<ModelIndex>::cairo_serialize(indexes));
        __calldata.extend(Layout::cairo_serialize(layout));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("delete_entities"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn delete_entity_getcall(
        &self,
        model_selector: &starknet::core::types::Felt,
        index: &ModelIndex,
        layout: &Layout,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(model_selector));
        __calldata.extend(ModelIndex::cairo_serialize(index));
        __calldata.extend(Layout::cairo_serialize(layout));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("delete_entity"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn delete_entity(
        &self,
        model_selector: &starknet::core::types::Felt,
        index: &ModelIndex,
        layout: &Layout,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(model_selector));
        __calldata.extend(ModelIndex::cairo_serialize(index));
        __calldata.extend(Layout::cairo_serialize(layout));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("delete_entity"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn emit_event_getcall(
        &self,
        event_selector: &starknet::core::types::Felt,
        keys: &Vec<starknet::core::types::Felt>,
        values: &Vec<starknet::core::types::Felt>,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(event_selector));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(keys));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(values));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("emit_event"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn emit_event(
        &self,
        event_selector: &starknet::core::types::Felt,
        keys: &Vec<starknet::core::types::Felt>,
        values: &Vec<starknet::core::types::Felt>,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(event_selector));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(keys));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(values));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("emit_event"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn emit_events_getcall(
        &self,
        event_selector: &starknet::core::types::Felt,
        keys: &Vec<Vec<starknet::core::types::Felt>>,
        values: &Vec<Vec<starknet::core::types::Felt>>,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(event_selector));
        __calldata.extend(Vec::<Vec<starknet::core::types::Felt>>::cairo_serialize(
            keys,
        ));
        __calldata.extend(Vec::<Vec<starknet::core::types::Felt>>::cairo_serialize(
            values,
        ));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("emit_events"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn emit_events(
        &self,
        event_selector: &starknet::core::types::Felt,
        keys: &Vec<Vec<starknet::core::types::Felt>>,
        values: &Vec<Vec<starknet::core::types::Felt>>,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(event_selector));
        __calldata.extend(Vec::<Vec<starknet::core::types::Felt>>::cairo_serialize(
            keys,
        ));
        __calldata.extend(Vec::<Vec<starknet::core::types::Felt>>::cairo_serialize(
            values,
        ));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("emit_events"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn grant_owner_getcall(
        &self,
        resource: &starknet::core::types::Felt,
        address: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(resource));
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            address,
        ));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("grant_owner"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn grant_owner(
        &self,
        resource: &starknet::core::types::Felt,
        address: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(resource));
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            address,
        ));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("grant_owner"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn grant_writer_getcall(
        &self,
        resource: &starknet::core::types::Felt,
        contract: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(resource));
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            contract,
        ));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("grant_writer"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn grant_writer(
        &self,
        resource: &starknet::core::types::Felt,
        contract: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(resource));
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            contract,
        ));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("grant_writer"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn init_contract_getcall(
        &self,
        selector: &starknet::core::types::Felt,
        init_calldata: &Vec<starknet::core::types::Felt>,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(selector));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            init_calldata,
        ));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("init_contract"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn init_contract(
        &self,
        selector: &starknet::core::types::Felt,
        init_calldata: &Vec<starknet::core::types::Felt>,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(selector));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            init_calldata,
        ));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("init_contract"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn register_contract_getcall(
        &self,
        salt: &starknet::core::types::Felt,
        namespace: &cainome::cairo_serde::ByteArray,
        class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(salt));
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(namespace));
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(class_hash));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("register_contract"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn register_contract(
        &self,
        salt: &starknet::core::types::Felt,
        namespace: &cainome::cairo_serde::ByteArray,
        class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(salt));
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(namespace));
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(class_hash));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("register_contract"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn register_event_getcall(
        &self,
        namespace: &cainome::cairo_serde::ByteArray,
        class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(namespace));
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(class_hash));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("register_event"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn register_event(
        &self,
        namespace: &cainome::cairo_serde::ByteArray,
        class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(namespace));
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(class_hash));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("register_event"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn register_model_getcall(
        &self,
        namespace: &cainome::cairo_serde::ByteArray,
        class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(namespace));
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(class_hash));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("register_model"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn register_model(
        &self,
        namespace: &cainome::cairo_serde::ByteArray,
        class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(namespace));
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(class_hash));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("register_model"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn register_namespace_getcall(
        &self,
        namespace: &cainome::cairo_serde::ByteArray,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(namespace));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("register_namespace"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn register_namespace(
        &self,
        namespace: &cainome::cairo_serde::ByteArray,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(namespace));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("register_namespace"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn revoke_owner_getcall(
        &self,
        resource: &starknet::core::types::Felt,
        address: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(resource));
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            address,
        ));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("revoke_owner"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn revoke_owner(
        &self,
        resource: &starknet::core::types::Felt,
        address: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(resource));
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            address,
        ));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("revoke_owner"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn revoke_writer_getcall(
        &self,
        resource: &starknet::core::types::Felt,
        contract: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(resource));
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            contract,
        ));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("revoke_writer"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn revoke_writer(
        &self,
        resource: &starknet::core::types::Felt,
        contract: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(resource));
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            contract,
        ));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("revoke_writer"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn set_entities_getcall(
        &self,
        model_selector: &starknet::core::types::Felt,
        indexes: &Vec<ModelIndex>,
        values: &Vec<Vec<starknet::core::types::Felt>>,
        layout: &Layout,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(model_selector));
        __calldata.extend(Vec::<ModelIndex>::cairo_serialize(indexes));
        __calldata.extend(Vec::<Vec<starknet::core::types::Felt>>::cairo_serialize(
            values,
        ));
        __calldata.extend(Layout::cairo_serialize(layout));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_entities"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn set_entities(
        &self,
        model_selector: &starknet::core::types::Felt,
        indexes: &Vec<ModelIndex>,
        values: &Vec<Vec<starknet::core::types::Felt>>,
        layout: &Layout,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(model_selector));
        __calldata.extend(Vec::<ModelIndex>::cairo_serialize(indexes));
        __calldata.extend(Vec::<Vec<starknet::core::types::Felt>>::cairo_serialize(
            values,
        ));
        __calldata.extend(Layout::cairo_serialize(layout));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_entities"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn set_entity_getcall(
        &self,
        model_selector: &starknet::core::types::Felt,
        index: &ModelIndex,
        values: &Vec<starknet::core::types::Felt>,
        layout: &Layout,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(model_selector));
        __calldata.extend(ModelIndex::cairo_serialize(index));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(values));
        __calldata.extend(Layout::cairo_serialize(layout));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_entity"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn set_entity(
        &self,
        model_selector: &starknet::core::types::Felt,
        index: &ModelIndex,
        values: &Vec<starknet::core::types::Felt>,
        layout: &Layout,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(model_selector));
        __calldata.extend(ModelIndex::cairo_serialize(index));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(values));
        __calldata.extend(Layout::cairo_serialize(layout));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_entity"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn set_metadata_getcall(&self, metadata: &ResourceMetadata) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(ResourceMetadata::cairo_serialize(metadata));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_metadata"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn set_metadata(&self, metadata: &ResourceMetadata) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(ResourceMetadata::cairo_serialize(metadata));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_metadata"),
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
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn upgrade_contract_getcall(
        &self,
        namespace: &cainome::cairo_serde::ByteArray,
        class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(namespace));
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(class_hash));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("upgrade_contract"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn upgrade_contract(
        &self,
        namespace: &cainome::cairo_serde::ByteArray,
        class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(namespace));
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(class_hash));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("upgrade_contract"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn upgrade_event_getcall(
        &self,
        namespace: &cainome::cairo_serde::ByteArray,
        class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(namespace));
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(class_hash));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("upgrade_event"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn upgrade_event(
        &self,
        namespace: &cainome::cairo_serde::ByteArray,
        class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(namespace));
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(class_hash));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("upgrade_event"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn upgrade_model_getcall(
        &self,
        namespace: &cainome::cairo_serde::ByteArray,
        class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(namespace));
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(class_hash));
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("upgrade_model"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn upgrade_model(
        &self,
        namespace: &cainome::cairo_serde::ByteArray,
        class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(namespace));
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(class_hash));
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("upgrade_model"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn uuid_getcall(&self) -> starknet::core::types::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("uuid"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn uuid(&self) -> starknet::accounts::ExecutionV3<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::Call {
            to: self.address,
            selector: starknet::macros::selector!("uuid"),
            calldata: __calldata,
        };
        self.account.execute_v3(vec![__call])
    }
}
impl<P: starknet::providers::Provider + Sync> CrosswordleWorldReader<P> {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn entities(
        &self,
        model_selector: &starknet::core::types::Felt,
        indexes: &Vec<ModelIndex>,
        layout: &Layout,
    ) -> cainome::cairo_serde::call::FCall<P, Vec<Vec<starknet::core::types::Felt>>> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(model_selector));
        __calldata.extend(Vec::<ModelIndex>::cairo_serialize(indexes));
        __calldata.extend(Layout::cairo_serialize(layout));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("entities"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn entity(
        &self,
        model_selector: &starknet::core::types::Felt,
        index: &ModelIndex,
        layout: &Layout,
    ) -> cainome::cairo_serde::call::FCall<P, Vec<starknet::core::types::Felt>> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(model_selector));
        __calldata.extend(ModelIndex::cairo_serialize(index));
        __calldata.extend(Layout::cairo_serialize(layout));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("entity"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_owner(
        &self,
        resource: &starknet::core::types::Felt,
        address: &cainome::cairo_serde::ContractAddress,
    ) -> cainome::cairo_serde::call::FCall<P, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(resource));
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            address,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_owner"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_writer(
        &self,
        resource: &starknet::core::types::Felt,
        contract: &cainome::cairo_serde::ContractAddress,
    ) -> cainome::cairo_serde::call::FCall<P, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(resource));
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            contract,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_writer"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn metadata(
        &self,
        resource_selector: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<P, ResourceMetadata> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(
            resource_selector,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("metadata"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn resource(
        &self,
        selector: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<P, Resource> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(selector));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("resource"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
}
