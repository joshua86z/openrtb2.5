// Copyright 2022 Google Inc. All Rights Reserved.

// OpenRTB extensions ("ext" fields in the spec & JSON representation)
// are represented here by Protocol Buffer extensions. This proto only
// reserves the range of IDs 100-9999 at every extensible object.
// Reserved ranges:
//    100-199:   Reserved for Google.
//    200-299:   Reserved for IAB's formal standard extensions.
//    300-999:   Free for use with other exchanges or projects.
//    1000-1999: Reserved for Google.
//    2000-9999: Free for use with other exchanges or projects.

use self::bool::Bool;
use bid_request::{App, Site};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use serde_repr::*;

/// OpenRTB 2.0: The top-level bid request object contains a globally unique
/// bid request or auction ID. This id attribute is required as is at least one
/// impression object (Section 3.2.2). Other attributes in this top-level object
/// establish rules and restrictions that apply to all impressions being offered.
///
/// There are also several subordinate objects that provide detailed data to
/// potential buyers. Among these are the Site and App objects, which describe
/// the type of published media in which the impression(s) appear.
/// These objects are highly recommended, but only one applies to a given
/// bid request depending on whether the media is browser-based web content
/// or a non-browser application, respectively.
#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BidRequest {
    /// Unique ID of the bid request, provided by the exchange.
    /// REQUIRED by the OpenRTB specification.
    pub id: String,

    /// Array of Imp objects (Section 3.2.2) representing the impressions offered.
    /// At least 1 Imp object is required.
    pub imp: Vec<bid_request::Imp>,

    /// Details via a Device object (Section 3.2.11) about the user's
    /// device to which the impression will be delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<bid_request::Device>,

    /// A Regs object (Section 3.2.16) that specifies any industry, legal,
    /// or governmental regulations in force for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regs: Option<bid_request::Regs>,

    /// Details via a User object (Section 3.2.13) about the human
    /// user of the device; the advertising audience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<bid_request::User>,

    /// Auction type, where 1 = First Price, 2 = Second Price Plus.
    /// Exchange-specific auction types can be defined using values > 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<AuctionType>,

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

    /// Indicator of test mode in which auctions are not billable,
    /// where false = live mode, true = test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<Bool>,

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
    pub source: Option<bid_request::Source>,

    /// Details via a Site object (Section 3.2.6) about the publisher's website.
    /// Only applicable and recommended for websites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,

    /// Details via an App object (Section 3.2.7) about the publisher's app
    /// (non-browser applications). Only applicable and recommended for apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,

    /// Extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Nested message and enum types in `BidRequest`.
pub mod bid_request {
    use super::bool::Bool;
    use super::{
        ConnectionType, ContentContext, DeviceType, LocationService, LocationType,
        ProductionQuality, QagMediaRating,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    /// OpenRTB 2.5: This object describes the nature and behavior of the entity
    /// that is the source of the bid request upstream from the exchange.
    /// The primary purpose of this object is to define post-auction or upstream
    /// decisioning when the exchange itself does not control the final decision.
    /// A common example of this is header bidding, but it can also apply to
    /// upstream server entities such as another RTB exchange, a mediation
    /// platform, or an ad server combines direct campaigns with 3rd party
    /// demand in decisioning.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
    pub struct Source {
        /// Entity responsible for the final impression sale decision,
        /// where false = exchange, true = upstream source
        /// RECOMMENDED by the OpenRTB specification.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fd: Option<Bool>,

        /// Transaction ID that must be common across all participants in
        /// this bid request (e.g., potentially multiple exchanges).
        /// RECOMMENDED by the OpenRTB specification.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tid: Option<String>,

        /// Payment ID chain string containing embedded syntax
        /// described in the TAG Payment ID Protocol v1.0.
        /// RECOMMENDED by the OpenRTB specification.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pchain: Option<String>,

        /// Extensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }

    /// OpenRTB 2.0: This object describes an ad placement or impression
    /// being auctioned.  A single bid request can include multiple Imp objects,
    /// a use case for which might be an exchange that supports selling all
    /// ad positions on a given page.  Each Imp object has a required ID so that
    /// bids can reference them individually.
    ///
    /// The presence of Banner (Section 3.2.3), Video (Section 3.2.4),
    /// and/or Native (Section 3.2.5) objects subordinate to the Imp object
    /// indicates the type of impression being offered. The publisher can choose
    /// one such type which is the typical case or mix them at their discretion.
    /// Any given bid for the impression must conform to one of the offered types.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
    pub struct Imp {
        /// A unique identifier for this impression within the context of the bid
        /// request (typically, value starts with 1, and increments up to n
        /// for n impressions).
        pub id: String,

        /// A Banner object (Section 3.2.3); required if this impression is
        /// offered as a banner ad opportunity.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub banner: Option<imp::Banner>,

        /// A Video object (Section 3.2.4); required if this impression is
        /// offered as a video ad opportunity.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub video: Option<imp::Video>,

        /// An Audio object; required if this impression is offered
        /// as an audio ad opportunity.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub audio: Option<imp::Audio>,

        /// Name of ad mediation partner, SDK technology, or player responsible
        /// for rendering ad (typically video or mobile). Used by some ad servers
        /// to customize ad code by partner. Recommended for video and/or apps.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub displaymanager: Option<String>,

        /// Version of ad mediation partner, SDK technology, or player responsible
        /// for rendering ad (typically video or mobile). Used by some ad servers
        /// to customize ad code by partner. Recommended for video and/or apps.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub displaymanagerver: Option<String>,

        /// true = the ad is interstitial or full screen, false = not interstitial.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub instl: Option<Bool>,

        /// Identifier for specific ad placement or ad tag that was used to
        /// initiate the auction. This can be useful for debugging of any issues,
        /// or for optimization by the buyer.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tagid: Option<String>,

        /// Minimum bid for this impression expressed in CPM.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bidfloor: Option<f64>,

        /// Currency specified using ISO-4217 alpha codes. This may be different
        /// from bid currency returned by bidder if this is allowed by the exchange.
        // #[p(string, optional, tag = "9", default = "USD")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bidfloorcur: Option<String>,

        /// Indicates the type of browser opened upon clicking the
        /// creative in an app, where false = embedded, true = native.
        /// Note that the Safari View Controller in iOS 9.x devices is considered
        /// a native browser for purposes of this attribute.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub clickbrowser: Option<Bool>,

        /// Flag to indicate if the impression requires secure HTTPS URL creative
        /// assets and markup.  If omitted, the secure state is unknown, but
        /// non-secure HTTP support can be assumed.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub secure: Option<Bool>,

        /// Array of exchange-specific names of supported iframe busters.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub iframebuster: Option<Vec<String>>,

        /// A Pmp object (Section 3.2.17) containing any private marketplace deals
        /// in effect for this impression.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pmp: Option<imp::Pmp>,

        /// A Native object (Section 3.2.5); required if this impression is
        /// offered as a native ad opportunity.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub native: Option<imp::Native>,

        /// Advisory as to the number of seconds that may elapse
        /// between the auction and the actual impression.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub exp: Option<i32>,

        /// An array of Metric object (Section 3.2.5).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub metric: Option<Vec<imp::Metric>>,

        /// Extensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }

    /// Nested message and enum types in `Imp`.
    pub mod imp {
        use super::super::bool::Bool;
        use super::super::{
            AdPosition, ApiFramework, BannerAdType, CompanionType, ContentDeliveryMethod,
            CreativeAttribute, ExpandableDirection, FeedType, NativeRequest, PlaybackCessationMode,
            PlaybackMethod, Protocol, VideoLinearity, VideoPlacementType, VolumeNormalizationMode,
        };
        use serde::{Deserialize, Serialize};
        use serde_json::Value;

        /// OpenRTB 2.5: This object is associated with an impression as
        /// an array of metrics. These metrics can offer insight into
        /// the impression to assist with decisioning such as average recent
        /// viewability, click-through rate, etc.  Each metric is identified
        /// by its type, reports the value of the metric, and optionally
        /// identifies the source or vendor measuring the value.
        #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct Metric {
            /// Type of metric being presented using exchange curated string
            /// names which should be published to bidders a priori.
            /// REQUIRED by the OpenRTB specification.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,

            /// Number representing the value of the metric.
            /// Probabilities must be in the range 0.0 - 1.0.
            /// REQUIRED by the OpenRTB specification.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub value: Option<f64>,

            /// Source of the value using exchange curated string names
            /// which should be published to bidders a priori.
            /// If the exchange itself is the source versus a third party,
            /// "EXCHANGE" is recommended.
            /// RECOMMENDED by the OpenRTB specification.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub vendor: Option<String>,

            /// Extensions.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ext: Option<Value>,
        }

        /// OpenRTB 2.0: This object represents the most general type of
        /// impression.  Although the term "banner" may have very specific meaning
        /// in other contexts, here it can be many things including a simple static
        /// image, an expandable ad unit, or even in-banner video (refer to the Video
        /// object in Section 3.2.4 for the more generalized and full featured video
        /// ad units). An array of Banner objects can also appear within the Video
        /// to describe optional companion ads defined in the VAST specification.
        ///
        /// The presence of a Banner as a subordinate of the Imp object indicates
        /// that this impression is offered as a banner type impression.
        /// At the publisher's discretion, that same impression may also be offered
        /// as video and/or native by also including as Imp subordinates the Video
        /// and/or Native objects, respectively. However, any given bid for the
        /// impression must conform to one of the offered types.
        #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct Banner {
            /// Width in device independent pixels (DIPS).
            /// If no format objects are specified, this is an exact width
            /// requirement. Otherwise it is a preferred width.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub w: Option<i32>,

            /// Height in device independent pixels (DIPS).
            /// If no format objects are specified, this is an exact height
            /// requirement. Otherwise it is a preferred height.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub h: Option<i32>,

            /// Array of format objects representing the banner sizes permitted.
            /// If none are specified, then use of the h and w attributes
            /// is highly recommended.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub format: Option<Vec<banner::Format>>,

            /// Unique identifier for this banner object. Recommended when Banner
            /// objects are used with a Video object (Section 3.2.4) to represent
            /// an array of companion ads. Values usually start at 1 and increase
            /// with each object; should be unique within an impression.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub id: Option<String>,

            /// Ad position on screen.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub pos: Option<AdPosition>,

            /// Blocked banner ad types.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub btype: Option<Vec<BannerAdType>>,

            /// Blocked creative attributes.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub battr: Option<Vec<CreativeAttribute>>,

            /// Allowlist of content MIME types supported. Popular MIME types include,
            /// but are not limited to "image/jpg", "image/gif" and
            /// "application/x-shockwave-flash".
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mimes: Option<Vec<String>>,

            /// Specify if the banner is delivered in the top frame (true)
            /// or in an iframe (false).
            #[serde(skip_serializing_if = "Option::is_none")]
            pub topframe: Option<Bool>,

            /// Directions in which the banner may expand.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub expdir: Option<Vec<ExpandableDirection>>,

            /// List of supported API frameworks for this impression.
            /// If an API is not explicitly listed, it is assumed not to be supported.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub api: Option<Vec<ApiFramework>>,

            /// Relevant only for Banner objects used with a Video object
            /// (Section 3.2.7) in an array of companion ads. Indicates the
            /// companion banner rendering mode relative to the associated
            /// video, where false = concurrent, true = end-card.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub vcm: Option<Bool>,

            /// DEPRECATED in OpenRTB 2.4+. Prefer the field <code>format</code>.
            /// Maximum width in device independent pixels (DIPS).
            #[deprecated]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub wmax: Option<i32>,

            /// DEPRECATED in OpenRTB 2.4+. Prefer the field <code>format</code>.
            /// Maximum height in device independent pixels (DIPS).
            #[deprecated]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub hmax: Option<i32>,

            /// DEPRECATED in OpenRTB 2.4+. Prefer the field <code>format</code>.
            /// Minimum width in device independent pixels (DIPS).
            #[deprecated]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub wmin: Option<i32>,

            /// DEPRECATED in OpenRTB 2.4+. Prefer the field <code>format</code>.
            /// Minimum height in device independent pixels (DIPS).
            #[deprecated]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub hmin: Option<i32>,

            /// Extensions.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ext: Option<Value>,
        }

        /// Nested message and enum types in `Banner`.
        pub mod banner {
            use serde::{Deserialize, Serialize};
            use serde_json::Value;

            /// OpenRTB 2.4: This object represents an allowed size (i.e.,
            /// height and width combination) for a banner impression.
            /// These are typically used in an array for an impression where
            /// multiple sizes are permitted.
            #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
            pub struct Format {
                /// Width in device independent pixels (DIPS).
                #[serde(skip_serializing_if = "Option::is_none")]
                pub w: Option<i32>,

                /// Height in device independent pixels (DIPS).
                #[serde(skip_serializing_if = "Option::is_none")]
                pub h: Option<i32>,

                /// Relative width when expressing size as a ratio.
                #[serde(skip_serializing_if = "Option::is_none")]
                pub wratio: Option<i32>,

                /// Relative height when expressing size as a ratio.
                #[serde(skip_serializing_if = "Option::is_none")]
                pub hratio: Option<i32>,

                /// The minimum width in device independent pixels (DIPS) at
                /// which the ad will be displayed when the size is expressed as a ratio.
                #[serde(skip_serializing_if = "Option::is_none")]
                pub wmin: Option<i32>,

                /// Extensions.
                #[serde(skip_serializing_if = "Option::is_none")]
                pub ext: Option<Value>,
            }
        }

        /// OpenRTB 2.0: This object represents an in-stream video impression.
        /// Many of the fields are non-essential for minimally viable transactions,
        /// but are included to offer fine control when needed. Video in OpenRTB
        /// generally assumes compliance with the VAST standard. As such, the notion
        /// of companion ads is supported by optionally including an array of Banner
        /// objects (refer to the Banner object in Section 3.2.3) that define these
        /// companion ads.
        ///
        /// The presence of a Video as a subordinate of the Imp object indicates
        /// that this impression is offered as a video type impression. At the
        /// publisher's discretion, that same impression may also be offered as
        /// banner and/or native by also including as Imp subordinates the Banner
        /// and/or Native objects, respectively. However, any given bid for the
        /// impression must conform to one of the offered types.
        #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct Video {
            /// Allowlist of content MIME types supported. Popular MIME types include,
            /// but are not limited to "image/jpg", "image/gif" and
            /// "application/x-shockwave-flash".
            /// REQUIRED by the OpenRTB specification: at least 1 element.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mimes: Option<Vec<String>>,

            /// Minimum video ad duration in seconds.
            /// RECOMMENDED by the OpenRTB specification.
            // #[p(int32, optional, tag = "3", default = "0")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub minduration: Option<i32>,

            /// Maximum video ad duration in seconds.
            /// RECOMMENDED by the OpenRTB specification.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub maxduration: Option<i32>,

            /// Indicates the start delay in seconds for pre-roll, mid-roll, or
            /// post-roll ad placements.
            /// Refer to enum StartDelay for generic values.
            /// RECOMMENDED by the OpenRTB specification.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub startdelay: Option<i32>,

            /// Array of supported video bid response protocols.
            /// At least one supported protocol must be specified.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub protocols: Option<Vec<Protocol>>,

            /// Width of the video player in device independent pixels (DIPS).
            /// RECOMMENDED by the OpenRTB specification.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub w: Option<i32>,

            /// Height of the video player in device independent pixels (DIPS).
            /// RECOMMENDED by the OpenRTB specification.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub h: Option<i32>,

            /// Placement type for the impression.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub placement: Option<VideoPlacementType>,

            /// Indicates if the impression must be linear, nonlinear, etc.
            /// If none specified, assume all are allowed.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub linearity: Option<VideoLinearity>,

            /// Indicates if the player will allow the video to be skipped.
            /// If a bidder sends markup/creative that is itself skippable, the
            /// Bid object should include the attr array with an element of
            /// 16 indicating skippable video.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub skip: Option<Bool>,

            /// Videos of total duration greater than this number of seconds
            /// can be skippable; only applicable if the ad is skippable.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub skipmin: Option<i32>,

            /// Number of seconds a video must play before skipping is
            /// enabled; only applicable if the ad is skippable.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub skipafter: Option<i32>,

            /// If multiple ad impressions are offered in the same bid request,
            /// the sequence number will allow for the coordinated delivery of
            /// multiple creatives.
            // #[p(int32, optional, tag = "9", default = "1")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub sequence: Option<i32>,

            /// Blocked creative attributes.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub battr: Option<Vec<CreativeAttribute>>,

            /// Maximum extended video ad duration, if extension is allowed.
            /// If blank or 0, extension is not allowed. If -1, extension is allowed,
            /// and there is no time limit imposed. If greater than 0, then the value
            /// represents the number of seconds of extended play supported beyond
            /// the maxduration value.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub maxextended: Option<i32>,

            /// Minimum bit rate in Kbps.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub minbitrate: Option<i32>,

            /// Maximum bit rate in Kbps.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub maxbitrate: Option<i32>,

            /// Indicates if letter-boxing of 4:3 content into a 16:9 window is
            /// allowed.
            // #[p(bool, optional, tag = "14", default = "true")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub boxingallowed: Option<Bool>,

            /// Playback methods that may be in use. If none are specified, any
            /// method may be used. Only one method is typically used in practice.
            /// As a result, this array may be converted to an integer in a future
            /// version of the specification. It is strongly advised to use only
            /// the first element of this array in preparation for this change.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub playbackmethod: Option<Vec<PlaybackMethod>>,

            /// The event that causes playback to end.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub playbackend: Option<PlaybackCessationMode>,

            /// Supported delivery methods (e.g., streaming, progressive).
            /// If none specified, assume all are supported.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub delivery: Option<Vec<ContentDeliveryMethod>>,

            /// Ad position on screen.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub pos: Option<AdPosition>,

            /// Array of Banner objects (Section 3.2.3) if companion ads are available.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub companionad: Option<Vec<Banner>>,

            /// List of supported API frameworks for this impression.
            /// If an API is not explicitly listed, it is assumed not to be supported.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub api: Option<Vec<ApiFramework>>,

            /// Supported VAST companion ad types.  Recommended if companion Banner
            /// objects are included via the companionad array.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub companiontype: Option<Vec<CompanionType>>,

            /// DEPRECATED in OpenRTB 2.3+. Prefer the field <code>protocols</code>.
            /// Video bid response protocol.
            #[deprecated]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub protocol: Option<Protocol>,

            /// Extensions.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ext: Option<Value>,
        }

        /// This object represents an audio type impression. Many of the fields
        /// are non-essential for minimally viable transactions, but are included
        /// to offer fine control when needed. Audio in OpenRTB generally assumes
        /// compliance with the DAAST standard. As such, the notion of companion
        /// ads is supported by optionally including an array of Banner objects
        /// that define these companion ads.
        ///
        /// The presence of a Audio as a subordinate of the Imp object indicates
        /// that this impression is offered as an audio type impression.
        /// At the publisher's discretion, that same impression may also be offered
        /// as banner, video, and/or native by also including as Imp subordinates
        /// objects of those types. However, any given bid for the impression must
        /// conform to one of the offered types.
        #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct Audio {
            /// Content MIME types supported (e.g., "audio/mp4").
            /// REQUIRED by the OpenRTB specification: at least 1 element.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mimes: Option<Vec<String>>,

            /// Minimum audio ad duration in seconds.
            /// RECOMMENDED by the OpenRTB specification.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub minduration: Option<i32>,

            /// Maximum audio ad duration in seconds.
            /// RECOMMENDED by the OpenRTB specification.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub maxduration: Option<i32>,

            /// Array of supported audio protocols.
            /// RECOMMENDED by the OpenRTB specification.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub protocols: Option<Vec<Protocol>>,

            /// Indicates the start delay in seconds for pre-roll, mid-roll, or
            /// post-roll ad placements.
            /// Refer to enum StartDelay for generic values.
            /// RECOMMENDED by the OpenRTB specification.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub startdelay: Option<i32>,

            /// If multiple ad impressions are offered in the same bid request,
            /// the sequence number will allow for the coordinated delivery of
            /// multiple creatives.
            // #[p(int32, optional, tag = "6", default = "1")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub sequence: Option<i32>,

            /// Blocked creative attributes.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub battr: Option<Vec<CreativeAttribute>>,

            /// Maximum extended video ad duration, if extension is allowed.
            /// If blank or 0, extension is not allowed. If -1, extension is allowed,
            /// and there is no time limit imposed. If greater than 0, then the value
            /// represents the number of seconds of extended play supported beyond
            /// the maxduration value.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub maxextended: Option<i32>,

            /// Minimum bit rate in Kbps.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub minbitrate: Option<i32>,

            /// Maximum bit rate in Kbps.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub maxbitrate: Option<i32>,

            /// Supported delivery methods (e.g., streaming, progressive).
            /// If none specified, assume all are supported.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub delivery: Option<Vec<ContentDeliveryMethod>>,

            /// Array of Banner objects if companion ads are available.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub companionad: Option<Vec<Banner>>,

            /// List of supported API frameworks for this impression.
            /// If an API is not explicitly listed, it is assumed not to be supported.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub api: Option<Vec<ApiFramework>>,

            /// Supported DAAST companion ad types.  Recommended if companion Banner
            /// objects are included via the companionad array.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub companiontype: Option<Vec<CompanionType>>,

            /// The maximum number of ads that can be played in an ad pod.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub maxseq: Option<i32>,

            /// Type of audio feed.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub feed: Option<FeedType>,

            /// Indicates if the ad is stitched with audio content or delivered
            /// independently.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub stitched: Option<Bool>,

            /// Volume normalization mode.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub nvol: Option<VolumeNormalizationMode>,

            /// Extensions.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ext: Option<Value>,
        }

        /// OpenRTB 2.3: This object represents a native type impression.
        /// Native ad units are intended to blend seamlessly into the surrounding
        /// content (e.g., a sponsored Twitter or Facebook post). As such, the
        /// response must be well-structured to afford the publisher fine-grained
        /// control over rendering.
        ///
        /// The Native Subcommittee has developed a companion specification to
        /// OpenRTB called the Native Ad Specification. It defines the request
        /// parameters and response markup structure of native ad units.
        /// This object provides the means of transporting request parameters as an
        /// opaque string so that the specific parameters can evolve separately
        /// under the auspices of the Native Ad Specification. Similarly, the
        /// ad markup served will be structured according to that specification.
        ///
        /// The presence of a Native as a subordinate of the Imp object indicates
        /// that this impression is offered as a native type impression.
        /// At the publisher's discretion, that same impression may also be offered
        /// as banner and/or video by also including as Imp subordinates the Banner
        /// and/or Video objects, respectively. However, any given bid for the
        /// impression must conform to one of the offered types.
        #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct Native {
            /// Request payload complying with the Native Ad Specification.
            /// Exactly one of {request, request_native} should be used;
            /// this is the OpenRTB-compliant field for JSON serialization.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub request: Option<String>,

            /// Request payload complying with the Native Ad Specification.
            /// Exactly one of {request, request_native} should be used;
            /// this is an alternate field preferred for Protobuf serialization.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub request_native: Option<NativeRequest>,

            /// Version of the Native Ad Specification to which request complies.
            /// RECOMMENDED by the OpenRTB specification.
            pub ver: Option<String>,

            /// List of supported API frameworks for this impression.
            /// If an API is not explicitly listed, it is assumed not to be supported.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub api: Option<Vec<ApiFramework>>,

            /// Blocked creative attributes.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub battr: Option<Vec<CreativeAttribute>>,

            /// Extensions.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ext: Option<Value>,
        }
        // /// Nested message and enum types in `Native`.
        // pub mod native {
        //     #[derive(Clone, PartialEq, ::prost::Oneof)]
        //     pub enum RequestOneof {
        //         /// Request payload complying with the Native Ad Specification.
        //         /// Exactly one of {request, request_native} should be used;
        //         /// this is the OpenRTB-compliant field for JSON serialization.
        //         #[serde(skip_serializing_if = "Option::is_none")]
        //         Request(String),
        //         /// Request payload complying with the Native Ad Specification.
        //         /// Exactly one of {request, request_native} should be used;
        //         /// this is an alternate field preferred for Protobuf serialization.
        //         #[serde(skip_serializing_if = "Option::is_none")]
        //         RequestNative(super::super::super::NativeRequest),
        //     }
        // }
        /// OpenRTB 2.2: This object is the private marketplace container for
        /// direct deals between buyers and sellers that may pertain to this
        /// impression. The actual deals are represented as a collection of
        /// Deal objects. Refer to Section 7.2 for more details.
        #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct Pmp {
            /// Indicator of auction eligibility to seats named in the Direct Deals
            /// object, where false = all bids are accepted, true = bids are restricted
            /// to the deals specified and the terms thereof.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub private_auction: Option<Bool>,

            /// Array of Deal (Section 3.2.18) objects that convey the specific deals
            /// applicable to this impression.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub deals: Option<Vec<pmp::Deal>>,

            /// Extensions.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ext: Option<Value>,
        }

        /// Nested message and enum types in `Pmp`.
        pub mod pmp {
            use super::super::super::AuctionType;
            use serde::{Deserialize, Serialize};
            use serde_json::Value;

            /// OpenRTB 2.2: This object constitutes a specific deal that was struck
            /// a priori between a buyer and a seller. Its presence with the Pmp
            /// collection indicates that this impression is available under the terms
            /// of that deal. Refer to Section 7.2 for more details.
            #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
            pub struct Deal {
                /// A unique identifier for the direct deal.
                /// REQUIRED by the OpenRTB specification.
                pub id: String,

                /// Minimum bid for this impression expressed in CPM.
                #[serde(skip_serializing_if = "Option::is_none")]
                pub bidfloor: Option<f64>,

                /// Currency specified using ISO-4217 alpha codes. This may be different
                /// from bid currency returned by bidder if this is allowed
                /// by the exchange.
                // #[prost(string, optional, tag = "3", default = "USD")]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub bidfloorcur: Option<String>,

                /// Allowlist of buyer seats (e.g., advertisers, agencies) that can bid
                /// on this deal. IDs of seats and knowledge of the buyer's customers to
                /// which they refer must be coordinated between bidders and the exchange
                /// a priori. Omission implies no seat restrictions.
                #[serde(skip_serializing_if = "Option::is_none")]
                pub wseat: Option<Vec<String>>,

                /// Array of advertiser domains (e.g., advertiser.com) allowed to
                /// bid on this deal. Omission implies no advertiser restrictions.
                #[serde(skip_serializing_if = "Option::is_none")]
                pub wadomain: Option<Vec<String>>,

                /// Optional override of the overall auction type of the bid request,
                /// where 1 = First Price, 2 = Second Price Plus, 3 = the value passed
                /// in bidfloor is the agreed upon deal price. Additional auction types
                /// can be defined by the exchange.
                #[serde(skip_serializing_if = "Option::is_none")]
                pub at: Option<AuctionType>,

                /// Extensions.
                #[serde(skip_serializing_if = "Option::is_none")]
                pub ext: Option<Value>,
            }
        }
    }

    /// OpenRTB 2.0: This object should be included if the ad supported content
    /// is a website as opposed to a non-browser application. A bid request must
    /// not contain both a Site and an App object. At a minimum, it is useful to
    /// provide a site ID or page URL, but this is not strictly required.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
    pub struct Site {
        /// Site ID on the exchange.
        /// RECOMMENDED by the OpenRTB specification.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Site name (may be masked at publisher's request).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        /// Domain of the site, used for advertiser side blocking.
        /// For example, "foo.com".
        #[serde(skip_serializing_if = "Option::is_none")]
        pub domain: Option<String>,

        /// Array of IAB content categories of the site.
        /// See enum ContentCategory.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cat: Option<Vec<String>>,

        /// Array of IAB content categories that describe the current section
        /// of the site.
        /// See enum ContentCategory.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sectioncat: Option<Vec<String>>,

        /// Array of IAB content categories that describe the current page or view
        /// of the site.
        /// See enum ContentCategory.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pagecat: Option<Vec<String>>,

        /// URL of the page where the impression will be shown.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<String>,

        /// Indicates if the site has a privacy policy.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub privacypolicy: Option<Bool>,

        /// Referrer URL that caused navigation to the current page.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#ref: Option<String>,

        /// Search string that caused navigation to the current page.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub search: Option<String>,

        /// Details about the Publisher (Section 3.2.8) of the site.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub publisher: Option<Publisher>,

        /// Details about the Content (Section 3.2.9) within the site.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub content: Option<Content>,

        /// Comma separated list of keywords about this site.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub keywords: Option<String>,

        /// Indicates if the site has been programmed to optimize layout
        /// when viewed on mobile devices.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mobile: Option<Bool>,

        /// Extensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }

    /// OpenRTB 2.0: This object should be included if the ad supported content
    /// is a non-browser application (typically in mobile) as opposed to a website.
    /// A bid request must not contain both an App and a Site object.
    /// At a minimum, it is useful to provide an App ID or bundle,
    /// but this is not strictly required.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
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

        /// Extensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }

    /// OpenRTB 2.0: This object describes the publisher of the media in which
    /// the ad will be displayed. The publisher is typically the seller
    /// in an OpenRTB transaction.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
    pub struct Publisher {
        /// Exchange-specific publisher ID.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Publisher name (may be aliased at publisher's request).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        /// Array of IAB content categories that describe the publisher.
        /// See enum ContentCategory.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cat: Option<Vec<String>>,

        /// Highest level domain of the publisher (e.g., "publisher.com").
        #[serde(skip_serializing_if = "Option::is_none")]
        pub domain: Option<String>,

        /// Extensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }

    /// OpenRTB 2.0: This object describes the content in which the impression
    /// will appear, which may be syndicated or non-syndicated content.
    /// This object may be useful when syndicated content contains impressions and
    /// does not necessarily match the publisher's general content.
    /// The exchange might or might not have knowledge of the page where the
    /// content is running, as a result of the syndication method.
    /// For example might be a video impression embedded in an iframe on an
    /// unknown web property or device.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
    pub struct Content {
        /// ID uniquely identifying the content.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Content episode number (typically applies to video content).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub episode: Option<i32>,

        /// Content title.
        /// Video Examples: "Search Committee" (television), "A New Hope" (movie),
        /// or "Endgame" (made for web).
        /// Non-Video Example: "Why an Antarctic Glacier Is Melting So Quickly"
        /// (Time magazine article).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,

        /// Content series.
        /// Video Examples: "The Office" (television), "Star Wars" (movie),
        /// or "Arby 'N' The Chief" (made for web).
        /// Non-Video Example: "Ecocentric" (Time Magazine blog).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub series: Option<String>,

        /// Content season; typically for video content (e.g., "Season 3").
        #[serde(skip_serializing_if = "Option::is_none")]
        pub season: Option<String>,

        /// Artist credited with the content.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub artist: Option<String>,

        /// Genre that best describes the content (e.g., rock, pop, etc).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub genre: Option<String>,

        /// Album to which the content belongs; typically for audio.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub album: Option<String>,

        /// International Standard Recording Code conforming to ISO-3901.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub isrc: Option<String>,

        /// Details about the content Producer (Section 3.2.10).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub producer: Option<Producer>,

        /// URL of the content, for buy-side contextualization or review.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,

        /// Array of IAB content categories that describe the content.
        /// See enum ContentCategory.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cat: Option<Vec<String>>,

        /// Production quality.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prodq: Option<ProductionQuality>,

        /// Type of content (game, video, text, etc.).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub context: Option<ContentContext>,

        /// Content rating (e.g., MPAA).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contentrating: Option<String>,

        /// User rating of the content (e.g., number of stars, likes, etc.).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub userrating: Option<String>,

        /// Media rating per QAG guidelines.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub qagmediarating: Option<QagMediaRating>,

        /// Comma separated list of keywords describing the content.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub keywords: Option<String>,

        /// false = not live, true = content is live (e.g., stream, live blog).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub livestream: Option<Bool>,

        /// false = indirect, true = direct.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sourcerelationship: Option<Bool>,

        /// Length of content in seconds; appropriate for video or audio.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub len: Option<i32>,

        /// Content language using ISO-639-1-alpha-2.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub language: Option<String>,

        /// Indicator of whether or not the content is embeddable (e.g., an
        /// embeddable video player).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub embeddable: Option<Bool>,

        /// Additional content data. Each object represents a different data source.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<Vec<Data>>,

        /// DEPRECATED in OpenRTB 2.4+. Prefer the field <code>prodq</code>.
        /// Video quality per IAB's classification.
        #[deprecated]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub videoquality: Option<ProductionQuality>,

        /// Extensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }

    /// OpenRTB 2.0: This object defines the producer of the content in which
    /// the ad will be shown. This is particularly useful when the content is
    /// syndicated and may be distributed through different publishers and thus
    /// when the producer and publisher are not necessarily the same entity.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
    pub struct Producer {
        /// Content producer or originator ID. Useful if content is syndicated,
        /// and may be posted on a site using embed tags.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Content producer or originator name (e.g., "Warner Bros").
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        /// Array of IAB content categories that describe the content producer.
        /// See enum ContentCategory.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cat: Option<Vec<String>>,

        /// Highest level domain of the content producer (e.g., "producer.com").
        #[serde(skip_serializing_if = "Option::is_none")]
        pub domain: Option<String>,

        /// Extensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }

    /// OpenRTB 2.0: This object provides information pertaining to the device
    /// through which the user is interacting. Device information includes its
    /// hardware, platform, location, and carrier data. The device can refer to a
    /// mobile handset, a desktop computer, set top box, or other digital device.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
    pub struct Device {
        /// Location of the device assumed to be the user's current location defined
        /// by a Geo object (Section 3.2.12).
        /// RECOMMENDED by the OpenRTB specification.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub geo: Option<Geo>,

        /// Standard "Do Not Track" flag as set in the header by the browser,
        /// where false = tracking is unrestricted, true = do not track.
        /// RECOMMENDED by the OpenRTB specification.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dnt: Option<Bool>,

        /// "Limit Ad Tracking" signal commercially endorsed (e.g., iOS, Android),
        /// where false = tracking is unrestricted, true = tracking must be limited
        /// per commercial guidelines.
        /// RECOMMENDED by the OpenRTB specification.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub lmt: Option<Bool>,

        /// Browser user agent string. Certain data may be redacted or replaced.
        /// RECOMMENDED by the OpenRTB specification.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ua: Option<String>,

        /// IPv4 address closest to device.
        /// RECOMMENDED by the OpenRTB specification.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ip: Option<String>,

        /// IPv6 address closest to device.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ipv6: Option<String>,

        /// The general type of device.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub devicetype: Option<DeviceType>,

        /// Device make (e.g., "Apple").
        #[serde(skip_serializing_if = "Option::is_none")]
        pub make: Option<String>,

        /// Device model (e.g., "iPhone").
        #[serde(skip_serializing_if = "Option::is_none")]
        pub model: Option<String>,

        /// Device operating system (e.g., "iOS").
        #[serde(skip_serializing_if = "Option::is_none")]
        pub os: Option<String>,

        /// Device operating system version (e.g., "3.1.2").
        #[serde(skip_serializing_if = "Option::is_none")]
        pub osv: Option<String>,

        /// Hardware version of the device (e.g., "5S" for iPhone 5S).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hwv: Option<String>,

        /// Physical width of the screen in pixels.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub w: Option<i32>,

        /// Physical height of the screen in pixels.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub h: Option<i32>,

        /// Screen size as pixels per linear inch.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ppi: Option<i32>,

        /// The ratio of physical pixels to device independent pixels.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pxratio: Option<f64>,

        /// Support for JavaScript.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub js: Option<Bool>,

        /// Indicates if the geolocation API will be available to JavaScript
        /// code running in the banner.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub geofetch: Option<Bool>,

        /// Version of Flash supported by the browser.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub flashver: Option<String>,

        /// Browser language using ISO-639-1-alpha-2.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub language: Option<String>,

        /// Carrier or ISP (e.g., "VERIZON") using exchange curated string
        /// names which should be published to bidders a priori.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub carrier: Option<String>,

        /// Mobile carrier as the concatenated MCC-MNC code (e.g.,
        /// "310-005" identifies Verizon Wireless CDMA in the USA).
        /// Refer to <https://en.wikipedia.org/wiki/Mobile_country_code>
        /// for further examples. Note that the dash between the MCC
        /// and MNC parts is required to remove parsing ambiguity.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mccmnc: Option<String>,

        /// Network connection type.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub connectiontype: Option<ConnectionType>,

        /// ID sanctioned for advertiser use in the clear (i.e., not hashed).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ifa: Option<String>,

        /// Hardware device ID (e.g., IMEI); hashed via SHA1.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub didsha1: Option<String>,

        /// Hardware device ID (e.g., IMEI); hashed via MD5.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub didmd5: Option<String>,

        /// Platform device ID (e.g., Android ID); hashed via SHA1.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dpidsha1: Option<String>,

        /// Platform device ID (e.g., Android ID); hashed via MD5.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dpidmd5: Option<String>,

        /// MAC address of the device; hashed via SHA1.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub macsha1: Option<String>,

        /// MAC address of the device; hashed via MD5.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub macmd5: Option<String>,

        /// Extensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }

    /// OpenRTB 2.0: This object encapsulates various methods for specifying a
    /// geographic location. When subordinate to a Device object, it indicates the
    /// location of the device which can also be interpreted as the user's current
    /// location. When subordinate to a User object, it indicates the location of
    /// the user's home base (i.e., not necessarily their current location).
    ///
    /// The lat/lon attributes should only be passed if they conform to the
    /// accuracy depicted in the type attribute. For example, the centroid of a
    /// geographic region such as postal code should not be passed.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
    pub struct Geo {
        /// Latitude from -90.0 to +90.0, where negative is south.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub lat: Option<f64>,

        /// Longitude from -180.0 to +180.0, where negative is west.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub lon: Option<f64>,

        /// Country using ISO-3166-1 Alpha-3.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub country: Option<String>,

        /// Region code using ISO-3166-2; 2-letter state code if USA.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,

        /// Region of a country using FIPS 10-4 notation. While OpenRTB supports
        /// this attribute, it has been withdrawn by NIST in 2008.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub regionfips104: Option<String>,

        /// Google metro code; similar to but not exactly Nielsen DMAs.
        /// See Appendix A for a link to the codes.
        /// (<http://code.google.com/apis/adwords/docs/appendix/metrocodes.html>).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub metro: Option<String>,

        /// City using United Nations Code for Trade & Transport Locations.
        /// See Appendix A for a link to the codes.
        /// (<http://www.unece.org/cefact/locode/service/location.htm>).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,

        /// Zip/postal code.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zip: Option<String>,

        /// Source of location data; recommended when passing lat/lon.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<LocationType>,

        /// Estimated location accuracy in meters; recommended when lat/lon
        /// are specified and derived from a device's location services
        /// (i.e., type = 1). Note that this is the accuracy as reported
        /// from the device. Consult OS specific documentation
        /// (e.g., Android, iOS) for exact interpretation.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub accuracy: Option<i32>,

        /// Number of seconds since this geolocation fix was established.
        /// Note that devices may cache location data across multiple fetches.
        /// Ideally, this value should be from the time the actual fix was taken.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub lastfix: Option<i32>,

        /// Service or provider used to determine geolocation from IP
        /// address if applicable (i.e., type = 2).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ipservice: Option<LocationService>,

        /// Local time as the number +/- of minutes from UTC.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub utcoffset: Option<i32>,

        /// Extensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }

    /// OpenRTB 2.0: This object contains information known or derived about
    /// the human user of the device (i.e., the audience for advertising).
    /// The user id is an exchange artifact and may be subject to rotation or other
    /// privacy policies. However, this user ID must be stable long enough to serve
    /// reasonably as the basis for frequency capping and retargeting.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
    pub struct User {
        /// Exchange-specific ID for the user. At least one of id or buyeruid
        /// is recommended.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Buyer-specific ID for the user as mapped by the exchange for the buyer.
        /// At least one of buyeruid or id is recommended.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub buyeruid: Option<String>,

        /// Year of birth as a 4-digit integer.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub yob: Option<i32>,

        /// Gender as "M" male, "F" female, "O" Other. (Null indicates unknown)
        #[serde(skip_serializing_if = "Option::is_none")]
        pub gender: Option<String>,

        /// Comma separated list of keywords, interests, or intent.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub keywords: Option<String>,

        /// Optional feature to pass bidder data set in the exchange's cookie.
        /// The string must be in base85 cookie safe characters and be in any format.
        /// Proper JSON encoding must be used to include "escaped" quotation marks.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub customdata: Option<String>,

        /// Location of the user's home base defined by a Geo object
        /// (Section 3.2.12). This is not necessarily their current location.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub geo: Option<Geo>,

        /// Additional user data. Each Data object (Section 3.2.14) represents a
        /// different data source.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<Vec<Data>>,

        /// Extensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }

    /// OpenRTB 2.0: The data and segment objects together allow additional data
    /// about the user to be specified. This data may be from multiple sources
    /// whether from the exchange itself or third party providers as specified by
    /// the id field. A bid request can mix data objects from multiple providers or
    /// can have multiple data objects.
    /// The specific data providers in use should be published by the exchange
    /// a priori to its bidders.
    /// This is used to send detected verticals to the buyer.
    /// For exchange bidding, this is also used to send key
    /// value pairs from the publisher to the buyer.
    /// <https://support.google.com/admanager/answer/177381>
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
    pub struct Data {
        /// Exchange-specific ID for the data provider.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Exchange-specific name for the data provider.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        /// Array of Segment (Section 3.2.15) objects that contain the actual
        /// data values.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub segment: Option<Vec<data::Segment>>,

        /// Extensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }

    /// Nested message and enum types in `Data`.
    pub mod data {
        use serde::{Deserialize, Serialize};
        use serde_json::Value;

        /// OpenRTB 2.0: Segment objects are essentially key-value pairs that
        /// convey specific units of data about the user. The parent Data object
        /// is a collection of such values from a given data provider.
        /// The specific segment names and value options must be published by the
        /// exchange a priori to its bidders.
        #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct Segment {
            /// ID of the data segment specific to the data provider.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub id: Option<String>,

            /// Name of the data segment specific to the data provider.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: Option<String>,

            /// String representation of the data segment value.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub value: Option<String>,

            /// Extensions.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ext: Option<Value>,
        }
    }
    /// OpenRTB 2.2: This object contains any legal, governmental, or industry
    /// regulations that apply to the request. The coppa flag signals whether
    /// or not the request falls under the United States Federal Trade Commission's
    /// regulations for the United States Children's Online Privacy Protection Act
    /// ("COPPA"). Refer to Section 7.1 for more information.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
    pub struct Regs {
        /// Flag indicating if this request is subject to the COPPA regulations
        /// established by the USA FTC.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coppa: Option<Bool>,

        /// Extensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }
    // #[derive(Serialize, Deserialize)]
    // #[derive(Clone, PartialEq, ::prost::Oneof)]
    // pub enum DistributionchannelOneof {
    //     /// Details via a Site object (Section 3.2.6) about the publisher's website.
    //     /// Only applicable and recommended for websites.
    //     #[prost(message, tag="3")]
    //     #[serde(skip_serializing_if = "Option::is_none")]
    //     Site(Site),
    //     /// Details via an App object (Section 3.2.7) about the publisher's app
    //     /// (non-browser applications). Only applicable and recommended for apps.
    //     #[prost(message, tag="4")]
    //     #[serde(skip_serializing_if = "Option::is_none")]
    //     App(App),
    // }
}

/// OpenRTB 2.0: This object is the top-level bid response object (i.e., the
/// unnamed outer JSON object). The id attribute is a reflection of the bid
/// request ID for logging purposes. Similarly, bidid is an optional response
/// tracking ID for bidders. If specified, it can be included in the subsequent
/// win notice call if the bidder wins. At least one seatbid object is required,
/// which contains at least one bid for an impression. Other attributes are
/// optional. To express a "no-bid", the options are to return an empty response
/// with HTTP 204. Alternately if the bidder wishes to convey to the exchange a
/// reason for not bidding, just a BidResponse object is returned with a
/// reason code in the nbr attribute.
#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BidResponse {
    /// ID of the bid request to which this is a response.
    /// REQUIRED by the OpenRTB specification.
    pub id: String,
    /// Array of seatbid objects; 1+ required if a bid is to be made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seatbid: Option<Vec<bid_response::SeatBid>>,

    /// Bidder generated response ID to assist with logging/tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidid: Option<String>,

    /// Bid currency using ISO-4217 alpha codes. This field will be required
    /// starting in Q4 2022. If this field is populated and differs from the
    /// bidding currency, the bid will be filtered. If this field is not populated,
    /// the currency will be assumed to be the bidding currency. The bidding
    /// currency is determined by:
    /// 1. The bidder-level currency, if configured in RTB account settings.
    /// 2. Otherwise, the currency of the buyer account indicated by the
    /// billing ID in the cid field of the bid response.
    /// 3. Otherwise, if cid is not populated in the bid response, the
    /// currency of the buyer account indicated by the sole billing ID in the
    /// bid request.
    /// The currency of the buyer account is set on account creation and can be
    /// checked by contacting a Technical Account Manager.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cur: Option<String>,

    /// Optional feature to allow a bidder to set data in the exchange's cookie.
    /// The string must be in base85 cookie safe characters and be in any format.
    /// Proper JSON encoding must be used to include "escaped" quotation marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customdata: Option<String>,

    /// Reason for not bidding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbr: Option<NoBidReason>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Nested message and enum types in `BidResponse`.
pub mod bid_response {
    use super::bool::Bool;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    /// OpenRTB 2.0: A bid response can contain multiple SeatBid objects, each on
    /// behalf of a different bidder seat and each containing one or more
    /// individual bids. If multiple impressions are presented in the request, the
    /// group attribute can be used to specify if a seat is willing to accept any
    /// impressions that it can win (default) or if it is only interested in
    /// winning any if it can win them all as a group.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
    pub struct SeatBid {
        /// Array of 1+ Bid objects (Section 4.2.3) each related to an impression.
        /// Multiple bids can relate to the same impression.
        pub bid: Vec<seat_bid::Bid>,

        /// ID of the buyer seat (e.g., advertiser, agency) on whose behalf
        /// this bid is made.
        ///
        /// This ID will be used to breakdown spend and invalid traffic metrics in
        /// IVT transparency reporting, given that it is no longer than 64 bytes.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub seat: Option<String>,

        /// false = impressions can be won individually;
        /// true = impressions must be won or lost as a group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub group: Option<Bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }

    /// Nested message and enum types in `SeatBid`.
    pub mod seat_bid {
        use super::super::{
            ApiFramework, CreativeAttribute, NativeResponse, Protocol, QagMediaRating,
        };
        use serde::{Deserialize, Serialize};
        use serde_json::Value;

        /// OpenRTB 2.0: A SeatBid object contains one or more Bid objects,
        /// each of which relates to a specific impression in the bid request
        /// via the impid attribute and constitutes an offer to buy that impression
        /// for a given price.
        #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct Bid {
            /// Bidder generated bid ID to assist with logging/tracking.
            /// REQUIRED by the OpenRTB specification.
            pub id: String,
            /// ID of the Imp object in the related bid request.
            /// REQUIRED by the OpenRTB specification.
            pub impid: String,
            /// Bid price expressed as CPM although the actual transaction is for a
            /// unit impression only. Note that while the type indicates float, integer
            /// math is highly recommended when handling currencies
            /// (e.g., BigDecimal in Java).
            /// REQUIRED by the OpenRTB specification.
            pub price: f64,
            /// Win notice URL called by the exchange if the bid wins; optional means
            /// of serving ad markup.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub nurl: Option<String>,

            /// Billing notice URL called by the exchange when a winning bid
            /// becomes billable based on exchange-specific business policy
            /// (e.g., typically delivered, viewed, etc.). Substitution macros
            /// (Section 4.4) may be included.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub burl: Option<String>,

            /// Loss notice URL called by the exchange when a bid is known to
            /// have been lost. Substitution macros (Section 4.4) may be
            /// included. Exchange-specific policy may preclude support for
            /// loss notices or the disclosure of winning clearing prices
            /// resulting in ${AUCTION_PRICE} macros being removed (i.e.,
            /// replaced with a zero-length string).
            #[serde(skip_serializing_if = "Option::is_none")]
            pub lurl: Option<String>,

            /// ID of a preloaded ad to serve if the bid wins.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub adid: Option<String>,

            /// Advertiser domain for block list checking (e.g., "ford.com"). This can
            /// be an array of for the case of rotating creatives. Exchanges can
            /// mandate that only one domain is allowed.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub adomain: Option<Vec<String>>,

            /// A platform-specific application identifier intended to be
            /// unique to the app and independent of the exchange. On Android,
            /// this should be a bundle or package name (e.g., com.foo.mygame).
            /// On iOS, it is a numeric ID.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub bundle: Option<String>,

            /// URL without cache-busting to an image that is representative of the
            /// content of the campaign for ad quality/safety checking.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub iurl: Option<String>,

            /// Campaign ID to assist with ad quality checking; the collection of
            /// creatives for which iurl should be representative.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cid: Option<String>,

            /// Creative ID to assist with ad quality checking.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub crid: Option<String>,

            /// Tactic ID to enable buyers to label bids for reporting to the
            /// exchange the tactic through which their bid was submitted.
            /// The specific usage and meaning of the tactic ID should be
            /// communicated between buyer and exchanges a priori.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub tactic: Option<String>,

            /// IAB content categories of the creative.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cat: Option<Vec<String>>,

            /// Set of attributes describing the creative.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub attr: Option<Vec<CreativeAttribute>>,

            /// API required by the markup if applicable.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub api: Option<ApiFramework>,

            /// Video response protocol of the markup if applicable.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub protocol: Option<Protocol>,

            /// Creative media rating per QAG guidelines.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub qagmediarating: Option<QagMediaRating>,

            /// Language of the creative using ISO-639-1-alpha-2. The nonstandard
            /// code "xx" may also be used if the creative has no
            /// linguistic content (e.g., a banner with just a company logo).
            #[serde(skip_serializing_if = "Option::is_none")]
            pub language: Option<String>,

            /// Reference to the deal.id from the bid request if this bid pertains to a
            /// private marketplace direct deal.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub dealid: Option<String>,

            /// Width of the creative in device independent pixels (DIPS).
            #[serde(skip_serializing_if = "Option::is_none")]
            pub w: Option<i32>,

            /// Height of the creative in device independent pixels (DIPS).
            #[serde(skip_serializing_if = "Option::is_none")]
            pub h: Option<i32>,

            /// Relative width of the creative when expressing size as a ratio.
            /// Required for Flex Ads.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub wratio: Option<i32>,

            /// Relative height of the creative when expressing size as a ratio.
            /// Required for Flex Ads.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub hratio: Option<i32>,

            /// Advisory as to the number of seconds the bidder is willing to
            /// wait between the auction and the actual impression.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub exp: Option<i32>,

            /// Optional means of conveying ad markup in case the bid wins;
            /// supersedes the win notice if markup is included in both.
            /// For native ad bids, exactly one of {adm, adm_native} should be used;
            /// this is the OpenRTB-compliant field for JSON serialization.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub adm: Option<String>,

            /// Native ad response.
            /// For native ad bids, exactly one of {adm, adm_native} should be used;
            /// this is the field used for Protobuf serialization.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub adm_native: Option<NativeResponse>,

            /// Extensions.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ext: Option<Value>,
        }
        // /// Nested message and enum types in `Bid`.
        // pub mod bid {
        //     #[derive(Clone, PartialEq, ::prost::Oneof)]
        //     pub enum AdmOneof {
        //         /// Optional means of conveying ad markup in case the bid wins;
        //         /// supersedes the win notice if markup is included in both.
        //         /// For native ad bids, exactly one of {adm, adm_native} should be used;
        //         /// this is the OpenRTB-compliant field for JSON serialization.
        //         #[prost(string, tag = "6")]
        //         #[serde(skip_serializing_if = "Option::is_none")]
        //         Adm(String),
        //         /// Native ad response.
        //         /// For native ad bids, exactly one of {adm, adm_native} should be used;
        //         /// this is the field used for Protobuf serialization.
        //         #[prost(message, tag = "50")]
        //         #[serde(skip_serializing_if = "Option::is_none")]
        //         AdmNative(super::super::super::NativeResponse),
        //     }
        // }
    }
}

/// OpenRTB Native 1.0: The Native Object defines the native advertising
/// opportunity available for bid via this bid request. It must be included
/// directly in the impression object if the impression offered for auction
/// is a native ad format.
#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct NativeRequest {
    /// Version of the Native Markup version in use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,

    /// The context in which the ad appears.
    /// RECOMMENDED by the OpenRTB Native specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ContextType>,

    /// A more detailed context in which the ad appears.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contextsubtype: Option<ContextSubtype>,

    /// The design/format/layout of the ad unit being offered.
    /// RECOMMENDED by the OpenRTB Native specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plcmttype: Option<PlacementType>,

    /// The number of identical placements in this Layout.
    // #[p(int32, optional, tag = "4", default = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plcmtcnt: Option<i32>,

    /// 0 for the first ad, 1 for the second ad, and so on. Note this would
    /// generally NOT be used in combination with plcmtcnt - either you are
    /// auctioning multiple identical placements (in which case
    /// plcmtcnt>1, seq=0) or you are holding separate auctions for distinct
    /// items in the feed (in which case plcmtcnt=1, seq>=1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<i32>,

    /// Any bid must comply with the array of elements expressed by the Exchange.
    /// REQUIRED by the OpenRTB Native specification: at least 1 element.
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Vec<native_request::Asset>,

    /// Whether the supply source / impression supports returning an assetsurl
    /// instead of an asset object. false or the absence of the field indicates no
    /// such support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aurlsupport: Option<Bool>,

    /// Whether the supply source / impression supports returning a DCO URL
    /// instead of an asset object. false or the absence of the field indicates no
    /// such support. Beta feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durlsupport: Option<Bool>,

    /// Specifies what type of event tracking is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventtrackers: Option<Vec<native_request::EventTrackers>>,

    /// Set to true when the native ad supports buyer-specific privacy notice.
    /// Set to false (or field absent) when the native ad doesn't support custom
    /// privacy links or if support is unknown.
    /// RECOMMENDED by the OpenRTB Native specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Bool>,

    /// DEPRECATED in OpenRTB Native 1.1, REMOVED in 1.2+.
    /// Use field <code>plcmttype</code>.
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<LayoutId>,

    /// DEPRECATED in OpenRTB Native 1.1, REMOVED in 1.2+.
    /// Use fields <code>context</code> and <code>contextsubtype</code>.
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adunit: Option<AdUnitId>,

    /// Extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Nested message and enum types in `NativeRequest`.
pub mod native_request {
    use super::super::native_request::asset::{Data, Image, Title};
    use super::super::{bid_request::imp::Video, EventTrackingMethod, EventType};
    use super::bool::Bool;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    /// OpenRTB Native 1.0: The main container object for each asset requested or
    /// supported by Exchange on behalf of the rendering client.
    /// Any object that is required is to be flagged as such. Only one of the
    /// {title,img,video,data} objects should be present in each object.
    /// All others should be null/absent. The id is to be unique within the
    /// Asset array so that the response can be aligned.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
    pub struct Asset {
        /// Unique asset ID, assigned by exchange. Typically a counter for the array.
        /// REQUIRED by the OpenRTB Native specification.
        pub id: i32,

        /// Set to true if asset is required
        /// (exchange will not accept a bid without it).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub required: Option<Bool>,

        /// Title object for title assets.
        pub title: Option<Title>,

        /// Image object for image assets.
        pub img: Option<Image>,

        /// Video object for video assets.
        /// Note that in-stream video ads are not part of Native.
        /// Native ads may contain a video as the ad creative itself.
        pub video: Option<Video>,

        /// Data object for brand name, description, ratings, prices etc.
        pub data: Option<Data>,

        /// Extensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }

    /// Nested message and enum types in `Asset`.
    pub mod asset {
        use super::super::{DataAssetType, ImageAssetType};
        use serde::{Deserialize, Serialize};
        use serde_json::Value;

        /// OpenRTB Native 1.0: The Title object is to be used for title element
        /// of the Native ad.
        #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct Title {
            /// Maximum length of the text in the title element.
            /// RECOMMENDED that the value be either of: 25, 90, 140.
            /// REQUIRED by the OpenRTB Native specification.
            pub len: i32,

            /// Extensions.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ext: Option<Value>,
        }
        /// OpenRTB Native 1.0: The Image object to be used for all image elements
        /// of the Native ad such as Icons, Main Image, etc.
        /// RECOMMENDED sizes and aspect ratios are included in ImageAssetType.
        #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct Image {
            /// Type ID of the image element supported by the publisher.
            /// The publisher can display this information in an appropriate format.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<ImageAssetType>,

            /// Width of the image in pixels.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub w: Option<i32>,

            /// Height of the image in pixels.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub h: Option<i32>,

            /// The minimum requested width of the image in pixels. This option should
            /// be used for any rescaling of images by the client. Either w or wmin
            /// should be transmitted. If only w is included, it should be considered
            /// an exact requirement.
            /// RECOMMENDED by the OpenRTB Native specification.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub wmin: Option<i32>,

            /// The minimum requested height of the image in pixels. This option should
            /// be used for any rescaling of images by the client. Either h or hmin
            /// should be transmitted. If only h is included, it should be considered
            /// an exact requirement.
            /// RECOMMENDED by the OpenRTB Native specification.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub hmin: Option<i32>,

            /// Allowlist of content MIME types supported. Popular MIME types include,
            /// but are not limited to "image/jpg" and "image/gif". Each implementing
            /// Exchange should have their own list of supported types in the
            /// integration docs. See Wikipedia's MIME page for more information and
            /// links to all IETF RFCs. If blank, assume all types are allowed.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mimes: Option<Vec<String>>,

            /// Extensions.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ext: Option<Value>,
        }

        /// OpenRTB Native 1.0: The Data Object is to be used for all non-core
        /// elements of the native unit such as Ratings, Review Count, Stars,
        /// Download count, descriptions etc. It is also generic for future of Native
        /// elements not contemplated at the time of the writing of this document.
        #[derive(Clone, PartialEq, Serialize, Deserialize)]
        pub struct Data {
            /// Type ID of the element supported by the publisher. The publisher can
            /// display this information in an appropriate format.
            /// REQUIRED by the OpenRTB Native specification.
            pub r#type: DataAssetType,

            /// Maximum length of the text in the element's response. Longer strings
            /// may be truncated and ellipsized by Ad Exchange or the publisher during
            /// rendering.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub len: Option<i32>,

            /// Extensions.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ext: Option<Value>,
        }
        // /// RECOMMENDED by the OpenRTB Native specification.
        // #[derive(Clone, PartialEq)]
        // // #[derive(Clone, PartialEq, ::prost::Oneof)]
        // pub enum AssetOneof {
        //     /// Title object for title assets.
        //     Title(Title),
        //     /// Image object for image assets.
        //     Img(Image),
        //     /// Video object for video assets.
        //     /// Note that in-stream video ads are not part of Native.
        //     /// Native ads may contain a video as the ad creative itself.
        //     Video(super::super::bid_request::imp::Video),
        //     /// Data object for brand name, description, ratings, prices etc.
        //     Data(Data),
        // }
    }

    /// OpenRTB Native 1.2: The EventTrackers object specifies the type of events
    /// the bidder can request to be tracked in the bid response, and which types
    /// of tracking are available for each event type, and is included as an array
    /// in the request.
    #[derive(Clone, PartialEq, Serialize, Deserialize)]
    pub struct EventTrackers {
        /// Type of event available for tracking.
        /// REQUIRED by the OpenRTB Native specification.
        pub event: EventType,

        /// Array of types of tracking available for the given event.
        /// REQUIRED by the OpenRTB Native specification.
        pub methods: Vec<EventTrackingMethod>,
    }
}
/// OpenRTB Native 1.0: The native response object is the top level JSON object
/// which identifies an native response.
#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct NativeResponse {
    /// Version of the Native Markup version in use.
    /// RECOMMENDED by the OpenRTB Native specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,

    /// List of native ad's assets.
    /// RECOMMENDED in 1.0, 1.1, or in 1.2 as a fallback if assetsurl is provided.
    /// REQUIRED in 1.2, if not assetsurl is provided.
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Vec<native_response::Asset>,

    /// URL of alternate source for the assets object. The expected response is a
    /// JSON object mirroring the asset object in the bid response, subject to
    /// certain requirements as specified in the individual objects.
    /// Where present, overrides the assets object in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assetsurl: Option<String>,

    /// URL where a dynamic creative specification may be found for populating this
    /// ad, per the Dynamic Content Ads Specification.
    /// Note this is a beta option as the interpretation of the Dynamic Content Ads
    /// Specification and how to assign those elementes into a native ad is outside
    /// the scope of this spec and must be agreed offline between parties or as may
    /// be specified in a future revision of the Dynamic Content Ads spec.
    /// Where present, overrides the assets object in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcourl: Option<String>,

    /// Destination Link. This is default link object for the ad.
    /// Individual assets can also have a link object which applies if the asset is
    /// activated (clicked). If the asset doesn't have a link object, the parent
    /// link object applies.
    /// See ResponseLink definition.
    /// REQUIRED by the OpenRTB Native specification.
    pub link: native_response::Link,
    /// DEPRECATED in OpenRTB Native 1.2+. Prefer object <code>EventTracker</code>.
    /// Array of impression tracking URLs, expected to return a 1x1 image or
    /// 204 response - typically only passed when using 3rd party trackers.
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imptrackers: Option<Vec<String>>,

    /// DEPRECATED in OpenRTB Native 1.2+. Prefer object <code>EventTracker</code>.
    /// Optional javascript impression tracker. Contains <script> tags to be
    /// executed at impression time where it can be supported.
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jstracker: Option<String>,

    /// Array of response event trackers to run with the ad, in response to the
    /// declared supported methods in the NativeRequest. Replaces imptrackers and
    /// jstrackers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventtrackers: Option<Vec<native_response::EventTracker>>,

    /// If support was indicated in the request, URL of a page informing the user
    /// about the buyer's targeting activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,

    /// Extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Nested message and enum types in `NativeResponse`.
pub mod native_response {
    use super::super::native_response::asset::{Data, Image, Title, Video};
    use super::super::{EventTrackingMethod, EventType};
    use super::bool::Bool;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    /// OpenRTB Native 1.0: Used for "call to action" assets, or other links from
    /// the Native ad. This Object should be associated to its peer object in the
    /// parent Asset Object or as the primary link in the top level NativeResponse
    /// object. When that peer object is activated (clicked) the action should take
    /// the user to the location of the link.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
    pub struct Link {
        /// Landing URL of the clickable link.
        /// REQUIRED by the OpenRTB Native specification.
        pub url: String,
        /// List of third-party tracker URLs to be fired on click of the URL.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub clicktrackers: Option<Vec<String>>,

        /// Fallback URL for deeplink. To be used if the URL given in url is not
        /// supported by the device.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fallback: Option<String>,

        /// Extensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }
    /// OpenRTB Native 1.0: Corresponds to the Asset Object in the request.
    /// The main container object for each asset requested or supported by Exchange
    /// on behalf of the rendering client. Any object that is required is to be
    /// flagged as such. Only one of the {title,img,video,data} objects should be
    /// present in each object. All others should be null/absent. The id is to be
    /// unique within the Asset array so that the response can be aligned.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
    pub struct Asset {
        /// Unique asset ID, assigned by exchange, must match one of the asset IDs
        /// in request.
        /// REQUIRED in 1.0, or in 1.2 if embedded asset is being used.
        pub id: i32,

        /// Set to true if asset is required. (bidder requires it to be displayed).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub required: Option<Bool>,

        /// Link object for call to actions.
        /// This link object applies if the asset item is activated (clicked).
        /// If there is no link object on the asset, the parent link object on the
        /// bid response apply.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub link: Option<Link>,

        /// Title object for title assets.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<Title>,

        /// Image object for image assets.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub img: Option<Image>,

        /// Video object for video assets.
        /// Note that in-stream video ads are not part of Native.
        /// Native ads may contain a video as the ad creative itself.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub video: Option<Video>,

        /// Data object for ratings, prices etc.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<Data>,

        /// Extensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }

    /// Nested message and enum types in `Asset`.
    pub mod asset {
        use super::super::{DataAssetType, ImageAssetType};
        use serde::{Deserialize, Serialize};
        use serde_json::Value;

        /// OpenRTB Native 1.0: Corresponds to the Title Object in the request,
        /// with the value filled in.
        /// If using assetsurl or dcourl response rather than embedded asset
        /// response, it is recommended that three title objects be provided, the
        /// length of each is less than or equal to the three recommended maximum
        /// title lengths (25,90,140).
        #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct Title {
            /// The text associated with the text element.
            /// REQUIRED by the OpenRTB Native specification.
            pub text: String,

            /// The length of the title being provided.
            /// REQUIRED if using assetsurl/dcourl representation.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub len: Option<i32>,

            /// Extensions.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ext: Option<Value>,
        }

        /// OpenRTB Native 1.0: Corresponds to the Image Object in the request.
        /// The Image object to be used for all image elements of the Native ad
        /// such as Icons, Main Image, etc.
        /// It is recommended that if assetsurl/dcourl is being used rather than
        /// embbedded assets, that an image of each recommended aspect ratio
        /// (per ImageType enum) be provided for image type 3 (MAIN_IMAGE).
        #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct Image {
            /// The type of image element being submitted from the ImageType enum.
            /// REQUIRED for assetsurl or dcourl responses,
            /// not required to embedded asset responses.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<ImageAssetType>,

            /// URL of the image asset.
            /// REQUIRED by the OpenRTB Native specification.
            pub url: String,

            /// Width of the image in pixels.
            /// RECOMMENDED in 1.0, 1.1, or in 1.2 for embedded asset responses.
            /// REQUIRED in 1.2 for assetsurl or dcourl if multiple assets
            /// of the same type submitted.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub w: Option<i32>,

            /// Height of the image in pixels.
            /// RECOMMENDED in 1.0, 1.1, or in 1.2 for embedded asset responses.
            /// REQUIRED in 1.2 for assetsurl or dcourl if multiple assets
            /// of the same type submitted.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub h: Option<i32>,

            /// Extensions.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ext: Option<Value>,
        }

        /// OpenRTB Native 1.0: Corresponds to the Data Object in the request, with
        /// the value filled in. The Data Object is to be used for all miscellaneous
        /// elements of the native unit such as Brand Name, Ratings, Review Count,
        /// Stars, Downloads, etc. It is also generic for future of native elements
        /// not contemplated at the time of the writing of this document.
        #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct Data {
            /// The type of data element being submitted from the DataAssetTypes enum.
            /// REQUIRED in 1.2 for assetsurl or dcourl responses.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<DataAssetType>,

            /// The length of the data element being submitted. Where applicable, must
            /// comply with the recommended maximum lengths in the DataAssetType enum.
            /// REQUIRED in 1.2 for assetsurl or dcourl responses.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub len: Option<i32>,

            /// DEPRECATED in OpenRTB Native 1.2+. No replacement.
            /// The optional formatted string name of the data type to be displayed.
            #[deprecated]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub label: Option<String>,

            /// The formatted string of data to be displayed. Can contain a formatted
            /// value such as "5 stars" or "$10" or "3.4 stars out of 5".
            /// REQUIRED by the OpenRTB Native specification.
            pub value: String,

            /// Extensions.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ext: Option<Value>,
        }

        /// OpenRTB Native 1.0: Corresponds to the Video Object in the request,
        /// yet containing a value of a conforming VAST tag as a value.
        #[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct Video {
            /// VAST xml.
            /// REQUIRED by the OpenRTB Native specification.
            pub vasttag: String,

            /// Extensions.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ext: Option<Value>,
        }
        // /// RECOMMENDED by the OpenRTB Native specification.
        // // #[derive(Clone, PartialEq, ::prost::Oneof)]
        // #[derive(Clone, PartialEq)]
        // pub enum AssetOneof {
        //     /// Title object for title assets.
        //     Title(Title),
        //     /// Image object for image assets.
        //     Img(Image),
        //     /// Video object for video assets.
        //     /// Note that in-stream video ads are not part of Native.
        //     /// Native ads may contain a video as the ad creative itself.
        //     Video(Video),
        //     /// Data object for ratings, prices etc.
        //     Data(Data),
        // }
    }

    /// OpenRTB Native 1.2: The event trackers response is an array of objects and
    /// specifies the types of events the bidder wishes to track and the
    /// URLs/information to track them. Bidder must only respond with methods
    /// indicated as available in the request. Note that most javascript trackers
    /// expect to be loaded at impression time, so it's not generally recommended
    /// for the buyer to respond with javascript trackers on other events, but the
    /// appropriateness of this is up to each buyer.
    #[derive(Clone, PartialEq, Serialize, Deserialize)]
    pub struct EventTracker {
        /// Type of event to track.
        /// REQUIRED if embedded asset is being used.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub event: Option<EventType>,

        /// Type of tracking requested.
        /// REQUIRED if embedded asset is being used.
        pub method: EventTrackingMethod,

        /// The URL of the image or js.
        /// REQUIRED for image or js, optional for custom.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,

        /// Extensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ext: Option<Value>,
    }
}

// ***** OpenRTB Core enums ****************************************************

/// OpenRTB 2.0: The following list represents the IAB's contextual taxonomy for
/// categorization. Standard IDs have been adopted to easily support the
/// communication of primary and secondary categories for various objects.
///
/// This OpenRTB table has values derived from the IAB Quality Assurance
/// Guidelines (QAG). Practitioners should keep in sync with updates to the
/// QAG values as published on IAB.net.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum ContentCategory {
    /// This value is not part of the specification.
    Undefined = 0,
    /// Arts & Entertainment
    Iab1 = 1,
    /// Books & Literature
    Iab11 = 2,
    /// Celebrity Fan/Gossip
    Iab12 = 3,
    /// Fine Art
    Iab13 = 4,
    /// Humor
    Iab14 = 5,
    /// Movies
    Iab15 = 6,
    /// Music
    Iab16 = 7,
    /// Television
    Iab17 = 8,
    /// Automotive
    Iab2 = 9,
    /// Auto Parts
    Iab21 = 10,
    /// Auto Repair
    Iab22 = 11,
    /// Buying/Selling Cars
    Iab23 = 12,
    /// Car Culture
    Iab24 = 13,
    /// Certified Pre-Owned
    Iab25 = 14,
    /// Convertible
    Iab26 = 15,
    /// Coupe
    Iab27 = 16,
    /// Crossover
    Iab28 = 17,
    /// Diesel
    Iab29 = 18,
    /// Electric Vehicle
    Iab210 = 19,
    /// Hatchback
    Iab211 = 20,
    /// Hybrid
    Iab212 = 21,
    /// Luxury
    Iab213 = 22,
    /// MiniVan
    Iab214 = 23,
    /// Motorcycles
    Iab215 = 24,
    /// Off-Road Vehicles
    Iab216 = 25,
    /// Performance Vehicles
    Iab217 = 26,
    /// Pickup
    Iab218 = 27,
    /// Road-Side Assistance
    Iab219 = 28,
    /// Sedan
    Iab220 = 29,
    /// Trucks & Accessories
    Iab221 = 30,
    /// Vintage Cars
    Iab222 = 31,
    /// Wagon
    Iab223 = 32,
    /// Business
    Iab3 = 33,
    /// Advertising
    Iab31 = 34,
    /// Agriculture
    Iab32 = 35,
    /// Biotech/Biomedical
    Iab33 = 36,
    /// Business Software
    Iab34 = 37,
    /// Construction
    Iab35 = 38,
    /// Forestry
    Iab36 = 39,
    /// Government
    Iab37 = 40,
    /// Green Solutions
    Iab38 = 41,
    /// Human Resources
    Iab39 = 42,
    /// Logistics
    Iab310 = 43,
    /// Marketing
    Iab311 = 44,
    /// Metals
    Iab312 = 45,
    /// Careers
    Iab4 = 46,
    /// Career Planning
    Iab41 = 47,
    /// College
    Iab42 = 48,
    /// Financial  Aid
    Iab43 = 49,
    /// Job Fairs
    Iab44 = 50,
    /// Job Search
    Iab45 = 51,
    /// Resume Writing/Advice
    Iab46 = 52,
    /// Nursing
    Iab47 = 53,
    /// Scholarships
    Iab48 = 54,
    /// Telecommuting
    Iab49 = 55,
    /// U.S. Military
    Iab410 = 56,
    /// Career Advice
    Iab411 = 57,
    /// Education
    Iab5 = 58,
    /// 7-12 Education
    Iab51 = 59,
    /// Adult Education
    Iab52 = 60,
    /// Art History
    Iab53 = 61,
    /// College Administration
    Iab54 = 62,
    /// College Life
    Iab55 = 63,
    /// Distance Learning
    Iab56 = 64,
    /// English as a 2nd Language
    Iab57 = 65,
    /// Language Learning
    Iab58 = 66,
    /// Graduate School
    Iab59 = 67,
    /// Homeschooling
    Iab510 = 68,
    /// Homework/Study Tips
    Iab511 = 69,
    /// K-6 Educators
    Iab512 = 70,
    /// Private School
    Iab513 = 71,
    /// Special Education
    Iab514 = 72,
    /// Studying Business
    Iab515 = 73,
    /// Family & Parenting
    Iab6 = 74,
    /// Adoption
    Iab61 = 75,
    /// Babies & Toddlers
    Iab62 = 76,
    /// Daycare/Pre School
    Iab63 = 77,
    /// Family Internet
    Iab64 = 78,
    /// Parenting - K-6 Kids
    Iab65 = 79,
    /// Parenting teens
    Iab66 = 80,
    /// Pregnancy
    Iab67 = 81,
    /// Special Needs Kids
    Iab68 = 82,
    /// Eldercare
    Iab69 = 83,
    /// Health & Fitness
    Iab7 = 84,
    /// Exercise
    Iab71 = 85,
    /// A.D.D.
    Iab72 = 86,
    /// AIDS/HIV
    Iab73 = 87,
    /// Allergies
    Iab74 = 88,
    /// Alternative Medicine
    Iab75 = 89,
    /// Arthritis
    Iab76 = 90,
    /// Asthma
    Iab77 = 91,
    /// Autism/PDD
    Iab78 = 92,
    /// Bipolar Disorder
    Iab79 = 93,
    /// Brain Tumor
    Iab710 = 94,
    /// Cancer
    Iab711 = 95,
    /// Cholesterol
    Iab712 = 96,
    /// Chronic Fatigue Syndrome
    Iab713 = 97,
    /// Chronic Pain
    Iab714 = 98,
    /// Cold & Flu
    Iab715 = 99,
    /// Deafness
    Iab716 = 100,
    /// Dental Care
    Iab717 = 101,
    /// Depression
    Iab718 = 102,
    /// Dermatology
    Iab719 = 103,
    /// Diabetes
    Iab720 = 104,
    /// Epilepsy
    Iab721 = 105,
    /// GERD/Acid Reflux
    Iab722 = 106,
    /// Headaches/Migraines
    Iab723 = 107,
    /// Heart Disease
    Iab724 = 108,
    /// Herbs for Health
    Iab725 = 109,
    /// Holistic Healing
    Iab726 = 110,
    /// IBS/Crohn's Disease
    Iab727 = 111,
    /// Incest/Abuse Support
    Iab728 = 112,
    /// Incontinence
    Iab729 = 113,
    /// Infertility
    Iab730 = 114,
    /// Men's Health
    Iab731 = 115,
    /// Nutrition
    Iab732 = 116,
    /// Orthopedics
    Iab733 = 117,
    /// Panic/Anxiety Disorders
    Iab734 = 118,
    /// Pediatrics
    Iab735 = 119,
    /// Physical Therapy
    Iab736 = 120,
    /// Psychology/Psychiatry
    Iab737 = 121,
    /// Senor Health
    Iab738 = 122,
    /// Sexuality
    Iab739 = 123,
    /// Sleep Disorders
    Iab740 = 124,
    /// Smoking Cessation
    Iab741 = 125,
    /// Substance Abuse
    Iab742 = 126,
    /// Thyroid Disease
    Iab743 = 127,
    /// Weight Loss
    Iab744 = 128,
    /// Women's Health
    Iab745 = 129,
    /// Food & Drink
    Iab8 = 130,
    /// American Cuisine
    Iab81 = 131,
    /// Barbecues & Grilling
    Iab82 = 132,
    /// Cajun/Creole
    Iab83 = 133,
    /// Chinese Cuisine
    Iab84 = 134,
    /// Cocktails/Beer
    Iab85 = 135,
    /// Coffee/Tea
    Iab86 = 136,
    /// Cuisine-Specific
    Iab87 = 137,
    /// Desserts & Baking
    Iab88 = 138,
    /// Dining Out
    Iab89 = 139,
    /// Food Allergies
    Iab810 = 140,
    /// French Cuisine
    Iab811 = 141,
    /// Health/Lowfat Cooking
    Iab812 = 142,
    /// Italian Cuisine
    Iab813 = 143,
    /// Japanese Cuisine
    Iab814 = 144,
    /// Mexican Cuisine
    Iab815 = 145,
    /// Vegan
    Iab816 = 146,
    /// Vegetarian
    Iab817 = 147,
    /// Wine
    Iab818 = 148,
    /// Hobbies & Interests
    Iab9 = 149,
    /// Art/Technology
    Iab91 = 150,
    /// Arts & Crafts
    Iab92 = 151,
    /// Beadwork
    Iab93 = 152,
    /// Birdwatching
    Iab94 = 153,
    /// Board Games/Puzzles
    Iab95 = 154,
    /// Candle & Soap Making
    Iab96 = 155,
    /// Card Games
    Iab97 = 156,
    /// Chess
    Iab98 = 157,
    /// Cigars
    Iab99 = 158,
    /// Collecting
    Iab910 = 159,
    /// Comic Books
    Iab911 = 160,
    /// Drawing/Sketching
    Iab912 = 161,
    /// Freelance Writing
    Iab913 = 162,
    /// Geneaology
    Iab914 = 163,
    /// Getting Published
    Iab915 = 164,
    /// Guitar
    Iab916 = 165,
    /// Home Recording
    Iab917 = 166,
    /// Investors & Patents
    Iab918 = 167,
    /// Jewelry Making
    Iab919 = 168,
    /// Magic & Illusion
    Iab920 = 169,
    /// Needlework
    Iab921 = 170,
    /// Painting
    Iab922 = 171,
    /// Photography
    Iab923 = 172,
    /// Radio
    Iab924 = 173,
    /// Roleplaying Games
    Iab925 = 174,
    /// Sci-Fi & Fantasy
    Iab926 = 175,
    /// Scrapbooking
    Iab927 = 176,
    /// Screenwriting
    Iab928 = 177,
    /// Stamps & Coins
    Iab929 = 178,
    /// Video & Computer Games
    Iab930 = 179,
    /// Woodworking
    Iab931 = 180,
    /// Home & Garden
    Iab10 = 181,
    /// Appliances
    Iab101 = 182,
    /// Entertaining
    Iab102 = 183,
    /// Environmental Safety
    Iab103 = 184,
    /// Gardening
    Iab104 = 185,
    /// Home Repair
    Iab105 = 186,
    /// Home Theater
    Iab106 = 187,
    /// Interior  Decorating
    Iab107 = 188,
    /// Landscaping
    Iab108 = 189,
    /// Remodeling & Construction
    Iab109 = 190,
    /// Law, Gov't & Politics
    Iab11x = 191,
    /// Immigration
    Iab111 = 192,
    /// Legal Issues
    Iab112 = 193,
    /// U.S. Government Resources
    Iab113 = 194,
    /// Politics
    Iab114 = 195,
    /// Commentary
    Iab115 = 196,
    /// News
    Iab12x = 197,
    /// International News
    Iab121 = 198,
    /// National News
    Iab122 = 199,
    /// Local News
    Iab123 = 200,
    /// Personal Finance
    Iab13x = 201,
    /// Beginning Investing
    Iab131 = 202,
    /// Credit/Debt & Loans
    Iab132 = 203,
    /// Financial News
    Iab133 = 204,
    /// Financial Planning
    Iab134 = 205,
    /// Hedge Fund
    Iab135 = 206,
    /// Insurance
    Iab136 = 207,
    /// Investing
    Iab137 = 208,
    /// Mutual Funds
    Iab138 = 209,
    /// Options
    Iab139 = 210,
    /// Retirement Planning
    Iab1310 = 211,
    /// Stocks
    Iab1311 = 212,
    /// Tax Planning
    Iab1312 = 213,
    /// Society
    Iab14x = 214,
    /// Dating
    Iab141 = 215,
    /// Divorce Support
    Iab142 = 216,
    /// Gay Life
    Iab143 = 217,
    /// Marriage
    Iab144 = 218,
    /// Senior Living
    Iab145 = 219,
    /// Teens
    Iab146 = 220,
    /// Weddings
    Iab147 = 221,
    /// Ethnic Specific
    Iab148 = 222,
    /// Science
    Iab15x = 223,
    /// Astrology
    Iab151 = 224,
    /// Biology
    Iab152 = 225,
    /// Chemistry
    Iab153 = 226,
    /// Geology
    Iab154 = 227,
    /// Paranormal Phenomena
    Iab155 = 228,
    /// Physics
    Iab156 = 229,
    /// Space/Astronomy
    Iab157 = 230,
    /// Geography
    Iab158 = 231,
    /// Botany
    Iab159 = 232,
    /// Weather
    Iab1510 = 233,
    /// Pets
    Iab16x = 234,
    /// Aquariums
    Iab161 = 235,
    /// Birds
    Iab162 = 236,
    /// Cats
    Iab163 = 237,
    /// Dogs
    Iab164 = 238,
    /// Large Animals
    Iab165 = 239,
    /// Reptiles
    Iab166 = 240,
    /// Veterinary Medicine
    Iab167 = 241,
    /// Sports
    Iab17x = 242,
    /// Auto Racing
    Iab171 = 243,
    /// Baseball
    Iab172 = 244,
    /// Bicycling
    Iab173 = 245,
    /// Bodybuilding
    Iab174 = 246,
    /// Boxing
    Iab175 = 247,
    /// Canoeing/Kayaking
    Iab176 = 248,
    /// Cheerleading
    Iab177 = 249,
    /// Climbing
    Iab178 = 250,
    /// Cricket
    Iab179 = 251,
    /// Figure Skating
    Iab1710 = 252,
    /// Fly Fishing
    Iab1711 = 253,
    /// Football
    Iab1712 = 254,
    /// Freshwater Fishing
    Iab1713 = 255,
    /// Game & Fish
    Iab1714 = 256,
    /// Golf
    Iab1715 = 257,
    /// Horse Racing
    Iab1716 = 258,
    /// Horses
    Iab1717 = 259,
    /// Hunting/Shooting
    Iab1718 = 260,
    /// Inline  Skating
    Iab1719 = 261,
    /// Martial Arts
    Iab1720 = 262,
    /// Mountain Biking
    Iab1721 = 263,
    /// NASCAR Racing
    Iab1722 = 264,
    /// Olympics
    Iab1723 = 265,
    /// Paintball
    Iab1724 = 266,
    /// Power & Motorcycles
    Iab1725 = 267,
    /// Pro Basketball
    Iab1726 = 268,
    /// Pro Ice Hockey
    Iab1727 = 269,
    /// Rodeo
    Iab1728 = 270,
    /// Rugby
    Iab1729 = 271,
    /// Running/Jogging
    Iab1730 = 272,
    /// Sailing
    Iab1731 = 273,
    /// Saltwater Fishing
    Iab1732 = 274,
    /// Scuba Diving
    Iab1733 = 275,
    /// Skateboarding
    Iab1734 = 276,
    /// Skiing
    Iab1735 = 277,
    /// Snowboarding
    Iab1736 = 278,
    /// Surfing/Bodyboarding
    Iab1737 = 279,
    /// Swimming
    Iab1738 = 280,
    /// Table Tennis/Ping-Pong
    Iab1739 = 281,
    /// Tennis
    Iab1740 = 282,
    /// Volleyball
    Iab1741 = 283,
    /// Walking
    Iab1742 = 284,
    /// Waterski/Wakeboard
    Iab1743 = 285,
    /// World Soccer
    Iab1744 = 286,
    /// Style & Fashion
    Iab18 = 287,
    /// Beauty
    Iab181 = 288,
    /// Body Art
    Iab182 = 289,
    /// Fashion
    Iab183 = 290,
    /// Jewelry
    Iab184 = 291,
    /// Clothing
    Iab185 = 292,
    /// Accessories
    Iab186 = 293,
    /// Technology & Computing
    Iab19 = 294,
    /// 3-D Graphics
    Iab191 = 295,
    /// Animation
    Iab192 = 296,
    /// Antivirus Software
    Iab193 = 297,
    /// C/C++
    Iab194 = 298,
    /// Cameras & Camcorders
    Iab195 = 299,
    /// Cell  Phones
    Iab196 = 300,
    /// Computer Certification
    Iab197 = 301,
    /// Computer Networking
    Iab198 = 302,
    /// Computer Peripherals
    Iab199 = 303,
    /// Computer Reviews
    Iab1910 = 304,
    /// Data Centers
    Iab1911 = 305,
    /// Databases
    Iab1912 = 306,
    /// Desktop Publishing
    Iab1913 = 307,
    /// Desktop Video
    Iab1914 = 308,
    /// Email
    Iab1915 = 309,
    /// Graphics Software
    Iab1916 = 310,
    /// Home Video/DVD
    Iab1917 = 311,
    /// Internet Technology
    Iab1918 = 312,
    /// Java
    Iab1919 = 313,
    /// Javascript
    Iab1920 = 314,
    /// Mac Support
    Iab1921 = 315,
    /// MP3/MIDI
    Iab1922 = 316,
    /// Net Conferencing
    Iab1923 = 317,
    /// Net for Beginners
    Iab1924 = 318,
    /// Network Security
    Iab1925 = 319,
    /// Palmtops/PDAs
    Iab1926 = 320,
    /// PC Support
    Iab1927 = 321,
    /// Portable
    Iab1928 = 322,
    /// Entertainment
    Iab1929 = 323,
    /// Shareware/Freeware
    Iab1930 = 324,
    /// Unix
    Iab1931 = 325,
    /// Visual Basic
    Iab1932 = 326,
    /// Web Clip Art
    Iab1933 = 327,
    /// Web Design/HTML
    Iab1934 = 328,
    /// Web Search
    Iab1935 = 329,
    /// Windows
    Iab1936 = 330,
    /// Travel
    Iab20 = 331,
    /// Adventure Travel
    Iab201 = 332,
    /// Africa
    Iab202 = 333,
    /// Air Travel
    Iab203 = 334,
    /// Australia & New Zealand
    Iab204 = 335,
    /// Bed & Breakfasts
    Iab205 = 336,
    /// Budget Travel
    Iab206 = 337,
    /// Business Travel
    Iab207 = 338,
    /// By US Locale
    Iab208 = 339,
    /// Camping
    Iab209 = 340,
    /// Canada
    Iab2010 = 341,
    /// Caribbean
    Iab2011 = 342,
    /// Cruises
    Iab2012 = 343,
    /// Eastern  Europe
    Iab2013 = 344,
    /// Europe
    Iab2014 = 345,
    /// France
    Iab2015 = 346,
    /// Greece
    Iab2016 = 347,
    /// Honeymoons/Getaways
    Iab2017 = 348,
    /// Hotels
    Iab2018 = 349,
    /// Italy
    Iab2019 = 350,
    /// Japan
    Iab2020 = 351,
    /// Mexico & Central America
    Iab2021 = 352,
    /// National Parks
    Iab2022 = 353,
    /// South America
    Iab2023 = 354,
    /// Spas
    Iab2024 = 355,
    /// Theme Parks
    Iab2025 = 356,
    /// Traveling with Kids
    Iab2026 = 357,
    /// United Kingdom
    Iab2027 = 358,
    /// Real Estate
    Iab21x = 359,
    /// Apartments
    Iab211x = 360,
    /// Architects
    Iab212x = 361,
    /// Buying/Selling Homes
    Iab213x = 362,
    /// Shopping
    Iab22x = 363,
    /// Contests & Freebies
    Iab221x = 364,
    /// Couponing
    Iab222x = 365,
    /// Comparison
    Iab223x = 366,
    /// Engines
    Iab224 = 367,
    /// Religion & Spirituality
    Iab23x = 368,
    /// Alternative Religions
    Iab231 = 369,
    /// Atheism/Agnosticism
    Iab232 = 370,
    /// Buddhism
    Iab233 = 371,
    /// Catholicism
    Iab234 = 372,
    /// Christianity
    Iab235 = 373,
    /// Hinduism
    Iab236 = 374,
    /// Islam
    Iab237 = 375,
    /// Judaism
    Iab238 = 376,
    /// Latter-Day Saints
    Iab239 = 377,
    /// Paga/Wiccan
    Iab2310 = 378,
    /// Uncategorized
    Iab24x = 379,
    /// Non-Standard Content
    Iab25x = 380,
    /// Unmoderated UGC
    Iab251 = 381,
    /// Extreme Graphic/Explicit Violence
    Iab252 = 382,
    /// Pornography
    Iab253 = 383,
    /// Profane Content
    Iab254 = 384,
    /// Hate Content
    Iab255 = 385,
    /// Under Construction
    Iab256 = 386,
    /// Incentivized
    Iab257 = 387,
    /// Illegal Content
    Iab26x = 388,
    /// Illegal Content
    Iab261 = 389,
    /// Warez
    Iab262 = 390,
    /// Spyware/Malware
    Iab263 = 391,
    /// Copyright Infringement
    Iab264 = 392,
}
impl ContentCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContentCategory::Undefined => "UNDEFINED",
            ContentCategory::Iab1 => "IAB1",
            ContentCategory::Iab11 => "IAB1_1",
            ContentCategory::Iab12 => "IAB1_2",
            ContentCategory::Iab13 => "IAB1_3",
            ContentCategory::Iab14 => "IAB1_4",
            ContentCategory::Iab15 => "IAB1_5",
            ContentCategory::Iab16 => "IAB1_6",
            ContentCategory::Iab17 => "IAB1_7",
            ContentCategory::Iab2 => "IAB2",
            ContentCategory::Iab21 => "IAB2_1",
            ContentCategory::Iab22 => "IAB2_2",
            ContentCategory::Iab23 => "IAB2_3",
            ContentCategory::Iab24 => "IAB2_4",
            ContentCategory::Iab25 => "IAB2_5",
            ContentCategory::Iab26 => "IAB2_6",
            ContentCategory::Iab27 => "IAB2_7",
            ContentCategory::Iab28 => "IAB2_8",
            ContentCategory::Iab29 => "IAB2_9",
            ContentCategory::Iab210 => "IAB2_10",
            ContentCategory::Iab211 => "IAB2_11",
            ContentCategory::Iab212 => "IAB2_12",
            ContentCategory::Iab213 => "IAB2_13",
            ContentCategory::Iab214 => "IAB2_14",
            ContentCategory::Iab215 => "IAB2_15",
            ContentCategory::Iab216 => "IAB2_16",
            ContentCategory::Iab217 => "IAB2_17",
            ContentCategory::Iab218 => "IAB2_18",
            ContentCategory::Iab219 => "IAB2_19",
            ContentCategory::Iab220 => "IAB2_20",
            ContentCategory::Iab221 => "IAB2_21",
            ContentCategory::Iab222 => "IAB2_22",
            ContentCategory::Iab223 => "IAB2_23",
            ContentCategory::Iab3 => "IAB3",
            ContentCategory::Iab31 => "IAB3_1",
            ContentCategory::Iab32 => "IAB3_2",
            ContentCategory::Iab33 => "IAB3_3",
            ContentCategory::Iab34 => "IAB3_4",
            ContentCategory::Iab35 => "IAB3_5",
            ContentCategory::Iab36 => "IAB3_6",
            ContentCategory::Iab37 => "IAB3_7",
            ContentCategory::Iab38 => "IAB3_8",
            ContentCategory::Iab39 => "IAB3_9",
            ContentCategory::Iab310 => "IAB3_10",
            ContentCategory::Iab311 => "IAB3_11",
            ContentCategory::Iab312 => "IAB3_12",
            ContentCategory::Iab4 => "IAB4",
            ContentCategory::Iab41 => "IAB4_1",
            ContentCategory::Iab42 => "IAB4_2",
            ContentCategory::Iab43 => "IAB4_3",
            ContentCategory::Iab44 => "IAB4_4",
            ContentCategory::Iab45 => "IAB4_5",
            ContentCategory::Iab46 => "IAB4_6",
            ContentCategory::Iab47 => "IAB4_7",
            ContentCategory::Iab48 => "IAB4_8",
            ContentCategory::Iab49 => "IAB4_9",
            ContentCategory::Iab410 => "IAB4_10",
            ContentCategory::Iab411 => "IAB4_11",
            ContentCategory::Iab5 => "IAB5",
            ContentCategory::Iab51 => "IAB5_1",
            ContentCategory::Iab52 => "IAB5_2",
            ContentCategory::Iab53 => "IAB5_3",
            ContentCategory::Iab54 => "IAB5_4",
            ContentCategory::Iab55 => "IAB5_5",
            ContentCategory::Iab56 => "IAB5_6",
            ContentCategory::Iab57 => "IAB5_7",
            ContentCategory::Iab58 => "IAB5_8",
            ContentCategory::Iab59 => "IAB5_9",
            ContentCategory::Iab510 => "IAB5_10",
            ContentCategory::Iab511 => "IAB5_11",
            ContentCategory::Iab512 => "IAB5_12",
            ContentCategory::Iab513 => "IAB5_13",
            ContentCategory::Iab514 => "IAB5_14",
            ContentCategory::Iab515 => "IAB5_15",
            ContentCategory::Iab6 => "IAB6",
            ContentCategory::Iab61 => "IAB6_1",
            ContentCategory::Iab62 => "IAB6_2",
            ContentCategory::Iab63 => "IAB6_3",
            ContentCategory::Iab64 => "IAB6_4",
            ContentCategory::Iab65 => "IAB6_5",
            ContentCategory::Iab66 => "IAB6_6",
            ContentCategory::Iab67 => "IAB6_7",
            ContentCategory::Iab68 => "IAB6_8",
            ContentCategory::Iab69 => "IAB6_9",
            ContentCategory::Iab7 => "IAB7",
            ContentCategory::Iab71 => "IAB7_1",
            ContentCategory::Iab72 => "IAB7_2",
            ContentCategory::Iab73 => "IAB7_3",
            ContentCategory::Iab74 => "IAB7_4",
            ContentCategory::Iab75 => "IAB7_5",
            ContentCategory::Iab76 => "IAB7_6",
            ContentCategory::Iab77 => "IAB7_7",
            ContentCategory::Iab78 => "IAB7_8",
            ContentCategory::Iab79 => "IAB7_9",
            ContentCategory::Iab710 => "IAB7_10",
            ContentCategory::Iab711 => "IAB7_11",
            ContentCategory::Iab712 => "IAB7_12",
            ContentCategory::Iab713 => "IAB7_13",
            ContentCategory::Iab714 => "IAB7_14",
            ContentCategory::Iab715 => "IAB7_15",
            ContentCategory::Iab716 => "IAB7_16",
            ContentCategory::Iab717 => "IAB7_17",
            ContentCategory::Iab718 => "IAB7_18",
            ContentCategory::Iab719 => "IAB7_19",
            ContentCategory::Iab720 => "IAB7_20",
            ContentCategory::Iab721 => "IAB7_21",
            ContentCategory::Iab722 => "IAB7_22",
            ContentCategory::Iab723 => "IAB7_23",
            ContentCategory::Iab724 => "IAB7_24",
            ContentCategory::Iab725 => "IAB7_25",
            ContentCategory::Iab726 => "IAB7_26",
            ContentCategory::Iab727 => "IAB7_27",
            ContentCategory::Iab728 => "IAB7_28",
            ContentCategory::Iab729 => "IAB7_29",
            ContentCategory::Iab730 => "IAB7_30",
            ContentCategory::Iab731 => "IAB7_31",
            ContentCategory::Iab732 => "IAB7_32",
            ContentCategory::Iab733 => "IAB7_33",
            ContentCategory::Iab734 => "IAB7_34",
            ContentCategory::Iab735 => "IAB7_35",
            ContentCategory::Iab736 => "IAB7_36",
            ContentCategory::Iab737 => "IAB7_37",
            ContentCategory::Iab738 => "IAB7_38",
            ContentCategory::Iab739 => "IAB7_39",
            ContentCategory::Iab740 => "IAB7_40",
            ContentCategory::Iab741 => "IAB7_41",
            ContentCategory::Iab742 => "IAB7_42",
            ContentCategory::Iab743 => "IAB7_43",
            ContentCategory::Iab744 => "IAB7_44",
            ContentCategory::Iab745 => "IAB7_45",
            ContentCategory::Iab8 => "IAB8",
            ContentCategory::Iab81 => "IAB8_1",
            ContentCategory::Iab82 => "IAB8_2",
            ContentCategory::Iab83 => "IAB8_3",
            ContentCategory::Iab84 => "IAB8_4",
            ContentCategory::Iab85 => "IAB8_5",
            ContentCategory::Iab86 => "IAB8_6",
            ContentCategory::Iab87 => "IAB8_7",
            ContentCategory::Iab88 => "IAB8_8",
            ContentCategory::Iab89 => "IAB8_9",
            ContentCategory::Iab810 => "IAB8_10",
            ContentCategory::Iab811 => "IAB8_11",
            ContentCategory::Iab812 => "IAB8_12",
            ContentCategory::Iab813 => "IAB8_13",
            ContentCategory::Iab814 => "IAB8_14",
            ContentCategory::Iab815 => "IAB8_15",
            ContentCategory::Iab816 => "IAB8_16",
            ContentCategory::Iab817 => "IAB8_17",
            ContentCategory::Iab818 => "IAB8_18",
            ContentCategory::Iab9 => "IAB9",
            ContentCategory::Iab91 => "IAB9_1",
            ContentCategory::Iab92 => "IAB9_2",
            ContentCategory::Iab93 => "IAB9_3",
            ContentCategory::Iab94 => "IAB9_4",
            ContentCategory::Iab95 => "IAB9_5",
            ContentCategory::Iab96 => "IAB9_6",
            ContentCategory::Iab97 => "IAB9_7",
            ContentCategory::Iab98 => "IAB9_8",
            ContentCategory::Iab99 => "IAB9_9",
            ContentCategory::Iab910 => "IAB9_10",
            ContentCategory::Iab911 => "IAB9_11",
            ContentCategory::Iab912 => "IAB9_12",
            ContentCategory::Iab913 => "IAB9_13",
            ContentCategory::Iab914 => "IAB9_14",
            ContentCategory::Iab915 => "IAB9_15",
            ContentCategory::Iab916 => "IAB9_16",
            ContentCategory::Iab917 => "IAB9_17",
            ContentCategory::Iab918 => "IAB9_18",
            ContentCategory::Iab919 => "IAB9_19",
            ContentCategory::Iab920 => "IAB9_20",
            ContentCategory::Iab921 => "IAB9_21",
            ContentCategory::Iab922 => "IAB9_22",
            ContentCategory::Iab923 => "IAB9_23",
            ContentCategory::Iab924 => "IAB9_24",
            ContentCategory::Iab925 => "IAB9_25",
            ContentCategory::Iab926 => "IAB9_26",
            ContentCategory::Iab927 => "IAB9_27",
            ContentCategory::Iab928 => "IAB9_28",
            ContentCategory::Iab929 => "IAB9_29",
            ContentCategory::Iab930 => "IAB9_30",
            ContentCategory::Iab931 => "IAB9_31",
            ContentCategory::Iab10 => "IAB10",
            ContentCategory::Iab101 => "IAB10_1",
            ContentCategory::Iab102 => "IAB10_2",
            ContentCategory::Iab103 => "IAB10_3",
            ContentCategory::Iab104 => "IAB10_4",
            ContentCategory::Iab105 => "IAB10_5",
            ContentCategory::Iab106 => "IAB10_6",
            ContentCategory::Iab107 => "IAB10_7",
            ContentCategory::Iab108 => "IAB10_8",
            ContentCategory::Iab109 => "IAB10_9",
            ContentCategory::Iab11x => "IAB11X",
            ContentCategory::Iab111 => "IAB11_1",
            ContentCategory::Iab112 => "IAB11_2",
            ContentCategory::Iab113 => "IAB11_3",
            ContentCategory::Iab114 => "IAB11_4",
            ContentCategory::Iab115 => "IAB11_5",
            ContentCategory::Iab12x => "IAB12X",
            ContentCategory::Iab121 => "IAB12_1",
            ContentCategory::Iab122 => "IAB12_2",
            ContentCategory::Iab123 => "IAB12_3",
            ContentCategory::Iab13x => "IAB13X",
            ContentCategory::Iab131 => "IAB13_1",
            ContentCategory::Iab132 => "IAB13_2",
            ContentCategory::Iab133 => "IAB13_3",
            ContentCategory::Iab134 => "IAB13_4",
            ContentCategory::Iab135 => "IAB13_5",
            ContentCategory::Iab136 => "IAB13_6",
            ContentCategory::Iab137 => "IAB13_7",
            ContentCategory::Iab138 => "IAB13_8",
            ContentCategory::Iab139 => "IAB13_9",
            ContentCategory::Iab1310 => "IAB13_10",
            ContentCategory::Iab1311 => "IAB13_11",
            ContentCategory::Iab1312 => "IAB13_12",
            ContentCategory::Iab14x => "IAB14X",
            ContentCategory::Iab141 => "IAB14_1",
            ContentCategory::Iab142 => "IAB14_2",
            ContentCategory::Iab143 => "IAB14_3",
            ContentCategory::Iab144 => "IAB14_4",
            ContentCategory::Iab145 => "IAB14_5",
            ContentCategory::Iab146 => "IAB14_6",
            ContentCategory::Iab147 => "IAB14_7",
            ContentCategory::Iab148 => "IAB14_8",
            ContentCategory::Iab15x => "IAB15X",
            ContentCategory::Iab151 => "IAB15_1",
            ContentCategory::Iab152 => "IAB15_2",
            ContentCategory::Iab153 => "IAB15_3",
            ContentCategory::Iab154 => "IAB15_4",
            ContentCategory::Iab155 => "IAB15_5",
            ContentCategory::Iab156 => "IAB15_6",
            ContentCategory::Iab157 => "IAB15_7",
            ContentCategory::Iab158 => "IAB15_8",
            ContentCategory::Iab159 => "IAB15_9",
            ContentCategory::Iab1510 => "IAB15_10",
            ContentCategory::Iab16x => "IAB16X",
            ContentCategory::Iab161 => "IAB16_1",
            ContentCategory::Iab162 => "IAB16_2",
            ContentCategory::Iab163 => "IAB16_3",
            ContentCategory::Iab164 => "IAB16_4",
            ContentCategory::Iab165 => "IAB16_5",
            ContentCategory::Iab166 => "IAB16_6",
            ContentCategory::Iab167 => "IAB16_7",
            ContentCategory::Iab17x => "IAB17X",
            ContentCategory::Iab171 => "IAB17_1",
            ContentCategory::Iab172 => "IAB17_2",
            ContentCategory::Iab173 => "IAB17_3",
            ContentCategory::Iab174 => "IAB17_4",
            ContentCategory::Iab175 => "IAB17_5",
            ContentCategory::Iab176 => "IAB17_6",
            ContentCategory::Iab177 => "IAB17_7",
            ContentCategory::Iab178 => "IAB17_8",
            ContentCategory::Iab179 => "IAB17_9",
            ContentCategory::Iab1710 => "IAB17_10",
            ContentCategory::Iab1711 => "IAB17_11",
            ContentCategory::Iab1712 => "IAB17_12",
            ContentCategory::Iab1713 => "IAB17_13",
            ContentCategory::Iab1714 => "IAB17_14",
            ContentCategory::Iab1715 => "IAB17_15",
            ContentCategory::Iab1716 => "IAB17_16",
            ContentCategory::Iab1717 => "IAB17_17",
            ContentCategory::Iab1718 => "IAB17_18",
            ContentCategory::Iab1719 => "IAB17_19",
            ContentCategory::Iab1720 => "IAB17_20",
            ContentCategory::Iab1721 => "IAB17_21",
            ContentCategory::Iab1722 => "IAB17_22",
            ContentCategory::Iab1723 => "IAB17_23",
            ContentCategory::Iab1724 => "IAB17_24",
            ContentCategory::Iab1725 => "IAB17_25",
            ContentCategory::Iab1726 => "IAB17_26",
            ContentCategory::Iab1727 => "IAB17_27",
            ContentCategory::Iab1728 => "IAB17_28",
            ContentCategory::Iab1729 => "IAB17_29",
            ContentCategory::Iab1730 => "IAB17_30",
            ContentCategory::Iab1731 => "IAB17_31",
            ContentCategory::Iab1732 => "IAB17_32",
            ContentCategory::Iab1733 => "IAB17_33",
            ContentCategory::Iab1734 => "IAB17_34",
            ContentCategory::Iab1735 => "IAB17_35",
            ContentCategory::Iab1736 => "IAB17_36",
            ContentCategory::Iab1737 => "IAB17_37",
            ContentCategory::Iab1738 => "IAB17_38",
            ContentCategory::Iab1739 => "IAB17_39",
            ContentCategory::Iab1740 => "IAB17_40",
            ContentCategory::Iab1741 => "IAB17_41",
            ContentCategory::Iab1742 => "IAB17_42",
            ContentCategory::Iab1743 => "IAB17_43",
            ContentCategory::Iab1744 => "IAB17_44",
            ContentCategory::Iab18 => "IAB18",
            ContentCategory::Iab181 => "IAB18_1",
            ContentCategory::Iab182 => "IAB18_2",
            ContentCategory::Iab183 => "IAB18_3",
            ContentCategory::Iab184 => "IAB18_4",
            ContentCategory::Iab185 => "IAB18_5",
            ContentCategory::Iab186 => "IAB18_6",
            ContentCategory::Iab19 => "IAB19",
            ContentCategory::Iab191 => "IAB19_1",
            ContentCategory::Iab192 => "IAB19_2",
            ContentCategory::Iab193 => "IAB19_3",
            ContentCategory::Iab194 => "IAB19_4",
            ContentCategory::Iab195 => "IAB19_5",
            ContentCategory::Iab196 => "IAB19_6",
            ContentCategory::Iab197 => "IAB19_7",
            ContentCategory::Iab198 => "IAB19_8",
            ContentCategory::Iab199 => "IAB19_9",
            ContentCategory::Iab1910 => "IAB19_10",
            ContentCategory::Iab1911 => "IAB19_11",
            ContentCategory::Iab1912 => "IAB19_12",
            ContentCategory::Iab1913 => "IAB19_13",
            ContentCategory::Iab1914 => "IAB19_14",
            ContentCategory::Iab1915 => "IAB19_15",
            ContentCategory::Iab1916 => "IAB19_16",
            ContentCategory::Iab1917 => "IAB19_17",
            ContentCategory::Iab1918 => "IAB19_18",
            ContentCategory::Iab1919 => "IAB19_19",
            ContentCategory::Iab1920 => "IAB19_20",
            ContentCategory::Iab1921 => "IAB19_21",
            ContentCategory::Iab1922 => "IAB19_22",
            ContentCategory::Iab1923 => "IAB19_23",
            ContentCategory::Iab1924 => "IAB19_24",
            ContentCategory::Iab1925 => "IAB19_25",
            ContentCategory::Iab1926 => "IAB19_26",
            ContentCategory::Iab1927 => "IAB19_27",
            ContentCategory::Iab1928 => "IAB19_28",
            ContentCategory::Iab1929 => "IAB19_29",
            ContentCategory::Iab1930 => "IAB19_30",
            ContentCategory::Iab1931 => "IAB19_31",
            ContentCategory::Iab1932 => "IAB19_32",
            ContentCategory::Iab1933 => "IAB19_33",
            ContentCategory::Iab1934 => "IAB19_34",
            ContentCategory::Iab1935 => "IAB19_35",
            ContentCategory::Iab1936 => "IAB19_36",
            ContentCategory::Iab20 => "IAB20",
            ContentCategory::Iab201 => "IAB20_1",
            ContentCategory::Iab202 => "IAB20_2",
            ContentCategory::Iab203 => "IAB20_3",
            ContentCategory::Iab204 => "IAB20_4",
            ContentCategory::Iab205 => "IAB20_5",
            ContentCategory::Iab206 => "IAB20_6",
            ContentCategory::Iab207 => "IAB20_7",
            ContentCategory::Iab208 => "IAB20_8",
            ContentCategory::Iab209 => "IAB20_9",
            ContentCategory::Iab2010 => "IAB20_10",
            ContentCategory::Iab2011 => "IAB20_11",
            ContentCategory::Iab2012 => "IAB20_12",
            ContentCategory::Iab2013 => "IAB20_13",
            ContentCategory::Iab2014 => "IAB20_14",
            ContentCategory::Iab2015 => "IAB20_15",
            ContentCategory::Iab2016 => "IAB20_16",
            ContentCategory::Iab2017 => "IAB20_17",
            ContentCategory::Iab2018 => "IAB20_18",
            ContentCategory::Iab2019 => "IAB20_19",
            ContentCategory::Iab2020 => "IAB20_20",
            ContentCategory::Iab2021 => "IAB20_21",
            ContentCategory::Iab2022 => "IAB20_22",
            ContentCategory::Iab2023 => "IAB20_23",
            ContentCategory::Iab2024 => "IAB20_24",
            ContentCategory::Iab2025 => "IAB20_25",
            ContentCategory::Iab2026 => "IAB20_26",
            ContentCategory::Iab2027 => "IAB20_27",
            ContentCategory::Iab21x => "IAB21X",
            ContentCategory::Iab211x => "IAB21_1X",
            ContentCategory::Iab212x => "IAB21_2X",
            ContentCategory::Iab213x => "IAB21_3X",
            ContentCategory::Iab22x => "IAB22X",
            ContentCategory::Iab221x => "IAB22_1X",
            ContentCategory::Iab222x => "IAB22_2X",
            ContentCategory::Iab223x => "IAB22_3X",
            ContentCategory::Iab224 => "IAB22_4",
            ContentCategory::Iab23x => "IAB23X",
            ContentCategory::Iab231 => "IAB23_1",
            ContentCategory::Iab232 => "IAB23_2",
            ContentCategory::Iab233 => "IAB23_3",
            ContentCategory::Iab234 => "IAB23_4",
            ContentCategory::Iab235 => "IAB23_5",
            ContentCategory::Iab236 => "IAB23_6",
            ContentCategory::Iab237 => "IAB23_7",
            ContentCategory::Iab238 => "IAB23_8",
            ContentCategory::Iab239 => "IAB23_9",
            ContentCategory::Iab2310 => "IAB23_10",
            ContentCategory::Iab24x => "IAB24X",
            ContentCategory::Iab25x => "IAB25X",
            ContentCategory::Iab251 => "IAB25_1",
            ContentCategory::Iab252 => "IAB25_2",
            ContentCategory::Iab253 => "IAB25_3",
            ContentCategory::Iab254 => "IAB25_4",
            ContentCategory::Iab255 => "IAB25_5",
            ContentCategory::Iab256 => "IAB25_6",
            ContentCategory::Iab257 => "IAB25_7",
            ContentCategory::Iab26x => "IAB26X",
            ContentCategory::Iab261 => "IAB26_1",
            ContentCategory::Iab262 => "IAB26_2",
            ContentCategory::Iab263 => "IAB26_3",
            ContentCategory::Iab264 => "IAB26_4",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AuctionType {
    FirstPrice,
    #[default]
    SecondPrice,
    FixedPrice(u32),
}

impl Serialize for AuctionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            AuctionType::FirstPrice => serializer.serialize_u32(1),
            AuctionType::SecondPrice => serializer.serialize_u32(2),
            AuctionType::FixedPrice(v) => serializer.serialize_u32(v),
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
            Ok(2) => Ok(AuctionType::SecondPrice),
            Ok(v) => Ok(AuctionType::FixedPrice(v)),
            Err(e) => Err(e),
        }
    }
}

// impl AuctionType {
//     /// String value of the enum field names used in the ProtoBuf definition.
//     ///
//     /// The values are not transformed in any way and thus are considered stable
//     /// (if the ProtoBuf definition does not change) and safe for programmatic use.
//     pub fn as_str_name(&self) -> &'static str {
//         match self {
//             AuctionType::FirstPrice => "FIRST_PRICE",
//             AuctionType::SecondPrice => "SECOND_PRICE",
//             AuctionType::FixedPrice => "FIXED_PRICE",
//         }
//     }
// }

/// OpenRTB 2.0: types of ads that can be accepted by the exchange unless
/// restricted by publisher site settings.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum BannerAdType {
    /// "Usually mobile".
    XhtmlTextAd = 1,
    /// "Usually mobile".
    XhtmlBannerAd = 2,
    /// Javascript must be valid XHTML (ie, script tags included).
    JavascriptAd = 3,
    /// Iframe.
    Iframe = 4,
}
impl BannerAdType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BannerAdType::XhtmlTextAd => "XHTML_TEXT_AD",
            BannerAdType::XhtmlBannerAd => "XHTML_BANNER_AD",
            BannerAdType::JavascriptAd => "JAVASCRIPT_AD",
            BannerAdType::Iframe => "IFRAME",
        }
    }
}
/// OpenRTB 2.0: The following table specifies a standard list of creative
/// attributes that can describe an ad being served or serve as restrictions
/// of thereof.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum CreativeAttribute {
    AudioAutoPlay = 1,
    AudioUserInitiated = 2,
    ExpandableAutomatic = 3,
    ExpandableClickInitiated = 4,
    ExpandableRolloverInitiated = 5,
    VideoInBannerAutoPlay = 6,
    VideoInBannerUserInitiated = 7,
    /// Pop (e.g., Over, Under, or upon Exit).
    Pop = 8,
    ProvocativeOrSuggestive = 9,
    /// Defined as "Shaky, Flashing, Flickering, Extreme Animation, Smileys".
    Annoying = 10,
    Surveys = 11,
    TextOnly = 12,
    /// Eg, embedded games.
    UserInteractive = 13,
    WindowsDialogOrAlertStyle = 14,
    HasAudioOnOffButton = 15,
    AdCanBeSkipped = 16,
    Flash = 17,
}
impl CreativeAttribute {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CreativeAttribute::AudioAutoPlay => "AUDIO_AUTO_PLAY",
            CreativeAttribute::AudioUserInitiated => "AUDIO_USER_INITIATED",
            CreativeAttribute::ExpandableAutomatic => "EXPANDABLE_AUTOMATIC",
            CreativeAttribute::ExpandableClickInitiated => "EXPANDABLE_CLICK_INITIATED",
            CreativeAttribute::ExpandableRolloverInitiated => "EXPANDABLE_ROLLOVER_INITIATED",
            CreativeAttribute::VideoInBannerAutoPlay => "VIDEO_IN_BANNER_AUTO_PLAY",
            CreativeAttribute::VideoInBannerUserInitiated => "VIDEO_IN_BANNER_USER_INITIATED",
            CreativeAttribute::Pop => "POP",
            CreativeAttribute::ProvocativeOrSuggestive => "PROVOCATIVE_OR_SUGGESTIVE",
            CreativeAttribute::Annoying => "ANNOYING",
            CreativeAttribute::Surveys => "SURVEYS",
            CreativeAttribute::TextOnly => "TEXT_ONLY",
            CreativeAttribute::UserInteractive => "USER_INTERACTIVE",
            CreativeAttribute::WindowsDialogOrAlertStyle => "WINDOWS_DIALOG_OR_ALERT_STYLE",
            CreativeAttribute::HasAudioOnOffButton => "HAS_AUDIO_ON_OFF_BUTTON",
            CreativeAttribute::AdCanBeSkipped => "AD_CAN_BE_SKIPPED",
            CreativeAttribute::Flash => "FLASH",
        }
    }
}

/// OpenRTB 2.0: The following table is a list of API frameworks supported
/// by the publisher.  Note that MRAID-1 is a subset of MRAID-2.
/// In OpenRTB 2.1 and prior, value "3" was "MRAID".  However, not all
/// MRAID capable APIs understand MRAID-2 features and as such the only
/// safe interpretation of value "3" is MRAID-1. In OpenRTB 2.2, this was
/// made explicit and MRAID-2 has been added as value "5".
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum ApiFramework {
    Vpaid1 = 1,
    Vpaid2 = 2,
    Mraid1 = 3,
    Ormma = 4,
    Mraid2 = 5,
    Mraid3 = 6,
    Omid1 = 7,
}
impl ApiFramework {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ApiFramework::Vpaid1 => "VPAID_1",
            ApiFramework::Vpaid2 => "VPAID_2",
            ApiFramework::Mraid1 => "MRAID_1",
            ApiFramework::Ormma => "ORMMA",
            ApiFramework::Mraid2 => "MRAID_2",
            ApiFramework::Mraid3 => "MRAID_3",
            ApiFramework::Omid1 => "OMID_1",
        }
    }
}
/// OpenRTB 2.0: The following table specifies the position of the ad as a
/// relative measure of visibility or prominence.
///
/// This OpenRTB table has values derived from the IAB Quality Assurance
/// Guidelines (QAG). Practitioners should keep in sync with updates to the
/// QAG values as published on IAB.net. Values "3" - "6" apply to apps
/// per the mobile addendum to QAG version 1.5.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum AdPosition {
    Unknown = 0,
    AboveTheFold = 1,
    /// DEPRECATED in OpenRTB 2.1+. No replacement.
    /// May or may not be immediately visible depending on screen size and
    /// resolution.
    LikelyBelowTheFold = 2,
    BelowTheFold = 3,
    /// [OpenRTB->AdX: SlotVisibility.ABOVE_THE_FOLD]
    /// Equivalent to
    /// `sticky_settings.top_horizontal_stickiness` in Authorized
    /// Buyers RTB protocol.
    Header = 4,
    /// [OpenRTB->AdX: SlotVisibility.ABOVE_THE_FOLD]
    /// Equivalent to `sticky_settings.bottom_horizontal_stickiness` in Authorized
    /// Buyers RTB protocol.
    Footer = 5,
    /// [OpenRTB->AdX: SlotVisibility.ABOVE_THE_FOLD]
    /// Equivalent to `sticky_settings.vertical_stickiness` in Authorized Buyers
    /// RTB protocol.
    Sidebar = 6,
    /// [OpenRTB->AdX: SlotVisibility.ABOVE_THE_FOLD]
    Fullscreen = 7,
}
impl AdPosition {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AdPosition::Unknown => "UNKNOWN",
            AdPosition::AboveTheFold => "ABOVE_THE_FOLD",
            AdPosition::LikelyBelowTheFold => "LIKELY_BELOW_THE_FOLD",
            AdPosition::BelowTheFold => "BELOW_THE_FOLD",
            AdPosition::Header => "HEADER",
            AdPosition::Footer => "FOOTER",
            AdPosition::Sidebar => "SIDEBAR",
            AdPosition::Fullscreen => "AD_POSITION_FULLSCREEN",
        }
    }
}
/// OpenRTB 2.0: The following table indicates the options for video
/// linearity. "In-stream" or "linear" video refers to pre-roll, post-roll,
/// or mid-roll video ads where the user is forced to watch ad in order to
/// see the video content. "Overlay" or "non-linear" refer to ads that are
/// shown on top of the video content.
///
/// This field is optional. The following is the interpretation of the
/// bidder based upon presence or absence of the field in the bid request:
/// - If no value is set, any ad (linear or not) can be present
///    in the response.
/// - If a value is set, only ads of the corresponding type can be present
///    in the response.
///
/// This OpenRTB table has values derived from the IAB Quality Assurance
/// Guidelines (QAG). Practitioners should keep in sync with updates to the
/// QAG values as published on IAB.net.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum VideoLinearity {
    /// Linear/In-stream
    Linear = 1,
    /// Non-linear/Overlay
    NonLinear = 2,
}

impl VideoLinearity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VideoLinearity::Linear => "LINEAR",
            VideoLinearity::NonLinear => "NON_LINEAR",
        }
    }
}
/// OpenRTB 2.0: The following table lists the options for the various
/// bid response protocols that could be supported by an exchange.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum Protocol {
    Vast10 = 1,
    Vast20 = 2,
    Vast30 = 3,
    Vast10Wrapper = 4,
    Vast20Wrapper = 5,
    Vast30Wrapper = 6,
    Vast40 = 7,
    Vast40Wrapper = 8,
    Daast10 = 9,
    Daast10Wrapper = 10,
}

impl Protocol {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Protocol::Vast10 => "VAST_1_0",
            Protocol::Vast20 => "VAST_2_0",
            Protocol::Vast30 => "VAST_3_0",
            Protocol::Vast10Wrapper => "VAST_1_0_WRAPPER",
            Protocol::Vast20Wrapper => "VAST_2_0_WRAPPER",
            Protocol::Vast30Wrapper => "VAST_3_0_WRAPPER",
            Protocol::Vast40 => "VAST_4_0",
            Protocol::Vast40Wrapper => "VAST_4_0_WRAPPER",
            Protocol::Daast10 => "DAAST_1_0",
            Protocol::Daast10Wrapper => "DAAST_1_0_WRAPPER",
        }
    }
}
/// OpenRTB 2.0: The following table lists the various playback methods.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum PlaybackMethod {
    /// Initiates on Page Load with Sound On.
    AutoPlaySoundOn = 1,
    /// Initiates on Page Load with Sound Off by Default.
    AutoPlaySoundOff = 2,
    /// Initiates on Click with Sound On.
    ClickToPlay = 3,
    /// Initiates on Mouse-Over with Sound On.
    MouseOver = 4,
    /// Initiates on Entering Viewport with Sound On.
    EnterSoundOn = 5,
    /// Initiates on Entering Viewport with Sound Off by Default.
    EnterSoundOff = 6,
}

impl PlaybackMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlaybackMethod::AutoPlaySoundOn => "AUTO_PLAY_SOUND_ON",
            PlaybackMethod::AutoPlaySoundOff => "AUTO_PLAY_SOUND_OFF",
            PlaybackMethod::ClickToPlay => "CLICK_TO_PLAY",
            PlaybackMethod::MouseOver => "MOUSE_OVER",
            PlaybackMethod::EnterSoundOn => "ENTER_SOUND_ON",
            PlaybackMethod::EnterSoundOff => "ENTER_SOUND_OFF",
        }
    }
}

/// OpenRTB 2.0: The following table lists the various options for the
/// audio/video start delay.  If the start delay value is greater than 0,
/// then the position is mid-roll and the value indicates the start delay.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum StartDelay {
    PreRoll = 0,
    GenericMidRoll = -1,
    GenericPostRoll = -2,
}
impl StartDelay {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StartDelay::PreRoll => "PRE_ROLL",
            StartDelay::GenericMidRoll => "GENERIC_MID_ROLL",
            StartDelay::GenericPostRoll => "GENERIC_POST_ROLL",
        }
    }
}
/// OpenRTB 2.5: The following table lists the various types of video placements
/// derived largely from the IAB Digital Video Guidelines.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize_repr,
    Deserialize_repr,
)]
#[repr(i32)]
pub enum VideoPlacementType {
    /// The video placement is not defined.
    /// Default value.
    #[default]
    UndefinedVideoPlacement = 0,
    /// Played before, during or after the streaming video content
    /// that the consumer has requested.
    /// E.G.: Pre-roll, Mid-roll, Post-roll.
    InStreamPlacement = 1,
    /// Exists within a web banner that leverages the banner space
    /// to deliver a video experience as opposed to another static
    /// or rich media format.
    /// The format relies on the existence of display ad inventory
    /// on the page for its delivery.
    InBannerPlacement = 2,
    /// Loads and plays dynamically between paragraphs of editorial content;
    /// existing as a standalone branded message.
    InArticlePlacement = 3,
    /// In-Feed - Found in content, social, or product feeds.
    InFeedPlacement = 4,
    /// Interstitial/Slider/Floating.
    /// Covers the entire or a portion of screen area,
    /// but is always on screen while displayed
    /// (i.e. cannot be scrolled out of view).
    /// Note that a full-screen interstitial (e.g., in mobile)
    /// can be distinguished from a floating/slider unit by the imp.instl field.
    FloatingPlacement = 5,
}
impl VideoPlacementType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VideoPlacementType::UndefinedVideoPlacement => "UNDEFINED_VIDEO_PLACEMENT",
            VideoPlacementType::InStreamPlacement => "IN_STREAM_PLACEMENT",
            VideoPlacementType::InBannerPlacement => "IN_BANNER_PLACEMENT",
            VideoPlacementType::InArticlePlacement => "IN_ARTICLE_PLACEMENT",
            VideoPlacementType::InFeedPlacement => "IN_FEED_PLACEMENT",
            VideoPlacementType::FloatingPlacement => "FLOATING_PLACEMENT",
        }
    }
}
/// OpenRTB 2.5: The various modes for when playback terminates.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum PlaybackCessationMode {
    /// On Video Completion or when Terminated by User
    CompletionOrUser = 1,
    /// On Leaving Viewport or when Terminated by User
    LeavingOrUser = 2,
    /// On Leaving Viewport Continues as a Floating/Slider Unit until
    /// Video Completion or when Terminated by User
    LeavingContinuesOrUser = 3,
}
impl PlaybackCessationMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlaybackCessationMode::CompletionOrUser => "COMPLETION_OR_USER",
            PlaybackCessationMode::LeavingOrUser => "LEAVING_OR_USER",
            PlaybackCessationMode::LeavingContinuesOrUser => "LEAVING_CONTINUES_OR_USER",
        }
    }
}
/// OpenRTB 2.0: The following table lists the various options for the
/// type of device connectivity.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum ConnectionType {
    ConnectionUnknown = 0,
    Ethernet = 1,
    Wifi = 2,
    CellUnknown = 3,
    Cell2g = 4,
    Cell3g = 5,
    Cell4g = 6,
}
impl ConnectionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConnectionType::ConnectionUnknown => "CONNECTION_UNKNOWN",
            ConnectionType::Ethernet => "ETHERNET",
            ConnectionType::Wifi => "WIFI",
            ConnectionType::CellUnknown => "CELL_UNKNOWN",
            ConnectionType::Cell2g => "CELL_2G",
            ConnectionType::Cell3g => "CELL_3G",
            ConnectionType::Cell4g => "CELL_4G",
        }
    }
}
/// OpenRTB 2.0: The following table lists the directions in which an
/// expandable ad may expand, given the positioning of the ad unit on the
/// page and constraints imposed by the content.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum ExpandableDirection {
    Left = 1,
    Right = 2,
    Up = 3,
    Down = 4,
    ExpandableFullscreen = 5,
}

impl ExpandableDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExpandableDirection::Left => "LEFT",
            ExpandableDirection::Right => "RIGHT",
            ExpandableDirection::Up => "UP",
            ExpandableDirection::Down => "DOWN",
            ExpandableDirection::ExpandableFullscreen => "EXPANDABLE_FULLSCREEN",
        }
    }
}
/// OpenRTB 2.0: The following table lists the various options for the
/// delivery of video content.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum ContentDeliveryMethod {
    Streaming = 1,
    Progressive = 2,
}
impl ContentDeliveryMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContentDeliveryMethod::Streaming => "STREAMING",
            ContentDeliveryMethod::Progressive => "PROGRESSIVE",
        }
    }
}
/// OpenRTB 2.0: The following table lists the various options for
/// indicating the type of content in which the impression will appear.
///
/// This OpenRTB table has values derived from the IAB Quality Assurance
/// Guidelines (QAG). Practitioners should keep in sync with updates to the
/// QAG values as published on IAB.net.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum ContentContext {
    Video = 1,
    Game = 2,
    Music = 3,
    Application = 4,
    Text = 5,
    Other = 6,
    ContextUnknown = 7,
}
impl ContentContext {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContentContext::Video => "VIDEO",
            ContentContext::Game => "GAME",
            ContentContext::Music => "MUSIC",
            ContentContext::Application => "APPLICATION",
            ContentContext::Text => "TEXT",
            ContentContext::Other => "OTHER",
            ContentContext::ContextUnknown => "CONTEXT_UNKNOWN",
        }
    }
}
/// OpenRTB 2.0: The following table lists the options for content quality.
/// These values are defined by the IAB -
/// <http://www.iab.net/media/file/long-form-video-final.pdf.>
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum ProductionQuality {
    QualityUnknown = 0,
    Professional = 1,
    Prosumer = 2,
    UserGenerated = 3,
}
impl ProductionQuality {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProductionQuality::QualityUnknown => "QUALITY_UNKNOWN",
            ProductionQuality::Professional => "PROFESSIONAL",
            ProductionQuality::Prosumer => "PROSUMER",
            ProductionQuality::UserGenerated => "USER_GENERATED",
        }
    }
}
/// OpenRTB 2.0: The following table lists the options to indicate how the
/// geographic information was determined.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum LocationType {
    /// GPS / Location Services.
    GpsLocation = 1,
    /// IP Geolocation.
    Ip = 2,
    /// User-provided, e.g. registration data.
    UserProvided = 3,
}
impl LocationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LocationType::GpsLocation => "GPS_LOCATION",
            LocationType::Ip => "IP",
            LocationType::UserProvided => "USER_PROVIDED",
        }
    }
}
/// OpenRTB 2.4: The following table lists the services and/or vendors used for
/// resolving IP addresses to geolocations.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum LocationService {
    Ip2location = 1,
    Neustar = 2,
    Maxmind = 3,
    Netacuity = 4,
}
impl LocationService {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LocationService::Ip2location => "IP2LOCATION",
            LocationService::Neustar => "NEUSTAR",
            LocationService::Maxmind => "MAXMIND",
            LocationService::Netacuity => "NETACUITY",
        }
    }
}
/// OpenRTB 2.0: The following table lists the type of device from which the
/// impression originated.
///
/// OpenRTB version 2.2 of the specification added distinct values for Mobile
/// and Tablet. It is recommended that any bidder adding support for 2.2
/// treat a value of 1 as an acceptable alias of 4 & 5.
///
/// This OpenRTB table has values derived from the IAB Quality Assurance
/// Guidelines (QAG). Practitioners should keep in sync with updates to the
/// QAG values as published on IAB.net.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum DeviceType {
    /// Mobile (OpenRTB 2.2+: obsolete, alias for PHONE or TABLET).
    Mobile = 1,
    /// Personal Computer.
    PersonalComputer = 2,
    /// Connected TV.
    ConnectedTv = 3,
    /// Phone.
    HighendPhone = 4,
    /// Tablet.
    Tablet = 5,
    /// Connected device.
    ConnectedDevice = 6,
    /// Set top box.
    SetTopBox = 7,
}
impl DeviceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeviceType::Mobile => "MOBILE",
            DeviceType::PersonalComputer => "PERSONAL_COMPUTER",
            DeviceType::ConnectedTv => "CONNECTED_TV",
            DeviceType::HighendPhone => "HIGHEND_PHONE",
            DeviceType::Tablet => "TABLET",
            DeviceType::ConnectedDevice => "CONNECTED_DEVICE",
            DeviceType::SetTopBox => "SET_TOP_BOX",
        }
    }
}
/// OpenRTB 2.1: The following table lists the options for the
/// video quality. These values are defined by the IAB -
/// <http://www.iab.net/media/file/long-form-video-final.pdf.>
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum CompanionType {
    Static = 1,
    Html = 2,
    CompanionIframe = 3,
}
impl CompanionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CompanionType::Static => "STATIC",
            CompanionType::Html => "HTML",
            CompanionType::CompanionIframe => "COMPANION_IFRAME",
        }
    }
}
/// OpenRTB 2.1: The following table lists the media ratings used in
/// describing content based on the QAG categorization.
/// Refer to <http://www.iab.net/ne_guidelines> for more information.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum QagMediaRating {
    AllAudiences = 1,
    EveryoneOver12 = 2,
    Mature = 3,
}
impl QagMediaRating {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            QagMediaRating::AllAudiences => "ALL_AUDIENCES",
            QagMediaRating::EveryoneOver12 => "EVERYONE_OVER_12",
            QagMediaRating::Mature => "MATURE",
        }
    }
}
/// OpenRTB 2.2: The following table lists the options for a bidder to signal
/// the exchange as to why it did not offer a bid for the impression.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum NoBidReason {
    UnknownError = 0,
    TechnicalError = 1,
    InvalidRequest = 2,
    KnownWebSpider = 3,
    SuspectedNonhumanTraffic = 4,
    CloudDatacenterProxyip = 5,
    UnsupportedDevice = 6,
    BlockedPublisher = 7,
    UnmatchedUser = 8,
    DailyReaderCap = 9,
    DailyDomainCap = 10,
}
impl NoBidReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NoBidReason::UnknownError => "UNKNOWN_ERROR",
            NoBidReason::TechnicalError => "TECHNICAL_ERROR",
            NoBidReason::InvalidRequest => "INVALID_REQUEST",
            NoBidReason::KnownWebSpider => "KNOWN_WEB_SPIDER",
            NoBidReason::SuspectedNonhumanTraffic => "SUSPECTED_NONHUMAN_TRAFFIC",
            NoBidReason::CloudDatacenterProxyip => "CLOUD_DATACENTER_PROXYIP",
            NoBidReason::UnsupportedDevice => "UNSUPPORTED_DEVICE",
            NoBidReason::BlockedPublisher => "BLOCKED_PUBLISHER",
            NoBidReason::UnmatchedUser => "UNMATCHED_USER",
            NoBidReason::DailyReaderCap => "DAILY_READER_CAP",
            NoBidReason::DailyDomainCap => "DAILY_DOMAIN_CAP",
        }
    }
}
/// OpenRTB 2.5: The following table lists the options for an exchange
/// to inform a bidder as to the reason why they did not win an impression.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum LossReason {
    BidWon = 0,
    InternalError = 1,
    ImpExpired = 2,
    InvalidBid = 3,
    InvalidDealId = 4,
    InvalidAuctionId = 5,
    InvalidAdomain = 6,
    MissingMarkup = 7,
    MissingCreativeId = 8,
    MissingPrice = 9,
    MissingMinCreativeApprovalData = 10,
    BidBelowFloor = 100,
    BidBelowDealFloor = 101,
    LostHigherBid = 102,
    LostPmpDeal = 103,
    SeatBlocked = 104,
    CreativeReasonUnknown = 200,
    CreativePending = 201,
    CreativeDisapproved = 202,
    CreativeSize = 203,
    CreativeFormat = 204,
    CreativeAdvertiserExclusion = 205,
    CreativeAppExclusion = 206,
    CreativeNotSecure = 207,
    CreativeLanguageExclusion = 208,
    CreativeCategoryExclusion = 209,
    CreativeAttributeExclusion = 210,
    CreativeAdtypeExclusion = 211,
    CreativeAnimationLong = 212,
    CreativeNotAllowedPmp = 213,
}
impl LossReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LossReason::BidWon => "BID_WON",
            LossReason::InternalError => "INTERNAL_ERROR",
            LossReason::ImpExpired => "IMP_EXPIRED",
            LossReason::InvalidBid => "INVALID_BID",
            LossReason::InvalidDealId => "INVALID_DEAL_ID",
            LossReason::InvalidAuctionId => "INVALID_AUCTION_ID",
            LossReason::InvalidAdomain => "INVALID_ADOMAIN",
            LossReason::MissingMarkup => "MISSING_MARKUP",
            LossReason::MissingCreativeId => "MISSING_CREATIVE_ID",
            LossReason::MissingPrice => "MISSING_PRICE",
            LossReason::MissingMinCreativeApprovalData => "MISSING_MIN_CREATIVE_APPROVAL_DATA",
            LossReason::BidBelowFloor => "BID_BELOW_FLOOR",
            LossReason::BidBelowDealFloor => "BID_BELOW_DEAL_FLOOR",
            LossReason::LostHigherBid => "LOST_HIGHER_BID",
            LossReason::LostPmpDeal => "LOST_PMP_DEAL",
            LossReason::SeatBlocked => "SEAT_BLOCKED",
            LossReason::CreativeReasonUnknown => "CREATIVE_REASON_UNKNOWN",
            LossReason::CreativePending => "CREATIVE_PENDING",
            LossReason::CreativeDisapproved => "CREATIVE_DISAPPROVED",
            LossReason::CreativeSize => "CREATIVE_SIZE",
            LossReason::CreativeFormat => "CREATIVE_FORMAT",
            LossReason::CreativeAdvertiserExclusion => "CREATIVE_ADVERTISER_EXCLUSION",
            LossReason::CreativeAppExclusion => "CREATIVE_APP_EXCLUSION",
            LossReason::CreativeNotSecure => "CREATIVE_NOT_SECURE",
            LossReason::CreativeLanguageExclusion => "CREATIVE_LANGUAGE_EXCLUSION",
            LossReason::CreativeCategoryExclusion => "CREATIVE_CATEGORY_EXCLUSION",
            LossReason::CreativeAttributeExclusion => "CREATIVE_ATTRIBUTE_EXCLUSION",
            LossReason::CreativeAdtypeExclusion => "CREATIVE_ADTYPE_EXCLUSION",
            LossReason::CreativeAnimationLong => "CREATIVE_ANIMATION_LONG",
            LossReason::CreativeNotAllowedPmp => "CREATIVE_NOT_ALLOWED_PMP",
        }
    }
}
/// OpenRTB 2.4: The following table lists the types of feeds,
/// typically for audio.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum FeedType {
    MusicService = 1,
    Broadcast = 2,
    Podcast = 3,
}
impl FeedType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FeedType::MusicService => "MUSIC_SERVICE",
            FeedType::Broadcast => "BROADCAST",
            FeedType::Podcast => "PODCAST",
        }
    }
}
/// OpenRTB 2.4: The following table lists the types of volume normalization
/// modes, typically for audio.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum VolumeNormalizationMode {
    None = 0,
    AverageVolume = 1,
    PeakVolume = 2,
    Loudness = 3,
    CustomVolume = 4,
}
impl VolumeNormalizationMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VolumeNormalizationMode::None => "NONE",
            VolumeNormalizationMode::AverageVolume => "AVERAGE_VOLUME",
            VolumeNormalizationMode::PeakVolume => "PEAK_VOLUME",
            VolumeNormalizationMode::Loudness => "LOUDNESS",
            VolumeNormalizationMode::CustomVolume => "CUSTOM_VOLUME",
        }
    }
}
// ***** OpenRTB Native enums **************************************************

/// DEPRECATED in OpenRTB Native 1.1, REMOVED in 1.2+.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum LayoutId {
    ContentWall = 1,
    AppWall = 2,
    NewsFeed = 3,
    ChatList = 4,
    Carousel = 5,
    ContentStream = 6,
    /// Exchange-specific values above 500.
    Grid = 7,
}
impl LayoutId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LayoutId::ContentWall => "CONTENT_WALL",
            LayoutId::AppWall => "APP_WALL",
            LayoutId::NewsFeed => "NEWS_FEED",
            LayoutId::ChatList => "CHAT_LIST",
            LayoutId::Carousel => "CAROUSEL",
            LayoutId::ContentStream => "CONTENT_STREAM",
            LayoutId::Grid => "GRID",
        }
    }
}
/// DEPRECATED in OpenRTB Native 1.1, REMOVED in 1.2+.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum AdUnitId {
    PaidSearchUnit = 1,
    RecommendationWidget = 2,
    PromotedListing = 3,
    IabInAdNative = 4,
    /// Exchange-specific values above 500.
    AdunitidCustom = 5,
}
impl AdUnitId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AdUnitId::PaidSearchUnit => "PAID_SEARCH_UNIT",
            AdUnitId::RecommendationWidget => "RECOMMENDATION_WIDGET",
            AdUnitId::PromotedListing => "PROMOTED_LISTING",
            AdUnitId::IabInAdNative => "IAB_IN_AD_NATIVE",
            AdUnitId::AdunitidCustom => "ADUNITID_CUSTOM",
        }
    }
}
/// OpenRTB Native 1.1: The context in which the ad appears - what type
/// of content is surrounding the ad on the page at a high level.
/// This maps directly to the new Deep Dive on In-Feed Ad Units.
/// This denotes the primary context, but does not imply other content
/// may not exist on the page - for example it's expected that most
/// content platforms have some social components, etc.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum ContextType {
    /// Content-centric context such as newsfeed, article, image gallery,
    /// video gallery, or similar.
    Content = 1,
    /// Social-centric context such as social network feed, email,
    /// chat, or similar.
    Social = 2,
    /// Product context such as product listings, details, recommendations,
    /// reviews, or similar.
    Product = 3,
}
impl ContextType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContextType::Content => "CONTENT",
            ContextType::Social => "SOCIAL",
            ContextType::Product => "PRODUCT",
        }
    }
}
/// OpenRTB Native 1.1: Next-level context in which the ad appears.
/// Again this reflects the primary context, and does not imply no presence
/// of other elements. For example, an article is likely to contain images
/// but is still first and foremost an article. SubType should only be
/// combined with the primary context type as indicated (ie for a context
/// type of 1, only context subtypes that start with 1 are valid).
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum ContextSubtype {
    ContentGeneralOrMixed = 10,
    ContentArticle = 11,
    ContentVideo = 12,
    ContentAudio = 13,
    ContentImage = 14,
    ContentUserGenerated = 15,
    SocialGeneral = 20,
    SocialEmail = 21,
    SocialChatIm = 22,
    ProductSelling = 30,
    ProductMarketplace = 31,
    ProductReview = 32,
}
impl ContextSubtype {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContextSubtype::ContentGeneralOrMixed => "CONTENT_GENERAL_OR_MIXED",
            ContextSubtype::ContentArticle => "CONTENT_ARTICLE",
            ContextSubtype::ContentVideo => "CONTENT_VIDEO",
            ContextSubtype::ContentAudio => "CONTENT_AUDIO",
            ContextSubtype::ContentImage => "CONTENT_IMAGE",
            ContextSubtype::ContentUserGenerated => "CONTENT_USER_GENERATED",
            ContextSubtype::SocialGeneral => "SOCIAL_GENERAL",
            ContextSubtype::SocialEmail => "SOCIAL_EMAIL",
            ContextSubtype::SocialChatIm => "SOCIAL_CHAT_IM",
            ContextSubtype::ProductSelling => "PRODUCT_SELLING",
            ContextSubtype::ProductMarketplace => "PRODUCT_MARKETPLACE",
            ContextSubtype::ProductReview => "PRODUCT_REVIEW",
        }
    }
}
/// OpenRTB Native 1.1: The FORMAT of the ad you are purchasing,
/// separate from the surrounding context.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum PlacementType {
    /// In the feed of content - for example as an item inside the organic
    /// feed/grid/listing/carousel.
    InFeed = 1,
    /// In the atomic unit of the content - IE in the article page
    /// or single image page.
    AtomicUnit = 2,
    /// Outside the core content - for example in the ads section on the
    /// right rail, as a banner-style placement near the content, etc.
    Outside = 3,
    /// Recommendation widget, most commonly presented below
    /// the article content.
    Recommendation = 4,
}
impl PlacementType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlacementType::InFeed => "IN_FEED",
            PlacementType::AtomicUnit => "ATOMIC_UNIT",
            PlacementType::Outside => "OUTSIDE",
            PlacementType::Recommendation => "RECOMMENDATION",
        }
    }
}
/// OpenRTB Native 1.0: Common asset element types of native advertising.
/// This list is non-exhaustive and intended to be extended by the buyers
/// and sellers as the format evolves. An implementing exchange may not
/// support all asset variants or introduce new ones unique to that system.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum DataAssetType {
    /// Sponsored By message where response should contain the brand name
    /// of the sponsor.
    /// Format: Text; Max length: 25 or longer.
    Sponsored = 1,
    /// Descriptive text associated with the product or service being advertised.
    /// Format: Text; Max length: 140 or longer.
    Desc = 2,
    /// Rating of the product being offered to the user.
    /// For example an app's rating in an app store from 0-5.
    /// Format: Number (1-5 digits) formatted as string.
    Rating = 3,
    /// Number of social ratings or "likes" of product being offered to the user.
    /// Format: Number formatted as string.
    Likes = 4,
    /// Number downloads/installs of this product.
    /// Format: Number formatted as string.
    Downloads = 5,
    /// Price for product / app / in-app purchase.
    /// Value should include currency symbol in localised format.
    /// Format: Number formatted as string.
    Price = 6,
    /// Sale price that can be used together with price to indicate a discounted
    /// price compared to a regular price. Value should include currency symbol
    /// in localised format.
    /// Format: Number formatted as string.
    Saleprice = 7,
    /// Phone number.
    /// Format: Formatted string.
    Phone = 8,
    /// Address.
    /// Format: Text.
    Address = 9,
    /// Additional descriptive text associated with the product or service
    /// being advertised.
    /// Format: Text.
    Desc2 = 10,
    /// Display URL for the text ad.
    /// Format: Text.
    Displayurl = 11,
    /// Text describing a 'call to action' button for the destination URL.
    /// Format: Text.
    Ctatext = 12,
}
impl DataAssetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataAssetType::Sponsored => "SPONSORED",
            DataAssetType::Desc => "DESC",
            DataAssetType::Rating => "RATING",
            DataAssetType::Likes => "LIKES",
            DataAssetType::Downloads => "DOWNLOADS",
            DataAssetType::Price => "PRICE",
            DataAssetType::Saleprice => "SALEPRICE",
            DataAssetType::Phone => "PHONE",
            DataAssetType::Address => "ADDRESS",
            DataAssetType::Desc2 => "DESC2",
            DataAssetType::Displayurl => "DISPLAYURL",
            DataAssetType::Ctatext => "CTATEXT",
        }
    }
}
/// OpenRTB Native 1.0: Common image asset element types of native advertising
/// at the time of writing this spec. This list is non-exhaustive and intended
/// to be extended by the buyers and sellers as the format evolves.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum ImageAssetType {
    /// Icon image.
    /// Max height: at least 50; Aspect ratio: 1:1.
    Icon = 1,
    /// DEPRECATED in OpenRTB Native 1.2+. Prefer type <code>ICON</code>.
    /// Logo image for the brand/app.
    Logo = 2,
    /// Large image preview for the ad.
    /// At least one of 2 size variants required:
    /// Small Variant: max height: 200+, max width: 200+, 267, or 382,
    ///                 aspect ratio: 1:1, 4:3, or 1.91:1.
    /// Large Variant: max height: 627+, max width: 627+, 836, or 1198,
    ///                 aspect ratio: 1:1, 4:3, or 1.91:1.
    Main = 3,
}
impl ImageAssetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ImageAssetType::Icon => "ICON",
            ImageAssetType::Logo => "LOGO",
            ImageAssetType::Main => "MAIN",
        }
    }
}
/// OpenRTB Native 1.2.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum EventType {
    /// Impression
    Impression = 1,
    /// Visible impression using MRC definition at 50% in view for 1 second.
    ViewableMrc50 = 2,
    /// 100% in view for 1 second (ie GroupM standard).
    ViewableMrc100 = 3,
    /// Visible impression for video using MRC definition at 50% in view
    /// for 2 seconds.
    ViewableVideo50 = 4,
}
impl EventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventType::Impression => "IMPRESSION",
            EventType::ViewableMrc50 => "VIEWABLE_MRC_50",
            EventType::ViewableMrc100 => "VIEWABLE_MRC_100",
            EventType::ViewableVideo50 => "VIEWABLE_VIDEO_50",
        }
    }
}
/// OpenRTB Native 1.2.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(i32)]
pub enum EventTrackingMethod {
    /// Image-pixel tracking - URL provided will be insterted as a 1x1 pixel at the
    /// time of the event.
    Img = 1,
    /// Javascript-based tracking - URL provided will be insterted as a js tag at
    /// the time of the event.
    Js = 2,
}
impl EventTrackingMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventTrackingMethod::Img => "IMG",
            EventTrackingMethod::Js => "JS",
        }
    }
}

pub mod bool {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Clone, PartialEq)]
    pub enum Bool {
        True,
        False,
    }

    impl Serialize for Bool {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match *self {
                Bool::True => serializer.serialize_u8(1),
                Bool::False => serializer.serialize_u8(0),
            }
        }
    }

    impl<'de> Deserialize<'de> for Bool {
        fn deserialize<D>(deserializer: D) -> Result<Bool, D::Error>
        where
            D: Deserializer<'de>,
        {
            let n = u8::deserialize(deserializer)?;
            match n {
                0 => Ok(Bool::False),
                _ => Ok(Bool::True),
                // x => Err(format!("Cant' parse {} to Bool", x)),
            }
        }
    }
}
