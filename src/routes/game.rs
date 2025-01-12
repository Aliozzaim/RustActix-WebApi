#[get("/games")]
pub async fn get_games(state: web::Data<AppState>) -> impl Responder {
    let query = "SELECT * FROM games";

    match sqlx::query_as::<_, GameModel>(query)
        .fetch_all(&state.db)
        .await
    {
        Ok(games) => HttpResponse::Ok().json(games), // Return the list of games as JSON
        Err(e) => {
            eprintln!("Database query failed: {}", e); // Log the error
            HttpResponse::InternalServerError().json("Failed to fetch games") // Generic error message
        }
    }
}

#[post("/games")]

pub async fn create_game(
    data: web::Data<AppState>,
    body: web::Json<CreateGameSchema>,
) -> impl Responder {
    let query = "INSERT INTO games (title, genre, price) VALUES ($1, $2, $3) RETURNING *";

    match sqlx::query_as::<_, GameModel>(query)
        .bind(&game.title)
        .bind(&game.genre)
        .bind(game.price)
        .fetch_one(&state.db)
        .await
    {
        Ok(game) => HttpResponse::Created().json(game),
        Err(e) => {
            eprintln!("Database query failed: {}", e);
            HttpResponse::InternalServerError().json("Failed to create game")
        }
    }
}
