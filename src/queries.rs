/*! The `queries` module provides functionality for querying the Vinted API.

 This module contains the `VintedWrapper` struct, which represents the main wrapper for interacting with the Vinted API. It provides methods for retrieving items based on filters and handling cookies.

 # Examples

 ```rust
 use crate::vinted_rs::model::items::Items;
 use crate::vinted_rs::model::filter::Filter;
 use crate::vinted_rs::queries::VintedWrapper;
 use crate::vinted_rs::queries::VintedWrapperError;
 use fang::FangError;

 async fn get_items_example() {
     let wrapper = VintedWrapper::new();
     let filter: Filter = Filter::builder().search_text(String::from("shoes")).build();
     let num = 10;

     match wrapper.get_items(&filter, num).await {
         Ok(items) => {
             println!("Retrieved {} items: {:?}", items.items.len(), items);
             assert_eq!(items.items.len(), 10);
         }
         Err(err) => match err {
             VintedWrapperError::ItemNumberError => unreachable!(),
             VintedWrapperError::CookiesError(_) => (),
         },
     }
 }
 ```
*/
use fang::FangError;
use lazy_static::lazy_static;
use log::debug;
use log::error;
use rand::Rng;
use reqwest::Client;
use reqwest::Proxy;
use reqwest::Response;
use reqwest::StatusCode;
use reqwest_cookie_store::CookieStore;
use reqwest_cookie_store::CookieStoreMutex;
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use thiserror::Error;

use crate::model::filter::Currency;
use crate::model::filter::Filter;
use crate::model::item::AdvancedItem;
use crate::model::items::AdvancedItems;
use crate::model::items::Items;

#[derive(Error, Debug)]
pub enum CookieError {
    #[error(transparent)]
    ReqWestError(#[from] reqwest::Error),
    #[error("Error to get cookies")]
    GetCookiesError((StatusCode, String, String)),
}

#[derive(Error, Debug)]
pub enum VintedWrapperError {
    #[error(transparent)]
    ReqWestError(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeError(#[from] SerdeJSONError),
    #[error(transparent)]
    CookiesError(#[from] CookieError),
    #[error("Number of items must be non-zero value")]
    ItemNumberError,
    #[error("Could not get deatiled info for item `{2}` with code: {0}")]
    ItemError(StatusCode, Option<i32>, String),
}

#[derive(Debug, Error)]
pub struct SerdeJSONError {
    raw_json: String,
    serde_error: serde_json::Error,
}

impl SerdeJSONError {
    fn new(raw_json: String, serde_error: serde_json::Error) -> Self {
        SerdeJSONError {
            raw_json,
            serde_error,
        }
    }
}

impl fmt::Display for SerdeJSONError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "JSON: {}, SerdeError: {}",
            self.raw_json, self.serde_error
        )
    }
}

impl From<VintedWrapperError> for FangError {
    fn from(value: VintedWrapperError) -> FangError {
        FangError {
            description: format!("{value:?}"),
        }
    }
}

const DOMAINS: [&str; 17] = [
    "fr", "es", "lu", "nl", "lt", "de", "at", "it", "co.uk", "pt", "com", "cz", "sk", "pl", "se",
    "ro", "hu",
    //"be",
];

const DEFAULT_USER_AGENT: &str = "PostmanRuntime/7.32.3";

#[derive(Debug, Clone)]
pub enum Host {
    Fr,
    //Be,
    Es,
    Lu,
    Nl,
    Lt,
    De,
    At,
    It,
    Uk,
    Pt,
    Com,
    Cz,
    Sk,
    Pl,
    Se,
    Ro,
    Hu,
}

impl Host {
    /// Returns true if a Host has the Euro as the currency
    pub fn is_euro_host(&self) -> bool {
        !matches!(
            self,
            Host::Com | Host::Uk | Host::Cz | Host::Pl | Host::Se | Host::Ro | Host::Hu
        )
    }

    /// Returns a Host that has Euro as currency
    pub fn random_euro_host() -> Self {
        let domains_euro: Vec<Host> = DOMAINS
            .iter()
            .map(|domain_str| (*domain_str).into())
            .filter(|domain: &Host| domain.is_euro_host())
            .collect();

        let random_index = rand::thread_rng().gen_range(0..domains_euro.len());

        domains_euro[random_index].clone()
    }
}

impl From<&str> for Host {
    fn from(string: &str) -> Self {
        match string {
            "fr" => Host::Fr,
            "es" => Host::Es,
            "lu" => Host::Lu,
            "nl" => Host::Nl,
            "lt" => Host::Lt,
            "de" => Host::De,
            "at" => Host::At,
            "it" => Host::It,
            "co.uk" => Host::Uk,
            "pt" => Host::Pt,
            "com" => Host::Com,
            "cz" => Host::Cz,
            "sk" => Host::Sk,
            "pl" => Host::Pl,
            "se" => Host::Se,
            "ro" => Host::Ro,
            "hu" => Host::Hu,
            //"be" => Host::Be,
            _ => panic!("Not valid host"),
        }
    }
}

impl From<Host> for &str {
    fn from(val: Host) -> Self {
        match val {
            Host::Fr => DOMAINS[0],
            Host::Es => DOMAINS[1],
            Host::Lu => DOMAINS[2],
            Host::Nl => DOMAINS[3],
            Host::Lt => DOMAINS[4],
            Host::De => DOMAINS[5],
            Host::At => DOMAINS[6],
            Host::It => DOMAINS[7],
            Host::Uk => DOMAINS[8],
            Host::Pt => DOMAINS[9],
            Host::Com => DOMAINS[10],
            Host::Cz => DOMAINS[11],
            Host::Sk => DOMAINS[12],
            Host::Pl => DOMAINS[13],
            Host::Se => DOMAINS[14],
            Host::Ro => DOMAINS[15],
            Host::Hu => DOMAINS[16],
            //Host::Be => DOMAINS[17],
        }
    }
}

/// Returns a random Vinted host domain.
///
/// The `random_host` function selects a random Vinted host domain from a predefined list of domains.
///
/// # Returns
///
/// A string reference containing the randomly selected Vinted host domain.
///
/// # Examples
///
/// ```rust
/// use vinted_rs::queries::random_host;
/// let host = random_host();
/// println!("Random host: {}", host);
/// ```
pub fn random_host<'a>() -> &'a str {
    let random_index = rand::thread_rng().gen_range(0..DOMAINS.len());
    DOMAINS[random_index]
}

lazy_static! {
    pub static ref COOKIE_STORE: Arc<CookieStoreMutex> = {
        let cookie_store = CookieStore::new(None);
        let cookie_store = CookieStoreMutex::new(cookie_store);
        Arc::new(cookie_store)
    };
}

async fn get_client(user_agent: Option<&str>, proxy: Option<Proxy>) -> Client {
    if let Some(proxy) = proxy {
        create_client_proxy(user_agent, proxy)
    } else {
        create_client(user_agent)
    }
}

fn create_client_proxy(user_agent: Option<&str>, proxy: Proxy) -> Client {
    reqwest::ClientBuilder::new()
        .user_agent(user_agent.unwrap_or(DEFAULT_USER_AGENT))
        .proxy(proxy)
        .cookie_provider(COOKIE_STORE.clone())
        .build()
        .unwrap()
}

fn create_client(user_agent: Option<&str>) -> Client {
    reqwest::ClientBuilder::new()
        .user_agent(user_agent.unwrap_or(DEFAULT_USER_AGENT))
        .cookie_provider(COOKIE_STORE.clone())
        .build()
        .unwrap()
}
/// This will allow you to operate with multiple hosts using just one struct
#[derive(Debug, Clone)]
pub struct VintedWrappers<'a> {
    wrappers: Vec<VintedWrapper<'a>>,
    pub len: usize,
}

impl VintedWrappers<'_> {
    pub fn new_with_hosts(hosts: Vec<Host>) -> Self {
        let len = hosts.len();

        let wrappers = hosts
            .into_iter()
            .map(VintedWrapper::new_with_host)
            .collect();

        VintedWrappers { wrappers, len }
    }

    pub fn all_wrappers() -> Self {
        let hosts = vec![
            Host::Es,
            Host::Fr,
            Host::Lu,
            Host::Pt,
            Host::It,
            Host::Nl,
            Host::Lt,
            Host::De,
            Host::At,
            Host::Uk,
            Host::Com,
            Host::Cz,
            Host::Sk,
            Host::Pl,
            Host::Se,
            Host::Ro,
            Host::Hu,
        ];
        VintedWrappers::new_with_hosts(hosts)
    }

    pub fn get_wrapper(&self, index: usize) -> &VintedWrapper<'_> {
        &self.wrappers[index]
    }

    pub async fn lineal_fetch(
        &mut self,
        filters: &Filter,
        num: u32,
        current: usize,
        user_agent: Option<&str>,
        proxy_cookies: Option<Proxy>,
        proxy_fetch: Option<Proxy>,
    ) -> Result<Items, VintedWrapperError> {
        let vinted_wrapper = &self.wrappers[current];

        vinted_wrapper
            .get_items(filters, num, user_agent, proxy_cookies, proxy_fetch)
            .await
    }

    pub async fn lineal_to_advance_items(
        &mut self,
        item_id: i64,
        current: usize,
        user_agent: Option<&str>,
        proxy_cookies: Option<Proxy>,
        proxy_fetch: Option<Proxy>,
    ) -> Result<AdvancedItem, VintedWrapperError> {
        let vinted_wrapper = &self.wrappers[current];

        vinted_wrapper
            .get_advanced_item(item_id, user_agent, proxy_cookies, proxy_fetch)
            .await
    }
}

impl Default for VintedWrappers<'_> {
    fn default() -> Self {
        let hosts = vec![Host::Es, Host::Fr, Host::Lu, Host::Pt, Host::It, Host::Nl];
        VintedWrappers::new_with_hosts(hosts)
    }
}

/// Represents the main wrapper for interacting with the Vinted API.
///
/// The `VintedWrapper` struct provides methods for retrieving items based on filters and handling cookies.
///
#[derive(Debug, Clone)]
pub struct VintedWrapper<'a> {
    id: usize,
    host: &'a str,
}

static WRAPPER_ID: AtomicUsize = AtomicUsize::new(0);

impl Default for VintedWrapper<'_> {
    fn default() -> Self {
        Self::new_with_host(Host::Es)
    }
}

impl VintedWrapper<'_> {
    /// Creates a new `VintedWrapper` with a random host.
    ///
    /// The `new` function creates a new `VintedWrapper` instance with a random host domain. It initializes the cookie store and client for making requests to the Vinted API.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vinted_rs::VintedWrapper;
    /// let wrapper = VintedWrapper::new();
    /// ```
    pub fn new() -> Self {
        let id = WRAPPER_ID.fetch_add(1, Ordering::SeqCst);

        VintedWrapper {
            host: random_host(),
            id,
        }
    }
    /// Creates a new `VintedWrapper` with the specified host.
    ///
    /// The `new_with_host` function creates a new `VintedWrapper` instance with the specified host domain. It initializes the cookie store and client for making requests to the Vinted API.
    ///
    /// # Arguments
    ///
    /// * `host` - The host domain to use for the wrapper.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vinted_rs::VintedWrapper;
    /// use vinted_rs::queries::Host;
    /// let wrapper = VintedWrapper::new_with_host(Host::Fr);
    /// ```
    pub fn new_with_host(host: Host) -> Self {
        let id = WRAPPER_ID.fetch_add(1, Ordering::SeqCst);

        VintedWrapper {
            host: host.into(),
            id,
        }
    }
    /// Creates a new `VintedWrapper` with the specified currency.
    ///
    /// The `new_with_currency` function creates a new `VintedWrapper` instance with the specified currency. It initializes the cookie store and client for making requests to the Vinted API.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency is translated to a Host that will be used in the wrapper.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vinted_rs::VintedWrapper;
    /// use vinted_rs::model::filter::Currency;
    /// let wrapper = VintedWrapper::new_with_currency(Currency::EUR);
    /// ```
    pub fn new_with_currency(currency: Currency) -> Self {
        VintedWrapper::new_with_host(currency.into())
    }
    /// Returns the current host domain.
    ///
    /// The `get_host` method returns the current host domain used by the `VintedWrapper` instance.
    ///
    /// # Returns
    ///
    /// A string reference containing the current host domain.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vinted_rs::VintedWrapper;
    /// let wrapper = VintedWrapper::new();
    /// let host = wrapper.get_host();
    /// println!("Current host: {}", host);
    /// ```
    pub fn get_host(&self) -> &str {
        self.host
    }

    pub fn get_id(&self) -> &usize {
        &self.id
    }
    /// After changing host is always necessary to refresh cookies
    pub fn set_new_random_host(&mut self) {
        self.host = random_host();
    }

    /// After changing host is always necessary to refresh cookies
    pub fn set_new_host(&mut self, host: Host) {
        self.host = host.into();
    }

    pub fn set_host_by_currency(&mut self, currency: Currency) {
        let host: Host = currency.into();

        let actual_host: Host = self.host.into();

        if !host.is_euro_host() || !actual_host.is_euro_host() {
            let host_str: &str = host.into();

            self.host = host_str;
        }
    }
    /// Refreshes the cookies for the Vinted API.
    ///
    /// The `refresh_cookies` method clears the existing cookies, sends a request to refresh the cookies from the Vinted API, and retrieves the updated cookies.
    ///
    /// # Returns
    ///
    /// A `Result` enum indicating the success or failure of refreshing the cookies. An `Ok` value indicates that the cookies were successfully refreshed, while an `Err` value contains a `CookieError` indicating the specific error encountered during the process.
    ///
    /// # Errors
    ///
    /// The method can return a `CookieError` if there was an error in retrieving the cookies. The possible error variants are:
    ///
    /// - `ReqWestError`: Represents an error from the `reqwest` crate while making the HTTP request.
    /// - `GetCookiesError`: Indicates an error occurred while trying to get the cookies.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vinted_rs::{VintedWrapper, CookieError};
    ///
    /// async fn refresh_cookies_example() {
    ///     let wrapper = VintedWrapper::new();
    ///
    ///     match wrapper.refresh_cookies().await {
    ///         Ok(()) => println!("Cookies refreshed successfully"),
    ///         Err(err) => match err {
    ///             CookieError::ReqWestError(_) => (),
    ///             CookieError::GetCookiesError => (),
    ///         },
    ///     }
    /// }
    /// ```
    pub async fn refresh_cookies(
        &self,
        user_agent: Option<&str>,
        proxy: Option<Proxy>,
    ) -> Result<(), CookieError> {
        let client = get_client(user_agent, proxy).await;

        let request = format!("https://www.vinted.{}/auth/token_refresh", self.host);

        let mut response_cookies = client.post(&request).send().await?;
        let max_retries = 3;
        let mut i = 0;

        while response_cookies.status() != StatusCode::OK && i < max_retries {
            response_cookies = client.post(&request).send().await?;
            // tokio::time::sleep(Duration::from_millis(100)).await;
            i += 1;
        }

        if response_cookies.status() != StatusCode::OK {
            return Err(CookieError::GetCookiesError((
                response_cookies.status(),
                String::from(self.get_host()),
                user_agent.unwrap_or(DEFAULT_USER_AGENT).to_string(),
            )));
        }

        Ok(())
    }

    fn substitute_if_first(first: &mut bool, url: &mut String) {
        if *first {
            url.replace_range(0..1, "?");
            *first = false;
        }
    }

    /// Retrieves items from the Vinted API based on the provided filters.
    ///
    /// # Arguments
    ///
    /// * `filters` - A reference to a `Filter` struct containing the filter parameters.
    /// * `num` - The number of items to retrieve. Defaults to 1 if not specified.
    ///
    /// # Returns
    ///
    /// A `Result` enum containing either the retrieved `Items` or a `CookieError`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tokio::main;
    /// use crate::vinted_rs::model::items::Items;
    /// use crate::vinted_rs::model::filter::Filter;
    /// use crate::vinted_rs::queries::VintedWrapper;
    /// use crate::vinted_rs::queries::VintedWrapperError;
    ///
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let wrapper = VintedWrapper::new();
    ///     let filter: Filter = Filter::builder().search_text(String::from("shoes")).build();
    ///     let num = 10;
    ///
    ///    match wrapper.get_items(&filter, num).await {
    ///    Ok(items) => {
    ///        println!("Retrieved {} items: {:?}", items.items.len(), items);
    ///        assert_eq!(items.items.len(), 10);
    ///    }
    ///    Err(err) => match err {
    ///        VintedWrapperError::ItemNumberError => unreachable!(),
    ///        VintedWrapperError::CookiesError(_) => (),
    ///    },
    /// }
    /// }
    /// ```
    pub async fn get_items(
        &self,
        filters: &Filter,
        num: u32,
        user_agent: Option<&str>,
        proxy_cookies: Option<Proxy>,
        proxy_fetch: Option<Proxy>,
    ) -> Result<Items, VintedWrapperError> {
        if num == 0 {
            return Err(VintedWrapperError::ItemNumberError);
        }

        let client = get_client(user_agent, proxy_fetch).await;

        let domain: &str = &format!("vinted.{}", self.host);

        let cookie_not_valid;

        {
            let cookie_store_clone = COOKIE_STORE.lock().unwrap();
            cookie_not_valid = cookie_store_clone.get(domain, "/", "__cf_bm").is_none();
        }

        if cookie_not_valid {
            debug!(
                "[{}] POST_GET_COOKIES -> Get {} items @ {}",
                self.id, num, self.host
            );
            self.refresh_cookies(user_agent, proxy_cookies).await?;
        }

        let mut first = true;

        let mut url = format!("https://www.vinted.{}/api/v2/catalog/items", self.host);

        // Filter search text
        if let Some(text) = &filters.search_text {
            url = format!("{url}?search_text={text}");
            first = false;
        }

        // Filter catalogo
        if let Some(catalog_ids) = &filters.catalog_ids {
            let mut catalog_args: String = format!("&catalog_ids={}", catalog_ids);

            VintedWrapper::substitute_if_first(&mut first, &mut catalog_args);

            url = format!("{url}{catalog_args}");
        }

        // Filter colors
        if let Some(color_ids) = &filters.color_ids {
            let mut color_args: String = format!("&color_ids={}", color_ids);

            VintedWrapper::substitute_if_first(&mut first, &mut color_args);

            url = format!("{url}{color_args}");
        }

        //Filter brand
        if let Some(brand_ids) = &filters.brand_ids {
            let mut brand_args: String = format!("&brand_ids={}", brand_ids);

            VintedWrapper::substitute_if_first(&mut first, &mut brand_args);

            url = format!("{url}{brand_args}");
        }

        //Filter sizes
        if let Some(size_ids) = &filters.size_ids {
            let mut size_args: String = format!("&size_ids={}", size_ids);

            VintedWrapper::substitute_if_first(&mut first, &mut size_args);

            url = format!("{url}{size_args}");
        }

        //Filter materials
        if let Some(material_ids) = &filters.material_ids {
            let mut material_args: String = format!("&material_ids={}", material_ids);

            VintedWrapper::substitute_if_first(&mut first, &mut material_args);

            url = format!("{url}{material_args}");
        }
        //Filter country
        if let Some(countries_ids) = &filters.countries_ids {
            let mut countries_args: String = format!("&country_ids={}", countries_ids);

            VintedWrapper::substitute_if_first(&mut first, &mut countries_args);

            url = format!("{url}{countries_args}");
        }

        //Filter price from
        if let Some(price_from) = &filters.price_from {
            let mut price_from_arg: String = format!("&price_from={}", price_from);

            VintedWrapper::substitute_if_first(&mut first, &mut price_from_arg);

            url = format!("{url}{price_from_arg}");
        }

        //Filter price to
        if let Some(price_to) = &filters.price_to {
            let mut price_to_arg: String = format!("&price_to={}", price_to);

            VintedWrapper::substitute_if_first(&mut first, &mut price_to_arg);

            url = format!("{url}{price_to_arg}");
        }

        // Filter article_status
        if let Some(vec) = &filters.article_status {
            let querify_vec: Vec<&str> = vec.iter().map(|status| status.into()).collect();

            let mut article_status_args: String = format!("&status_ids={}", querify_vec.join(","));

            VintedWrapper::substitute_if_first(&mut first, &mut article_status_args);

            url = format!("{url}{article_status_args}");
        }

        //Order by
        if let Some(sort_by) = &filters.sort_by {
            let sort_by_str: &str = sort_by.into();

            let mut sort_by_arg = format!("&order={}", sort_by_str);

            VintedWrapper::substitute_if_first(&mut first, &mut sort_by_arg);

            url = format!("{url}{sort_by_arg}");
        }

        // Limitar el articulo a 1
        let mut per_page_args = format!("&per_page={num}");

        VintedWrapper::substitute_if_first(&mut first, &mut per_page_args);

        url = format!("{url}{per_page_args}");

        debug!("[{}] GET_{}_ITEMS @ {}", self.id, num, self.host);

        let json: Response = client.get(url).send().await?;

        match json.status() {
            StatusCode::OK => {
                // First, get the response body as text to enable debugging if deserialization fails
                let raw_json = json.text().await?;

                // Try to parse the JSON into Items
                match serde_json::from_str::<Items>(&raw_json) {
                    Ok(items) => Ok(items),
                    Err(serde_error) => {
                        let error = SerdeJSONError::new(raw_json, serde_error);
                        error!("Failed to deserialize: {}", error); // Or use a logger
                        Err(VintedWrapperError::SerdeError(error))
                    }
                }
            }

            code => {
                let retry_after = json
                    .headers()
                    .get("retry-after")
                    .map(|value| value.to_str().unwrap().to_string().parse().unwrap());
                Err(VintedWrapperError::ItemError(
                    code,
                    retry_after,
                    format!("{}::{}", self.host, json.url()),
                ))
            }
        }
    }

    /// Results additional information from an item based on its id
    ///
    /// **Warning** This query result is affected by the host country, it has to be the same as the item is hosted at
    pub async fn get_advanced_item(
        &self,
        item_id: i64,
        user_agent: Option<&str>,
        proxy_cookies: Option<Proxy>,
        proxy_fetch: Option<Proxy>,
    ) -> Result<AdvancedItem, VintedWrapperError> {
        let client = get_client(user_agent, proxy_fetch).await;

        let domain: &str = &format!("vinted.{}", self.host);

        let cookie_not_valid;

        {
            let cookie_store_clone = COOKIE_STORE.lock().unwrap();
            cookie_not_valid = cookie_store_clone.get(domain, "/", "__cf_bm").is_none();
        }

        if cookie_not_valid {
            debug!(
                "[{}] POST_GET_COOKIES -> Get item {} @ {}",
                self.id, item_id, self.host
            );
            self.refresh_cookies(user_agent, proxy_cookies).await?;
        }

        let url = format!("https://www.vinted.{}/api/v2/items/{}", self.host, item_id);
        debug!(
            "[{}] GET_ADVANCED_ITEM_{} @ {}",
            self.id, item_id, self.host
        );
        let json: Response = client.get(url).send().await?;

        match json.status() {
            StatusCode::OK => {
                let raw_json = json.text().await?;
                match serde_json::from_str::<AdvancedItems>(&raw_json) {
                    Ok(items) => Ok(items.item),
                    Err(serde_error) => {
                        // Log or debug the raw JSON content
                        let error = SerdeJSONError::new(raw_json, serde_error);
                        error!("Failed to deserialize: {}", error); // Or use a logger
                        Err(VintedWrapperError::SerdeError(error))
                    }
                }
            }

            code => {
                let retry_after = json
                    .headers()
                    .get("retry-after")
                    .map(|value| value.to_str().unwrap().to_string().parse().unwrap());
                Err(VintedWrapperError::ItemError(
                    code,
                    retry_after,
                    format!("{}::{}", self.host, item_id),
                ))
            }
        }
    }
}
