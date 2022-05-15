use tonic::{Request, Response, Status};

use crate::proto_service::{self, CreateItemRequest, CreateItemResponse};




type ServerResult<T> = Result<Response<T>, Status>;

#[derive(Default)]
pub struct SSTServer;

#[tonic::async_trait]
impl proto_service::todo_list_server::TodoList for SSTServer {
    async fn create_item(&self, request: Request<CreateItemRequest>) -> ServerResult<CreateItemResponse> {
        Ok(Response::new(CreateItemResponse { success: true, item_id: "abcd".into() }))
    }
}
