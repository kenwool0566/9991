use crate::AppState;
use axum::extract::{OriginalUri, State};

pub async fn get(State(state): State<AppState>, uri: OriginalUri) -> String {
    const HOST: &str = "optionalres-hw.sl916.com";
    let uri = format!("https://{}{}", HOST, uri.path_and_query().unwrap().as_str());

    if let Ok(v) = state.http_client.get(&uri).send().await {
        return v.text().await.unwrap_or(default_check());
    }

    default_check()
}

fn default_check() -> String {
    String::from(
        r###"{"res-HD":{"res":[{"hash":"dbc58e35477c3fbeec503ff8bc251f72","name":"60001_2_101.65_res-HD.zip","length":699217531,"order":2}],"latest_ver":"101.65","download_url":"https://optionalres-res-hw.sl916.com/uploadzip/60001/4/63","download_url_bak":"https://optionalres-res-bak-hw.sl916.com/uploadzip/60001/4/63"},"jp":{"res":[{"hash":"abc4ae72007a1a19903f81fd1f067c17","name":"merge/19d9948a357e12800fd5e99e8dd8a40b_1.zip","length":1056587991,"order":1},{"hash":"87e5d01b191e9e6d7ca239cfd3bdebf8","name":"merge/19d9948a357e12800fd5e99e8dd8a40b_2.zip","length":488630379,"order":1}],"latest_ver":"101.65","download_url":"https://optionalres-res-hw.sl916.com/uploadzip/60001/4/63","download_url_bak":"https://optionalres-res-bak-hw.sl916.com/uploadzip/60001/4/63"},"kr":{"res":[{"hash":"a1fae0bfbd2dc5bea5f12e53b9822f74","name":"merge/5fb4b570e04c7c45d4fc8cf5366ef1c0_1.zip","length":1061323119,"order":1},{"hash":"9d5b8219a5a5c1d74654584dbba3cc69","name":"merge/5fb4b570e04c7c45d4fc8cf5366ef1c0_2.zip","length":460264550,"order":1}],"latest_ver":"101.67","download_url":"https://optionalres-res-hw.sl916.com/uploadzip/60001/4/63","download_url_bak":"https://optionalres-res-bak-hw.sl916.com/uploadzip/60001/4/63"},"zh":{"res":[{"hash":"3341a3fc7542aae6ebec9a17ed5724b1","name":"merge/892a13943c7a68e691a629b0fd917901_1.zip","length":1049167416,"order":1},{"hash":"a4d0ef2b327a1fd0bd3e9a033bcbb693","name":"merge/892a13943c7a68e691a629b0fd917901_2.zip","length":641050577,"order":1}],"latest_ver":"101.65","download_url":"https://optionalres-res-hw.sl916.com/uploadzip/60001/4/63","download_url_bak":"https://optionalres-res-bak-hw.sl916.com/uploadzip/60001/4/63"}}"###,
    )
}
