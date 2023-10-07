mod settlements;
mod cities;
mod warehouses;
use settlements::SearchSettlementsBuilder;
use cities::GetCitiesBuilder;

use self::warehouses::GetWarehousesBuilder;

use super::NPClient;

pub struct AddressHandler<'c> {
    client: &'c NPClient,
}

impl<'cli> AddressHandler<'cli> {
    pub(crate) fn new(client: &'cli NPClient) -> Self {
        Self { client }
    }

    pub fn search_settlements(&self, val: String) -> SearchSettlementsBuilder<'cli> {
        SearchSettlementsBuilder::new(self.client, val)
    }

    pub fn get_cities(&self) -> GetCitiesBuilder<'cli> {
        GetCitiesBuilder::new(self.client)
    }

    pub fn get_warehouses(&self) -> GetWarehousesBuilder<'cli> {
        GetWarehousesBuilder::new(self.client)
    }
}