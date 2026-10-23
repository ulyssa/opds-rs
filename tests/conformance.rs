//! OPDS Catalog 2.0 conformance tests.
//!
//! These tests assert behaviors mandated by the finalized OPDS 2.0
//! specification at <https://specs.opds.io/opds-2.0.html>.

use opds::v2_0::{Feed, Link};

// ---------------------------------------------------------------------------
// Spec §5.3 lists `acquisition`, `borrow`, `buy`, `download`, `preview`, and
// `subscribe` as compliant relation values. Each must resolve to an
// `AcquisitionKind` rather than being degraded to `Relation::Custom`.
// ---------------------------------------------------------------------------

#[test]
fn simplified_acquisition_relations_recognized() {
    for rel in ["acquisition", "borrow", "buy", "download", "subscribe"] {
        let json = format!(
            r#"{{"rel":"{rel}","href":"http://example.test/file.epub","type":"application/epub+zip"}}"#
        );
        let link: Link<'_> = serde_json::from_str(&json).expect("link should parse");

        assert!(
            link.get_acquisition().is_some(),
            "OPDS 2.0 simplified relation `{rel}` must resolve to an acquisition kind"
        );
    }
}

// ---------------------------------------------------------------------------
// The OPDS 1.x alias for borrowing, `http://opds-spec.org/acquisition/borrow`,
// must be recognized as an acquisition, and serialize back out as the simplified
// OPDS 2.0 value `borrow`.
// ---------------------------------------------------------------------------

#[test]
fn borrow_acquisition_url_recognized() {
    let link = parse_link(
        r#"{
            "rel": "http://opds-spec.org/acquisition/borrow",
            "href": "http://example.test/file.epub",
            "type": "application/epub+zip"
        }"#,
    );

    let kind = link
        .get_acquisition()
        .expect("the borrow alias must resolve to an acquisition kind");

    assert_eq!(
        serde_json::to_string(&kind).unwrap(),
        "\"borrow\"",
        "an acquisition kind must serialize as its simplified OPDS 2.0 value"
    );
}

#[test]
fn borrow_acquisition_recognized_in_feed() {
    // Modeled on the `archive.org` feed: an OPDS Publication whose loan link
    // uses the borrow acquisition relation.
    let feed = parse_feed(
        r#"{
            "metadata": { "title": "T" },
            "publications": [{
                "metadata": { "title": "P" },
                "links": [
                    {
                        "rel": "http://opds-spec.org/acquisition/borrow",
                        "href": "http://example.test/loan",
                        "type": "application/opds-publication+json",
                        "properties": { "availability": { "state": "available" } }
                    }
                ]
            }]
        }"#,
    );

    let loan = &feed.publications[0].links[0];
    assert!(
        loan.get_acquisition().is_some(),
        "a borrow acquisition link must be recognized as an acquisition"
    );
}

#[test]
fn preview_relation_serializes_as_simplified() {
    for rel in ["preview", "http://opds-spec.org/acquisition/sample"] {
        let json = format!(
            r#"{{"rel":"{rel}","href":"http://example.test/preview.epub","type":"application/epub+zip"}}"#
        );
        let link = parse_link(&json);

        let kind = link
            .get_acquisition()
            .expect("preview/sample is an acquisition relation");

        assert_eq!(
            serde_json::to_string(&kind).unwrap(),
            "\"preview\"",
            "`{rel}` must normalize to the simplified OPDS 2.0 value `preview`"
        );
    }
}

fn parse_feed(json: &str) -> Feed<'_> {
    serde_json::from_str(json).expect("feed should parse")
}

fn parse_link(json: &str) -> Link<'_> {
    serde_json::from_str(json).expect("link should parse")
}

// ---------------------------------------------------------------------------
// The contributor schema allows `role` to be a single string or an array of
// strings, e.g. `"role": "sng"` in the OPDS 2.0 test catalog.
// ---------------------------------------------------------------------------

#[test]
fn contributor_role_string_or_array() {
    use opds::v2_0::Publication;

    for role in [r#""sng""#, r#"["sng", "edt"]"#] {
        let json = format!(
            r#"{{"metadata":{{"title":"T","contributor":{{"name":"Janet Singer","role":{role}}}}},"links":[]}}"#
        );
        let publication: Publication<'_> =
            serde_json::from_str(&json).expect("role parses as a string or an array");
        assert_eq!(publication.metadata.contributor[0].role[0], "sng");
    }
}

fn parse_publication(json: &str) -> opds::v2_0::Publication<'_> {
    serde_json::from_str(json).expect("publication should parse")
}

fn round_trip<T: serde::Serialize>(value: &T) -> serde_json::Value {
    serde_json::to_value(value).expect("value should serialize")
}

// ---------------------------------------------------------------------------
// The metadata schema allows `subject` to be a string, a subject object, or an
// array of either, e.g. `"subject": "Science Fiction"` in the OPDS 2.0 test
// catalog.
// ---------------------------------------------------------------------------

#[test]
fn subject_string_object_or_array() {
    for subject in [
        r#""Science Fiction""#,
        r#"{"name": "Science Fiction", "code": "FIC028000"}"#,
        r#"["Science Fiction", {"name": "Classics"}]"#,
    ] {
        let json = format!(r#"{{"metadata":{{"title":"T","subject":{subject}}},"links":[]}}"#);
        let publication = parse_publication(&json);
        let subjects = round_trip(&publication.metadata.subject);
        assert_eq!(subjects[0]["name"], "Science Fiction", "{subject}");
    }
}

// ---------------------------------------------------------------------------
// `abridged` is a boolean, but some catalogs (Feedbooks) send `"false"`. The
// string forms of the two booleans are accepted; anything else is rejected.
// ---------------------------------------------------------------------------

#[test]
fn abridged_accepts_boolean_strings() {
    for (value, expected) in [
        ("false", false),
        ("true", true),
        (r#""false""#, false),
        (r#""true""#, true),
    ] {
        let json = format!(r#"{{"metadata":{{"title":"T","abridged":{value}}},"links":[]}}"#);
        assert_eq!(
            parse_publication(&json).metadata.abridged,
            Some(expected),
            "{value}"
        );
    }
    let json = r#"{"metadata":{"title":"T","abridged":"maybe"},"links":[]}"#;
    assert!(serde_json::from_str::<opds::v2_0::Publication>(json).is_err());
}

// ---------------------------------------------------------------------------
// A series `position` is a number, which may be fractional (`0.2` in the OPDS
// 2.0 test catalog); some catalogs (Feedbooks) send it as a string. Whole
// numbers are written back without a fractional part.
// ---------------------------------------------------------------------------

#[test]
fn series_position_is_a_lenient_number() {
    for (value, expected) in [("1", 1.0), (r#""1""#, 1.0), ("0.2", 0.2), (r#""2.5""#, 2.5)] {
        let json = format!(
            r#"{{"metadata":{{"title":"T","belongsTo":{{"series":{{"name":"S","position":{value}}}}}}},"links":[]}}"#
        );
        let publication = parse_publication(&json);
        let series = &publication.metadata.belongs_to.as_ref().unwrap().series[0];
        assert_eq!(series.position, Some(expected), "{value}");
    }

    let json = r#"{"metadata":{"title":"T","belongsTo":{"series":{"name":"S","position":"1"}}},"links":[]}"#;
    let out = serde_json::to_string(&parse_publication(json)).unwrap();
    assert!(
        out.contains(r#""position":1}"#) || out.contains(r#""position":1,"#),
        "{out}"
    );
}

// ---------------------------------------------------------------------------
// Members this crate does not model are kept in `extensions` and written back
// unchanged, so a parse-and-serialize round trip does not lose them.
// ---------------------------------------------------------------------------

#[test]
fn link_property_extensions_survive() {
    let link = parse_link(
        r#"{
            "href": "http://example.test/loan",
            "rel": "http://opds-spec.org/acquisition/borrow",
            "properties": {
                "authenticate": {"href": "http://example.test/auth", "type": "application/opds-authentication+json"},
                "https://schema.org/reservationStatus": "https://schema.org/ReservationPending"
            }
        }"#,
    );
    let props = &link.properties.extensions;
    assert_eq!(props["authenticate"]["href"], "http://example.test/auth");
    assert!(props.contains_key("https://schema.org/reservationStatus"));

    let out = round_trip(&link);
    assert_eq!(
        out["properties"]["authenticate"]["href"],
        "http://example.test/auth"
    );
    assert_eq!(
        out["properties"]["https://schema.org/reservationStatus"],
        "https://schema.org/ReservationPending"
    );
}

#[test]
fn publication_extensions_survive() {
    let publication = parse_publication(
        r#"{
            "metadata": {
                "@type": "http://schema.org/Book",
                "title": "T",
                "presentation": {"layout": "reflowable"},
                "schema:workExample": [{"schema:isbn": "9780000000000"}]
            },
            "links": [],
            "licenses": [{"metadata": {"identifier": "urn:uuid:x"}}]
        }"#,
    );
    assert_eq!(
        publication.metadata.schema.as_deref(),
        Some("http://schema.org/Book")
    );
    assert!(
        !publication.metadata.extensions.contains_key("@type"),
        "modeled members are never duplicated into extensions"
    );

    let out = round_trip(&publication);
    assert_eq!(out["metadata"]["presentation"]["layout"], "reflowable");
    assert_eq!(
        out["metadata"]["schema:workExample"][0]["schema:isbn"],
        "9780000000000"
    );
    assert_eq!(out["licenses"][0]["metadata"]["identifier"], "urn:uuid:x");
    assert_eq!(out["metadata"]["@type"], "http://schema.org/Book");
}

#[test]
fn feed_collection_extensions_survive() {
    let feed = parse_feed(
        r#"{
            "metadata": {"title": "Libraries", "adobe_vendor_id": "Vendor"},
            "links": [],
            "catalogs": [{"metadata": {"title": "A library"}, "links": []}]
        }"#,
    );
    let out = round_trip(&feed);
    assert_eq!(out["metadata"]["adobe_vendor_id"], "Vendor");
    assert_eq!(out["catalogs"][0]["metadata"]["title"], "A library");
}

#[test]
fn authentication_document_extensions_survive() {
    use opds::authentication::AuthenticationDocument;

    let json = r#"{
        "id": "http://example.test/auth",
        "title": "Library",
        "authentication": [{"type": "http://opds-spec.org/auth/basic", "x-flow": 1}],
        "service_description": "Sign in with your library card"
    }"#;
    let doc: AuthenticationDocument<'_> =
        serde_json::from_str(json).expect("document should parse");
    let out = round_trip(&doc);
    assert_eq!(out["service_description"], "Sign in with your library card");
    assert_eq!(out["authentication"][0]["x-flow"], 1);
}

// ---------------------------------------------------------------------------
// Every feed fixture (see tests/README.md) reaches a fixed point after one
// parse-and-serialize: serializing a re-parsed output changes nothing.
// ---------------------------------------------------------------------------

#[test]
fn feed_fixtures_round_trip_to_a_fixed_point() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let mut checked = 0;
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        if !(name.starts_with("test-feed-") && name.ends_with(".in.json")) {
            continue;
        }
        let input = std::fs::read_to_string(&path).unwrap();
        let once = serde_json::to_string(&parse_feed(&input)).unwrap();
        let twice = serde_json::to_string(&parse_feed(&once)).unwrap();
        assert_eq!(once, twice, "{name} is not stable across round trips");
        checked += 1;
    }
    assert!(
        checked >= 6,
        "expected the feed fixtures in {}",
        dir.display()
    );
}

// ---------------------------------------------------------------------------
// `identifier` should be a URI, but catalogs also send bare values such as
// `"ISBN 978..."`. Those are kept as `Identifier::Other` instead of failing the
// whole document, and are written back unchanged.
// ---------------------------------------------------------------------------

#[test]
fn identifier_accepts_uris_and_bare_values() {
    use opds::v2_0::metadata::Identifier;

    let publication = parse_publication(
        r#"{"metadata":{"title":"T","identifier":"urn:isbn:9780441478125"},"links":[]}"#,
    );
    let id = publication.metadata.identifier.as_ref().unwrap();
    assert!(matches!(id, Identifier::Url(_)), "{id:?}");
    assert_eq!(id.as_str(), "urn:isbn:9780441478125");

    let json = r#"{"metadata":{"title":"T","identifier":"ISBN 9780441478125"},"links":[]}"#;
    let publication = parse_publication(json);
    assert_eq!(
        publication.metadata.identifier,
        Some(Identifier::Other("ISBN 9780441478125".into()))
    );
    assert_eq!(
        round_trip(&publication)["metadata"]["identifier"],
        "ISBN 9780441478125"
    );

    let feed = parse_feed(r#"{"metadata":{"title":"F","identifier":"catalog-7"},"links":[]}"#);
    assert_eq!(feed.metadata.identifier.unwrap().to_string(), "catalog-7");
}

// ---------------------------------------------------------------------------
// `duration` is a number of seconds, which may be fractional, on both links
// and publications. Strings holding a number are accepted, and whole numbers
// are written back without a fractional part.
// ---------------------------------------------------------------------------

#[test]
fn duration_is_a_lenient_number_everywhere() {
    for (value, expected) in [("3600", 3600.0), ("12.5", 12.5), (r#""90""#, 90.0)] {
        let json = format!(
            r#"{{"metadata":{{"title":"T","duration":{value}}},"links":[{{"href":"a.mp3","duration":{value}}}]}}"#
        );
        let publication = parse_publication(&json);
        assert_eq!(publication.metadata.duration, Some(expected), "{value}");
        assert_eq!(publication.links[0].duration, Some(expected), "{value}");
    }
    let publication = parse_publication(
        r#"{"metadata":{"title":"T","duration":3600},"links":[{"href":"a","duration":3600}]}"#,
    );
    let out = round_trip(&publication);
    assert_eq!(out["metadata"]["duration"], serde_json::json!(3600));
    assert_eq!(out["links"][0]["duration"], serde_json::json!(3600));
}

// ---------------------------------------------------------------------------
// Values outside the enumerations this crate knows (a newer spec, or a
// publisher's own) are kept as written rather than rejecting the document.
// ---------------------------------------------------------------------------

#[test]
fn unknown_enumeration_values_are_kept() {
    use opds::v2_0::metadata::{
        AccessMode, AccessibilityFeature, PublicationLayout, ReadingProgression,
    };

    let json = r#"{
        "metadata": {
            "title": "T",
            "layout": "fixed",
            "readingProgression": "ttb",
            "accessibility": {
                "accessMode": ["textual", "olfactory"],
                "feature": ["ARIA", "hapticFeedback"]
            }
        },
        "links": [{"href": "p1", "properties": {"page": "spread"}}]
    }"#;
    let publication = parse_publication(json);
    let m = &publication.metadata;
    assert_eq!(m.layout, Some(PublicationLayout::Fixed));
    assert_eq!(
        m.reading_progression,
        Some(ReadingProgression::Other("ttb".into()))
    );
    let a11y = m.accessibility.as_ref().unwrap();
    assert_eq!(
        a11y.access_mode,
        [AccessMode::Textual, AccessMode::Other("olfactory".into())]
    );
    assert_eq!(
        a11y.feature,
        [
            AccessibilityFeature::Aria,
            AccessibilityFeature::Other("hapticFeedback".into())
        ]
    );

    let out = round_trip(&publication);
    assert_eq!(out["metadata"]["readingProgression"], "ttb");
    assert_eq!(
        out["metadata"]["accessibility"]["accessMode"],
        serde_json::json!(["textual", "olfactory"])
    );
    assert_eq!(out["links"][0]["properties"]["page"], "spread");
}

// ---------------------------------------------------------------------------
// `conformsTo` and a link's `language` may be a single string or an array.
// ---------------------------------------------------------------------------

#[test]
fn conforms_to_and_link_language_accept_a_single_string() {
    let json = r#"{
        "metadata": {"title": "T", "conformsTo": "https://readium.org/webpub-manifest/profiles/audiobook"},
        "links": [{"href": "a", "language": "fr"}, {"href": "b", "language": ["en", "de"]}]
    }"#;
    let publication = parse_publication(json);
    assert_eq!(publication.metadata.conforms_to.len(), 1);
    assert_eq!(publication.links[0].language.len(), 1);
    assert_eq!(publication.links[0].language[0].as_str(), "fr");
    assert_eq!(publication.links[1].language.len(), 2);
}

// ---------------------------------------------------------------------------
// Builder methods on `Link`, and the registered `start` relation.
// ---------------------------------------------------------------------------

#[test]
fn link_builders_and_start_relation() {
    use opds::v2_0::metadata::Relation;
    use std::borrow::Cow;

    let link = Link::new(
        Cow::Borrowed("/opds"),
        Some(Cow::Borrowed(opds::mime::APPLICATION_OPDS_JSON)),
    )
    .with_rel(Relation::Start)
    .with_rel(Relation::Myself)
    .with_title("Home")
    .with_count(12);
    let out = round_trip(&link);
    assert_eq!(out["rel"], serde_json::json!(["start", "self"]));
    assert_eq!(out["title"], "Home");
    assert_eq!(out["properties"]["numberOfItems"], 12);

    assert_eq!(
        parse_link(r#"{"href": "/", "rel": "start"}"#).rel,
        [Relation::Start]
    );
}
