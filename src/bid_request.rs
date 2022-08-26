use super::app::App;
use super::bool::Bool;
use super::device::Device;
use super::imp::Imp;
use super::regs::Regs;
use super::site::Site;
use super::source::Source;
use super::user::User;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Clone)]
pub struct BidRequest {
    /// Unique ID of the bid request, provided by the exchange.
    /// REQUIRED by the OpenRTB specification.
    pub id: String,

    /// Array of Imp objects (Section 3.2.2) representing the impressions offered.
    /// At least 1 Imp object is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imp: Option<Vec<Imp>>,

    /// Details via a Site object (Section 3.2.6) about the publisher's website.
    /// Only applicable and recommended for websites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,

    /// Details via an App object (Section 3.2.7) about the publisher's app
    /// (non-browser applications). Only applicable and recommended for apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,

    /// Details via a Device object (Section 3.2.11) about the user's
    /// device to which the impression will be delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,

    /// A Regs object (Section 3.2.16) that specifies any industry, legal,
    /// or governmental regulations in force for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regs: Option<Regs>,

    /// Details via a User object (Section 3.2.13) about the human
    /// user of the device; the advertising audience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    /// Auction type, where 1 = First Price, 2 = Second Price Plus.
    /// Exchange-specific auction types can be defined using values > 500.
    pub at: AuctionType,

    /// Maximum time in milliseconds to submit a bid to avoid timeout.
    /// This value is commonly communicated offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmax: Option<i32>,

    /// Allowlist of buyer seats (e.g., advertisers, agencies) that can bid on this
    /// impression. IDs of seats and knowledge of the buyer's customers to which
    /// they refer must be coordinated between bidders and the exchange a priori.
    /// Omission implies no seat restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wseat: Option<Vec<String>>,

    /// Flag to indicate if Exchange can verify that the impressions offered
    /// represent all of the impressions available in context (e.g., all on the
    /// web page, all video spots such as pre/mid/post roll) to support
    /// road-blocking. false = no or unknown, true = yes, the impressions offered
    /// represent all that are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allimps: Option<Bool>,

    /// Array of allowed currencies for bids on this bid request using ISO-4217
    /// alpha codes. Recommended only if the exchange accepts multiple currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cur: Option<Vec<String>>,

    /// Blocked advertiser categories using the IAB content categories.
    /// Refer to enum ContentCategory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcat: Option<Vec<String>>,

    /// Block list of advertisers by their domains (e.g., "ford.com").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badv: Option<Vec<String>>,

    /// Block list of applications by their platform-specific exchange
    /// independent application identifiers. On Android, these should
    /// be bundle or package names (e.g., com.foo.mygame).
    /// On iOS, these are numeric IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bapp: Option<Vec<String>>,

    /// Block list of buyer seats (e.g., advertisers, agencies) restricted
    /// from bidding on this impression. IDs of seats and knowledge
    /// of the buyer's customers to which they refer must be
    /// coordinated between bidders and the exchange a priori.
    /// At most, only one of wseat and bseat should be used in the
    /// same request. Omission of both implies no seat restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bseat: Option<Vec<String>>,

    /// Allowlist of languages for creatives using ISO-639-1-alpha-2.
    /// Omission implies no specific restrictions, but buyers would be
    /// advised to consider language attribute in the Device and/or
    /// Content objects if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wlang: Option<Vec<String>>,

    /// A Source object (Section 3.2.2) that provides data about the
    /// inventory source and which entity makes the final decision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    /// Indicator of test mode in which auctions are not billable,
    /// where false = live mode, true = test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<Bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AuctionType {
    FirstPrice,
    SecondPricePlus,
    ExchangeSpecific(u32),
}

impl serde::Serialize for AuctionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            AuctionType::FirstPrice => serializer.serialize_u32(1),
            AuctionType::SecondPricePlus => serializer.serialize_u32(2),
            AuctionType::ExchangeSpecific(t) => serializer.serialize_u32(t),
        }
    }
}

impl<'de> Deserialize<'de> for AuctionType {
    fn deserialize<D>(deserializer: D) -> Result<AuctionType, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Deserialize::deserialize(deserializer) {
            Ok(1) => Ok(AuctionType::FirstPrice),
            Ok(2) => Ok(AuctionType::SecondPricePlus),
            Ok(t) => Ok(AuctionType::ExchangeSpecific(t)),
            Err(e) => Err(e),
        }
    }
}
