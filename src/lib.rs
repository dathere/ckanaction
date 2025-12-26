//! # ckanaction
//!
//! Rust library crate to access [CKAN](https://ckan.org) Action API endpoints through Rust builders. Based on the CKAN Action API v3. Endpoints are expected to return with an output of type `serde_json::Value`.
//!
//! Learn more at [github.com/dathere/ckanaction](https://github.com/dathere/ckanaction).

use bon::bon;
use serde_json::json;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct CKAN {
    url: String,
    token: Option<String>,
}

fn hashmap_to_json(
    map: &HashMap<&str, serde_json::Value>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(serde_json::to_string(&map)?.as_str())?)
}

fn opsert<'a, T: serde::ser::Serialize>(
    name: &'a str,
    value: Option<T>,
    map: &mut HashMap<&'a str, serde_json::Value>,
) {
    if value.is_some() {
        map.insert(name, json!(value));
    };
}

macro_rules! opsert_input_handler {
    ($m: expr, $v: ident) => {{
        opsert(stringify!($v), Some($v), $m);
    }};
    ($m: expr, (exact $v: ident)) => {{
        $m.insert(stringify!($v), $v);
    }};
    ($m: expr, ($k: literal, $v: expr)) => {{
        opsert($k, Some($v), $m);
    }};
    ($m: expr, (json $v: expr)) => {{
        $m.insert(stringify!($v), json!($v));
    }};
}

macro_rules! opsert {
    ($m: expr, $($v: tt),*) => {
        $(
            opsert_input_handler!($m, $v);
        )*
    };
}

macro_rules! post {
    ($self: expr, $endpoint: literal, $($v: tt),* $(; (upload $u: expr))?) => {{
        let endpoint = $self.url.clone() + "/api/3/action/" + $endpoint;
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        $(opsert!(&mut map, $v);)*
        let body = hashmap_to_json(&map)?;
        $(return Self::post($self).endpoint(endpoint).body(body).maybe_upload($u).call().await;)?
        #[allow(unreachable_code)]
        Self::post($self).endpoint(endpoint).body(body).call().await
    }};
    ($self: expr, $endpoint: literal) => {{
        let endpoint = $self.url.clone() + "/api/3/action/" + $endpoint;
        Self::post($self).endpoint(endpoint).call().await
    }};
}

#[bon]
impl CKAN {
    #[builder]
    pub fn new(url: &str, token: Option<String>) -> Self {
        Self {
            url: url.to_string(),
            token,
        }
    }

    async fn get(&self, endpoint: String) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let mut req_builder = client.get(endpoint);
        if self.token.is_some() {
            req_builder = req_builder.header("Authorization", self.token.clone().unwrap());
        }
        Ok(req_builder
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?)
    }

    #[builder]
    async fn post(
        &self,
        endpoint: String,
        body: Option<serde_json::Value>,
        upload: Option<PathBuf>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let mut req_builder = client.post(endpoint);
        if self.token.is_some() {
            req_builder = req_builder.header("Authorization", self.token.clone().unwrap());
        }
        if let Some(file_pathbuf) = upload {
            let mut form = reqwest::multipart::Form::new();
            if let Some(body_as_value) = body {
                for entry in body_as_value.as_object().unwrap().iter() {
                    form = form.text(entry.0.to_owned(), entry.1.as_str().unwrap().to_owned());
                }
            }
            form = form.file("upload", file_pathbuf).await?;
            req_builder = req_builder.multipart(form);
            let res = req_builder.send().await?.json().await?;
            Ok(res)
        } else {
            let res = req_builder.json(&body).send().await?.json().await?;
            Ok(res)
        }
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.package_list
    #[builder]
    pub async fn package_list(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "package_list", limit, offset)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.current_package_list_with_resources
    #[builder]
    pub async fn current_package_list_with_resources(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
        page: Option<u32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(
            &self,
            "current_package_list_with_resources",
            limit,
            offset,
            page
        )
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.member_list
    #[builder]
    pub async fn member_list(
        &self,
        id: String,
        object_type: Option<String>,
        capacity: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "member_list", (json id), object_type, capacity)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.package_collaborator_list
    #[builder]
    pub async fn package_collaborator_list(
        &self,
        id: String,
        capacity: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "package_collaborator_list", (json id), capacity)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.package_collaborator_list_for_user
    #[builder]
    pub async fn package_collaborator_list_for_user(
        &self,
        id: String,
        capacity: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "package_collaborator_list_for_user", (json id), capacity)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.group_list
    #[builder]
    pub async fn group_list(
        &self,
        _type: Option<String>,
        order_by: Option<String>,
        sort: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
        groups: Option<Vec<String>>,
        all_fields: Option<bool>,
        include_dataset_count: Option<bool>,
        include_extras: Option<bool>,
        include_tags: Option<bool>,
        include_groups: Option<bool>,
        include_users: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(
            &self,
            "group_list",
            ("type", _type),
            order_by,
            sort,
            limit,
            offset,
            groups,
            all_fields,
            include_dataset_count,
            include_extras,
            include_tags,
            include_groups,
            include_users
        )
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.organization_list
    #[builder]
    pub async fn organization_list(
        &self,
        _type: Option<String>,
        order_by: Option<String>,
        sort: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
        organizations: Option<Vec<String>>,
        all_fields: Option<bool>,
        include_dataset_count: Option<bool>,
        include_extras: Option<bool>,
        include_tags: Option<bool>,
        include_groups: Option<bool>,
        include_users: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(
            &self,
            "organization_list",
            ("type", _type),
            order_by,
            sort,
            limit,
            offset,
            organizations,
            all_fields,
            include_dataset_count,
            include_extras,
            include_tags,
            include_groups,
            include_users
        )
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.group_list_authz
    #[builder]
    pub async fn group_list_authz(
        &self,
        available_only: Option<bool>,
        am_member: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "group_list_authz", available_only, am_member)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.organization_list_for_user
    #[builder]
    pub async fn organization_list_for_user(
        &self,
        id: Option<String>,
        permission: Option<String>,
        include_dataset_count: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(
            &self,
            "organization_list_for_user",
            id,
            permission,
            include_dataset_count
        )
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.license_list
    #[builder]
    pub async fn license_list(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "license_list")
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.tag_list
    #[builder]
    pub async fn tag_list(
        &self,
        query: Option<String>,
        vocabulary_id: Option<String>,
        all_fields: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "tag_list", query, vocabulary_id, all_fields)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.user_list
    #[builder]
    pub async fn user_list(
        &self,
        q: Option<String>,
        email: Option<String>,
        order_by: Option<String>,
        all_fields: Option<bool>,
        include_site_user: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(
            &self,
            "user_list",
            q,
            email,
            order_by,
            all_fields,
            include_site_user
        )
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.package_relationships_list
    #[builder]
    pub async fn package_relationships_list(
        &self,
        id: String,
        id2: String,
        rel: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(
            &self,
            "package_relationships_list",
            (json id),
            (json id2),
            rel
        )
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.resource_show
    #[builder]
    pub async fn resource_show(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "resource_show", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.resource_view_show
    #[builder]
    pub async fn resource_view_show(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "resource_view_show", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.resource_view_list
    #[builder]
    pub async fn resource_view_list(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "resource_view_list", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.group_show
    #[builder]
    pub async fn group_show(
        &self,
        id: String,
        include_datasets: Option<bool>,
        include_dataset_count: Option<bool>,
        include_extras: Option<bool>,
        include_users: Option<bool>,
        include_groups: Option<bool>,
        include_tags: Option<bool>,
        include_followers: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(
            &self,
            "group_show",
            (json id),
            include_datasets,
            include_dataset_count,
            include_extras,
            include_users,
            include_groups,
            include_tags,
            include_followers
        )
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.organization_show
    #[builder]
    pub async fn organization_show(
        &self,
        id: String,
        include_datasets: Option<bool>,
        include_dataset_count: Option<bool>,
        include_extras: Option<bool>,
        include_users: Option<bool>,
        include_groups: Option<bool>,
        include_tags: Option<bool>,
        include_followers: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(
            &self,
            "organization_show",
            (json id),
            include_datasets,
            include_dataset_count,
            include_extras,
            include_users,
            include_groups,
            include_tags,
            include_followers
        )
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.group_package_show
    #[builder]
    pub async fn group_package_show(
        &self,
        id: String,
        limit: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "group_package_show", (json id), limit)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.tag_show
    #[builder]
    pub async fn tag_show(
        &self,
        id: String,
        vocabulary_id: Option<String>,
        include_datasets: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "tag_show", (json id), vocabulary_id, include_datasets)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.user_show
    #[builder]
    pub async fn user_show(
        &self,
        id: String,
        include_datasets: Option<bool>,
        include_num_followers: Option<bool>,
        include_password_hash: Option<bool>,
        include_plugin_extras: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(
            &self,
            "user_show",
            (json id),
            include_datasets,
            include_num_followers,
            include_password_hash,
            include_plugin_extras
        )
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.package_autocomplete
    #[builder]
    pub async fn package_autocomplete(
        &self,
        q: String,
        limit: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "package_autocomplete", (json q), limit)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.format_autocomplete
    #[builder]
    pub async fn format_autocomplete(
        &self,
        q: String,
        limit: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "format_autocomplete", (json q), limit)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.user_autocomplete
    #[builder]
    pub async fn user_autocomplete(
        &self,
        q: String,
        limit: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "user_autocomplete", (json q), limit)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.group_autocomplete
    #[builder]
    pub async fn group_autocomplete(
        &self,
        q: String,
        limit: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "group_autocomplete", (json q), limit)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.organization_autocomplete
    #[builder]
    pub async fn organization_autocomplete(
        &self,
        q: String,
        limit: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "organization_autocomplete", (json q), limit)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.package_search
    #[builder]
    pub async fn package_search(
        &self,
        q: Option<String>,
        fq: Option<String>,
        fq_list: Option<Vec<String>>,
        sort: Option<String>,
        rows: Option<i32>,
        start: Option<i32>,
        facet: Option<String>,
        facet_mincount: Option<i32>,
        facet_limit: Option<i32>,
        facet_field: Option<Vec<String>>,
        include_drafts: Option<bool>,
        include_private: Option<bool>,
        use_default_schema: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(
            &self,
            "package_search",
            (json q),
            fq,
            fq_list,
            sort,
            rows,
            start,
            facet,
            ("facet.mincount", facet_mincount),
            ("facet.limit", facet_limit),
            ("facet.field", facet_field),
            include_drafts,
            include_private,
            use_default_schema
        )
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.resource_search
    #[builder]
    pub async fn resource_search<T: serde::Serialize>(
        &self,
        query: Option<T>,
        order_by: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "resource_search", query, order_by, offset, limit)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.tag_search
    #[builder]
    pub async fn tag_search<T: serde::Serialize>(
        &self,
        query: Option<T>,
        vocabulary_id: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "tag_search", query, vocabulary_id, limit, offset)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.tag_autocomplete
    #[builder]
    pub async fn tag_autocomplete(
        &self,
        query: String,
        vocabulary_id: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(
            &self,
            "tag_autocomplete",
            query,
            vocabulary_id,
            limit,
            offset
        )
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.task_status_show
    #[builder]
    pub async fn task_status_show(
        &self,
        id: Option<String>,
        entity_id: Option<String>,
        task_type: Option<String>,
        key: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "task_status_show", id, entity_id, task_type, key)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.term_translation_show
    #[builder]
    pub async fn term_translation_show(
        &self,
        terms: Option<Vec<String>>,
        lang_codes: Option<Vec<String>>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "term_translation_show", terms, lang_codes)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.get_site_user
    #[builder]
    pub async fn get_site_user(
        &self,
        defer_commit: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "get_site_user", defer_commit)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.status_show
    pub async fn status_show(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/status_show";
        Self::get(self, endpoint).await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.vocabulary_list
    pub async fn vocabulary_list(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/vocabulary_list";
        Self::get(self, endpoint).await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.vocabulary_show
    #[builder]
    pub async fn vocabulary_show(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "vocabulary_show", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.user_follower_count
    #[builder]
    pub async fn user_follower_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "user_follower_count", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.dataset_follower_count
    #[builder]
    pub async fn dataset_follower_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "dataset_follower_count", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.group_follower_count
    #[builder]
    pub async fn group_follower_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "group_follower_count", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.organization_follower_count
    #[builder]
    pub async fn organization_follower_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "organization_follower_count", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.user_follower_list
    #[builder]
    pub async fn user_follower_list(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "user_follower_list", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.dataset_follower_list
    #[builder]
    pub async fn dataset_follower_list(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "dataset_follower_list", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.group_follower_list
    #[builder]
    pub async fn group_follower_list(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "group_follower_list", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.am_following_user
    #[builder]
    pub async fn am_following_user(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "am_following_user", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.am_following_dataset
    #[builder]
    pub async fn am_following_dataset(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "am_following_dataset", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.am_following_group
    #[builder]
    pub async fn am_following_group(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "am_following_group", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.followee_count
    #[builder]
    pub async fn followee_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "followee_count", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.user_followee_count
    #[builder]
    pub async fn user_followee_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "user_followee_count", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.dataset_followee_count
    #[builder]
    pub async fn dataset_followee_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "dataset_followee_count", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.group_followee_count
    #[builder]
    pub async fn group_followee_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "group_followee_count", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.organization_followee_count
    #[builder]
    pub async fn organization_followee_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "organization_followee_count", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.followee_list
    #[builder]
    pub async fn followee_list(
        &self,
        id: String,
        q: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "followee_list", (json id), q)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.user_followee_list
    #[builder]
    pub async fn user_followee_list(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "user_followee_list", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.dataset_followee_list
    #[builder]
    pub async fn dataset_followee_list(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "dataset_followee_list", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.group_followee_list
    #[builder]
    pub async fn group_followee_list(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "group_followee_list", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.organization_followee_list
    #[builder]
    pub async fn organization_followee_list(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "organization_followee_list", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.member_roles_list
    #[builder]
    pub async fn member_roles_list(
        &self,
        group_type: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "member_roles_list", group_type)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.help_show
    #[builder]
    pub async fn help_show(
        &self,
        name: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "help_show", (json name))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.config_option_show
    #[builder]
    pub async fn config_option_show(
        &self,
        key: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "config_option_show", (json key))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.config_option_list
    pub async fn config_option_list(
        &self,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/config_option_list";
        Self::get(self, endpoint).await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.job_list
    #[builder]
    pub async fn job_list(
        &self,
        queues: Option<Vec<String>>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "job_list", queues)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.job_show
    #[builder]
    pub async fn job_show(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "job_show", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.api_token_list
    #[builder]
    pub async fn api_token_list(
        &self,
        user_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "api_token_list", (json user_id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.package_create
    #[builder]
    pub async fn package_create(
        &self,
        name: String,
        title: Option<String>,
        private: bool,
        author: Option<String>,
        author_email: Option<String>,
        maintainer: Option<String>,
        maintainer_email: Option<String>,
        license_id: Option<String>,
        notes: Option<String>,
        url: Option<String>,
        version: Option<String>,
        state: Option<String>,
        _type: Option<String>,
        resources: Option<Vec<serde_json::Value>>,
        tags: Option<Vec<serde_json::Value>>,
        extras: Option<Vec<serde_json::Value>>,
        plugin_data: Option<serde_json::Value>,
        relationships_as_object: Option<Vec<serde_json::Value>>,
        relationships_as_subject: Option<Vec<serde_json::Value>>,
        groups: Option<Vec<serde_json::Value>>,
        owner_org: Option<String>,
        custom_fields: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/package_create";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert!(&mut map, (json name), title, (json private), author, author_email, maintainer, maintainer_email, license_id, notes, url, version, state, ("type", _type), resources, tags, extras, plugin_data, relationships_as_object, relationships_as_subject, groups, owner_org);
        let mut custom_map: HashMap<String, serde_json::Value> = HashMap::new();
        if let Some(custom) = custom_fields
            && custom.is_object()
        {
            let custom_temp_map = custom.as_object().unwrap();
            custom_map.extend(
                custom_temp_map
                    .iter()
                    .map(|item| (item.0.to_owned(), item.1.to_owned())),
            );
        }
        map.extend(
            custom_map
                .iter()
                .map(|item| (item.0.as_str(), item.1.to_owned())),
        );
        let body = hashmap_to_json(&map)?;
        Self::post(self).endpoint(endpoint).body(body).call().await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.resource_create
    #[builder]
    pub async fn resource_create(
        &self,
        package_id: String,
        url: Option<String>,
        description: Option<String>,
        format: Option<String>,
        hash: Option<String>,
        name: Option<String>,
        resource_type: Option<String>,
        mimetype: Option<String>,
        mimetype_inner: Option<String>,
        cache_url: Option<String>,
        size: Option<i32>,
        created: Option<String>,
        last_modified: Option<String>,
        cache_last_updated: Option<String>,
        upload: Option<PathBuf>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "resource_create", (json package_id), url, description, format, hash, name, resource_type, mimetype, mimetype_inner, cache_url, size, created, last_modified, cache_last_updated; (upload upload))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.resource_view_create
    #[builder]
    pub async fn resource_view_create(
        &self,
        resource_id: String,
        title: String,
        description: Option<String>,
        view_type: String,
        config: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "resource_view_create", (json resource_id), (json title), description, (json view_type), config)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.create_default_resource_views
    #[builder]
    pub async fn create_default_resource_views(
        &self,
        resource: serde_json::Value,
        package: Option<serde_json::Value>,
        create_datastore_views: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "create_default_resource_views", (exact resource), package, create_datastore_views)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.package_create_default_resource_views
    #[builder]
    pub async fn package_create_default_resource_views(
        &self,
        package: serde_json::Value,
        create_datastore_views: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "package_create_default_resource_views", (exact package), create_datastore_views)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.package_relationship_create
    #[builder]
    pub async fn package_relationship_create(
        &self,
        subject: String,
        object: String,
        _type: String,
        comment: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "package_relationship_create", (json subject), (json object), ("type", json!(_type)), comment)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.member_create
    #[builder]
    pub async fn member_create(
        &self,
        id: String,
        object: String,
        object_type: String,
        capacity: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "member_create", (json id), (json object), (json object_type), (json capacity))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.package_collaborator_create
    #[builder]
    pub async fn package_collaborator_create(
        &self,
        id: String,
        user_id: String,
        capacity: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "package_collaborator_create", (json id), (json user_id), (json capacity))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.group_create
    #[builder]
    pub async fn group_create(
        &self,
        name: String,
        id: Option<String>,
        title: Option<String>,
        description: Option<String>,
        image_url: Option<String>,
        _type: Option<String>,
        state: Option<String>,
        approval_status: Option<String>,
        extras: Option<Vec<serde_json::Value>>,
        packages: Option<Vec<serde_json::Value>>,
        groups: Option<Vec<serde_json::Value>>,
        users: Option<Vec<serde_json::Value>>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "group_create", (json name), id, title, description, image_url, ("type", _type), state, approval_status, extras, packages, groups, users)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.organization_create
    #[builder]
    pub async fn organization_create(
        &self,
        name: String,
        id: Option<String>,
        title: Option<String>,
        description: Option<String>,
        image_url: Option<String>,
        state: Option<String>,
        approval_status: Option<String>,
        extras: Option<Vec<serde_json::Value>>,
        packages: Option<Vec<serde_json::Value>>,
        users: Option<Vec<serde_json::Value>>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "organization_create", (json name), id, title, description, image_url, state, approval_status, extras, packages, users)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.user_create
    #[builder]
    pub async fn user_create(
        &self,
        name: String,
        email: String,
        password: String,
        id: Option<String>,
        fullname: Option<String>,
        about: Option<String>,
        image_url: Option<String>,
        plugin_extras: Option<serde_json::Value>,
        with_apitoken: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "user_create", (json name), (json email), (json password), id, fullname, about, image_url, plugin_extras, with_apitoken)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.user_invite
    #[builder]
    pub async fn user_invite(
        &self,
        email: String,
        group_id: String,
        role: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "user_invite", (json email), (json group_id), (json role))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.vocabulary_create
    #[builder]
    pub async fn vocabulary_create(
        &self,
        name: String,
        tags: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "vocabulary_create", (json name), (json tags))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.tag_create
    #[builder]
    pub async fn tag_create(
        &self,
        name: String,
        vocabulary_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "tag_create", (json name), (json vocabulary_id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.follow_user
    #[builder]
    pub async fn follow_user(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "follow_user", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.follow_dataset
    #[builder]
    pub async fn follow_dataset(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "follow_dataset", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.group_member_create
    #[builder]
    pub async fn group_member_create(
        &self,
        id: String,
        username: String,
        role: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "group_member_create", (json id), (json username), (json role))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.organization_member_create
    #[builder]
    pub async fn organization_member_create(
        &self,
        id: String,
        username: String,
        role: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "organization_member_create", (json id), (json username), (json role))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.follow_group
    #[builder]
    pub async fn follow_group(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "follow_group", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.api_token_create
    #[builder]
    pub async fn api_token_create(
        &self,
        user: String,
        name: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "api_token_create", (json user), (json name))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.resource_update
    #[builder]
    pub async fn resource_update(
        &self,
        id: String,
        package_id: String,
        url: Option<String>,
        description: Option<String>,
        format: Option<String>,
        hash: Option<String>,
        name: Option<String>,
        resource_type: Option<String>,
        mimetype: Option<String>,
        mimetype_inner: Option<String>,
        cache_url: Option<String>,
        size: Option<i32>,
        created: Option<String>,
        last_modified: Option<String>,
        cache_last_updated: Option<String>,
        upload: Option<PathBuf>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "resource_update", (json id), (json package_id), url, description, format, hash, name, resource_type, mimetype, mimetype_inner, cache_url, size, created, last_modified, cache_last_updated; (upload upload))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.resource_view_update
    #[builder]
    pub async fn resource_view_update(
        &self,
        id: String,
        resource_id: String,
        title: String,
        description: Option<String>,
        view_type: String,
        config: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "resource_view_update", (json id), (json resource_id), (json title), description, (json view_type), config)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.resource_view_reorder
    #[builder]
    pub async fn resource_view_reorder(
        &self,
        id: String,
        order: Vec<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "resource_view_reorder", (json id), (json order))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.package_update
    #[builder]
    pub async fn package_update(
        &self,
        id: String,
        name: String,
        title: Option<String>,
        private: bool,
        author: Option<String>,
        author_email: Option<String>,
        maintainer: Option<String>,
        maintainer_email: Option<String>,
        license_id: Option<String>,
        notes: Option<String>,
        url: Option<String>,
        version: Option<String>,
        state: Option<String>,
        _type: Option<String>,
        resources: Option<Vec<serde_json::Value>>,
        tags: Option<Vec<serde_json::Value>>,
        extras: Option<Vec<serde_json::Value>>,
        plugin_data: Option<serde_json::Value>,
        relationships_as_object: Option<Vec<serde_json::Value>>,
        relationships_as_subject: Option<Vec<serde_json::Value>>,
        groups: Option<Vec<serde_json::Value>>,
        owner_org: Option<String>,
        custom_fields: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/package_update";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert!(&mut map, (json id), (json name), title, (json private), author, author_email, maintainer, maintainer_email, license_id, notes, url, version, state, ("type", _type), resources, tags, extras, plugin_data, relationships_as_object, relationships_as_subject, groups, owner_org);
        let mut custom_map: HashMap<String, serde_json::Value> = HashMap::new();
        if let Some(custom) = custom_fields
            && custom.is_object()
        {
            let custom_temp_map = custom.as_object().unwrap();
            custom_map.extend(
                custom_temp_map
                    .iter()
                    .map(|item| (item.0.to_owned(), item.1.to_owned())),
            );
        }
        map.extend(
            custom_map
                .iter()
                .map(|item| (item.0.as_str(), item.1.to_owned())),
        );
        let body = hashmap_to_json(&map)?;
        Self::post(self).endpoint(endpoint).body(body).call().await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.package_revise
    #[builder]
    pub async fn package_revise(
        &self,
        _match: serde_json::Value,
        filter: Option<Vec<String>>,
        update: serde_json::Value,
        include: Option<Vec<String>>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "package_revise", ("match", _match), filter, (exact update), include)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.package_resource_reorder
    #[builder]
    pub async fn package_resource_reorder(
        &self,
        id: String,
        order: Vec<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "package_resource_reorder", (json id), (json order))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.package_relationship_update
    #[builder]
    pub async fn package_relationship_update(
        &self,
        subject: String,
        object: String,
        _type: String,
        comment: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "package_relationship_update", (json subject), (json object), ("type", json!(_type)), comment)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.group_update
    #[builder]
    pub async fn group_update(
        &self,
        id: String,
        name: String,
        title: Option<String>,
        description: Option<String>,
        image_url: Option<String>,
        _type: Option<String>,
        state: Option<String>,
        approval_status: Option<String>,
        extras: Option<Vec<serde_json::Value>>,
        packages: Option<Vec<serde_json::Value>>,
        groups: Option<Vec<serde_json::Value>>,
        users: Option<Vec<serde_json::Value>>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "group_update", (json id), (json name), title, description, image_url, ("type", _type), state, approval_status, extras, packages, groups, users)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.organization_update
    #[builder]
    pub async fn organization_update(
        &self,
        id: String,
        name: String,
        title: Option<String>,
        description: Option<String>,
        image_url: Option<String>,
        state: Option<String>,
        approval_status: Option<String>,
        extras: Option<Vec<serde_json::Value>>,
        users: Option<Vec<serde_json::Value>>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "organization_update", (json id), (json name), title, description, image_url, state, approval_status, extras, users)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.user_update
    #[builder]
    pub async fn user_update(
        &self,
        id: String,
        name: String,
        email: String,
        password: String,
        fullname: Option<String>,
        about: Option<String>,
        image_url: Option<String>,
        plugin_extras: Option<serde_json::Value>,
        with_apitoken: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "user_update", (json id), (json name), (json email), (json password), fullname, about, image_url, plugin_extras, with_apitoken)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.task_status_update
    #[builder]
    pub async fn task_status_update(
        &self,
        id: String,
        entity_id: String,
        entity_type: String,
        task_type: String,
        key: String,
        value: Option<String>,
        state: Option<String>,
        last_updated: Option<String>,
        error: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "task_status_update", (json id), (json entity_id), (json entity_type), (json task_type), (json key), value, state, last_updated, error)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.task_status_update_many
    #[builder]
    pub async fn task_status_update_many(
        &self,
        data: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "task_status_update_many", (json data))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.term_translation_update
    #[builder]
    pub async fn term_translation_update(
        &self,
        term: String,
        term_translation: String,
        lang_code: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "term_translation_update", (json term), (json term_translation), (json lang_code))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.term_translation_update_many
    #[builder]
    pub async fn term_translation_update_many(
        &self,
        data: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "term_translation_update_many", (json data))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.vocabulary_update
    #[builder]
    pub async fn vocabulary_update(
        &self,
        id: String,
        name: String,
        tags: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "vocabulary_update", (json id), (json name), (json tags))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.package_owner_org_update
    #[builder]
    pub async fn package_owner_org_update(
        &self,
        id: String,
        organization_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "package_owner_org_update", (json id), (json organization_id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.bulk_update_private
    #[builder]
    pub async fn bulk_update_private(
        &self,
        datasets: Vec<String>,
        org_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "bulk_update_private", (json datasets), (json org_id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.bulk_update_public
    #[builder]
    pub async fn bulk_update_public(
        &self,
        datasets: Vec<String>,
        org_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "bulk_update_public", (json datasets), (json org_id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.bulk_update_delete
    #[builder]
    pub async fn bulk_update_delete(
        &self,
        datasets: Vec<String>,
        org_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "bulk_update_delete", (json datasets), (json org_id))
    }

    // TODO: Verify proper parameters
    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.config_option_update
    #[builder]
    pub async fn config_option_update(
        &self,
        options: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/config_option_update";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        let mut custom_map: HashMap<String, serde_json::Value> = HashMap::new();
        if let Some(options_obj) = options
            && options_obj.is_object()
        {
            let custom_temp_map = options_obj.as_object().unwrap();
            custom_map.extend(
                custom_temp_map
                    .iter()
                    .map(|item| (item.0.to_owned(), item.1.to_owned())),
            );
        }
        map.extend(
            custom_map
                .iter()
                .map(|item| (item.0.as_str(), item.1.to_owned())),
        );
        let body = hashmap_to_json(&map)?;
        Self::post(self).endpoint(endpoint).body(body).call().await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.patch.package_patch
    #[builder]
    pub async fn package_patch(
        &self,
        id: String,
        name: String,
        title: Option<String>,
        private: bool,
        author: Option<String>,
        author_email: Option<String>,
        maintainer: Option<String>,
        maintainer_email: Option<String>,
        license_id: Option<String>,
        notes: Option<String>,
        url: Option<String>,
        version: Option<String>,
        state: Option<String>,
        _type: Option<String>,
        resources: Option<Vec<serde_json::Value>>,
        tags: Option<Vec<serde_json::Value>>,
        extras: Option<Vec<serde_json::Value>>,
        plugin_data: Option<serde_json::Value>,
        relationships_as_object: Option<Vec<serde_json::Value>>,
        relationships_as_subject: Option<Vec<serde_json::Value>>,
        groups: Option<Vec<serde_json::Value>>,
        owner_org: Option<String>,
        custom_fields: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/package_patch";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert!(&mut map, (json id), (json name), title, (json private), author, author_email, maintainer, maintainer_email, license_id, notes, url, version, state, ("type", _type), resources, tags, extras, plugin_data, relationships_as_object, relationships_as_subject, groups, owner_org);
        let mut custom_map: HashMap<String, serde_json::Value> = HashMap::new();
        if let Some(custom) = custom_fields
            && custom.is_object()
        {
            let custom_temp_map = custom.as_object().unwrap();
            custom_map.extend(
                custom_temp_map
                    .iter()
                    .map(|item| (item.0.to_owned(), item.1.to_owned())),
            );
        }
        map.extend(
            custom_map
                .iter()
                .map(|item| (item.0.as_str(), item.1.to_owned())),
        );
        let body = hashmap_to_json(&map)?;
        Self::post(self).endpoint(endpoint).body(body).call().await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.patch.resource_patch
    #[builder]
    pub async fn resource_patch(
        &self,
        id: String,
        package_id: String,
        url: Option<String>,
        description: Option<String>,
        format: Option<String>,
        hash: Option<String>,
        name: Option<String>,
        resource_type: Option<String>,
        mimetype: Option<String>,
        mimetype_inner: Option<String>,
        cache_url: Option<String>,
        size: Option<i32>,
        created: Option<String>,
        last_modified: Option<String>,
        cache_last_updated: Option<String>,
        upload: Option<PathBuf>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "resource_patch", (json id), (json package_id), url, description, format, hash, name, resource_type, mimetype, mimetype_inner, cache_url, size, created, last_modified, cache_last_updated; (upload upload))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.group_patch
    #[builder]
    pub async fn group_patch(
        &self,
        id: String,
        name: String,
        title: Option<String>,
        description: Option<String>,
        image_url: Option<String>,
        _type: Option<String>,
        state: Option<String>,
        approval_status: Option<String>,
        extras: Option<Vec<serde_json::Value>>,
        packages: Option<Vec<serde_json::Value>>,
        groups: Option<Vec<serde_json::Value>>,
        users: Option<Vec<serde_json::Value>>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "group_patch", (json id), (json name), title, description, image_url, ("type", _type), state, approval_status, extras, packages, groups, users)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.patch.organization_patch
    #[builder]
    pub async fn organization_patch(
        &self,
        id: String,
        name: String,
        title: Option<String>,
        description: Option<String>,
        image_url: Option<String>,
        state: Option<String>,
        approval_status: Option<String>,
        extras: Option<Vec<serde_json::Value>>,
        users: Option<Vec<serde_json::Value>>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "organization_patch", (json id), (json name), title, description, image_url, state, approval_status, extras, users)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.patch.user_patch
    #[builder]
    pub async fn user_patch(
        &self,
        id: String,
        name: String,
        email: String,
        password: String,
        fullname: Option<String>,
        about: Option<String>,
        image_url: Option<String>,
        plugin_extras: Option<serde_json::Value>,
        with_apitoken: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "user_patch", (json id), (json name), (json email), (json password), fullname, about, image_url, plugin_extras, with_apitoken)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.user_delete
    #[builder]
    pub async fn user_delete(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "user_delete", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.package_delete
    #[builder]
    pub async fn package_delete(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "package_delete", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.dataset_purge
    #[builder]
    pub async fn dataset_purge(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "dataset_purge", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.resource_delete
    #[builder]
    pub async fn resource_delete(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "resource_delete", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.resource_view_delete
    #[builder]
    pub async fn resource_view_delete(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "resource_view_delete", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.resource_view_clear
    #[builder]
    pub async fn resource_view_clear(
        &self,
        view_types: Option<Vec<String>>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "resource_view_clear", view_types)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.package_relationship_delete
    #[builder]
    pub async fn package_relationship_delete(
        &self,
        subject: String,
        object: String,
        _type: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "package_relationship_delete", (json subject), (json object), ("type", json!(_type)))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.member_delete
    #[builder]
    pub async fn member_delete(
        &self,
        id: String,
        object: String,
        object_type: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "member_delete", (json id), (json object), (json object_type))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.package_collaborator_delete
    #[builder]
    pub async fn package_collaborator_delete(
        &self,
        id: String,
        user_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "package_collaborator_delete", (json id), (json user_id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.group_delete
    #[builder]
    pub async fn group_delete(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "group_delete", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.organization_delete
    #[builder]
    pub async fn organization_delete(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "organization_delete", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.group_purge
    #[builder]
    pub async fn group_purge(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "group_purge", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.organization_purge
    #[builder]
    pub async fn organization_purge(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "organization_purge", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.task_status_delete
    #[builder]
    pub async fn task_status_delete(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "task_status_delete", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.vocabulary_delete
    #[builder]
    pub async fn vocabulary_delete(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "vocabulary_delete", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.tag_delete
    #[builder]
    pub async fn tag_delete(
        &self,
        id: String,
        vocabulary_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "tag_delete", (json id), (json vocabulary_id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.unfollow_user
    #[builder]
    pub async fn unfollow_user(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "unfollow_user", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.unfollow_dataset
    #[builder]
    pub async fn unfollow_dataset(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "unfollow_dataset", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.group_member_delete
    #[builder]
    pub async fn group_member_delete(
        &self,
        id: String,
        username: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "group_member_delete", (json id), (json username))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.organization_member_delete
    #[builder]
    pub async fn organization_member_delete(
        &self,
        id: String,
        username: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "organization_member_delete", (json id), (json username))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.unfollow_group
    #[builder]
    pub async fn unfollow_group(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "unfollow_group", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.job_clear
    #[builder]
    pub async fn job_clear(
        &self,
        queues: Option<Vec<String>>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "job_clear", queues)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.job_cancel
    #[builder]
    pub async fn job_cancel(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "job_cancel", (json id))
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.api_token_revoke
    #[builder]
    pub async fn api_token_revoke(
        &self,
        token: String,
        jti: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        post!(&self, "api_token_revoke", (json token), jti)
    }
}
