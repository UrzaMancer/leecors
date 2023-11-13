use ::reqwest::blocking::Client;
use graphql_client::{reqwest::post_graphql_blocking, GraphQLQuery};
use std::error::Error;

type JSONString = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "data/schema/leetcode.schema.graphql",
    query_path = "data/queries/username_is_premium.graphql",
    response_derives = "Debug, Serialize, Deserialize, PartialEq"
)]
pub struct UsernameIsPremium;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "data/schema/leetcode.schema.graphql",
    query_path = "data/queries/get_question_detail.graphql",
    response_derives = "Debug, Serialize, Deserialize, PartialEq"
)]
pub struct GetQuestionDetail;

fn perform_my_query(variables: get_question_detail::Variables) -> Result<(), Box<dyn Error>> {

    let client = Client::new();
    let res = post_graphql_blocking::<GetQuestionDetail, _>(&client, "https://leetcode.com/graphql", variables).unwrap();
    let response_body: get_question_detail::ResponseData = res.data.expect("missing response data");
    println!("{:#?}", response_body);
    Ok(())
}

fn main() {
    let variables = get_question_detail::Variables { title_slug: "sort-vowels-in-a-string".to_string() };
    let _ = perform_my_query(variables);
}
