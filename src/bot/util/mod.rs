use {
    crate::bot::util::mapping::ExpiringHashMap,
    lazy_static::lazy_static,
};

pub mod mapping;

lazy_static! {
    pub static ref MEDIA_GROUP_MAPPING: ExpiringHashMap = ExpiringHashMap::new();
}
