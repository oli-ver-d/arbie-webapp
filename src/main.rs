use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use models::{Game, Run};
use mongodb::{bson::{doc, from_bson, Bson}, options::FindOneOptions, Client};
use tera::Tera;
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::bson::DateTime;
use dotenvy::dotenv;
use std::env;

mod models;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}

#[get("/")]
async fn index() -> impl Responder {
    let mut context = tera::Context::new();
    let (games, runtime) = get_most_recent_run_games().await.unwrap();
    if let Some(games) = games {
        context.insert("games", &games);
    }
    if let Some(runtime) = runtime {
        context.insert("timestamp", &runtime.try_to_rfc3339_string().unwrap())
    }
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 31770))?
        .run()
        .await

}

async fn get_most_recent_run_games() -> anyhow::Result<(Option<Vec<Game>>, Option<DateTime>)> {
    let client_uri = env::var("CONNECTION_STRING").expect("CONNECTION_STRING must be set");
    let client: Client = Client::with_uri_str(client_uri).await?;

    let db = client.database("arbie");
    let runs_coll: mongodb::Collection<mongodb::bson::Document> = db.collection("Runs");
    let games_coll: mongodb::Collection<mongodb::bson::Document> = db.collection("Games");

    let find_options: FindOneOptions = FindOneOptions::builder().sort(doc! { "timestamp": -1 }).build();
    let loaded_run = runs_coll.find_one(doc! {}).with_options(find_options).await?.expect("No runs found");

    let run: Run = from_bson(Bson::Document(loaded_run))?;
    
    if run.game_ids.is_empty() {
        return Ok((None, Some(run.timestamp)));
    }

    let mut games: Vec<Game> = vec![];
    let mut loaded_games = games_coll.find(doc! {"_id": { "$in": &run.game_ids } }).await?;
    while let Some(result) = loaded_games.try_next().await? {
        let game: Game = from_bson(Bson::Document(result))?;
        games.push(game);
    }

    return Ok((Some(games), Some(run.timestamp)));
}

#[tokio::test]
async fn test_recent_runs() -> Result<(), anyhow::Error> {
    get_most_recent_run_games().await?;
    Ok(())
}