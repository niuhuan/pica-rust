use pica::{Client, ComicEpPicture, ComicSimple, PageData};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn empty_page<T>() -> PageData<T> {
    PageData {
        total: 0,
        limit: 0,
        page: 1,
        pages: 1,
        docs: vec![],
    }
}

#[tokio::test]
async fn uses_custom_host_for_requests() {
    let server = MockServer::start().await;
    let client = Client::with_host(format!("{}/", server.uri())).await;

    Mock::given(method("GET"))
        .and(path("/comics/random"))
        .and(header("api-key", "C69BAF41DA5ABD1FFEDC6D2FEA56B"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": {
                "comics": Vec::<ComicSimple>::new()
            }
        })))
        .mount(&server)
        .await;

    let comics = client
        .comics_random()
        .await
        .expect("request should succeed");
    assert!(comics.is_empty());
}

#[tokio::test]
async fn passes_image_quality_header() {
    let server = MockServer::start().await;
    let client = Client::with_host(format!("{}/", server.uri())).await;

    Mock::given(method("GET"))
        .and(path("/comics/abc/order/1/pages"))
        .and(header("image-quality", "low"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": {
                "pages": empty_page::<ComicEpPicture>()
            }
        })))
        .mount(&server)
        .await;

    let page = client
        .comic_ep_pictures_with_quality("abc".to_string(), 1, 1, "low")
        .await
        .expect("request should succeed");
    assert!(page.docs.is_empty());
}
