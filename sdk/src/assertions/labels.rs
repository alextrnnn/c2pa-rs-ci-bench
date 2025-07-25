// Copyright 2022 Adobe. All rights reserved.
// This file is licensed to you under the Apache License,
// Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
// or the MIT license (http://opensource.org/licenses/MIT),
// at your option.

// Unless required by applicable law or agreed to in writing,
// this software is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR REPRESENTATIONS OF ANY KIND, either express or
// implied. See the LICENSE-MIT and LICENSE-APACHE files for the
// specific language governing permissions and limitations under
// each license.

#![deny(missing_docs)]

//! Labels for assertion types as defined in C2PA 1.0/2.x Specification.
//!
//! These constants do not include version suffixes.
//!
//! See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_c2pa_standard_assertions>.

/// Label prefix for a claim assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_overview_4>.
pub const CLAIM: &str = "c2pa.claim";

/// Label prefix for an assertion metadata assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_metadata_about_assertions>.
pub const ASSERTION_METADATA: &str = "c2pa.assertion.metadata";

/// Label prefix for a data hash assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_data_hash>.
pub const DATA_HASH: &str = "c2pa.hash.data";

/// Label prefix for a box hash assertion.
///
/// See <https://c2pa.org/specifications/specifications/1.3/specs/C2PA_Specification.html#_general_boxes_hash>.
pub const BOX_HASH: &str = "c2pa.hash.boxes";

/// Label prefix for a BMFF-based hash assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_bmff_based_hash>.
pub const BMFF_HASH: &str = "c2pa.hash.bmff";

/// Label prefix for a collection hash assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.1/specs/C2PA_Specification.html#_collection_data_hash>.
pub const COLLECTION_HASH: &str = "c2pa.hash.collection.data";

/// Label prefix for a soft binding assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_soft_binding_2>.
pub const SOFT_BINDING: &str = "c2pa.soft-binding";

/// Label prefix for a cloud data assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_cloud_data>.
pub const CLOUD_DATA: &str = "c2pa.cloud-data";

/// Label prefix for a thumbnail assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_thumbnail>.
pub const THUMBNAIL: &str = "c2pa.thumbnail";

/// Label prefix for a claim thumbnail assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_thumbnail>.
pub const CLAIM_THUMBNAIL: &str = "c2pa.thumbnail.claim";

/// Label prefix for an ingredient thumbnail assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_thumbnail>.
pub const INGREDIENT_THUMBNAIL: &str = "c2pa.thumbnail.ingredient";

/// Label prefix for a JPEG claim thumbnail assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_thumbnail>.
pub const JPEG_CLAIM_THUMBNAIL: &str = "c2pa.thumbnail.claim.jpeg";

/// Label prefix for a JPEG ingredient thumbnail assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_thumbnail>.
pub const JPEG_INGREDIENT_THUMBNAIL: &str = "c2pa.thumbnail.ingredient.jpeg";

/// Label prefix for a PNG claim thumbnail assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_thumbnail>.
pub const PNG_CLAIM_THUMBNAIL: &str = "c2pa.thumbnail.claim.png";

/// Label prefix for a PNG ingredient thumbnail assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_thumbnail>.
pub const PNG_INGREDIENT_THUMBNAIL: &str = "c2pa.thumbnail.ingredient.png";

/// Label prefix for a SVG claim thumbnail assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_thumbnail>.
pub const SVG_CLAIM_THUMBNAIL: &str = "c2pa.thumbnail.claim.svg";

/// Label prefix for a SVG ingredient thumbnail assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_thumbnail>.
pub const SVG_INGREDIENT_THUMBNAIL: &str = "c2pa.thumbnail.ingredient.svg";

/// Label prefix for an actions assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_actions>.
pub const ACTIONS: &str = "c2pa.actions";

/// Label prefix for an ingredient assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_ingredient>.
pub const INGREDIENT: &str = "c2pa.ingredient";

/// Label prefix for a depthmap assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_depthmap>.
pub const DEPTHMAP: &str = "c2pa.depthmap";

/// Label prefix for a asset type assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.1/specs/C2PA_Specification.html#_asset_type>.
pub const ASSET_TYPE: &str = "c2pa.asset-type";

/// Label prefix for a embedded data assertion.
///
/// See <https://spec.c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_embedded_data>.
pub const EMBEDDED_DATA: &str = "c2pa.embedded-data";

/// Label prefix for a Icon assertion.
///
/// See <https://spec.c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_generator_info_map>.
pub const ICON: &str = "c2pa.icon";

/// Label prefix for a GDepth assertion.
/// Label prefix for a GDepth depthmap assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_gdepth_depthmap>.
pub const DEPTHMAP_GDEPTH: &str = "c2pa.depthmap.GDepth";

/// Label prefix for an EXIF information assertion.
/// Hidden because it's now part of standard metadata assertions.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_exif_information>.
#[doc(hidden)]
pub const EXIF: &str = "stds.exif";

/// Label prefix for an IPTC photo metadata assertion.
/// Hidden because it's now part of standard metadata assertions.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_iptc_photo_metadata>.
#[doc(hidden)]
pub const IPTC_PHOTO_METADATA: &str = "stds.iptc.photo-metadata";

/// Label prefix for any assertion based on a schema.org grammar.
/// Hidden because it's now part of standard metadata assertions.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_use_of_schema_org>.
#[doc(hidden)]
pub const SCHEMA_ORG: &str = "schema.org";

/// Label prefix for a claim review assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_claim_review>.
pub const CLAIM_REVIEW: &str = "stds.schema-org.ClaimReview";

/// Label prefix for a creative work assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_creative_work>.
pub const CREATIVE_WORK: &str = "stds.schema-org.CreativeWork";

/// Label prefix for a timestamp assertion.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#timestamp_assertion>.
pub const TIMESTAMP: &str = "c2pa.time-stamp";

// Assertion store label
pub(crate) const ASSERTION_STORE: &str = "c2pa.assertions";

// Databoxes label
pub(crate) const DATABOX_STORE: &str = "c2pa.databoxes";

/// Label prefix for asset reference assertion.
///
/// See <https://spec.c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_asset_reference>
pub const ASSET_REFERENCE: &str = "c2pa.asset-ref";

/// Return the version suffix from an assertion label if it exists.
///
/// When an assertion's schema is changed in a backwards-compatible manner,
/// the label would consist of an incremented version number, for example
/// moving from `c2pa.ingredient` to `c2pa.ingredient.v2`.
///
/// If such a suffix exists (`.v(integer)`), return that; otherwise,
/// return `None`.
///
/// See <https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html#_versioning>.
///
/// # Examples
///
/// ```
/// use c2pa::assertions::labels;
///
/// assert_eq!(labels::version("c2pa.ingredient"), None);
/// assert_eq!(labels::version("c2pa.ingredient.v2"), Some(2));
/// assert_eq!(labels::version("c2pa.ingredient.V2"), None);
/// assert_eq!(labels::version("c2pa.ingredient.x2"), None);
/// assert_eq!(labels::version("c2pa.ingredient.v-2"), None);
/// ```
pub fn version(label: &str) -> Option<usize> {
    let components: Vec<&str> = label.split('.').collect();
    if let Some(last) = components.last() {
        if last.len() > 1 {
            let (ver, ver_inst_str) = last.split_at(1);
            if ver == "v" {
                if let Ok(ver) = ver_inst_str.parse::<usize>() {
                    return Some(ver);
                }
            }
        }
    }

    None
}

/// Set the version of a label.
/// If the version is 1, the original label is returned.
/// Otherwise, the label is suffixed with the version number.
/// This expects the label to not already have a version suffix.
pub fn set_version(base_label: &str, version: usize) -> String {
    if version == 1 {
        // c2pa does not include v1 labels
        base_label.to_string()
    } else {
        format!("{base_label}.v{version}")
    }
}

/// Given a thumbnail label prefix such as `CLAIM_THUMBNAIL` and a file
/// format (such as `png`), create a suitable label for an assertion.
///
/// # Examples
///
/// ```
/// use c2pa::assertions::labels;
///
/// assert_eq!(
///     labels::add_thumbnail_format(labels::CLAIM_THUMBNAIL, "image/jpeg"),
///     labels::JPEG_CLAIM_THUMBNAIL
/// );
///
/// assert_eq!(
///     labels::add_thumbnail_format(labels::INGREDIENT_THUMBNAIL, "image/png"),
///     labels::PNG_INGREDIENT_THUMBNAIL
/// );
/// ```
pub fn add_thumbnail_format(label: &str, format: &str) -> String {
    match format {
        "image/jpeg" | "jpeg" | "jpg" => format!("{label}.jpeg"),
        "image/png" | "png" => format!("{label}.png"),
        "image/svg+xml" | "svg" => format!("{label}.svg"),
        _ => {
            let p: Vec<&str> = format.split('/').collect();
            if p.len() == 2 && p[0] == "image" {
                format!("{}/{}", label, p[1]) // try to parse other image types
            } else {
                format!("{label}/{format}")
            }
        }
    }
}
