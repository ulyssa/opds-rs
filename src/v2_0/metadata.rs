//! Types used to represent the fields within the feed, link and publication metadata.
use std::borrow::Cow;
use std::collections::BTreeMap;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, SerializeMap, Serializer};

use super::*;

/// A relationship between a resource and a link.
///
/// See [Link Relations] for more information.
///
/// [Link Relations]: https://readium.org/webpub-manifest/relationships.html
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Relation {
    #[serde(rename = "self")]
    Myself,

    /// A link to a substitute for the link's context.
    Alternate,

    /// A link to a table of contents.
    Contents,

    /// A link to the cover of a publication.
    Cover,

    /// A link to a [Web Application Manifest].
    ///
    /// [Web Application Manifest]: https://www.w3.org/TR/appmanifest/
    Manifest,

    /// A link to a user's profile.
    Profile,

    /// A link to the first resource in a collection of resources.
    ///
    /// For example, the first page in a paginated view.
    First,

    /// A link to the previous resource in a collection of resources relative to the link's context.
    ///
    /// For example, the previous page in a paginated view.
    Previous,

    /// A link to the next resource in a collection of resources relative to the link's context.
    ///
    /// For example, the next page in a paginated view.
    Next,

    /// A link to the last resource in a collection of resources.
    ///
    /// For example, the last page in a paginated view.
    Last,

    /// A link that is the same as the link's context.
    Current,

    /// The link is either a URI or a templated URI for performing a search.
    Search,

    Subsection,

    #[serde(rename = "http://opds-spec.org/sort/new")]
    SortNew,

    #[serde(rename = "http://opds-spec.org/sort/popular")]
    SortPopular,

    #[serde(untagged)]
    Acquisition(AcquisitionKind),

    #[serde(untagged)]
    Custom(String),
}

impl Relation {
    pub fn as_acquisition(&self) -> Option<AcquisitionKind> {
        if let Self::Acquisition(kind) = self {
            Some(kind.clone())
        } else {
            None
        }
    }
}

impl From<String> for Relation {
    fn from(value: String) -> Self {
        Self::Custom(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum AcquisitionKind {
    /// Fallback acquisition relation when no other relation is a good fit
    /// to express the nature of the transaction.
    #[serde(rename = "http://opds-spec.org/acquisition")]
    Fallback,

    /// Indicates that a publication is freely accessible without any requirement,
    /// including authentication.
    #[serde(rename = "http://opds-spec.org/acquisition/open-access")]
    OpenAccess,

    /// Indicates that a publication can be purchased for a given price.
    #[serde(rename = "http://opds-spec.org/acquisition/buy")]
    Buy,

    /// Indicates that a sub-set of the full publication is freely accessible
    /// at a given URI, without any prior requirement.
    #[serde(rename = "http://opds-spec.org/acquisition/sample")]
    Sample,

    /// Indicates that a publication be subscribed to, usually as part of a
    /// purchase and for a limited period of time.
    #[serde(rename = "http://opds-spec.org/acquisition/subscribe")]
    Subscribe,

    /// Indicates that a sub-set of the full publication is freely accessible
    /// at a given URI, without any prior requirement.
    Preview,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PageDisplay {
    Left,
    Right,
    Center,
}

/// Hints for how the layout of the publication should be presented.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PublicationLayout {
    Fixed,
    Reflowable,
    Scrolled,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ReadingProgression {
    #[serde(rename = "rtl")]
    RightToLeft,
    #[serde(rename = "ltr")]
    #[default]
    LeftToRight,
}

/// The price of purchasing access to a publication from the acquisition link.
///
/// See [Section 5.3: Acquisition Links].
///
/// [Section 5.3: Acquisition Links]: https://drafts.opds.io/opds-2.0.html#53-acquisition-links
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    /// The number price for an acquisition.
    pub value: f32,
    /// The unit of currency for the price value.
    pub currency: String,
}

/// An OPDS acquisition object.
///
/// See [Section 5.3: Acquisition Links].
///
/// [Section 5.3: Acquisition Links]: https://drafts.opds.io/opds-2.0.html#53-acquisition-links
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Acquisition<'a> {
    /// The MIME type that will be acquired.
    ///
    /// See [crate::mime] for common OPDS MIME types.
    #[serde(rename = "type")]
    pub mime: Cow<'a, str>,
    /// An additional layer of indirection for further acquisition objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub child: Vec<Acquisition<'a>>,
}

/// Information about a library's holds for a publication.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Holds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<usize>,
}

/// Information about a library's number of copies for a publication.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Copies {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<usize>,
}

/// A resource's current availability state.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum AvailabilityState {
    Available,
    Unavailable,
    Reserved,
    Ready,
}

/// A resource's availability.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Availability<'a> {
    /// The current state of the resource.
    pub state: AvailabilityState,

    /// Timestamp for when the state change occurred.
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub since: Option<Cow<'a, str>>,

    /// Timestamp for when the next state change will occur.
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub until: Option<Cow<'a, str>>,
}

/// An identifier for a resource.
///
/// This is either a URL or a URN.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Identifier {
    Url(url::Url),
    Urn(urn::Urn),
}

/// An alternate identifier for a resource.
///
/// See the [JSON Schema] for more details.
///
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/altIdentifier.schema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AltIdentifier<'a> {
    pub value: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<url::Url>,
}

impl<'a> AltIdentifier<'a> {
    pub fn new(value: Cow<'a, str>) -> Self {
        Self {
            value,
            scheme: None,
        }
    }
}

impl<'a> From<String> for AltIdentifier<'a> {
    fn from(value: String) -> Self {
        Self::from(Cow::Owned(value))
    }
}

impl<'a> From<Cow<'a, str>> for AltIdentifier<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AccessMode {
    Auditory,
    ChartOnVisual,
    ChemOnVisual,
    ColorDependent,
    DiagramOnVisual,
    MathOnVisual,
    MusicOnVisual,
    Tactile,
    TextOnVisual,
    Textual,
    Visual,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum AccessibilityExemption {
    EaaDisproportionateBurden,
    EaaFundamentalAlteration,
    EaaMicroenterprise,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub enum AccessibilityFeature {
    #[serde(rename = "annotations")]
    Annotations,
    #[serde(rename = "ARIA")]
    Aria,
    #[serde(rename = "bookmarks")]
    Bookmarks,
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "pageBreakMarkers")]
    PageBreakMarkers,
    #[serde(rename = "printPageNumbers")]
    PrintPageNumbers,
    #[serde(rename = "pageNavigation")]
    PageNavigation,
    #[serde(rename = "readingOrder")]
    ReadingOrder,
    #[serde(rename = "structuralNavigation")]
    StructuralNavigation,
    #[serde(rename = "tableOfContents")]
    TableOfContents,
    #[serde(rename = "taggedPDF")]
    TaggedPdf,
    #[serde(rename = "alternativeText")]
    AlternativeText,
    #[serde(rename = "audioDescription")]
    AudioDescription,
    #[serde(rename = "closeCaptions")]
    CloseCaptions,
    #[serde(rename = "captions")]
    Captions,
    #[serde(rename = "describedMath")]
    DescribedMath,
    #[serde(rename = "longDescription")]
    LongDescription,
    #[serde(rename = "openCaptions")]
    OpenCaptions,
    #[serde(rename = "signLanguage")]
    SignLanguage,
    #[serde(rename = "transcript")]
    Transcript,
    #[serde(rename = "displayTransformability")]
    DisplayTransformability,
    #[serde(rename = "synchronizedAudioText")]
    SynchronizedAudioText,
    #[serde(rename = "timingControl")]
    TimingControl,
    #[serde(rename = "unlocked")]
    Unlocked,
    #[serde(rename = "ChemML")]
    ChemMl,
    #[serde(rename = "latex")]
    Latex,
    #[serde(rename = "latex-chemistry")]
    LatexChemistry,
    #[serde(rename = "MathML")]
    MathMl,
    #[serde(rename = "MathML-chemistry")]
    MathMlChemistry,
    #[serde(rename = "ttsMarkup")]
    TtsMarkup,
    #[serde(rename = "highContrastAudio")]
    HighContrastAudio,
    #[serde(rename = "highContrastDisplay")]
    HighContrastDisplay,
    #[serde(rename = "largePrint")]
    LargePrint,
    #[serde(rename = "braille")]
    Braille,
    #[serde(rename = "tactileGraphic")]
    TactileGraphic,
    #[serde(rename = "tactileObject")]
    TactileObject,
    #[serde(rename = "fullRubyAnnotations")]
    FullRubyAnnotations,
    #[serde(rename = "horizontalWriting")]
    HorizontalWriting,
    #[serde(rename = "rubyAnnotations")]
    RubyAnnotations,
    #[serde(rename = "verticalWriting")]
    VerticalWriting,
    #[serde(rename = "withAdditionalWordSegmentation")]
    WithAdditionalWordSegmentation,
    #[serde(rename = "withoutAdditionalWordSegmentation")]
    WithoutAdditionalWordSegmentation,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub enum AccessibilityHazard {
    #[serde(rename = "flashing")]
    Flashing,
    #[serde(rename = "motionSimulation")]
    MotionSimulation,
    #[serde(rename = "sound")]
    Sound,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "noFlashingHazard")]
    NoFlashingHazard,
    #[serde(rename = "noMotionSimulationHazard")]
    NoMotionSimulationHazard,
    #[serde(rename = "noSoundHazard")]
    NoSoundHazard,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "unknownFlashingHazard")]
    UnknownFlashingHazard,
    #[serde(rename = "unknownMotionSimulationHazard")]
    UnknownMotionSimulationHazard,
    #[serde(rename = "unknownSoundHazard")]
    UnknownSoundHazard,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessbilityCertification<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub certified_by: Option<Cow<'a, str>>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub credential: Option<Cow<'a, str>>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub report: Option<Cow<'a, str>>,
}

/// Accessibility metadata for a publication.
///
/// See [Default Context: Accessibility Metadata] and the associated [JSON Schema] for more
/// details.
///
/// [Default context: Accessibility Metadata]: https://readium.org/webpub-manifest/contexts/default/#accessibility-metadata
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/a11y.schema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityMetadata<'a> {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec"
    )]
    pub conforms_to: Vec<url::Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemption: Option<AccessibilityExemption>,

    #[serde(default)]
    pub access_mode: Vec<AccessMode>,

    #[serde(default)]
    pub feature: Vec<AccessibilityFeature>,

    #[serde(default)]
    pub hazard: Vec<AccessibilityHazard>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub certification: Option<AccessbilityCertification<'a>>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub summary: Option<Cow<'a, str>>,
}

/// A "belongs to" relationship.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BelongsTo<'a> {
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub collection: Vec<Collection<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub journal: Vec<Periodical<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub magazine: Vec<Periodical<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub newspaper: Vec<Periodical<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub periodical: Vec<Periodical<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub season: Vec<Season<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub series: Vec<Series<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub story_arc: Vec<StoryArc<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub volume: Vec<Volume<'a>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection<'a> {
    #[serde(borrow)]
    pub name: StringWithAlternates<'a>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<StringWithAlternates<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Identifier>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub alt_identifier: Vec<AltIdentifier<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<usize>,

    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,
}

/// A collection of related publications.
///
/// See the associate [JSON Schema].
///
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/collection.schema.json
impl<'a> Collection<'a> {
    pub fn new(name: impl Into<StringWithAlternates<'a>>) -> Self {
        let name = name.into();

        Self {
            name,
            sort_as: None,
            identifier: None,
            alt_identifier: vec![],
            position: None,
            links: vec![],
        }
    }
}

impl<'a> From<String> for Collection<'a> {
    fn from(name: String) -> Self {
        Self::from(Cow::Owned(name))
    }
}

impl<'a> From<Cow<'a, str>> for Collection<'a> {
    fn from(name: Cow<'a, str>) -> Self {
        Self::new(name)
    }
}

/// A periodical that may contain multiple publications.
///
/// See the associate [JSON Schema].
///
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/periodical.schema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Periodical<'a> {
    #[serde(borrow)]
    pub name: StringWithAlternates<'a>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<StringWithAlternates<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Identifier>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub alt_identifier: Vec<AltIdentifier<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<usize>,

    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub issue: Vec<Issue<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub volume: Vec<Volume<'a>>,
}

impl<'a> Periodical<'a> {
    pub fn new(name: impl Into<StringWithAlternates<'a>>) -> Self {
        let name = name.into();

        Self {
            name,
            sort_as: None,
            identifier: None,
            alt_identifier: vec![],
            position: None,
            links: vec![],
            issue: vec![],
            volume: vec![],
        }
    }
}

impl<'a> From<String> for Periodical<'a> {
    fn from(name: String) -> Self {
        Self::from(Cow::Owned(name))
    }
}

impl<'a> From<Cow<'a, str>> for Periodical<'a> {
    fn from(name: Cow<'a, str>) -> Self {
        Self::new(name)
    }
}

/// An episode of a show, podcast, or other episodic content.
///
/// See the associated [JSON Schema] for more information.
///
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/episode.schema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub name: Option<StringWithAlternates<'a>>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<StringWithAlternates<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Identifier>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub alt_identifier: Vec<AltIdentifier<'a>>,

    pub position: usize,

    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,
}

impl<'a> Episode<'a> {
    pub fn new(position: usize) -> Self {
        Self {
            position,

            name: None,
            sort_as: None,
            identifier: None,
            alt_identifier: vec![],
            links: vec![],
        }
    }
}

impl<'a> From<usize> for Episode<'a> {
    fn from(position: usize) -> Self {
        Self::new(position)
    }
}

/// A season of a show.
///
/// See the associated [JSON Schema] for more information.
///
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/season.schema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Season<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub name: Option<StringWithAlternates<'a>>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<StringWithAlternates<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Identifier>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub alt_identifier: Vec<AltIdentifier<'a>>,

    pub position: usize,

    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub article: Vec<Article<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub chapter: Vec<Chapter<'a>>,
}

impl<'a> Season<'a> {
    pub fn new(position: usize) -> Self {
        Self {
            position,

            name: None,
            sort_as: None,
            identifier: None,
            alt_identifier: vec![],
            links: vec![],
            article: vec![],
            chapter: vec![],
        }
    }
}

impl<'a> From<usize> for Season<'a> {
    fn from(position: usize) -> Self {
        Self::new(position)
    }
}

/// A story arc.
///
/// See the associated [JSON Schema] for more information.
///
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/storyArc.schema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoryArc<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub name: Option<StringWithAlternates<'a>>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<StringWithAlternates<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Identifier>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub alt_identifier: Vec<AltIdentifier<'a>>,

    pub position: usize,

    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub chapter: Vec<Chapter<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub issue: Vec<Issue<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub episode: Vec<Episode<'a>>,
}

impl<'a> StoryArc<'a> {
    pub fn new(position: usize) -> Self {
        Self {
            position,

            name: None,
            sort_as: None,
            identifier: None,
            alt_identifier: vec![],
            links: vec![],
            chapter: vec![],
            issue: vec![],
            episode: vec![],
        }
    }
}

impl<'a> From<usize> for StoryArc<'a> {
    fn from(position: usize) -> Self {
        Self::new(position)
    }
}

/// An issue number of a magazine or other periodical.
///
/// See the associated [JSON Schema] for more information.
///
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/issue.schema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub name: Option<StringWithAlternates<'a>>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<StringWithAlternates<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Identifier>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub alt_identifier: Vec<AltIdentifier<'a>>,

    pub position: usize,

    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub article: Vec<Article<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub chapter: Vec<Chapter<'a>>,
}

impl<'a> Issue<'a> {
    pub fn new(position: usize) -> Self {
        Self {
            position,

            name: None,
            sort_as: None,
            identifier: None,
            alt_identifier: vec![],
            links: vec![],
            article: vec![],
            chapter: vec![],
        }
    }
}

impl<'a> From<usize> for Issue<'a> {
    fn from(position: usize) -> Self {
        Self::new(position)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chapter<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub name: Option<StringWithAlternates<'a>>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<StringWithAlternates<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Identifier>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub alt_identifier: Vec<AltIdentifier<'a>>,

    pub position: usize,

    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub series: Vec<Series<'a>>,
}

impl<'a> Chapter<'a> {
    pub fn new(position: usize) -> Self {
        Self {
            position,

            name: None,
            sort_as: None,
            identifier: None,
            alt_identifier: vec![],
            links: vec![],
            series: vec![],
        }
    }
}

impl<'a> From<usize> for Chapter<'a> {
    fn from(position: usize) -> Self {
        Self::new(position)
    }
}

/// An article.
///
/// See the associated [JSON Schema] for comparison.
///
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/article.schema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Article<'a> {
    #[serde(borrow)]
    pub name: StringWithAlternates<'a>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<StringWithAlternates<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Identifier>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub alt_identifier: Vec<AltIdentifier<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub author: Vec<Contributor<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub translator: Vec<Contributor<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub editor: Vec<Contributor<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub artist: Vec<Contributor<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub illustrator: Vec<Contributor<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub contributor: Vec<Contributor<'a>>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_pages: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<usize>,

    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,
}

impl<'a> Article<'a> {
    pub fn new(name: impl Into<StringWithAlternates<'a>>) -> Self {
        let name = name.into();

        Self {
            name,

            sort_as: None,
            identifier: None,
            alt_identifier: vec![],
            author: vec![],
            translator: vec![],
            editor: vec![],
            artist: vec![],
            illustrator: vec![],
            contributor: vec![],
            description: None,
            number_of_pages: None,
            position: None,
            links: vec![],
        }
    }
}

impl<'a> From<String> for Article<'a> {
    fn from(name: String) -> Self {
        Self::from(Cow::Owned(name))
    }
}

impl<'a> From<Cow<'a, str>> for Article<'a> {
    fn from(name: Cow<'a, str>) -> Self {
        Self::new(name)
    }
}

/// A series of publications.
///
/// See the associated [JSON Schema] for comparison.
///
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/series.schema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Series<'a> {
    #[serde(borrow)]
    pub name: StringWithAlternates<'a>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<StringWithAlternates<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Identifier>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub alt_identifier: Vec<AltIdentifier<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<usize>,

    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub chapter: Vec<Chapter<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub episode: Vec<Episode<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub issue: Vec<Issue<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub season: Vec<Season<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub story_arc: Vec<StoryArc<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub volume: Vec<Volume<'a>>,
}

impl<'a> Series<'a> {
    pub fn new(name: impl Into<StringWithAlternates<'a>>) -> Self {
        let name = name.into();

        Self {
            name,
            sort_as: None,
            identifier: None,
            alt_identifier: vec![],
            position: None,
            links: vec![],
            chapter: vec![],
            episode: vec![],
            issue: vec![],
            season: vec![],
            story_arc: vec![],
            volume: vec![],
        }
    }
}

impl<'a> From<String> for Series<'a> {
    fn from(name: String) -> Self {
        Self::from(Cow::Owned(name))
    }
}

impl<'a> From<Cow<'a, str>> for Series<'a> {
    fn from(name: Cow<'a, str>) -> Self {
        Self::new(name)
    }
}

/// A volume of a publication.
///
/// See the associated [JSON Schema] for more information.
///
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/volume.schema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub name: Option<StringWithAlternates<'a>>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<StringWithAlternates<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Identifier>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub alt_identifier: Vec<AltIdentifier<'a>>,

    pub position: usize,

    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub chapter: Vec<Chapter<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub issue: Vec<Issue<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub story_arc: Vec<StoryArc<'a>>,
}

impl<'a> Volume<'a> {
    fn new(position: usize) -> Self {
        Self {
            position,

            name: None,
            sort_as: None,
            identifier: None,
            alt_identifier: vec![],
            links: vec![],
            chapter: vec![],
            issue: vec![],
            story_arc: vec![],
        }
    }
}

impl<'a> From<usize> for Volume<'a> {
    fn from(position: usize) -> Self {
        Self::new(position)
    }
}

/// A "contains" relationship.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Contains<'a> {
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub article: Vec<Article<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub chapter: Vec<Chapter<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub episode: Vec<Episode<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub issue: Vec<Issue<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub season: Vec<Season<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub series: Vec<Series<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub story_arc: Vec<StoryArc<'a>>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_numlike"
    )]
    pub volume: Vec<Volume<'a>>,
}

/// The subject matter of a publication.
///
/// See [Default Context: Subjects] or the associated [JSON Schema] for more information.
///
/// [Default Context: Subjects]: https://readium.org/webpub-manifest/contexts/default/#subjects
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/subject.schema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject<'a> {
    #[serde(borrow)]
    pub name: StringWithAlternates<'a>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<StringWithAlternates<'a>>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub code: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<url::Url>,

    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,
}

impl<'a> Subject<'a> {
    pub fn new(name: impl Into<StringWithAlternates<'a>>) -> Self {
        let name = name.into();

        Self {
            name,
            sort_as: None,
            code: None,
            scheme: None,
            links: Vec::new(),
        }
    }
}

impl<'a> From<String> for Subject<'a> {
    fn from(name: String) -> Self {
        Self::from(Cow::Owned(name))
    }
}

impl<'a> From<Cow<'a, str>> for Subject<'a> {
    fn from(name: Cow<'a, str>) -> Self {
        Self::new(name)
    }
}

/// Information about whether text and data mining is permitted for this publication.
///
/// See [Default Context: Text and Data Mining] for more information.
///
/// [Default Context: Text and Data Mining]: https://readium.org/webpub-manifest/contexts/default/#text-and-data-mining
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataMining {
    pub reservation: Reservation,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<url::Url>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Reservation {
    All,
    None,
}

/// A map of per-language choices, where each key is a BCP 47 language tag.
///
/// See the [JSON Schema] for more details.
///
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/language-map.schema.json
#[derive(Clone, Debug)]
pub struct TaggedStrings {
    choices: Cow<'static, [(Cow<'static, langtag::LangTag>, Cow<'static, str>)]>,
}

impl TaggedStrings {
    pub const fn from_static(
        choices: &'static [(Cow<'static, langtag::LangTag>, Cow<'static, str>)],
    ) -> Self {
        Self {
            choices: Cow::Borrowed(choices),
        }
    }
}

macro_rules! tagged_strings {
  [$(($lang: literal, $str: literal)),*] => {{
      const ARR: &'static [(
          std::borrow::Cow<'static, langtag::LangTag>,
          std::borrow::Cow<'static, str>
      )] = &[
          $((std::borrow::Cow::Borrowed(langtag::langtag!($lang)), std::borrow::Cow::Borrowed($str))),*
      ];
      $crate::v2_0::TaggedStrings::from_static(ARR)
  }};
}

impl Serialize for TaggedStrings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.choices.len()))?;

        for (k, v) in self.choices.iter() {
            map.serialize_entry(k, v)?;
        }

        map.end()
    }
}

impl<'de> Deserialize<'de> for TaggedStrings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = <BTreeMap<langtag::LangTagBuf, String>>::deserialize(deserializer)?;
        let choices = map
            .into_iter()
            .map(|(tag, s)| (Cow::Owned(tag), Cow::Owned(s)))
            .collect();
        Ok(Self { choices })
    }
}

/// Either a single string, or a map of per-language choices.
///
/// See the [JSON Schema] for more details.
///
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/language-map.schema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StringWithAlternates<'a> {
    #[serde(borrow)]
    Always(Cow<'a, str>),
    Variants(TaggedStrings),
}

impl StringWithAlternates<'static> {
    pub const AUTHORS: Self = Self::Variants(tagged_strings![
        ("de", "Autoren"),
        ("en", "Authors"),
        ("es", "Autoras"),
        ("fr", "Auteurs"),
        ("pt", "Autores")
    ]);

    pub const BOOKS_ALPHABETICAL: Self = Self::Variants(tagged_strings![
        ("de", "Bücher (Alphabetisch)"),
        ("en", "Books (Alphabetical)"),
        ("es", "Libros (Orden Alfabético)"),
        ("fr", "Livres (Par ordre alphabétique)"),
        ("pt", "Livros (Por ordem alfabética)")
    ]);

    pub const BOOKS_RECENTLY_ADDED: Self = Self::Variants(tagged_strings![
        ("de", "Bücher (Kürzlich hinzugefügt)"),
        ("en", "Books (Recently Added)"),
        ("es", "Libros (Añadidos recientemente)"),
        ("fr", "Livres (Ajouts récents)"),
        ("pt", "Livros (Adicionados Recentemente)")
    ]);

    pub const CATEGORIES: Self = Self::Variants(tagged_strings![
        ("de", "Kategorien"),
        ("en", "Categories"),
        ("es", "Categorías"),
        ("fr", "Catégories"),
        ("pt", "Categorias")
    ]);

    pub const FILE_FORMATS: Self = Self::Variants(tagged_strings![
        ("de", "Dateiformate"),
        ("en", "File Formats"),
        ("es", "Formatos de archivos"),
        ("fr", "Formats de fichiers"),
        ("pt", "Formatos de Ficheiros")
    ]);

    pub const LANGUAGES: Self = Self::Variants(tagged_strings![
        ("de", "Sprachen"),
        ("en", "Languages"),
        ("es", "Idiomas"),
        ("fr", "Langues"),
        ("pt", "Línguas")
    ]);

    pub const PUBLISHERS: Self = Self::Variants(tagged_strings![
        ("de", "Verlag"),
        ("en", "Publishers"),
        ("es", "Editores"),
        ("fr", "Éditeurs"),
        ("pt", "Editores")
    ]);
}

impl<'a> From<String> for StringWithAlternates<'a> {
    fn from(s: String) -> Self {
        Self::from(Cow::Owned(s))
    }
}

impl<'a> From<&'a str> for StringWithAlternates<'a> {
    fn from(s: &'a str) -> Self {
        Self::from(Cow::Borrowed(s))
    }
}

impl<'a> From<Cow<'a, str>> for StringWithAlternates<'a> {
    fn from(s: Cow<'a, str>) -> Self {
        Self::Always(s)
    }
}

/// Information about a contributor to a publication.
///
/// See [Default Context: Contributors] or the associated [JSON Schema] for more information.
///
/// [Default Context: Contributors]: https://readium.org/webpub-manifest/contexts/default/#contributors
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/contributor.schema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributor<'a> {
    pub name: StringWithAlternates<'a>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<StringWithAlternates<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Identifier>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub alt_identifier: Vec<AltIdentifier<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub role: Vec<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,
}

impl<'a> Contributor<'a> {
    pub fn new(name: impl Into<StringWithAlternates<'a>>) -> Self {
        let name = name.into();

        Self {
            name,
            sort_as: None,
            identifier: None,
            alt_identifier: Vec::new(),
            role: Vec::new(),
            links: Vec::new(),
        }
    }
}

impl<'a> From<String> for Contributor<'a> {
    fn from(name: String) -> Self {
        Self::from(Cow::Owned(name))
    }
}

impl<'a> From<Cow<'a, str>> for Contributor<'a> {
    fn from(name: Cow<'a, str>) -> Self {
        Self::new(name)
    }
}

/// Metadata for an OPDS feed or facet.
///
/// See the [JSON Schema] for more information.
///
/// [JSON Schema]: https://drafts.opds.io/schema/feed-metadata.schema.json
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedMetadata<'a> {
    #[serde(borrow)]
    pub title: StringWithAlternates<'a>,

    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub subtitle: Vec<StringWithAlternates<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<url::Url>,

    /// The schema to which this metadata adheres.
    ///
    /// See [crate::schema] for possible values.
    #[serde(borrow, skip_serializing_if = "Option::is_none", rename = "@type")]
    pub schema: Option<Cow<'a, str>>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub modified: Option<Cow<'a, str>>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_per_page: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_items: Option<usize>,
}

impl<'a> FeedMetadata<'a> {
    pub fn new(title: impl Into<StringWithAlternates<'a>>) -> Self {
        let title = title.into();

        Self {
            title,
            subtitle: vec![],
            description: None,
            modified: None,
            identifier: None,
            schema: None,
            items_per_page: None,
            current_page: None,
            number_of_items: None,
        }
    }
}

/// Metadata for an OPDS Publication.
///
/// More information about these fields can be found in:
///
/// - [Default Context]
/// - [JSON Schema]
/// - [JSON-LD Schema]
///
/// [Default Context]: https://readium.org/webpub-manifest/contexts/default/
/// [JSON Schema]: https://readium.org/webpub-manifest/schema/metadata.schema.json
/// [JSON-LD Schema]: https://readium.org/webpub-manifest/context.jsonld
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PublicationMetadata<'a> {
    /// The schema to which this metadata adheres.
    ///
    /// See [crate::schema] for possible values.
    #[serde(borrow, skip_serializing_if = "Option::is_none", rename = "@type")]
    pub schema: Option<Cow<'a, str>>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conforms_to: Vec<url::Url>,

    /// The title of a publication.
    ///
    /// See [Default Context: Title] for more information.
    ///
    /// [Default Context: Title]: https://readium.org/webpub-manifest/contexts/default/#title
    #[serde(borrow)]
    pub title: StringWithAlternates<'a>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<StringWithAlternates<'a>>,

    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<StringWithAlternates<'a>>,

    /// The authors who worked on this publication.
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub author: Vec<Contributor<'a>>,

    /// A simple description of the publication.
    ///
    /// See [Default Context: Description] for more information.
    ///
    /// [Default Context: Description]: https://readium.org/webpub-manifest/contexts/default/#description
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,

    /// A valid URI to help identify this publication.
    ///
    /// See [Default Context: Identifier] for more information.
    ///
    /// [Default Context: Identifier]: https://readium.org/webpub-manifest/contexts/default/#identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<url::Url>,

    /// Alternates to the primary `identifier`.
    ///
    /// See [Default Context: Identifier] for more information.
    ///
    /// [Default Context: Identifier]: https://readium.org/webpub-manifest/contexts/default/#identifier
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub alt_identifier: Vec<AltIdentifier<'a>>,

    /// Accessibility-related metadata for this publication.
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<AccessibilityMetadata<'a>>,

    /// When this publication was last modified.
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub modified: Option<Cow<'a, str>>,

    /// When this publication was published.
    ///
    /// See [Default Context: Publication Date] for more information.
    ///
    /// [Default Context: Publication Date]: https://readium.org/webpub-manifest/contexts/default/#publication-date
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub published: Option<Cow<'a, str>>,

    /// Expected language of the linked resource.
    ///
    /// Languages should be in [BCP 47] syntax. You can use the [langtag::langtag] macro
    /// to help validate and parse the syntax at compile-time.
    ///
    /// See [Default Context: Language] for more information.
    ///
    /// [BCP 47]: https://www.rfc-editor.org/info/bcp47
    /// [Default Context: Language]: https://readium.org/webpub-manifest/contexts/default/#language
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec"
    )]
    pub language: Vec<Cow<'a, langtag::LangTag>>,

    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<Subject<'a>>,

    /// The layout and rendering category of this publication.
    ///
    /// See [Default Context: Layout and reading progression] for more information.
    ///
    /// [Default Context: Layout and reading progression]: https://readium.org/webpub-manifest/contexts/default/#layout-and-reading-progression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<PublicationLayout>,

    /// The reading progression for reflowable or fixed layout publications.
    ///
    /// See [Default Context: Layout and reading progression] for more information.
    ///
    /// [Default Context: Layout and reading progression]: https://readium.org/webpub-manifest/contexts/default/#layout-and-reading-progression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reading_progression: Option<ReadingProgression>,

    /// The duration in seconds of this publication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<usize>,

    /// Whether or not this is an abridged edition of this publication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abridged: Option<bool>,

    /// The number of pages in this publication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_pages: Option<usize>,

    /// The set of collections that this publication belongs to.
    ///
    /// See [Default Context: Collections and Series] for more information.
    ///
    /// [Default Context: Collections and Series]: https://readium.org/webpub-manifest/contexts/default/#collections--series
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub belongs_to: Option<BelongsTo<'a>>,

    /// The set of collections that this publication contains.
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub contains: Option<Contains<'a>>,

    /// Information about whether third parties can use this publication's content for
    /// text and data mining.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tdm: Option<DataMining>,

    /// The translators who help translate this version of the publication or its contents.
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub translator: Vec<Contributor<'a>>,

    /// The editors of this publication.
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub editor: Vec<Contributor<'a>>,

    /// Artists who contributed to this publication.
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub artist: Vec<Contributor<'a>>,

    /// The [illustrators] who contributed to this publication.
    ///
    /// [illustrators]: https://en.wikipedia.org/wiki/Illustrator
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub illustrator: Vec<Contributor<'a>>,

    /// The [letterers] who contributed to this publication.
    ///
    /// [letterers]: https://en.wikipedia.org/wiki/Letterer
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub letterer: Vec<Contributor<'a>>,

    /// The [pencilers] who contributed to this publication.
    ///
    /// [pencilers]: https://en.wikipedia.org/wiki/Penciller
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub penciler: Vec<Contributor<'a>>,

    /// The [colorists] who contributed to this publication.
    ///
    /// [colorists]: https://en.wikipedia.org/wiki/Colorist
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub colorist: Vec<Contributor<'a>>,

    /// The [inkers] who contributed to this publication.
    ///
    /// [inkers]: https://en.wikipedia.org/wiki/Inker
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub inker: Vec<Contributor<'a>>,

    /// The narrator of this publication.
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub narrator: Vec<Contributor<'a>>,

    /// A generic contributor who worked on this publication.
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub contributor: Vec<Contributor<'a>>,

    /// The publisher of this publication.
    ///
    /// See [Default Context: Publisher] for more information.
    ///
    /// [Default Context: Publisher]: https://readium.org/webpub-manifest/contexts/default/#publisher
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub publisher: Vec<Contributor<'a>>,

    /// The [imprint] of the publisher for this publication.
    ///
    /// [imprint]: https://en.wikipedia.org/wiki/Imprint_(trade_name)
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_flattened_vec_stringy"
    )]
    pub imprint: Vec<Contributor<'a>>,
}

impl<'a> PublicationMetadata<'a> {
    pub fn new(title: impl Into<StringWithAlternates<'a>>) -> Self {
        let title = title.into();

        Self {
            title,

            schema: None,
            conforms_to: vec![],
            sort_as: None,
            subtitle: None,
            description: None,
            identifier: None,
            alt_identifier: vec![],
            accessibility: None,
            modified: None,
            published: None,
            language: vec![],
            author: vec![],
            translator: vec![],
            editor: vec![],
            artist: vec![],
            illustrator: vec![],
            letterer: vec![],
            penciler: vec![],
            colorist: vec![],
            inker: vec![],
            narrator: vec![],
            contributor: vec![],
            publisher: vec![],
            imprint: vec![],
            subject: vec![],
            layout: None,
            reading_progression: None,
            duration: None,
            abridged: None,
            number_of_pages: None,
            belongs_to: None,
            contains: None,
            tdm: None,
        }
    }
}

/// Properties of an OPDS link object.
///
/// See [JSON Schema].
///
/// [JSON Schema]: https://drafts.opds.io/schema/properties.schema.json
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LinkProperties<'a> {
    /// Provide a hint about the expected number of items returned.
    #[serde(skip_serializing_if = "Option::is_none", rename = "numberOfItems")]
    pub count: Option<usize>,

    /// Indicates how the linked resource should be displayed in a reading environment that
    /// displays synthetic spreads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<PageDisplay>,

    /// Indicates the availability of a given resource.
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability<'a>>,

    /// The price of the publication (tied to its acquisition link).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,

    /// A hint for the expected media type that will be acquired after additional steps.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indirect_acquisition: Vec<Acquisition<'a>>,

    /// Library-specific feature for unavailable books that support a hold list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holds: Option<Holds>,

    /// Library-specific feature that contains information about the copies that a library has acquired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copies: Option<Copies>,
}

impl<'a> LinkProperties<'a> {
    pub fn is_empty(&self) -> bool {
        matches!(self, LinkProperties {
            count: None,
            price: None,
            page: None,
            indirect_acquisition,
            holds: None,
            copies: None,
            availability: None,
        } if indirect_acquisition.is_empty())
    }
}
