//! # ckanaction
//!
//! Rust library crate to access [CKAN](https://ckan.org) Action API endpoints through Rust builders. Based on the CKAN Action API v3. Endpoints are expected to return with an output of type `serde_json::Value`.
//!
//! Learn more at [github.com/dathere/ckanaction](https://github.com/dathere/ckanaction).

use bon::bon;
use serde_json::json;
use std::{collections::HashMap, path::PathBuf};

pub struct CKAN {
    url: String,
    token: Option<String>,
}

fn opsert<'a, T: serde::ser::Serialize>(
    name: &'a str,
    value: Option<T>,
    map: &mut HashMap<&'a str, serde_json::Value>,
) {
    if value.is_some() {
        map.insert(name, json!(value));
        
    } ;
}

fn hashmap_to_json(
    map: &HashMap<&str, serde_json::Value>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(serde_json::to_string(&map)?.as_str())?)
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
        let endpoint = self.url.clone() + "/api/3/action/package_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("limit", limit, &mut map);
        opsert("offset", offset, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.current_package_list_with_resources
    #[builder]
    pub async fn current_package_list_with_resources(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
        page: Option<u32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/current_package_list_with_resources";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("limit", limit, &mut map);
        opsert("offset", offset, &mut map);
        opsert("page", page, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.member_list
    #[builder]
    pub async fn member_list(
        &self,
        id: String,
        object_type: Option<String>,
        capacity: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/member_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        opsert("object_type", object_type, &mut map);
        opsert("capacity", capacity, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.package_collaborator_list
    #[builder]
    pub async fn package_collaborator_list(
        &self,
        id: String,
        capacity: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/package_collaborator_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        opsert("capacity", capacity, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.package_collaborator_list_for_user
    #[builder]
    pub async fn package_collaborator_list_for_user(
        &self,
        id: String,
        capacity: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/package_collaborator_list_for_user";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        opsert("capacity", capacity, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/group_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("type", _type, &mut map);
        opsert("order_by", order_by, &mut map);
        opsert("sort", sort, &mut map);
        opsert("limit", limit, &mut map);
        opsert("offset", offset, &mut map);
        opsert("groups", groups, &mut map);
        opsert("all_fields", all_fields, &mut map);
        opsert("include_dataset_count", include_dataset_count, &mut map);
        opsert("include_extras", include_extras, &mut map);
        opsert("include_tags", include_tags, &mut map);
        opsert("include_groups", include_groups, &mut map);
        opsert("include_users", include_users, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/organization_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("type", _type, &mut map);
        opsert("order_by", order_by, &mut map);
        opsert("sort", sort, &mut map);
        opsert("limit", limit, &mut map);
        opsert("offset", offset, &mut map);
        opsert("organizations", organizations, &mut map);
        opsert("all_fields", all_fields, &mut map);
        opsert("include_dataset_count", include_dataset_count, &mut map);
        opsert("include_extras", include_extras, &mut map);
        opsert("include_tags", include_tags, &mut map);
        opsert("include_groups", include_groups, &mut map);
        opsert("include_users", include_users, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.group_list_authz
    #[builder]
    pub async fn group_list_authz(
        &self,
        available_only: Option<bool>,
        am_member: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/group_list_authz";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("available_only", available_only, &mut map);
        opsert("am_member", am_member, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.organization_list_for_user
    #[builder]
    pub async fn organization_list_for_user(
        &self,
        id: Option<String>,
        permission: Option<String>,
        include_dataset_count: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/organization_list_for_user";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("id", id, &mut map);
        opsert("permission", permission, &mut map);
        opsert("include_dataset_count", include_dataset_count, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.license_list
    #[builder]
    pub async fn license_list(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/license_list";
        Self::post(self).endpoint(endpoint).call().await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.tag_list
    #[builder]
    pub async fn tag_list(
        &self,
        query: Option<String>,
        vocabulary_id: Option<String>,
        all_fields: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/tag_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("query", query, &mut map);
        opsert("vocabulary_id", vocabulary_id, &mut map);
        opsert("all_fields", all_fields, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/user_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("q", q, &mut map);
        opsert("email", email, &mut map);
        opsert("order_by", order_by, &mut map);
        opsert("all_fields", all_fields, &mut map);
        opsert("include_site_user", include_site_user, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.package_relationships_list
    #[builder]
    pub async fn package_relationships_list(
        &self,
        id: String,
        id2: String,
        rel: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/package_relationships_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("id2", json!(id2));
        opsert("rel", rel, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.resource_show
    #[builder]
    pub async fn resource_show(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/resource_show";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.resource_view_show
    #[builder]
    pub async fn resource_view_show(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/resource_view_show";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.resource_view_list
    #[builder]
    pub async fn resource_view_list(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/resource_view_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/group_show";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        opsert("include_datasets", include_datasets, &mut map);
        opsert("include_dataset_count", include_dataset_count, &mut map);
        opsert("include_extras", include_extras, &mut map);
        opsert("include_users", include_users, &mut map);
        opsert("include_groups", include_groups, &mut map);
        opsert("include_tags", include_tags, &mut map);
        opsert("include_followers", include_followers, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/organization_show";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        opsert("include_datasets", include_datasets, &mut map);
        opsert("include_dataset_count", include_dataset_count, &mut map);
        opsert("include_extras", include_extras, &mut map);
        opsert("include_users", include_users, &mut map);
        opsert("include_groups", include_groups, &mut map);
        opsert("include_tags", include_tags, &mut map);
        opsert("include_followers", include_followers, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.group_package_show
    #[builder]
    pub async fn group_package_show(
        &self,
        id: String,
        limit: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/group_package_show";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        opsert("limit", limit, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.tag_show
    #[builder]
    pub async fn tag_show(
        &self,
        id: String,
        vocabulary_id: Option<String>,
        include_datasets: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/tag_show";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        opsert("vocabulary_id", vocabulary_id, &mut map);
        opsert("include_datasets", include_datasets, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/user_show";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        opsert("include_datasets", include_datasets, &mut map);
        opsert("include_num_followers", include_num_followers, &mut map);
        opsert("include_password_hash", include_password_hash, &mut map);
        opsert("include_plugin_extras", include_plugin_extras, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.package_autocomplete
    #[builder]
    pub async fn package_autocomplete(
        &self,
        q: String,
        limit: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/package_autocomplete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("q", json!(q));
        opsert("limit", limit, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.format_autocomplete
    #[builder]
    pub async fn format_autocomplete(
        &self,
        q: String,
        limit: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/format_autocomplete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("q", json!(q));
        opsert("limit", limit, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.user_autocomplete
    #[builder]
    pub async fn user_autocomplete(
        &self,
        q: String,
        limit: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/user_autocomplete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("q", json!(q));
        opsert("limit", limit, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.group_autocomplete
    #[builder]
    pub async fn group_autocomplete(
        &self,
        q: String,
        limit: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/group_autocomplete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("q", json!(q));
        opsert("limit", limit, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.organization_autocomplete
    #[builder]
    pub async fn organization_autocomplete(
        &self,
        q: String,
        limit: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/organization_autocomplete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("q", json!(q));
        opsert("limit", limit, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/package_search";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("q", q, &mut map);
        opsert("fq", fq, &mut map);
        opsert("fq_list", fq_list, &mut map);
        opsert("sort", sort, &mut map);
        opsert("rows", rows, &mut map);
        opsert("start", start, &mut map);
        opsert("facet", facet, &mut map);
        opsert("facet.mincount", facet_mincount, &mut map);
        opsert("facet.limit", facet_limit, &mut map);
        opsert("facet.field", facet_field, &mut map);
        opsert("include_drafts", include_drafts, &mut map);
        opsert("include_private", include_private, &mut map);
        opsert("use_default_schema", use_default_schema, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/resource_search";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("query", query, &mut map);
        opsert("order_by", order_by, &mut map);
        opsert("offset", offset, &mut map);
        opsert("limit", limit, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/tag_search";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("query", query, &mut map);
        opsert("vocabulary_id", vocabulary_id, &mut map);
        opsert("limit", limit, &mut map);
        opsert("offset", offset, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/tag_autocomplete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("query", json!(query));
        opsert("vocabulary_id", vocabulary_id, &mut map);
        opsert("limit", limit, &mut map);
        opsert("offset", offset, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/task_status_show";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("id", id, &mut map);
        opsert("entity_id", entity_id, &mut map);
        opsert("task_type", task_type, &mut map);
        opsert("key", key, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.term_translation_show
    #[builder]
    pub async fn term_translation_show(
        &self,
        terms: Option<Vec<String>>,
        lang_codes: Option<Vec<String>>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/term_translation_show";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("terms", terms, &mut map);
        opsert("lang_codes", lang_codes, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.get_site_user
    #[builder]
    pub async fn get_site_user(
        &self,
        defer_commit: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/get_site_user";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("defer_commit", defer_commit, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/vocabulary_show";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.user_follower_count
    #[builder]
    pub async fn user_follower_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/user_follower_count";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.dataset_follower_count
    #[builder]
    pub async fn dataset_follower_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/dataset_follower_count";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.group_follower_count
    #[builder]
    pub async fn group_follower_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/group_follower_count";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.organization_follower_count
    #[builder]
    pub async fn organization_follower_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/organization_follower_count";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.user_follower_list
    #[builder]
    pub async fn user_follower_list(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/user_follower_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.dataset_follower_list
    #[builder]
    pub async fn dataset_follower_list(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/dataset_follower_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.group_follower_list
    #[builder]
    pub async fn group_follower_list(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/group_follower_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.am_following_user
    #[builder]
    pub async fn am_following_user(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/am_following_user";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.am_following_dataset
    #[builder]
    pub async fn am_following_dataset(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/am_following_dataset";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.am_following_group
    #[builder]
    pub async fn am_following_group(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/am_following_group";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.followee_count
    #[builder]
    pub async fn followee_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/followee_count";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.user_followee_count
    #[builder]
    pub async fn user_followee_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/user_followee_count";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.dataset_followee_count
    #[builder]
    pub async fn dataset_followee_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/dataset_followee_count";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.group_followee_count
    #[builder]
    pub async fn group_followee_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/group_followee_count";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.organization_followee_count
    #[builder]
    pub async fn organization_followee_count(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/organization_followee_count";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.followee_list
    #[builder]
    pub async fn followee_list(
        &self,
        id: String,
        q: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/followee_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        opsert("q", q, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.user_followee_list
    #[builder]
    pub async fn user_followee_list(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/user_followee_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.dataset_followee_list
    #[builder]
    pub async fn dataset_followee_list(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/dataset_followee_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.group_followee_list
    #[builder]
    pub async fn group_followee_list(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/group_followee_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.organization_followee_list
    #[builder]
    pub async fn organization_followee_list(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/organization_followee_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.member_roles_list
    #[builder]
    pub async fn member_roles_list(
        &self,
        group_type: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/member_roles_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("group_type", group_type, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.help_show
    #[builder]
    pub async fn help_show(
        &self,
        name: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/help_show";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("name", json!(name));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.config_option_show
    #[builder]
    pub async fn config_option_show(
        &self,
        key: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/config_option_show";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("key", json!(key));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/job_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("queues", queues, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.job_show
    #[builder]
    pub async fn job_show(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/job_show";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.api_token_list
    #[builder]
    pub async fn api_token_list(
        &self,
        user_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/api_token_list";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("user_id", json!(user_id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        map.insert("name", json!(name));
        opsert("title", title, &mut map);
        map.insert("private", json!(private));
        opsert("author", author, &mut map);
        opsert("author_email", author_email, &mut map);
        opsert("maintainer", maintainer, &mut map);
        opsert("maintainer_email", maintainer_email, &mut map);
        opsert("license_id", license_id, &mut map);
        opsert("notes", notes, &mut map);
        opsert("url", url, &mut map);
        opsert("version", version, &mut map);
        opsert("state", state, &mut map);
        opsert("_type", _type, &mut map);
        opsert("resources", resources, &mut map);
        opsert("tags", tags, &mut map);
        opsert("extras", extras, &mut map);
        opsert("plugin_data", plugin_data, &mut map);
        opsert("relationships_as_object", relationships_as_object, &mut map);
        opsert(
            "relationships_as_subject",
            relationships_as_subject,
            &mut map,
        );
        let mut custom_map: HashMap<String, serde_json::Value> = HashMap::new();
        opsert("groups", groups, &mut map);
        opsert("owner_org", owner_org, &mut map);
        if let Some(custom) = custom_fields
            && custom.is_object() {
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
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/resource_create";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("package_id", json!(package_id));
        opsert("url", url, &mut map);
        opsert("description", description, &mut map);
        opsert("format", format, &mut map);
        opsert("hash", hash, &mut map);
        opsert("name", name, &mut map);
        opsert("resource_type", resource_type, &mut map);
        opsert("mimetype", mimetype, &mut map);
        opsert("mimetype_inner", mimetype_inner, &mut map);
        opsert("cache_url", cache_url, &mut map);
        opsert("size", size, &mut map);
        opsert("created", created, &mut map);
        opsert("last_modified", last_modified, &mut map);
        opsert("cache_last_updated", cache_last_updated, &mut map);

        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .maybe_upload(upload)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/resource_view_create";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("resource_id", json!(resource_id));
        map.insert("title", json!(title));
        opsert("description", description, &mut map);
        map.insert("view_type", json!(view_type));
        opsert("config", config, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.create_default_resource_views
    #[builder]
    pub async fn create_default_resource_views(
        &self,
        resource: serde_json::Value,
        package: Option<serde_json::Value>,
        create_datastore_views: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/create_default_resource_views";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("resource", resource);
        opsert("package", package, &mut map);
        opsert("create_datastore_views", create_datastore_views, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.package_create_default_resource_views
    #[builder]
    pub async fn package_create_default_resource_views(
        &self,
        package: serde_json::Value,
        create_datastore_views: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/package_create_default_resource_views";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("package", package);
        opsert("create_datastore_views", create_datastore_views, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/package_relationship_create";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("subject", json!(subject));
        map.insert("object", json!(object));
        map.insert("type", json!(_type));
        opsert("comment", comment, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/member_create";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("object", json!(object));
        map.insert("object_type", json!(object_type));
        map.insert("capacity", json!(capacity));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.package_collaborator_create
    #[builder]
    pub async fn package_collaborator_create(
        &self,
        id: String,
        user_id: String,
        capacity: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/package_collaborator_create";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("user_id", json!(user_id));
        map.insert("capacity", json!(capacity));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/group_create";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("name", json!(name));
        opsert("id", id, &mut map);
        opsert("title", title, &mut map);
        opsert("description", description, &mut map);
        opsert("image_url", image_url, &mut map);
        opsert("_type", _type, &mut map);
        opsert("state", state, &mut map);
        opsert("approval_status", approval_status, &mut map);
        opsert("extras", extras, &mut map);
        opsert("packages", packages, &mut map);
        opsert("groups", groups, &mut map);
        opsert("users", users, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/organization_create";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("name", json!(name));
        opsert("id", id, &mut map);
        opsert("title", title, &mut map);
        opsert("description", description, &mut map);
        opsert("image_url", image_url, &mut map);
        opsert("state", state, &mut map);
        opsert("approval_status", approval_status, &mut map);
        opsert("extras", extras, &mut map);
        opsert("packages", packages, &mut map);
        opsert("users", users, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/user_create";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("name", json!(name));
        map.insert("email", json!(email));
        map.insert("password", json!(password));
        opsert("id", id, &mut map);
        opsert("fullname", fullname, &mut map);
        opsert("about", about, &mut map);
        opsert("image_url", image_url, &mut map);
        opsert("plugin_extras", plugin_extras, &mut map);
        opsert("with_apitoken", with_apitoken, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.user_invite
    #[builder]
    pub async fn user_invite(
        &self,
        email: String,
        group_id: String,
        role: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/user_invite";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("email", json!(email));
        map.insert("group_id", json!(group_id));
        map.insert("role", json!(role));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.vocabulary_create
    #[builder]
    pub async fn vocabulary_create(
        &self,
        name: String,
        tags: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/vocabulary_create";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("name", json!(name));
        map.insert("tags", json!(tags));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.tag_create
    #[builder]
    pub async fn tag_create(
        &self,
        name: String,
        vocabulary_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/tag_create";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("name", json!(name));
        map.insert("vocabulary_id", json!(vocabulary_id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.follow_user
    #[builder]
    pub async fn follow_user(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/follow_user";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.follow_dataset
    #[builder]
    pub async fn follow_dataset(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/follow_dataset";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.group_member_create
    #[builder]
    pub async fn group_member_create(
        &self,
        id: String,
        username: String,
        role: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/group_member_create";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("username", json!(username));
        map.insert("role", json!(role));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.organization_member_create
    #[builder]
    pub async fn organization_member_create(
        &self,
        id: String,
        username: String,
        role: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/organization_member_create";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("username", json!(username));
        map.insert("role", json!(role));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.follow_group
    #[builder]
    pub async fn follow_group(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/follow_group";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.create.api_token_create
    #[builder]
    pub async fn api_token_create(
        &self,
        user: String,
        name: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/api_token_create";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("user", json!(user));
        map.insert("name", json!(name));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/resource_update";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("insert", json!(id));
        map.insert("package_id", json!(package_id));
        opsert("url", url, &mut map);
        opsert("description", description, &mut map);
        opsert("format", format, &mut map);
        opsert("hash", hash, &mut map);
        opsert("name", name, &mut map);
        opsert("resource_type", resource_type, &mut map);
        opsert("mimetype", mimetype, &mut map);
        opsert("mimetype_inner", mimetype_inner, &mut map);
        opsert("cache_url", cache_url, &mut map);
        opsert("size", size, &mut map);
        opsert("created", created, &mut map);
        opsert("last_modified", last_modified, &mut map);
        opsert("cache_last_updated", cache_last_updated, &mut map);

        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .maybe_upload(upload)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/resource_view_update";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("resource_id", json!(resource_id));
        map.insert("title", json!(title));
        opsert("description", description, &mut map);
        map.insert("view_type", json!(view_type));
        opsert("config", config, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.resource_view_reorder
    #[builder]
    pub async fn resource_view_reorder(
        &self,
        id: String,
        order: Vec<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/resource_view_reorder";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("order", json!(order));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        map.insert("id", json!(id));
        map.insert("name", json!(name));
        opsert("title", title, &mut map);
        map.insert("private", json!(private));
        opsert("author", author, &mut map);
        opsert("author_email", author_email, &mut map);
        opsert("maintainer", maintainer, &mut map);
        opsert("maintainer_email", maintainer_email, &mut map);
        opsert("license_id", license_id, &mut map);
        opsert("notes", notes, &mut map);
        opsert("url", url, &mut map);
        opsert("version", version, &mut map);
        opsert("state", state, &mut map);
        opsert("_type", _type, &mut map);
        opsert("resources", resources, &mut map);
        opsert("tags", tags, &mut map);
        opsert("extras", extras, &mut map);
        opsert("plugin_data", plugin_data, &mut map);
        opsert("relationships_as_object", relationships_as_object, &mut map);
        opsert(
            "relationships_as_subject",
            relationships_as_subject,
            &mut map,
        );
        let mut custom_map: HashMap<String, serde_json::Value> = HashMap::new();
        opsert("groups", groups, &mut map);
        opsert("owner_org", owner_org, &mut map);
        if let Some(custom) = custom_fields
            && custom.is_object() {
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
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/package_revise";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("match", _match);
        opsert("filter", filter, &mut map);
        map.insert("update", update);
        opsert("include", include, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.package_resource_reorder
    #[builder]
    pub async fn package_resource_reorder(
        &self,
        id: String,
        order: Vec<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/package_resource_reorder";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("order", json!(order));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/package_relationship_update";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("subject", json!(subject));
        map.insert("object", json!(object));
        map.insert("type", json!(_type));
        opsert("comment", comment, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/group_update";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("name", json!(name));
        opsert("title", title, &mut map);
        opsert("description", description, &mut map);
        opsert("image_url", image_url, &mut map);
        opsert("_type", _type, &mut map);
        opsert("state", state, &mut map);
        opsert("approval_status", approval_status, &mut map);
        opsert("extras", extras, &mut map);
        opsert("packages", packages, &mut map);
        opsert("groups", groups, &mut map);
        opsert("users", users, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/organization_update";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("name", json!(name));
        opsert("title", title, &mut map);
        opsert("description", description, &mut map);
        opsert("image_url", image_url, &mut map);
        opsert("state", state, &mut map);
        opsert("approval_status", approval_status, &mut map);
        opsert("extras", extras, &mut map);
        opsert("users", users, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/user_update";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("name", json!(name));
        map.insert("email", json!(email));
        map.insert("password", json!(password));
        opsert("fullname", fullname, &mut map);
        opsert("about", about, &mut map);
        opsert("image_url", image_url, &mut map);
        opsert("plugin_extras", plugin_extras, &mut map);
        opsert("with_apitoken", with_apitoken, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/task_status_update";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("entity_id", json!(entity_id));
        map.insert("entity_type", json!(entity_type));
        map.insert("task_type", json!(task_type));
        map.insert("key", json!(key));
        opsert("value", value, &mut map);
        opsert("state", state, &mut map);
        opsert("last_updated", last_updated, &mut map);
        opsert("error", error, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.task_status_update_many
    #[builder]
    pub async fn task_status_update_many(
        &self,
        data: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/task_status_update_many";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("data", json!(data));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.term_translation_update
    #[builder]
    pub async fn term_translation_update(
        &self,
        term: String,
        term_translation: String,
        lang_code: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/term_translation_update";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("term", json!(term));
        map.insert("term_translation", json!(term_translation));
        map.insert("lang_code", json!(lang_code));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.term_translation_update_many
    #[builder]
    pub async fn term_translation_update_many(
        &self,
        data: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/term_translation_update_many";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("data", json!(data));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.vocabulary_update
    #[builder]
    pub async fn vocabulary_update(
        &self,
        id: String,
        name: String,
        tags: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/vocabulary_update";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("name", json!(name));
        map.insert("tags", json!(tags));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.package_owner_org_update
    #[builder]
    pub async fn package_owner_org_update(
        &self,
        id: String,
        organization_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/package_owner_org_update";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("organization_id", json!(organization_id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.bulk_update_private
    #[builder]
    pub async fn bulk_update_private(
        &self,
        datasets: Vec<String>,
        org_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/bulk_update_private";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("datasets", json!(datasets));
        map.insert("org_id", json!(org_id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.bulk_update_public
    #[builder]
    pub async fn bulk_update_public(
        &self,
        datasets: Vec<String>,
        org_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/bulk_update_public";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("datasets", json!(datasets));
        map.insert("org_id", json!(org_id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.update.bulk_update_delete
    #[builder]
    pub async fn bulk_update_delete(
        &self,
        datasets: Vec<String>,
        org_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/bulk_update_delete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("datasets", json!(datasets));
        map.insert("org_id", json!(org_id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
            && options_obj.is_object() {
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
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        map.insert("id", json!(id));
        map.insert("name", json!(name));
        opsert("title", title, &mut map);
        map.insert("private", json!(private));
        opsert("author", author, &mut map);
        opsert("author_email", author_email, &mut map);
        opsert("maintainer", maintainer, &mut map);
        opsert("maintainer_email", maintainer_email, &mut map);
        opsert("license_id", license_id, &mut map);
        opsert("notes", notes, &mut map);
        opsert("url", url, &mut map);
        opsert("version", version, &mut map);
        opsert("state", state, &mut map);
        opsert("_type", _type, &mut map);
        opsert("resources", resources, &mut map);
        opsert("tags", tags, &mut map);
        opsert("extras", extras, &mut map);
        opsert("plugin_data", plugin_data, &mut map);
        opsert("relationships_as_object", relationships_as_object, &mut map);
        opsert(
            "relationships_as_subject",
            relationships_as_subject,
            &mut map,
        );
        let mut custom_map: HashMap<String, serde_json::Value> = HashMap::new();
        opsert("groups", groups, &mut map);
        opsert("owner_org", owner_org, &mut map);
        if let Some(custom) = custom_fields
            && custom.is_object() {
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
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/resource_patch";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("insert", json!(id));
        map.insert("package_id", json!(package_id));
        opsert("url", url, &mut map);
        opsert("description", description, &mut map);
        opsert("format", format, &mut map);
        opsert("hash", hash, &mut map);
        opsert("name", name, &mut map);
        opsert("resource_type", resource_type, &mut map);
        opsert("mimetype", mimetype, &mut map);
        opsert("mimetype_inner", mimetype_inner, &mut map);
        opsert("cache_url", cache_url, &mut map);
        opsert("size", size, &mut map);
        opsert("created", created, &mut map);
        opsert("last_modified", last_modified, &mut map);
        opsert("cache_last_updated", cache_last_updated, &mut map);

        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .maybe_upload(upload)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/group_patch";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("name", json!(name));
        opsert("title", title, &mut map);
        opsert("description", description, &mut map);
        opsert("image_url", image_url, &mut map);
        opsert("_type", _type, &mut map);
        opsert("state", state, &mut map);
        opsert("approval_status", approval_status, &mut map);
        opsert("extras", extras, &mut map);
        opsert("packages", packages, &mut map);
        opsert("groups", groups, &mut map);
        opsert("users", users, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/organization_patch";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("name", json!(name));
        opsert("title", title, &mut map);
        opsert("description", description, &mut map);
        opsert("image_url", image_url, &mut map);
        opsert("state", state, &mut map);
        opsert("approval_status", approval_status, &mut map);
        opsert("extras", extras, &mut map);
        opsert("users", users, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
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
        let endpoint = self.url.clone() + "/api/3/action/user_patch";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("name", json!(name));
        map.insert("email", json!(email));
        map.insert("password", json!(password));
        opsert("fullname", fullname, &mut map);
        opsert("about", about, &mut map);
        opsert("image_url", image_url, &mut map);
        opsert("plugin_extras", plugin_extras, &mut map);
        opsert("with_apitoken", with_apitoken, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.user_delete
    #[builder]
    pub async fn user_delete(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/user_delete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.package_delete
    #[builder]
    pub async fn package_delete(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/package_delete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.dataset_purge
    #[builder]
    pub async fn dataset_purge(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/dataset_purge";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.resource_delete
    #[builder]
    pub async fn resource_delete(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/resource_delete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.resource_view_delete
    #[builder]
    pub async fn resource_view_delete(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/resource_view_delete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.resource_view_clear
    #[builder]
    pub async fn resource_view_clear(
        &self,
        view_types: Option<Vec<String>>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/resource_view_clear";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("view_types", view_types, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.package_relationship_delete
    #[builder]
    pub async fn package_relationship_delete(
        &self,
        subject: String,
        object: String,
        _type: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/package_relationship_delete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("subject", json!(subject));
        map.insert("object", json!(object));
        map.insert("type", json!(_type));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.member_delete
    #[builder]
    pub async fn member_delete(
        &self,
        id: String,
        object: String,
        object_type: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/member_delete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("object", json!(object));
        map.insert("object_type", json!(object_type));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.package_collaborator_delete
    #[builder]
    pub async fn package_collaborator_delete(
        &self,
        id: String,
        user_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/package_collaborator_delete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("user_id", json!(user_id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.group_delete
    #[builder]
    pub async fn group_delete(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/group_delete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.organization_delete
    #[builder]
    pub async fn organization_delete(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/organization_delete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.group_purge
    #[builder]
    pub async fn group_purge(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/group_purge";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.organization_purge
    #[builder]
    pub async fn organization_purge(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/organization_purge";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.task_status_delete
    #[builder]
    pub async fn task_status_delete(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/task_status_delete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.vocabulary_delete
    #[builder]
    pub async fn vocabulary_delete(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/vocabulary_delete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.tag_delete
    #[builder]
    pub async fn tag_delete(
        &self,
        id: String,
        vocabulary_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/tag_delete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("vocabulary_id", json!(vocabulary_id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.unfollow_user
    #[builder]
    pub async fn unfollow_user(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/unfollow_user";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.unfollow_dataset
    #[builder]
    pub async fn unfollow_dataset(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/unfollow_dataset";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.group_member_delete
    #[builder]
    pub async fn group_member_delete(
        &self,
        id: String,
        username: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/group_member_delete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("username", json!(username));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.organization_member_delete
    #[builder]
    pub async fn organization_member_delete(
        &self,
        id: String,
        username: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/organization_member_delete";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        map.insert("username", json!(username));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.unfollow_group
    #[builder]
    pub async fn unfollow_group(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/unfollow_group";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.job_clear
    #[builder]
    pub async fn job_clear(
        &self,
        queues: Option<Vec<String>>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/job_clear";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("queues", queues, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.job_cancel
    #[builder]
    pub async fn job_cancel(
        &self,
        id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/job_cancel";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("id", json!(id));
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.delete.api_token_revoke
    #[builder]
    pub async fn api_token_revoke(
        &self,
        token: String,
        jti: Option<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/api_token_revoke";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        map.insert("token", json!(token));
        opsert("jti", jti, &mut map);
        let body = hashmap_to_json(&map)?;
        Self::post(self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await
    }
}
