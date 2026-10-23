//! Support for the [Authentication for OPDS] v1.0 draft specification
//!
//! An [AuthenticationDocument] tells a client how to authenticate with a
//! catalog, and is typically included as the body of a `401 Unauthorized`
//! response from a server that requires authentication to view the OPDS
//! catalog.
//!
//! When the response is sent, the `Content-Type` header should be set to
//! the [APPLICATION_OPDS_AUTHENTICATION_JSON] MIME type so that clients
//! know that it's a description of available authentication flows.
//!
//! See the [Authentication for OPDS] draft for more information.
//!
//! [APPLICATION_OPDS_AUTHENTICATION_JSON]: crate::mime::APPLICATION_OPDS_AUTHENTICATION_JSON
//! [Authentication for OPDS]: https://drafts.opds.io/authentication-for-opds-1.0
use std::borrow::Cow;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::v2_0::Link;

/// [RFC 2617] HTTP Basic Authentication.
///
/// [RFC 2617]: https://www.rfc-editor.org/rfc/rfc2617
pub const AUTH_BASIC: &str = "http://opds-spec.org/auth/basic";

/// [RFC 6749 Section 4.2] OAuth 2.0 Implicit Grant.
///
/// [RFC 6749 Section 4.2]: https://www.rfc-editor.org/rfc/rfc6749#section-4.2
pub const AUTH_OAUTH_IMPLICIT: &str = "http://opds-spec.org/auth/oauth/implicit";

/// [RFC 6749 Section 4.3] OAuth 2.0 Resource Owner Password Credentials Grant.
///
/// [RFC 6749 Section 4.3]: https://www.rfc-editor.org/rfc/rfc6749#section-4.3
pub const AUTH_OAUTH_PASSWORD: &str = "http://opds-spec.org/auth/oauth/password";

/// The relation used in an HTTP `Link` header to point to an Authentication Document.
pub const REL_AUTH_DOCUMENT: &str = "http://opds-spec.org/auth/document";

/// An Authentication Document.
///
/// See [Section 2.3: Authentication Document] for more information.
///
/// [Section 2.3: Authentication Document]: https://drafts.opds.io/authentication-for-opds-1.0#2-authentication-document
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct AuthenticationDocument<'a> {
    /// Unique identifier for the Catalog provider and canonical location of
    /// this document.
    #[serde(borrow)]
    pub id: Cow<'a, str>,

    /// Title of the Catalog being accessed.
    #[serde(borrow)]
    pub title: Cow<'a, str>,

    /// Description of the service, shown to the user.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,

    /// Links to resources such as a `logo`, `help`, or `register` page.
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,

    /// The authentication flows supported by the Catalog.
    ///
    /// Catalogs should always provide at least one value provided here.
    #[serde(borrow)]
    pub authentication: Vec<AuthenticationFlow<'a>>,

    /// Captures additional fields that this crate does not know about, such as custom extensions.
    ///
    /// These fields are preserved as they originally appear so that they can survive a round trip
    /// through being deserialized and then re-serialized by consumer applications that need to
    /// manipulate OPDS documents.
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

impl<'a> AuthenticationDocument<'a> {
    pub fn new(
        id: impl Into<Cow<'a, str>>,
        title: impl Into<Cow<'a, str>>,
        flow: AuthenticationFlow<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            links: vec![],
            authentication: vec![flow],
            extensions: BTreeMap::new(),
        }
    }

    pub fn with_description(mut self, description: impl Into<Cow<'a, str>>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_link(mut self, link: Link<'a>) -> Self {
        self.links.push(link);
        self
    }

    pub fn with_flow(mut self, flow: AuthenticationFlow<'a>) -> Self {
        self.authentication.push(flow);
        self
    }
}

/// One way of authenticating with a Catalog.
///
/// See [Section 3.1: Authentication Flows] for more information.
///
/// [Section 3.1: Authentication Flows]: https://drafts.opds.io/authentication-for-opds-1.0#3-authentication-flows
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct AuthenticationFlow<'a> {
    /// The URI identifying the flow, such as [AUTH_BASIC].
    #[serde(borrow, rename = "type")]
    pub kind: Cow<'a, str>,

    /// Alternate labels for the login and password inputs.
    #[serde(borrow, default, skip_serializing_if = "Labels::is_empty")]
    pub labels: Labels<'a>,

    /// Links used by the flow, such as `authenticate` and `refresh`.
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,

    /// Captures additional fields that this crate does not know about, such as custom extensions.
    ///
    /// These fields are preserved as they originally appear so that they can survive a round trip
    /// through being deserialized and then re-serialized by consumer applications that need to
    /// manipulate OPDS documents.
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

impl<'a> AuthenticationFlow<'a> {
    pub fn new(kind: impl Into<Cow<'a, str>>) -> Self {
        Self {
            kind: kind.into(),
            labels: Labels::default(),
            links: vec![],
            extensions: BTreeMap::new(),
        }
    }

    /// An HTTP Basic Authentication flow.
    pub fn basic() -> Self {
        Self::new(AUTH_BASIC)
    }

    pub fn with_labels(
        mut self,
        login: impl Into<Cow<'a, str>>,
        password: impl Into<Cow<'a, str>>,
    ) -> Self {
        self.labels = Labels {
            login: Some(login.into()),
            password: Some(password.into()),
            extensions: BTreeMap::new(),
        };
        self
    }

    pub fn with_link(mut self, link: Link<'a>) -> Self {
        self.links.push(link);
        self
    }
}

/// Alternate labels for the inputs of an authentication page.
///
/// See [Section 3.1.1: Labels] for more information.
///
/// [Section 3.1.1: Labels]: https://drafts.opds.io/authentication-for-opds-1.0#311-labels
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Labels<'a> {
    /// Alternate label for the login.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub login: Option<Cow<'a, str>>,

    /// Alternate label for the password.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub password: Option<Cow<'a, str>>,

    /// Captures additional fields that this crate does not know about, such as custom extensions.
    ///
    /// These fields are preserved as they originally appear so that they can survive a round trip
    /// through being deserialized and then re-serialized by consumer applications that need to
    /// manipulate OPDS documents.
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

impl Labels<'_> {
    pub fn is_empty(&self) -> bool {
        self.login.is_none() && self.password.is_none() && self.extensions.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_0::metadata::Relation;

    /// The example provided in Section 2.3.3 of the draft specification.
    const EXAMPLE: &str = r#"{
  "id": "http://example.com/auth.json",
  "title": "Public Library",
  "description": "Enter a valid library card number and PIN code to authenticate on our service.",
  "links": [
    {
      "href": "http://example.com/logo.jpg",
      "type": "image/jpeg",
      "rel": "logo",
      "height": 90,
      "width": 90
    },
    {
      "href": "mailto:support@example.org",
      "rel": "help"
    },
    {
      "href": "tel:1800836482",
      "rel": "help"
    },
    {
      "href": "http://example.com/support",
      "type": "text/html",
      "rel": "help"
    },
    {
      "href": "http://example.com/registration",
      "type": "text/html",
      "rel": "register"
    }
  ],
  "authentication": [
    {
      "type": "http://opds-spec.org/auth/basic",
      "labels": {
        "login": "Library card",
        "password": "PIN"
      }
    },
    {
      "type": "http://opds-spec.org/auth/oauth/implicit",
      "links": [
        {
          "href": "http://example.com/oauth",
          "type": "text/html",
          "rel": "authenticate"
        }
      ]
    }
  ]
}"#;

    #[test]
    fn test_example_round_trips() {
        let doc: AuthenticationDocument<'_> =
            serde_json::from_str(EXAMPLE).expect("can parse example");
        assert_eq!(doc.authentication.len(), 2);
        assert_eq!(doc.authentication[0].kind, AUTH_BASIC);
        assert_eq!(
            doc.authentication[0].labels.login.as_deref(),
            Some("Library card")
        );
        assert_eq!(doc.authentication[1].kind, AUTH_OAUTH_IMPLICIT);
        assert!(matches!(&doc.links[4].rel[..], [Relation::Register]));

        let json = serde_json::to_string_pretty(&doc).expect("can serialize example");
        pretty_assertions::assert_eq!(json, EXAMPLE);
    }

    #[test]
    fn test_basic_document() {
        let doc = AuthenticationDocument::new(
            "https://books.example/auth/document",
            "Books",
            AuthenticationFlow::basic().with_labels("Username", "Password"),
        );
        let json: serde_json::Value = serde_json::to_value(&doc).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "id": "https://books.example/auth/document",
                "title": "Books",
                "authentication": [{
                    "type": "http://opds-spec.org/auth/basic",
                    "labels": {"login": "Username", "password": "Password"}
                }]
            })
        );
    }

    #[test]
    fn test_missing_authentication_rejected() {
        let result =
            serde_json::from_str::<AuthenticationDocument<'_>>(r#"{"id": "x", "title": "y"}"#);
        assert!(result.is_err());
    }
}
