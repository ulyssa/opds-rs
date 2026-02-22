//! Constants for commonly-used values for the `@type` field
//!
//! The `@type` field is used to identify the schema which the metadata fields
//! fall under. This will typically be [SCHEMA_ORG_BOOK], but other schemas may
//! be used, as well.

/// An article within a newspaper, magazine, or other publication.
pub const SCHEMA_ORG_ARTICLE: &str = "http://schema.org/Article";

/// A book.
pub const SCHEMA_ORG_BOOK: &str = "http://schema.org/Book";

/// A comic book.
pub const SCHEMA_ORG_COMIC_STORY: &str = "http://schema.org/ComicStory";

/// A single feed providing structured information.
pub const SCHEMA_ORG_DATA_FEED: &str = "http://schema.org/DataFeed";

/// The most generic type of item.
pub const SCHEMA_ORG_THING: &str = "http://schema.org/Thing";
