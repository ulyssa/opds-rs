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

/// A slightly relaxed boolean deserializer.
///
/// Some catalogs send the string `"true"` or `"false"` instead of an actual JSON boolean,
/// so we check if that's the case and fix it to be an actual Rust [bool].
pub(crate) fn deserialize_lenient_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Lenient<'a> {
        Bool(bool),
        #[serde(borrow)]
        String(Cow<'a, str>),
    }

    match Option::<Lenient>::deserialize(deserializer)? {
        None => Ok(None),
        Some(Lenient::Bool(b)) => Ok(Some(b)),
        Some(Lenient::String(s)) => match s.trim() {
            "true" => Ok(Some(true)),
            "false" => Ok(Some(false)),
            other => Err(serde::de::Error::custom(format!(
                "expected a boolean, got {other:?}"
            ))),
        },
    }
}

/// A slightly relaxed number deserializer.
///
/// Some catalogs send a string for number fields, e.g. a series position of `"1"`, so
/// we check if that's the case and fix it to be an actual Rust [f64].
pub(crate) fn deserialize_lenient_number<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Lenient<'a> {
        Number(f64),
        #[serde(borrow)]
        String(Cow<'a, str>),
    }

    match Option::<Lenient>::deserialize(deserializer)? {
        None => Ok(None),
        Some(Lenient::Number(n)) => Ok(Some(n)),
        Some(Lenient::String(s)) => s
            .trim()
            .parse()
            .map(Some)
            .map_err(|_| serde::de::Error::custom(format!("expected a number, got {s:?}"))),
    }
}

/// Serialize non-fractional floats as [i64].
pub(crate) fn serialize_number<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(n) if n.fract() == 0.0 && n.abs() < 2f64.powi(53) => {
            serializer.serialize_i64(*n as i64)
        }
        Some(n) => serializer.serialize_f64(*n),
        None => serializer.serialize_none(),
    }
}
