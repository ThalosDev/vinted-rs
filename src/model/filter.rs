use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::queries::Host;
#[cfg(feature = "redis")]
use redis_macros::{FromRedisValue, ToRedisArgs};

/// Provides functionality related to filtering by brand.
pub mod brand;
/// Provides functionality related to filtering by category.
pub mod category;
/// Provides functionality for retrieving the category tree.
pub mod category_tree;
/// Provides functionality related to filtering by color.
pub mod colors;
/// Provides functionality related to filtering by country.
pub mod country;
/// Provides functionality related to filtering by material.
pub mod material;
/// Provides functionality related to filtering by size.
pub mod size;

/// Represents a filter for querying items.
///
/// Trait Implementations:
/// - `TypedBuilder`: Implements the builder pattern for constructing a `Filter` instance.
///
///
///You can create a filter with multiple constraints.
///
///### Example
///
///```rust
/// use vinted_rs::Filter;
///
/// let filter: Filter = Filter::builder()
///      .catalog_ids(Some(String::from("4,16")))
///      .brand_ids(Some(String::from("14,53")))
///      .build();
///
/// // Women shoes and Women clothes that are only from brands Adidas and Nike.
///
///
///```
///
///
/// `price_from` filter should be always <= `price_to` , otherwise Vinted will not find anything
///
#[derive(TypedBuilder, Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    ///The search text to filter items by.
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///let filter: Filter = Filter::builder().search_text(Some(String::from("shoes"))).build();
    ///```
    ///
    #[builder(default)]
    pub search_text: Option<String>,
    ///
    ///
    ///
    ///
    ///The catalog IDs to filter items by. Must be formatted as `^[\d+,]*\d+$` regex.
    ///
    ///
    ///If not formated with the specified regex, undefined behaviour. (Input will not be checked).
    ///
    ///
    ///Try it in [Regex 101](https://regex101.com/r/u8ZEpv/1)
    ///
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().catalog_ids(Some(String::from("4,16"))).build();
    /// // Where 4 and 16 are catalog_ids from Vinted
    /// // 4 is catalog_id for Women clothes
    /// // 16 is catalog_id for Women Shoes
    ///```
    #[builder(default)]
    pub catalog_ids: Option<String>,
    ///The color IDs to filter items by. Must be formatted as `^[\d+,]*\d+$` regex.
    ///
    ///If not formated with the specified regex, undefined behaviour. (Input will not be checked).
    ///
    ///
    ///Try it in [Regex 101](https://regex101.com/r/u8ZEpv/1)
    ///
    ///
    /// **Note:** Color names are only avalible in French in our database at the moment
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().color_ids(Some(String::from("1,5"))).build();
    /// // Where 1 and 5 are color_ids from Vinted
    /// // 1 is color_id for Black
    /// // 5 is color_id for Pink
    ///```
    #[builder(default)]
    pub color_ids: Option<String>,
    /// The brand IDs to filter items by. Must be formatted as `^[\d+,]*\d+$` regex.
    ///
    ///If not formated with the specified regex, undefined behaviour. (Input will not be checked).
    ///
    ///
    ///Try it in [Regex 101](https://regex101.com/r/u8ZEpv/1)
    ///
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().brand_ids(Some(String::from("14,53"))).build();
    /// // Where 14 and 53 are brand_ids from Vinted
    /// // 14 is brand_id for Adidas
    /// // 53 is brand_id for Nike
    ///```
    #[builder(default)]
    pub brand_ids: Option<String>,
    /// The country IDs to filter items by. Must be formatted as `^[\d+,]*\d+$` regex.
    ///
    ///If not formated with the specified regex, undefined behaviour. (Input will not be checked).
    ///
    ///
    ///Try it in [Regex 101](https://regex101.com/r/u8ZEpv/1)
    ///
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().countries_ids(Some(String::from("7,16"))).build();
    /// // Where 7 and 16 are country_ids from Vinted
    /// // 7 is country_id for Spain
    /// // 16 is country_id for France
    ///```
    #[builder(default)]
    pub countries_ids: Option<String>,
    /// The size IDs to filter items by. Must be formatted as `^[\d+,]*\d+$` regex.
    ///
    ///If not formated with the specified regex, undefined behaviour. (Input will not be checked).
    ///
    ///
    ///Try it in [Regex 101](https://regex101.com/r/u8ZEpv/1)
    ///
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().size_ids(Some(String::from("1226,102"))).build();
    /// // Where 7 and 16 are country_ids from Vinted
    /// // 1226 is size_id for XXXS / 30 / 2
    /// // 102 is size_id for XXS / 32 / 4
    ///```
    #[builder(default)]
    pub material_ids: Option<String>,
    /// The material IDs to filter items by. Must be formatted as `^[\d+,]*\d+$` regex.
    ///
    ///If not formated with the specified regex, undefined behaviour. (Input will not be checked).
    ///
    ///
    ///Try it in [Regex 101](https://regex101.com/r/u8ZEpv/1)
    ///
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().material_ids(Some(String::from("44,102"))).build();
    /// // Where 7 and 16 are country_ids from Vinted
    /// // 44 is material_id for coton
    /// // 49 is material_id for silk
    ///```
    #[builder(default)]
    pub size_ids: Option<String>,
    /// The article statuses to filter items by.
    ///
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    /// use vinted_rs::model::filter::ArticleStatus;
    ///
    /// let filter: Filter = Filter::builder().article_status(Some(vec![ArticleStatus::NewTags,ArticleStatus::NewNoTags])).build();
    ///```
    #[builder(default)]
    pub article_status: Option<Vec<ArticleStatus>>,
    /// The sort order for the retrieved items.
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    /// use vinted_rs::model::filter::SortBy;

    ///
    /// let filter: Filter = Filter::builder().sort_by(Some(SortBy::PriceAscendant)).build();
    ///```
    #[builder(default)]
    pub sort_by: Option<SortBy>,
    /// The minimum price of the article
    ///
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().price_from(Some(10f32)).build();
    ///```
    ///
    #[builder(default)]
    pub price_from: Option<f32>,
    /// The max price of the article
    ///
    ///### Example
    ///```rust
    /// use vinted_rs::Filter;
    ///
    ///
    /// let filter: Filter = Filter::builder().price_from(Some(20f32)).build();
    ///```
    ///
    #[builder(default)]
    pub price_to: Option<f32>,
}
/**
Represents the currency for filtering items

*/
// GBP => Host uk
// EUR => Pais de Europa sera el default
// USD => Host com
// CSK => Host cz
// PLN => Host pl
// SEK => Host se
// RON => Host ro
// HUF => Host hu
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "redis", derive(FromRedisValue, ToRedisArgs))]
pub enum Currency {
    /// Euro
    EUR,
    /// US Dollar
    USD,
    /// Great Britain Pound
    GBP,
    /// Czech korona
    CZK,
    /// Polish złoty
    PLN,
    /// Swedish krona
    SEK,
    /// Romanian leu
    RON,
    /// Hungarian forint
    HUF,
}

impl From<Currency> for Host {
    fn from(currency: Currency) -> Self {
        match currency {
            Currency::USD => Host::Com,
            Currency::GBP => Host::Uk,
            Currency::CZK => Host::Cz,
            Currency::PLN => Host::Pl,
            Currency::SEK => Host::Se,
            Currency::RON => Host::Ro,
            Currency::HUF => Host::Hu,
            Currency::EUR => Host::random_euro_host(),
        }
    }
}

impl From<Currency> for &str {
    fn from(currency: Currency) -> Self {
        match currency {
            Currency::USD => "USD",
            Currency::GBP => "GBP",
            Currency::CZK => "CZK",
            Currency::PLN => "PLN",
            Currency::SEK => "SEK",
            Currency::RON => "RON",
            Currency::HUF => "HUF",
            Currency::EUR => "EUR",
        }
    }
}

/**
Represents the article status for filtering items.

*/
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "redis", derive(FromRedisValue, ToRedisArgs))]
pub enum ArticleStatus {
    /// The article status for new items with tags.
    NewTags,
    /// The article status for new items without tags.
    NewNoTags,
    /// The article status for items in very good condition.
    VeryGood,
    /// Good`: The article status for items in good condition.
    Good,
    /// The article status for items in satisfactory condition.
    Satisfactory,
}

impl From<&ArticleStatus> for &str {
    /// From `&ArticleStatus` to `&str`
    fn from(status: &ArticleStatus) -> Self {
        match *status {
            ArticleStatus::NewTags => "6",
            ArticleStatus::NewNoTags => "1",
            ArticleStatus::VeryGood => "2",
            ArticleStatus::Good => "3",
            ArticleStatus::Satisfactory => "4",
        }
    }
}
/**
Represents the sort order for the retrieved items.
*/

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "redis", derive(FromRedisValue, ToRedisArgs))]
pub enum SortBy {
    /// Sort items by relevance.
    Relevance,
    /// Sort items by price in descending order.
    PriceDescendant,
    /// Sort items by price in ascending order.
    PriceAscendant,
    /// Sort items by newest first.
    NewestFirst,
}

impl From<&SortBy> for &str {
    /// From `&SortBy` to `&str`
    fn from(sort: &SortBy) -> Self {
        match *sort {
            SortBy::Relevance => "relevance",
            SortBy::PriceDescendant => "price_high_to_low",
            SortBy::PriceAscendant => "price_low_to_high",
            SortBy::NewestFirst => "newest_first",
        }
    }
}
