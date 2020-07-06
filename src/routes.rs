use actix_web::{web, middleware, App, HttpServer};
use crate::interface::controller;

pub async fn initialize() -> std::io::Result<()> {
    let bind = "0.0.0.0:8080";

    println!("Starting server at: {}", &bind);

    // どうしよ.....
    // つくってみますか(よくわからないけれど)
    // interfaceもstructも一つのファイルの中にかけるのだろうか
    
    // i := Registry.NewRegistory(conn)
    // registoryというインターフェイスをつくる感じです？？ ⇛ そのはず！！
    // interface Registory {
    //      
    // }
    //   TODO registory.rsにNewRegistoryというメソッドを作成する、これはregistoryがinterfaceとして持っていて
    //   registoryのstructを返す registoryは要素にconnを持つ

    // hogeController := i.NewHogeController()
    //   TODO registoryにNewHogeControllerというハンドラーを作成するメソッドを用意してcontroller structを返す
    //   controller structはGetUserメソッドをインターフェースに持ち、実装されている

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/user/{user_id}", web::get().to(controller::user::get_user))
            .route("/user", web::post().to(controller::user::add_user))
    })
    .bind(&bind)?
    .run()
    .await
}

