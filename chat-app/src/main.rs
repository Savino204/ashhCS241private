#[macro_use] extern crate rocket;

use rocket::{tokio::sync::broadcast::{channel, Sender, error::RecvError}, serde::{Serialize, Deserialize}, State, Shutdown, response::stream::{EventStream, Event}, fs::{relative, FileServer}};
use rocket::form::Form;
use rocket::tokio::select;

/* 
#[get("/world")]
fn world() -> &'static str {
    "Hello, world!!"
}
*/

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]

struct Message { 
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

#[post("/message", data = "<form>")] //to send messages
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    //a send 'fails' if there are no active subscribers. this is okay.
    let _res = queue.send(form.into_inner());
}

#[get("/events")] //handles get requests to events path
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] { //return type is an infinite stream of server-sent events (allows for client long-lived connection, and for the server to send data to client whenever)
    let mut rx = queue.subscribe(); //create a receiver, to listen for messages

    EventStream! {
        loop { 
            let msg = select! { //select waits on multiple branches and returns when one completes
                msg = rx.recv() => match msg { //new messages are mapped to msg, err-closed is for when nobody is in the server, err-lagged is when there is too big of a latency and is forcefully dc'd
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break, //second branch. waiting for shutdown future to resolve
            };

            yield Event::json(&msg);
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![post, events])
        .mount("/", FileServer::from(relative!("static")))
}