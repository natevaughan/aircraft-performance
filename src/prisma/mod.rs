use model::PrismaClient;
use prisma_client_rust::NewClientError;

pub mod model;

pub async fn client() -> PrismaClient {
    let client_res: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;
    return client_res.expect("Error creating client");
}