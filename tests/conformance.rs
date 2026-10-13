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
    for rel in [
        "acquisition",
        "borrow",
        "buy",
        "download",
        "subscribe",
    ] {
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
