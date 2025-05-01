use bon::bon;
use serde_json::json;
use std::collections::HashMap;

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
        ()
    } else {
    };
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

    async fn get(endpoint: String) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        Ok(reqwest::get(endpoint)
            .await?
            .json::<serde_json::Value>()
            .await?)
    }

    #[builder]
    async fn post(
        &self,
        endpoint: String,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let mut req_builder = client.post(endpoint);
        if self.token.is_some() {
            req_builder = req_builder.header("Authorization", self.token.clone().unwrap());
        }
        let res = req_builder.json(&body).send().await?.json().await?;
        Ok(res)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.license_list
    #[builder]
    pub async fn license_list(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/license_list";
        Ok(Self::post(&self).endpoint(endpoint).call().await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.get_site_user
    #[builder]
    pub async fn get_site_user<T: serde::Serialize>(
        &self,
        defer_commit: Option<bool>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/get_site_user";
        let mut map: HashMap<&str, serde_json::Value> = HashMap::new();
        opsert("defer_commit", defer_commit, &mut map);
        let body = hashmap_to_json(&map)?;
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.status_show
    pub async fn status_show(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/status_show";
        Ok(Self::get(endpoint).await?)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.vocabulary_list
    pub async fn vocabulary_list(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/vocabulary_list";
        Ok(Self::get(endpoint).await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
    }

    /// https://docs.ckan.org/en/2.11/api/index.html#ckan.logic.action.get.config_option_list
    pub async fn config_option_list(
        &self,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint = self.url.clone() + "/api/3/action/config_option_list";
        Ok(Self::get(endpoint).await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
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
        Ok(Self::post(&self)
            .endpoint(endpoint)
            .body(body)
            .call()
            .await?)
    }
}
