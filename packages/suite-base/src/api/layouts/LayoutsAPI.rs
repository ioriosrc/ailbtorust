```rust
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct CreateLayoutRequest {
    pub layout_id: String,
    pub data: String,
    pub name: String,
    pub permission: i32,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutApiResponse {
    pub layout_id: String,
    pub id: String,
    pub name: String,
    pub data: String,
    pub permission: i32,
    pub updated_by: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkspaceLayoutResponse;

#[derive(Serialize, Deserialize)]
pub struct UpdateLayoutRequest {
    pub name: String,
    pub data: String,
    pub permission: i32,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateLayoutRequestBody {
    pub name: String,
    pub data: String,
    pub permission: i32,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutResponse {
    pub status: &'static str,
    pub new_layout: RemoteLayout,
}

// Assuming HttpService is implemented elsewhere
use reqwest::Client;

struct HttpService {}

impl HttpService {
    async fn get<T>(url: &str) -> Result<T, serde_json::Error> {
        Client::new()
            .get(url)
            .send()
            .await?
            .json()
            .await
    }

    async fn post<T, U>(url: &str, payload: U) -> Result<T, serde_json::Error> {
        Client::new()
            .post(url)
            .json(payload)
            .send()
            .await?
            .json()
    }

    async fn put<T, U>(url: &str, payload: U) -> Result<T, serde_json::Error> {
        Client::new()
            .put(url)
            .json(payload)
            .send()
            .await?
            .json()
    }

    async fn delete<T>(url: &str) -> Result<Option<T>, serde_json::Error> {
        Client::new()
            .delete(url)
            .send()
            .await?
            .json::<Option<T>>()
    }
}

#[derive(Serialize, Deserialize)]
pub struct RemoteLayout {
    pub id: String,
    pub external_id: String,
    pub name: String,
    pub data: String,
    pub permission: i32,
    pub saved_at: Option<NaiveDateTime>,
}

struct LayoutsAPI {
    workspace: String,
}

impl LayoutsAPI {
    pub fn new(workspace: String) -> Self {
        Self { workspace }
    }

    async fn get_layouts(&self) -> Result<Vec<RemoteLayout>, serde_json::Error> {
        let response = HttpService::get::<LayoutApiResponse[]>(
            &format!("workspaces/{}/layouts", self.workspace),
        )?;
        Ok(response.into_iter().map(|layout| {
            RemoteLayout {
                id: layout.layout_id,
                external_id: layout.id,
                name: layout.name,
                data: layout.data,
                permission: layout.permission,
                saved_at: layout.updated_by.map(NaiveDateTime::from),
            }
        }).collect())
    }

    async fn get_layout(&self, _layout_id: &str) -> Result<Option<RemoteLayout>, serde_json::Error> {
        unimplemented!()
    }

    async fn save_new_layout(&self, params: CreateLayoutRequest) -> Result<RemoteLayout, serde_json::Error> {
        let request_payload = CreateLayoutRequest {
            layout_id: params.layout_id,
            data: params.data,
            name: params.name,
            permission: params.permission,
        };

        let response = HttpService::post::<WorkspaceLayoutResponse>(
            &format!("workspaces/{}/layout", self.workspace),
            request_payload,
        )?;
        Ok(RemoteLayout {
            id: response.layout_id,
            external_id: response.id,
            name: response.name,
            data: response.data,
            permission: response.permission,
            saved_at: response.updated_by.map(NaiveDateTime::from),
        })
    }

    async fn update_layout(&self, params: UpdateLayoutRequest) -> Result<LayoutResponse, serde_json::Error> {
        let requestBody = UpdateLayoutRequestBody {
            name: params.name,
            data: params.data,
            permission: params.permission,
        };

        let response = HttpService::put::<LayoutApiResponse>(
            &format!("layouts/{}", params.external_id),
            requestBody,
        )?;
        Ok(LayoutResponse {
            status: "success",
            new_layout: RemoteLayout {
                id: response.layout_id,
                external_id: response.id,
                name: response.name,
                data: response.data,
                permission: response.permission,
                saved_at: response.updated_by.map(NaiveDateTime::from),
            },
        })
    }

    async fn delete_layout(&self, layout_id: &str) -> Result<bool, serde_json::Error> {
        let deleted_layout = HttpService::delete::<RemoteLayout | Option<RemoteLayout>>(
            &format!("workspaces/{}/layout/{}", self.workspace, layout_id),
        )?;
        Ok(deleted_layout.is_some())
    }
}
```