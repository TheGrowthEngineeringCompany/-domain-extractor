use std::collections::HashMap;

use tldextract::TldOption;
// use serde_json::json;
use serde::{Serialize, Deserialize};
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[derive(Debug, Deserialize, Serialize)]
struct Domain {
    url: Box<str>,
    domain: Box<str>,
}

fn domain(url: &str) -> Domain {
    let ext = TldOption::default().build();
    let extracted = ext.extract(url).unwrap();
    let domain = extracted.domain.unwrap();
    return Domain { url: url.into(), domain: domain.into() }
} 

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Hello!"))
        .get("/extract", |req: Request, _| {
            let u = req.url().unwrap();
            let q: HashMap<_, _> = u.query_pairs().into_owned().collect();
            let url_to_match = match q.get("url") {
                Some(url) => url,
                None => return Response::error("Url param missing", 422)
            };
            let d = domain(&url_to_match);
            Response::from_json(&d)
        }) 
        .run(req, env)
        .await
}
