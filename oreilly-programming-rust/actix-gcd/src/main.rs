use std::fmt::format;

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    m: u64,
    n: u64,
}

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000");

    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .await
        .expect("error running server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
    )
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("its boring to compute GCD for zero");
    }

    let res = format!(
        "the gcd for {} and {} is {}",
        form.n,
        form.m,
        gcd(form.m, form.n)
    );
    HttpResponse::Ok().content_type("text/html").body(res)
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    while (m != 0) {
        if (m < n) {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    return n;
}
