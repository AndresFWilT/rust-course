use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SortRequest {
    array: Vec<i32>,
}

#[derive(Serialize)]
struct SortResponse {
    sorted: Vec<i32>,
    status_code: u16,
}

fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

async fn get_array_sorted(body: web::Json<SortRequest>) -> impl Responder {
    let mut array = body.array.clone();
    bubble_sort(&mut array);
    let response = SortResponse {
        sorted: array,
        status_code: 200,
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(get_array_sorted))
    });

    println!("Serving on http://localhost:3000...");
    server.bind("127.0.0.1:3000")?.run().await
}
