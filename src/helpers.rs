use std::borrow::Cow;

use serde::{Deserialize, Deserializer};
use serde::{Serialize, Serializer};

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum NumlikeObject<T> {
    Object(T),
    Number(usize),
}

impl<T> NumlikeObject<T> {
    fn into_inner(self) -> T
    where
        T: From<usize>,
    {
        match self {
            Self::Object(t) => t,
            Self::Number(s) => T::from(s),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum StringyObject<'a, T> {
    Object(T),
    #[serde(borrow)]
    String(Cow<'a, str>),
}

impl<'a, T> StringyObject<'a, T> {
    fn into_inner(self) -> T
    where
        T: From<Cow<'a, str>>,
    {
        match self {
            Self::Object(t) => t,
            Self::String(s) => T::from(s),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub(crate) enum FlattenedVec<T> {
    Vec(Vec<T>),
    Singleton(T),
}

pub(crate) fn deserialize_flattened_vec<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + 'de,
{
    match FlattenedVec::<T>::deserialize(deserializer)? {
        FlattenedVec::Singleton(t) => Ok(vec![t]),
        FlattenedVec::Vec(vs) => Ok(vs),
    }
}

pub(crate) fn deserialize_flattened_vec_numlike<'de, T, D>(
    deserializer: D,
) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: From<usize> + Deserialize<'de> + 'de,
{
    match FlattenedVec::<NumlikeObject<T>>::deserialize(deserializer)? {
        FlattenedVec::Singleton(NumlikeObject::Object(t)) => Ok(vec![t]),
        FlattenedVec::Singleton(NumlikeObject::Number(t)) => Ok(vec![T::from(t)]),
        FlattenedVec::Vec(vs) => Ok(vs.into_iter().map(|o| o.into_inner()).collect()),
    }
}

pub(crate) fn deserialize_flattened_vec_stringy<'de, T, D>(
    deserializer: D,
) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: From<Cow<'de, str>> + Deserialize<'de> + 'de,
{
    match FlattenedVec::<StringyObject<T>>::deserialize(deserializer)? {
        FlattenedVec::Singleton(StringyObject::Object(t)) => Ok(vec![t]),
        FlattenedVec::Singleton(StringyObject::String(t)) => Ok(vec![T::from(t)]),
        FlattenedVec::Vec(vs) => Ok(vs.into_iter().map(|o| o.into_inner()).collect()),
    }
}

pub(crate) fn serialize_flattened_vec<T, S>(
    input: &Vec<T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    if input.len() == 1 {
        input[0].serialize(serializer)
    } else {
        input.serialize(serializer)
    }
}
