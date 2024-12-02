use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn greet(req: HttpRequest) -> impl Responder {
    //obtiene el parametro de la url
    let name: &str = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
/**
 * es un main que se ejecuta en un contexto de actix_web
 */
async fn main() -> std::io::Result<()> {
    //retorna un io result
    HttpServer::new(|| {
        //crea un servidor http
        App::new() //crea una aplicacion
            .service(hello) //agrega la ruta hello
            .service(echo) //agrega la ruta echo
            .route("/hey", web::get().to(manual_hello)) //ruta que se ejecuta con el metodo get
            .route("/greet/{name}", web::get().to(greet)) //ruta que se ejecuta con el metodo get y recibe un parametro
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
