use graphql_client::GraphQLQuery;
type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.graphql",
    query_path = "src/gql/queries/strings/Project.graphql",
    response_derives = "Debug"
)]
pub struct Project;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.graphql",
    query_path = "src/gql/queries/strings/Projects.graphql",
    response_derives = "Debug"
)]
pub struct Projects;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.graphql",
    query_path = "src/gql/queries/strings/UserMeta.graphql",
    response_derives = "Debug"
)]
pub struct UserMeta;