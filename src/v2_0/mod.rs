//! Support for working with the OPDS v2.0 draft specification
//!
//! The types in this module can be used with [serde_json] to either parse or
//! generate the JSON objects for an OPDS feed.
//!
//! As noted in [Section 2: Collections] of the OPDS specification, an OPDS
//! [Feed] is made of several core concepts:
//!
//! - [Navigation][opds-spec-navigation], which is a a collection of [Link]
//!   objects that should be shown to the user, and accessed in this crate via
//!   [Feed::navigation].
//! - [Publications][opds-spec-publications], which are the actual
//!   publications available for download from the feed. They are represented in
//!   this crate with the [Publication] type, and stored in [Feed::publications]
//!   and [FeedGroup::publications].
//! - [Images][opds-spec-images], which are [Link] objects for the preview
//!   images for publications, and stored in [Publication::images] for each
//!   respective publication.
//! - [Facets][opds-spec-facets], which are groups of [Link] objects which
//!   provide a view of a subset of available publications. They are represented
//!   in this crate with the [Facet] type, and stored in [Feed::facets].
//! - [Groups][opds-spec-groups], which are groups of related navigation
//!   links and publications meant to make reading a feed easier. They are
//!   represented by the [FeedGroup] type, and stored in [Feed::groups].
//!
//! [serde_json]: https://docs.rs/serde_json/latest/serde_json/
//! [Section 2: Collections]: https://drafts.opds.io/opds-2.0.html#2-collections
//! [opds-spec-navigation]: https://drafts.opds.io/opds-2.0.html#21-navigation
//! [opds-spec-publications]: https://drafts.opds.io/opds-2.0.html#22-publications
//! [opds-spec-images]: https://drafts.opds.io/opds-2.0.html#23-images
//! [opds-spec-facets]: https://drafts.opds.io/opds-2.0.html#24-facets
//! [opds-spec-groups]: https://drafts.opds.io/opds-2.0.html#25-groups
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::helpers::*;
use crate::v2_0::metadata::*;

pub mod metadata;

/// An OPDS link object.
///
/// See [Section 2.4: The Link Object] or the [JSON Schema] for more information.
///
/// [Section 2.4: The Link Object]: https://readium.org/webpub-manifest/#24-the-link-object
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/link.schema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Link<'a> {
    /// Title of the linked resource.
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub title: Option<Cow<'a, str>>,

    /// URI or URI template of the linked resource.
    ///
    /// While this field should always be present according to the JSON schema, some catalogs
    /// will return link objects without an `href` for [AvailabilityState::Unavailable] content.
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub href: Option<Cow<'a, str>>,

    /// Indicates that a URI template is used in `href`.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub templated: bool,

    /// MIME type of the linked resource.
    ///
    /// See [crate::mime] for common OPDS MIME types.
    #[serde(borrow, skip_serializing_if = "Option::is_none", rename = "type")]
    pub mime: Option<Cow<'a, str>>,

    /// Relation between the linked resource and its containing collection.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "serialize_flattened_vec",
        deserialize_with = "deserialize_flattened_vec"
    )]
    pub rel: Vec<Relation>,

    /// Properties associated to the linked resource.
    #[serde(borrow, default, skip_serializing_if = "LinkProperties::is_empty")]
    pub properties: LinkProperties<'a>,

    /// Height of the linked resource in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<usize>,

    /// Width of the linked resource in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<usize>,

    /// Original size of the resource in bytes, prior to any use of encryption
    /// or compression in an archive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,

    /// Bitrate of the linked resource in kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,

    /// Length of the linked resource in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    /// Expected language of the linked resource.
    ///
    /// Languages should be in [BCP 47] syntax.
    ///
    /// [BCP 47]: https://www.rfc-editor.org/info/bcp47
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub language: Vec<Cow<'a, langtag::LangTag>>,

    /// Alternate resources for the linked resource.
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub alternate: Vec<Link<'a>>,

    /// Resources that are children of the linked resource, in the context of a
    /// given collection role.
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Link<'a>>,
}

impl<'a> Link<'a> {
    pub fn new(href: Cow<'a, str>, mime: Option<Cow<'a, str>>) -> Self {
        Link {
            href: Some(href),
            templated: false,
            mime,

            title: None,
            rel: vec![],
            properties: LinkProperties::default(),
            height: None,
            width: None,
            size: None,
            bitrate: None,
            duration: None,
            language: vec![],
            alternate: vec![],
            children: vec![],
        }
    }

    pub fn template(href: Cow<'a, str>, mime: Option<Cow<'a, str>>) -> Self {
        Link {
            href: Some(href),
            templated: true,
            mime,

            title: None,
            rel: vec![],
            properties: LinkProperties::default(),
            height: None,
            width: None,
            size: None,
            bitrate: None,
            duration: None,
            language: vec![],
            alternate: vec![],
            children: vec![],
        }
    }

    pub fn get_acquisition(&self) -> Option<AcquisitionKind> {
        self.rel.iter().flat_map(|rel| rel.as_acquisition()).next()
    }
}

/// An OPDS facet for helping to navigate a collection by viewing a subset or by providing
/// a specific sort.
///
/// See [Section 2.4: Facets][opds-spec-facets] for more information.
///
/// [opds-spec-facets]: https://drafts.opds.io/opds-2.0.html#24-facets
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Facet<'a> {
    #[serde(borrow)]
    pub metadata: FeedMetadata<'a>,
    #[serde(borrow)]
    pub links: Vec<Link<'a>>,
}

impl<'a> Facet<'a> {
    pub fn new(title: impl Into<StringWithAlternates<'a>>) -> Self {
        let metadata = FeedMetadata::new(title);

        Self {
            metadata,
            links: vec![],
        }
    }
}

/// An OPDS Publication object.
///
/// See [Section 5: Publications] and the associated [JSON Schema].
///
/// [Section 5: Publications]: https://drafts.opds.io/opds-2.0#5-publications
/// [JSON Schema]: https://drafts.opds.io/schema/publication.schema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Publication<'a> {
    #[serde(borrow)]
    pub metadata: PublicationMetadata<'a>,

    #[serde(borrow)]
    pub links: Vec<Link<'a>>,

    /// Links to preview images for a client to display when listing this publication.
    ///
    /// At least one of the images provided should be one of the following MIME types
    /// to allow clients to support a standard subset of image types:
    ///
    /// - `image/jpeg`
    /// - `image/webp`
    /// - `image/avif`
    /// - `image/png`
    /// - `image/jxl`
    /// - `image/gif`
    ///
    /// See [Section 2.3: Images][opds-spec-images] for more information.
    ///
    /// [opds-spec-images]: https://drafts.opds.io/opds-2.0#23-images
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<Link<'a>>,
}

/// A group within an OPDS feed.
///
/// See [Section 2.5: Groups][opds-spec-groups] for more information.
///
/// [opds-spec-groups]: https://drafts.opds.io/opds-2.0.html#25-groups
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct FeedGroup<'a> {
    #[serde(borrow)]
    pub metadata: FeedMetadata<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,

    /// Links for the client to show the end user in order to help browse the catalog.
    ///
    /// See [Section 2.1: Navigation][opds-spec-navigation] for more information.
    ///
    /// [opds-spec-navigation]: https://drafts.opds.io/opds-2.0.html#21-navigation
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub navigation: Vec<Link<'a>>,

    /// Publications for the client to show the end user they can download.
    ///
    /// See [Section 2.2: Publications][opds-spec-publications] for more information.
    ///
    /// [opds-spec-publications]: https://drafts.opds.io/opds-2.0.html#21-navigation
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub publications: Vec<Publication<'a>>,
}

impl<'a> FeedGroup<'a> {
    pub fn new(title: impl Into<StringWithAlternates<'a>>) -> Self {
        let metadata = FeedMetadata::new(title);

        Self {
            metadata,
            links: vec![],
            navigation: vec![],
            publications: vec![],
        }
    }
}

/// The main OPDS feed.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Feed<'a> {
    #[serde(borrow)]
    pub metadata: FeedMetadata<'a>,

    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,

    /// Links for the client to show the end user in order to help browse the catalog.
    ///
    /// See [Section 2.1: Navigation][opds-spec-navigation] for more information.
    ///
    /// [opds-spec-navigation]: https://drafts.opds.io/opds-2.0.html#21-navigation
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub navigation: Vec<Link<'a>>,

    /// Links to views of a subset of publications within the catalog.
    ///
    /// See [Section 2.4: Facets][opds-spec-facets] for more information.
    ///
    /// [opds-spec-facets]: https://drafts.opds.io/opds-2.0.html#24-facets
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub facets: Vec<Facet<'a>>,

    /// Publications for the client to show the end user they can download.
    ///
    /// See [Section 2.2: Publications][opds-spec-publications] for more information.
    ///
    /// [opds-spec-publications]: https://drafts.opds.io/opds-2.0.html#21-navigation
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub publications: Vec<Publication<'a>>,

    /// Groups to help provide a curated experience by grouping related publications
    /// or navigation links together.
    ///
    /// See [Section 2.5: Groups][opds-spec-groups] for more information.
    ///
    /// [opds-spec-groups]: https://drafts.opds.io/opds-2.0.html#25-groups
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<FeedGroup<'a>>,
}

impl<'a> Feed<'a> {
    pub fn new(title: impl Into<StringWithAlternates<'a>>) -> Self {
        let metadata = FeedMetadata::new(title);

        Self {
            metadata,
            links: vec![],
            navigation: vec![],
            facets: vec![],
            publications: vec![],
            groups: vec![],
        }
    }

    pub fn with_link(mut self, link: Link<'a>) -> Self {
        self.links.push(link);
        self
    }

    pub fn with_navigation(mut self, link: Link<'a>) -> Self {
        self.navigation.push(link);
        self
    }

    pub fn with_facet(mut self, facet: Facet<'a>) -> Self {
        self.facets.push(facet);
        self
    }

    pub fn with_publication(mut self, publication: Publication<'a>) -> Self {
        self.publications.push(publication);
        self
    }

    pub fn with_group(mut self, group: FeedGroup<'a>) -> Self {
        self.groups.push(group);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;
    use std::path::PathBuf;

    const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");

    fn get_prefixes(prefix: &str) -> Vec<String> {
        let crate_dir = PathBuf::from(CRATE_DIR);
        let tests = crate_dir.join("tests");

        std::fs::read_dir(&tests)
            .unwrap()
            .map(|f| f.unwrap().path())
            .filter(|f| {
                f.file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or_default()
                    .starts_with(prefix)
            })
            .flat_map(|f| {
                let f = f.to_str()?;
                let p = f.strip_suffix(".in.json")?;
                Some(p.to_string())
            })
            .collect()
    }

    fn read_files(prefix: &str) -> (String, String) {
        let input = format!("{prefix}.in.json");
        let output = format!("{prefix}.out.json");

        let json_in = std::fs::read_to_string(&input)
            .with_context(|| format!("can load {input:?}"))
            .expect("valid file input");
        let json_exp = std::fs::read_to_string(&output)
            .with_context(|| format!("can load {output:?}"))
            .expect("valid file output");

        (json_in, json_exp)
    }

    #[test]
    fn test_feed() {
        for prefix in get_prefixes("test-feed") {
            let (json_in, json_exp) = read_files(&prefix);

            let feed: Feed<'_> = serde_json::from_str(&json_in)
                .with_context(|| format!("can parse {prefix:?} input as a Feed"))
                .expect("can parse feed");

            let json_out = serde_json::to_string_pretty(&feed)
                .with_context(|| format!("can serialize {prefix:?} input as a Feed"))
                .expect("can serialize feed");

            pretty_assertions::assert_eq!(
                json_out.trim_end(),
                json_exp.trim_end(),
                "{prefix} input matches expected output file after parsing"
            );
        }
    }
}
