use get_question_detail::{ResponseData, GetQuestionDetailQuestion};
use ::reqwest::blocking::Client;
use graphql_client::{reqwest::post_graphql_blocking, GraphQLQuery};
use serde_json::Value;


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
    query_path = "data/queries/get_question_setup.graphql",
    response_derives = "Debug, Serialize, Deserialize, PartialEq"
)]
pub struct GetQuestionDetail;

#[derive(Debug)]
#[allow(dead_code)]
pub struct QuestionSetup {
    id: i32,
    title: String,
    content: JSONString, // GraphQL presents this as JSONString and
                     // we eventually want it as its own parsed
                     // struct
    stats: String,
    difficulty: String,
    meta_data: Value,
    lang_code: String,
    code_definition: String,
    sample_testcases: String,
    env_info: String,
}

impl QuestionSetup {
    fn new(resp: GetQuestionDetailQuestion, lang: &str) -> QuestionSetup {
        let lang_env_info: Value = serde_json::from_str(&resp.env_info.expect("question env info")).expect("env info valid json");
        QuestionSetup {
            id: resp.question_id.expect("question id").parse::<i32>().expect("question id is not an integer"),
            title: resp.title,
            content: resp.content.expect("question content"),
            stats: resp.stats.expect("question stats"),
            difficulty: resp.difficulty.expect("question difficulty"),
            meta_data: serde_json::from_str(&resp.meta_data.expect("question meta data")).expect("meta data valid json"),
            lang_code: resp.code_snippets.expect("code snippets").iter().find_map(|snippet| {
                if let Some(snip) = snippet {
                    if snip.lang_slug == Some(lang.to_string()) { snip.code.clone() } else { None }
                } else { None }
            }).expect(format!("find a code snippet for {} in list", lang).as_str()),
            code_definition: Self::find_lang_code_definition(resp.code_definition.expect("question code definition"), lang).expect("code definition for language"),
            sample_testcases: resp.sample_test_case.expect("question sample testcases"),
            env_info: lang_env_info[lang][1].to_string(),
        }
    }

    fn find_lang_code_definition(cdef: String, lang: &str) -> Option<String> {
        let cdef_json: Value = serde_json::from_str(&cdef).expect("code definition valid json");
        cdef_json.as_array().expect("code definition json is array").iter()
            .find_map(|cdef_obj| {
                if cdef_obj.get("value").unwrap().as_str().unwrap() == lang
                    { cdef_obj.get("defaultCode") }
                else { None }
            })
            .map(|cdef_obj| cdef_obj.to_string())
    }
}

fn perform_my_query(variables: get_question_detail::Variables) {

    let client = Client::new();
    let res = post_graphql_blocking::<GetQuestionDetail, _>(&client, "https://leetcode.com/graphql", variables).unwrap();
    let response_body: ResponseData = res.data.expect("missing response data");

    let new_thing = QuestionSetup::new(response_body.question.expect("missing question"), "rust");
    println!("{:#?}", new_thing);
}

fn main() {
    let variables = get_question_detail::Variables { title_slug: "minimize-maximum-pair-sum-in-array".to_string() };
    let _ = perform_my_query(variables);
}
