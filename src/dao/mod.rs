use rbatis::rbatis::Rbatis;
use actix_web::{HttpResponse, web};
use rbatis::crud::CRUD;
use serde_json::json;



lazy_static!{
  pub static ref RB:Rbatis = Rbatis::new();
}

#[crud_enable]
#[derive(Clone, Debug)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub delete_flag: Option<i32>,
}

#[post("/insert")]
pub async fn save(ba: web::Json<BizActivity>) -> HttpResponse {
  // let activity = BizActivity {
  //     id: Some("1".to_string()),
  //     name: Some("zs".to_string()),
  //     delete_flag: Some(1),
  // };
  println!("ba = {:#?}", ba);
  let result = RB.save("",&ba.0).await;

  if let Err(err) = result {
      return HttpResponse::Ok().body(format!("database save fail: {}", err))
  }
  HttpResponse::Ok().body("database save ok")
  
}

#[get("/del")]
pub async fn del(q: web::Query<BizActivity>) -> HttpResponse {
  if let Some(id) = q.id.as_ref() {
    let result = RB.remove_by_id::<BizActivity>("", &id.to_string()).await;
    if let Err(err) = result {
      return HttpResponse::Ok().body(format!("database del fail: {}", err));
      }
      return HttpResponse::Ok().body("database del ok");
  }
  HttpResponse::Ok().body("bad request")
  
}
#[get("/queryAll")]
pub async fn query() -> HttpResponse {
  let result: Vec<BizActivity> = RB.fetch_list("").await.unwrap();
  println!("query success! {:#?}", result);
  HttpResponse::Ok().body(json!(result))
}

