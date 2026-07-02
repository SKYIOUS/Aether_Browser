pub(crate) use std::format;

pub(crate) type String = std::string::String;
pub(crate) type DefaultCheapStr = String;
pub(crate) type Map<K, V> = std::collections::HashMap<K, V, std::collections::hash_map::RandomState>;
pub(crate) type Vec<A> = std::vec::Vec<A>;
pub(crate) type ChildrenVec<A> = std::vec::Vec<A>;
pub(crate) type GridTrackVec<A> = std::vec::Vec<A>;

pub(crate) fn new_vec_with_capacity<A>(capacity: usize) -> Vec<A> {
    Vec::with_capacity(capacity)
}

pub(crate) fn single_value_vec<A>(value: A) -> Vec<A> {
    vec![value]
}

pub(crate) fn round(value: f32) -> f32 {
    (value + 0.5).floor()
}

pub(crate) fn ceil(value: f32) -> f32 {
    value.ceil()
}

pub(crate) fn floor(value: f32) -> f32 {
    value.floor()
}

pub(crate) fn abs(value: f32) -> f32 {
    value.abs()
}

pub(crate) fn f32_max(a: f32, b: f32) -> f32 {
    a.max(b)
}

pub(crate) fn f32_min(a: f32, b: f32) -> f32 {
    a.min(b)
}
