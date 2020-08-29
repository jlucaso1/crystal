/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct PluginLolRiotMessagingServiceApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PluginLolRiotMessagingServiceApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PluginLolRiotMessagingServiceApiClient<C> {
        PluginLolRiotMessagingServiceApiClient {
            configuration,
        }
    }
}

pub trait PluginLolRiotMessagingServiceApi {
    fn delete_lol_rms_v1_champion_mastery_leaveup_update_by_id(&self, id: i64) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn get_lol_rms_v1_champion_mastery_leaveup_update(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolRiotMessagingServiceChampionMasteryLevelUp>, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>PluginLolRiotMessagingServiceApi for PluginLolRiotMessagingServiceApiClient<C> {
    fn delete_lol_rms_v1_champion_mastery_leaveup_update_by_id(&self, id: i64) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-rms/v1/champion-mastery-leaveup-update/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_rms_v1_champion_mastery_leaveup_update(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolRiotMessagingServiceChampionMasteryLevelUp>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-rms/v1/champion-mastery-leaveup-update".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

}