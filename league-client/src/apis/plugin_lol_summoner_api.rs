use crate::helpers::JoinIterator;

/*
 *
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct PluginLolSummonerApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PluginLolSummonerApiClient<C> {
    pub fn new(
        configuration: Rc<configuration::Configuration<C>>,
    ) -> PluginLolSummonerApiClient<C> {
        PluginLolSummonerApiClient { configuration }
    }
}

pub trait PluginLolSummonerApi {
    fn get_lol_summoner_v1_check_name_availability_by_name(
        &self,
        name: &str,
    ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>>;
    fn get_lol_summoner_v1_check_name_availability_new_summoners_by_name(
        &self,
        name: &str,
    ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>>;
    fn get_lol_summoner_v1_current_summoner(
        &self,
    ) -> Box<dyn Future<Item = crate::models::LolSummonerSummoner, Error = Error<serde_json::Value>>>;
    fn get_lol_summoner_v1_current_summoner_autofill(
        &self,
    ) -> Box<
        dyn Future<
            Item = Vec<crate::models::LolSummonerAutoFillQueueDto>,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_lol_summoner_v1_current_summoner_jwt(
        &self,
    ) -> Box<dyn Future<Item = String, Error = Error<serde_json::Value>>>;
    fn get_lol_summoner_v1_current_summoner_reroll_points(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::LolSummonerSummonerRerollPoints,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_lol_summoner_v1_current_summoner_summoner_profile(
        &self,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_lol_summoner_v1_status(
        &self,
    ) -> Box<dyn Future<Item = crate::models::LolSummonerStatus, Error = Error<serde_json::Value>>>;
    fn get_lol_summoner_v1_summoner_profile(
        &self,
        puuid: &str,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_lol_summoner_v1_summoner_requests_ready(
        &self,
    ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>>;
    fn get_lol_summoner_v1_summoners(
        &self,
        name: &str,
    ) -> Box<dyn Future<Item = crate::models::LolSummonerSummoner, Error = Error<serde_json::Value>>>;
    fn get_lol_summoner_v1_summoners_by_id(
        &self,
        id: i64,
    ) -> Box<dyn Future<Item = crate::models::LolSummonerSummoner, Error = Error<serde_json::Value>>>;
    fn get_lol_summoner_v1_summoners_by_puuid_cached_by_puuid(
        &self,
        puuid: &str,
    ) -> Box<dyn Future<Item = crate::models::LolSummonerSummoner, Error = Error<serde_json::Value>>>;
    fn get_lol_summoner_v2_summoner_icons(
        &self,
        ids: Vec<i64>,
    ) -> Box<
        dyn Future<
            Item = Vec<crate::models::LolSummonerSummonerIdAndIcon>,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_lol_summoner_v2_summoner_names(
        &self,
        ids: Vec<i64>,
    ) -> Box<
        dyn Future<
            Item = Vec<crate::models::LolSummonerSummonerIdAndName>,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_lol_summoner_v2_summoners(
        &self,
        ids: Option<Vec<i64>>,
    ) -> Box<
        dyn Future<
            Item = Vec<crate::models::LolSummonerSummoner>,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_lol_summoner_v2_summoners_puuid_by_puuid(
        &self,
        puuid: &str,
    ) -> Box<dyn Future<Item = crate::models::LolSummonerSummoner, Error = Error<serde_json::Value>>>;
    fn post_lol_summoner_v1_current_summoner_name(
        &self,
        name: &str,
    ) -> Box<dyn Future<Item = crate::models::LolSummonerSummoner, Error = Error<serde_json::Value>>>;
    fn post_lol_summoner_v1_current_summoner_summoner_profile(
        &self,
        body: crate::models::LolSummonerSummonerProfileUpdate,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn post_lol_summoner_v1_summoners(
        &self,
        name: crate::models::LolSummonerSummonerRequestedName,
    ) -> Box<
        dyn Future<
            Item = crate::models::LolSummonerInternalSummoner,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn post_lol_summoner_v2_summoners_names(
        &self,
        summoner_names: Vec<String>,
    ) -> Box<
        dyn Future<
            Item = Vec<crate::models::LolSummonerSummoner>,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn post_lol_summoner_v2_summoners_puuid(
        &self,
        puuids: Vec<String>,
    ) -> Box<
        dyn Future<
            Item = Vec<crate::models::LolSummonerSummoner>,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn put_lol_summoner_v1_current_summoner_icon(
        &self,
        body: crate::models::LolSummonerSummonerIcon,
    ) -> Box<dyn Future<Item = crate::models::LolSummonerSummoner, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> PluginLolSummonerApi for PluginLolSummonerApiClient<C> {
    fn get_lol_summoner_v1_check_name_availability_by_name(
        &self,
        name: &str,
    ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v1/check-name-availability/{name}".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_summoner_v1_check_name_availability_new_summoners_by_name(
        &self,
        name: &str,
    ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v1/check-name-availability-new-summoners/{name}".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_summoner_v1_current_summoner(
        &self,
    ) -> Box<dyn Future<Item = crate::models::LolSummonerSummoner, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v1/current-summoner".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn get_lol_summoner_v1_current_summoner_autofill(
        &self,
    ) -> Box<
        dyn Future<
            Item = Vec<crate::models::LolSummonerAutoFillQueueDto>,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v1/current-summoner/autofill".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn get_lol_summoner_v1_current_summoner_jwt(
        &self,
    ) -> Box<dyn Future<Item = String, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v1/current-summoner/jwt".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn get_lol_summoner_v1_current_summoner_reroll_points(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::LolSummonerSummonerRerollPoints,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v1/current-summoner/rerollPoints".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn get_lol_summoner_v1_current_summoner_summoner_profile(
        &self,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v1/current-summoner/summoner-profile".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn get_lol_summoner_v1_status(
        &self,
    ) -> Box<dyn Future<Item = crate::models::LolSummonerStatus, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v1/status".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn get_lol_summoner_v1_summoner_profile(
        &self,
        puuid: &str,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v1/summoner-profile".to_string(),
        );
        req = req.with_query_param("puuid".to_string(), puuid.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_summoner_v1_summoner_requests_ready(
        &self,
    ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v1/summoner-requests-ready".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn get_lol_summoner_v1_summoners(
        &self,
        name: &str,
    ) -> Box<dyn Future<Item = crate::models::LolSummonerSummoner, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v1/summoners".to_string(),
        );
        req = req.with_query_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_summoner_v1_summoners_by_id(
        &self,
        id: i64,
    ) -> Box<dyn Future<Item = crate::models::LolSummonerSummoner, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v1/summoners/{id}".to_string(),
        );
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_summoner_v1_summoners_by_puuid_cached_by_puuid(
        &self,
        puuid: &str,
    ) -> Box<dyn Future<Item = crate::models::LolSummonerSummoner, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v1/summoners-by-puuid-cached/{puuid}".to_string(),
        );
        req = req.with_path_param("puuid".to_string(), puuid.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_summoner_v2_summoner_icons(
        &self,
        ids: Vec<i64>,
    ) -> Box<
        dyn Future<
            Item = Vec<crate::models::LolSummonerSummonerIdAndIcon>,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v2/summoner-icons".to_string(),
        );
        req = req.with_query_param("ids".to_string(), ids.join(",").to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_summoner_v2_summoner_names(
        &self,
        ids: Vec<i64>,
    ) -> Box<
        dyn Future<
            Item = Vec<crate::models::LolSummonerSummonerIdAndName>,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v2/summoner-names".to_string(),
        );
        req = req.with_query_param("ids".to_string(), ids.join(",").to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_summoner_v2_summoners(
        &self,
        ids: Option<Vec<i64>>,
    ) -> Box<
        dyn Future<
            Item = Vec<crate::models::LolSummonerSummoner>,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v2/summoners".to_string(),
        );
        if let Some(ref s) = ids {
            req = req.with_query_param("ids".to_string(), s.join(",").to_string());
        }

        req.execute(self.configuration.borrow())
    }

    fn get_lol_summoner_v2_summoners_puuid_by_puuid(
        &self,
        puuid: &str,
    ) -> Box<dyn Future<Item = crate::models::LolSummonerSummoner, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-summoner/v2/summoners/puuid/{puuid}".to_string(),
        );
        req = req.with_path_param("puuid".to_string(), puuid.to_string());

        req.execute(self.configuration.borrow())
    }

    fn post_lol_summoner_v1_current_summoner_name(
        &self,
        name: &str,
    ) -> Box<dyn Future<Item = crate::models::LolSummonerSummoner, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/lol-summoner/v1/current-summoner/name".to_string(),
        );
        req = req.with_body_param(name);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_summoner_v1_current_summoner_summoner_profile(
        &self,
        body: crate::models::LolSummonerSummonerProfileUpdate,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/lol-summoner/v1/current-summoner/summoner-profile".to_string(),
        );
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_summoner_v1_summoners(
        &self,
        name: crate::models::LolSummonerSummonerRequestedName,
    ) -> Box<
        dyn Future<
            Item = crate::models::LolSummonerInternalSummoner,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/lol-summoner/v1/summoners".to_string(),
        );
        req = req.with_body_param(name);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_summoner_v2_summoners_names(
        &self,
        summoner_names: Vec<String>,
    ) -> Box<
        dyn Future<
            Item = Vec<crate::models::LolSummonerSummoner>,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/lol-summoner/v2/summoners/names".to_string(),
        );
        req = req.with_body_param(summoner_names);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_summoner_v2_summoners_puuid(
        &self,
        puuids: Vec<String>,
    ) -> Box<
        dyn Future<
            Item = Vec<crate::models::LolSummonerSummoner>,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/lol-summoner/v2/summoners/puuid".to_string(),
        );
        req = req.with_body_param(puuids);

        req.execute(self.configuration.borrow())
    }

    fn put_lol_summoner_v1_current_summoner_icon(
        &self,
        body: crate::models::LolSummonerSummonerIcon,
    ) -> Box<dyn Future<Item = crate::models::LolSummonerSummoner, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Put,
            "/lol-summoner/v1/current-summoner/icon".to_string(),
        );
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }
}