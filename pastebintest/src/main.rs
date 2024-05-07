#[macro_use] extern crate rocket;

pub mod score;
use crate::score::Board;

fn display(board: Board) -> String {
    let mut finished: String = "Leaderboard".to_string() + "\n";
    for item in board.list {
        let add = item.name + " " + &item.score.to_string() + "\n";
        finished = finished + &add; 
    }
    return finished;
}

#[get("/")]
fn home() -> String {
    let leaderboard = Board::new();
    let displayable = display(leaderboard);
    format!("{}", displayable)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![home])
}