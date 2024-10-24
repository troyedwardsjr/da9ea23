use axum::{extract::Path, http::StatusCode, routing::{get, post}, Json, Router};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize)]
struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool,
}

type ID = String;

// HashMaps mocking database
static mut MOVIES: Lazy<HashMap<ID, Movie>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("0".to_string(), Movie {
        id: format!("{}", 0),
        name: "Titanic".to_string(),
        year: 1990,
        was_good: false
    });
    m.insert("1".to_string(), Movie {
        id: format!("{}", 1),
        name: "Fight Club".to_string(),
        year: 1999,
        was_good: true
    });
    m
});

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/movie/:id", get(get_movie))
        .route("/movie", post(post_movie));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_movie(Path(id): Path<String>) -> Result<Json<Movie>, StatusCode> {
    let movie = unsafe { MOVIES.get(&id) }
        .ok_or_else(|| StatusCode::NOT_FOUND)?;
    let movie: Json<Movie> = Json(Movie {
        id: movie.id.clone(),
        name: movie.name.clone(),
        year: movie.year,
        was_good: movie.was_good
    });
    Ok(movie)
}

async fn post_movie(Json(payload): Json<Movie>) -> Result<StatusCode, StatusCode> {
    if payload.id.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    unsafe {
        MOVIES.insert(payload.id.clone(), Movie {
            id: payload.id.clone(),
            name: payload.name.clone(),
            year: payload.year,
            was_good: payload.was_good
        });
    }
    Ok(StatusCode::ACCEPTED)
}