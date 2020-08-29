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

pub struct PluginLolShutdownApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PluginLolShutdownApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PluginLolShutdownApiClient<C> {
        PluginLolShutdownApiClient {
            configuration,
        }
    }
}

pub trait PluginLolShutdownApi {
    fn get_lol_shutdown_v1_notification(&self, ) -> Box<dyn Future<Item = crate::models::LolShutdownShutdownNotification, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>PluginLolShutdownApi for PluginLolShutdownApiClient<C> {
    fn get_lol_shutdown_v1_notification(&self, ) -> Box<dyn Future<Item = crate::models::LolShutdownShutdownNotification, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-shutdown/v1/notification".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

}