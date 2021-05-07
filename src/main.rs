use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let readme = warp::path("readme")
        .and(warp::get())
        .and(warp::path::end())
        .and(warp::fs::file("./README.md"));

    let index = warp::path::end().and(warp::fs::file("./static/index.html"));

    let routes = warp::get().and(hello.or(readme).or(index));

    warp::serve(routes).run(([127, 0, 0, 1], 22022)).await;
}
