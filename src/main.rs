use anyhow::Result;
use rmcp::{
    ErrorData, ServerHandler, ServiceExt,
    model::{
        CallToolRequestParam, CallToolResult, Content, ErrorCode, ListToolsResult, PaginatedRequestParam, Tool,
        ServerCapabilities,
    },
    service::RequestContext,
    transport::stdio,
    RoleServer,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Cat {
    id: u32,
    name: String,
    age: u32,
    breed: String,
    color: String,
    is_indoor: bool,
    favorite_toy: String,
}

struct CatServer {
    cats: HashMap<u32, Cat>,
}

impl CatServer {
    fn new() -> Self {
        let mut cats = HashMap::new();
        
        // Initialize with sample cat data
        cats.insert(1, Cat {
            id: 1,
            name: "Mike".to_string(),
            age: 3,
            breed: "Calico".to_string(),
            color: "Calico".to_string(),
            is_indoor: true,
            favorite_toy: "Mouse toy".to_string(),
        });
        
        cats.insert(2, Cat {
            id: 2,
            name: "Shiro".to_string(),
            age: 5,
            breed: "Persian".to_string(),
            color: "White".to_string(),
            is_indoor: true,
            favorite_toy: "Yarn ball".to_string(),
        });
        
        cats.insert(3, Cat {
            id: 3,
            name: "Kuro".to_string(),
            age: 2,
            breed: "Black cat".to_string(),
            color: "Black".to_string(),
            is_indoor: false,
            favorite_toy: "Butterfly".to_string(),
        });
        
        cats.insert(4, Cat {
            id: 4,
            name: "Chatora".to_string(),
            age: 7,
            breed: "Orange tabby".to_string(),
            color: "Orange tabby".to_string(),
            is_indoor: true,
            favorite_toy: "Catnip".to_string(),
        });

        Self { cats }
    }
}

impl ServerHandler for CatServer {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        rmcp::model::ServerInfo {
            protocol_version: rmcp::model::ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: rmcp::model::Implementation {
                name: "cat-database-server".to_string(),
                version: "1.0.0".to_string(),
            },
            instructions: Some("A Cat Database MCP Server that provides tools to manage and query cat data. Use the available tools to list all cats, get specific cat information by ID, search by breed, or filter for indoor cats only.".to_string()),
        }
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        let tools = vec![
            Tool {
                name: "list_all_cats".into(),
                description: Some("Get a list of all cats".into()),
                input_schema: {
                    let mut map = serde_json::Map::new();
                    map.insert("type".to_string(), serde_json::Value::String("object".to_string()));
                    map.insert("properties".to_string(), serde_json::json!({}));
                    map.insert("required".to_string(), serde_json::json!([]));
                    Arc::new(map)
                },
                annotations: None,
            },
            Tool {
                name: "get_cat_by_id".into(),
                description: Some("Get information about a specific cat by ID".into()),
                input_schema: {
                    let mut map = serde_json::Map::new();
                    map.insert("type".to_string(), serde_json::Value::String("object".to_string()));
                    map.insert("properties".to_string(), serde_json::json!({
                        "id": {
                            "type": "number",
                            "description": "Cat ID"
                        }
                    }));
                    map.insert("required".to_string(), serde_json::json!(["id"]));
                    Arc::new(map)
                },
                annotations: None,
            },
            Tool {
                name: "search_by_breed".into(),
                description: Some("Search for cats by breed".into()),
                input_schema: {
                    let mut map = serde_json::Map::new();
                    map.insert("type".to_string(), serde_json::Value::String("object".to_string()));
                    map.insert("properties".to_string(), serde_json::json!({
                        "breed": {
                            "type": "string",
                            "description": "Breed to search for"
                        }
                    }));
                    map.insert("required".to_string(), serde_json::json!(["breed"]));
                    Arc::new(map)
                },
                annotations: None,
            },
            Tool {
                name: "get_indoor_cats".into(),
                description: Some("Get only indoor cats".into()),
                input_schema: {
                    let mut map = serde_json::Map::new();
                    map.insert("type".to_string(), serde_json::Value::String("object".to_string()));
                    map.insert("properties".to_string(), serde_json::json!({}));
                    map.insert("required".to_string(), serde_json::json!([]));
                    Arc::new(map)
                },
                annotations: None,
            },
        ];
        
        Ok(ListToolsResult {
            tools,
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = match request.name.as_ref() {
            "list_all_cats" => {
                let cats: Vec<&Cat> = self.cats.values().collect();
                let content = serde_json::to_string_pretty(&cats).map_err(|e| ErrorData {
                    code: ErrorCode::INTERNAL_ERROR,
                    message: format!("Serialization error: {}", e).into(),
                    data: None,
                })?;
                
                vec![Content::text(format!("All registered cats ({} cats):\n{}", cats.len(), content))]
            },
            "get_cat_by_id" => {
                let id: u32 = request.arguments
                    .as_ref()
                    .and_then(|args| args.get("id"))
                    .and_then(|v| v.as_u64())
                    .map(|v| v as u32)
                    .ok_or_else(|| ErrorData {
                        code: ErrorCode::INVALID_PARAMS,
                        message: "ID is required".into(),
                        data: None,
                    })?;
                
                if let Some(cat) = self.cats.get(&id) {
                    let content = serde_json::to_string_pretty(cat).map_err(|e| ErrorData {
                        code: ErrorCode::INTERNAL_ERROR,
                        message: format!("Serialization error: {}", e).into(),
                        data: None,
                    })?;
                    vec![Content::text(format!("Cat details (ID: {}):\n{}", id, content))]
                } else {
                    vec![Content::text(format!("Cat with ID {} not found", id))]
                }
            },
            "search_by_breed" => {
                let breed = request.arguments
                    .as_ref()
                    .and_then(|args| args.get("breed"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ErrorData {
                        code: ErrorCode::INVALID_PARAMS,
                        message: "Breed is required".into(),
                        data: None,
                    })?;
                
                let matching_cats: Vec<&Cat> = self.cats
                    .values()
                    .filter(|cat| cat.breed.contains(breed))
                    .collect();
                
                if matching_cats.is_empty() {
                    vec![Content::text(format!("No cats found with breed \"{}\"", breed))]
                } else {
                    let content = serde_json::to_string_pretty(&matching_cats).map_err(|e| ErrorData {
                        code: ErrorCode::INTERNAL_ERROR,
                        message: format!("Serialization error: {}", e).into(),
                        data: None,
                    })?;
                    vec![Content::text(format!("Cats with breed \"{}\" ({} cats):\n{}", breed, matching_cats.len(), content))]
                }
            },
            "get_indoor_cats" => {
                let indoor_cats: Vec<&Cat> = self.cats
                    .values()
                    .filter(|cat| cat.is_indoor)
                    .collect();
                
                let content = serde_json::to_string_pretty(&indoor_cats).map_err(|e| ErrorData {
                    code: ErrorCode::INTERNAL_ERROR,
                    message: format!("Serialization error: {}", e).into(),
                    data: None,
                })?;
                vec![Content::text(format!("Indoor cats ({} cats):\n{}", indoor_cats.len(), content))]
            },
            _ => return Err(ErrorData {
                code: ErrorCode::METHOD_NOT_FOUND,
                message: format!("Unknown tool: {}", request.name).into(),
                data: None,
            }),
        };
        
        Ok(CallToolResult {
            content: result,
            is_error: Some(false),
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("🐱 Starting Cat Database MCP Server...");

    let server = CatServer::new();

    info!("📡 Starting MCP server with stdio transport");
    let service = server.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;
    
    service.waiting().await?;
    
    Ok(())
}
