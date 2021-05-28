use std::collections::HashMap;

use crate::{
    error::Result,
    types::{gtfs::Entity, ATResponse, Header},
    BASE_API_URL,
};
use reqwest::{Client, Method};

/// A client for interacting with the Auckland Transport GTFS realtime API.
pub struct Realtime<'a> {
    client: Client,
    api_key: &'a str,
}

impl<'a> Realtime<'a> {
    /// Creates a new Auckland Transport GTFS realtime client.
    ///
    /// # Parameters
    ///
    /// * `api_key` - The API key to use when interacting with the API.
    pub fn new(api_key: &'a str) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Fetches both trip updates and vehicle positions from the AT API.
    ///
    /// AT sends the trip updates and vehicle positions seperate, these are joined together upon
    /// collection in the function, joined by trip ID.
    ///
    /// Parameters can be used to query for specific vehicles or trips. If [`None`] is given for
    /// both fields, all vehicles will be returned.
    ///
    /// # Parameters
    ///
    /// * `trip_ids` - A list of trip IDs to search for.
    /// * `vehicle_ids` - A list of vehicle IDs to search for.
    ///
    /// # Returns
    ///
    /// Returns a tuple where the first item is the response header received from AT, and the
    /// second item is a vector of AT vehicles.
    ///
    /// [`None`]: std::option::Option::None
    pub async fn fetch_combined<'b>(
        &self,
        trip_ids: Option<&Vec<&'b str>>,
        vehicle_ids: Option<&Vec<&'b str>>,
    ) -> Result<(Header, Vec<Entity>)> {
        let url = format!("{}/public/realtime", BASE_API_URL);
        let mut params = vec![];

        if let Some(trips) = trip_ids {
            params.push(("tripid", trips.join(",")));
        }

        if let Some(vehicles) = vehicle_ids {
            params.push(("vehicleid", vehicles.join(",")));
        }

        let resp = self
            .request(Method::GET, Self::build_query(url, &params))
            .send()
            .await?
            .json::<ATResponse>()
            .await?;

        let mut merged = vec![];
        let entities: HashMap<_, _> = resp
            .response
            .entity
            .into_iter()
            .map(|e| (e.id.clone(), e))
            .collect();

        fn merge(ent: &Entity, hm: &HashMap<String, Entity>) -> Option<Entity> {
            let trip_id = ent.vehicle.as_ref()?.trip.as_ref()?.trip_id.as_ref()?;
            let tu_ent = hm.get(trip_id)?;
            let mut entity = ent.clone();

            if let Some(trip_update) = tu_ent.trip_update.as_ref() {
                entity.trip_update = Some(trip_update.clone());
            }

            Some(entity)
        }

        for (_, ent) in entities.iter() {
            if let Some(ent) = merge(ent, &entities) {
                merged.push(ent);
            }
        }

        Ok((resp.response.header, merged))
    }

    /// Creates a new Reqwest request builder with the given method and URL, with the
    /// authentication header preset.
    ///
    /// # Parameters
    ///
    /// * `method` - The HTTP method to build the request with.
    /// * `url` - The URL to send the request to.
    fn request(&self, method: Method, url: String) -> reqwest::RequestBuilder {
        self.client
            .request(method, url)
            .header("Ocp-Apim-Subscription-Key", self.api_key)
    }

    /// Builds a query string.
    ///
    /// This is used instead of `RequestBuilder::query` as the AT API requires commas to seperate
    /// the vehicle and trip IDs, but reqwest escapes commas which AT does not support.
    fn build_query(mut url: String, params: &[(&str, String)]) -> String {
        let mut queries = vec![];
        for (k, v) in params {
            queries.push(format!("{}={}", k, v));
        }

        if !queries.is_empty() {
            let queries = format!("?{}", queries.join("&"));
            url += queries.as_str();
        }

        url
    }
}
