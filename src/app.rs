use super::bool::Bool;
use super::content::Content;
use super::publisher::Publisher;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// OpenRTB 2.0: This object should be included if the ad supported content
/// is a non-browser application (typically in mobile) as opposed to a website.
/// A bid request must not contain both an App and a Site object.
/// At a minimum, it is useful to provide an App ID or bundle,
/// but this is not strictly required.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct App {
    /// Application ID on the exchange.
    /// RECOMMENDED by the OpenRTB specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Application name (may be aliased at publisher's request). App names for
    /// SDK-less requests (mostly from connected TVs) can be provided by the
    /// publisher directly in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Domain of the application. For example, "mygame.foo.com".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Array of IAB content categories of the app.
    /// See enum ContentCategory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,

    /// Array of IAB content categories that describe the current section
    /// of the app.
    /// See enum ContentCategory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<String>>,

    /// Array of IAB content categories that describe the current page or view
    /// of the app.
    /// See enum ContentCategory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<String>>,

    /// Application version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,

    /// A platform-specific application identifier intended to be
    /// unique to the app and independent of the exchange. On Android,
    /// this should be a bundle or package name (e.g., com.foo.mygame).
    /// On iOS, it is a numeric ID. For SDK-less requests (mostly from connected
    /// TVs), it can be provided by the publisher directly in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<String>,

    /// Indicates if the app has a privacy policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacypolicy: Option<Bool>,

    /// false = app is free, true = the app is a paid version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<Bool>,

    /// Details about the Publisher (Section 3.2.8) of the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Publisher>,

    /// Details about the Content (Section 3.2.9) within the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,

    /// Comma separated list of keywords about the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// App store URL for an installed app; for QAG 1.5 compliance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storeurl: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}
