use actix_web::{App, HttpServer};
use dotenv::dotenv;
use rbatis::core::runtime::runtime::Builder;
use std::env;
use web_demo::dao::{RB, save, del, query};

#[actix_web::main]
async fn main() ->std::io::Result<()>{ 
    dotenv().ok();

    let tokio_runtime = Builder::new_multi_thread().enable_all().build().unwrap();
    let database_url = env::var("MYSQL_DATABASE_URL").expect("must set databse_url");

    tokio_runtime.block_on(async {
        RB.link(&database_url)
            .await
            .expect("connection database fail");
    });

    HttpServer::new(|| {
        App::new()
            .service(save)
            .service(del)
            .service(query)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
        
}
