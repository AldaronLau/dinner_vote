use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// A dinner option
#[derive(Serialize, Deserialize, Debug)]
struct Dinner {
    // Short description
    short: String,
    // Long description
    long: String,
    // Who voted for this one, if anyone
    vote: Option<String>,
}

// A person
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    // Number of votes
    votes: u16,
}

// Database of dinners & votes
#[derive(Serialize, Deserialize, Debug)]
struct DatabaseData {
    // Key is dinner name,
    dinners: HashMap<String, Dinner>,
    // Key is person name
    people: HashMap<String, Person>,
}

// A "database"
struct Database {
    data: std::sync::Mutex<DatabaseData>,
}

impl Database {
    fn new() -> Self {
        let data: std::sync::Mutex<DatabaseData> = if std::path::Path::new("database").exists() {
            std::sync::Mutex::new(bincode::deserialize(&std::fs::read("database").unwrap()).unwrap())
        } else {
            std::sync::Mutex::new(DatabaseData {
                dinners: HashMap::new(),
                people: HashMap::new(),
            })
        };

        Database {
            data
        }
    }

    fn update<F: FnOnce(&mut DatabaseData)>(&self, closure: F) {
        closure(&mut self.data.lock().unwrap());

        let encoded: Vec<u8> = bincode::serialize(&self.data).unwrap();

        // Create temp file
        std::fs::write("temp", encoded).unwrap();

        // Move temp file onto old file, deleting old file
        std::fs::rename("temp", "database").unwrap();
    }
}

enum DbEvent {
    NewUser { name: String },
    Vote { name: String },
    Unvote { name: String },
    ViewVotes { name: String },
    NewDinner { user: String, name: String },
    EditShortname { user: String, index: String, name: String},
    EditLongname { user: String, index: String, name: String},
    EditDetails { user: String, index: String, name: String},
    EditPhoto { user: String, index: String, photo: Vec<u8>},
    DeleteDinner { user: String, index: String},
    SetRating { user: String, index: String, rating: String},
    ViewAnalytics { user: String, index: Option<String>},
}

fn database_thread(database: std::sync::Arc<Database>, recv: std::sync::mpsc::Receiver<DbEvent>) {
    while let Ok(event) = recv.recv() {
        match event {
            DbEvent::NewUser { name } => {
                database.update(|db| {
                });
            }
            DbEvent::Vote { name } => {
                database.update(|db| {
                });
            }
            DbEvent::Unvote { name } => {
                database.update(|db| {
                });
            }
            DbEvent::ViewVotes { name } => {
                database.update(|db| {
                });
            }
            DbEvent::NewDinner { user, name } => {
                database.update(|db| {
                });
            }
            DbEvent::EditShortname { user, index, name } => {
                database.update(|db| {
                });
            }
            DbEvent::EditLongname { user, index, name } => {
                database.update(|db| {
                });
            }
            DbEvent::EditDetails { user, index, name } => {
                database.update(|db| {
                });
            }
            DbEvent::EditPhoto { user, index, photo} => {
            }
            DbEvent::DeleteDinner { user, index } => {
            }
            DbEvent::SetRating { user, index, rating} => {
            }
            DbEvent::ViewAnalytics { user, index } => {
            }
        }
    }
}

struct Server {
    send: std::sync::Mutex<std::sync::mpsc::Sender<DbEvent>>,
    database: std::sync::Arc<Database>,
}

async fn handle_event(mut request: tide::Request<Server>) -> Result<String, tide::Error> {
    let post = request.body_string().await.unwrap_or_else(|_| "".to_string());
    let mut out = String::new();

    match post {
        // Get entire list of dinner options
        a if a.starts_with("l") => {
            for key in (*request.state().database.data.lock().unwrap()).dinners.keys() {
                out.push_str(key);
                out.push('\r');
            }
            out.push('\r');
        }
        // Get details for a specific dinner option (pass index)
        a if a.starts_with("g") => {
            if a.chars().nth(1).unwrap() == ' ' {
                if let Some(details) = (*request.state().database.data.lock().unwrap())
                    .dinners.get(&a[2..])
                {
                    out.push_str(&details.short);
                    out.push('\r');
                    out.push_str(&details.long);
                    out.push_str("\r\r");
                }
            }
        }
        //{}" => Vote (pass User ID)
        a if a.starts_with("v") => {
            let args = a.split(' ');
            if let Some(user_id) = args.skip(1).next() {
                let _ = request.state().send.lock().unwrap().send(DbEvent::Vote {
                    name: user_id.to_string(),
                });
            }
        }
        //{}" => Revoke Vote (pass User ID)
        a if a.starts_with("u") => {
            let args = a.split(' ');
            if let Some(user_id) = args.skip(1).next() {
                let _ = request.state().send.lock().unwrap().send(DbEvent::Unvote {
                    name: user_id.to_string(),
                });
            }
        }
        //{}" => View all votes (pass User ID)
        a if a.starts_with("a") => {
            let args = a.split(' ');
            if let Some(user_id) = args.skip(1).next() {
                let _ = request.state().send.lock().unwrap().send(DbEvent::ViewVotes {
                    name: user_id.to_string(),
                });
            }
        }
        //{}" => Create account (pass user's name)
        a if a.starts_with("c") => {
            let args = a.split(' ');
            if let Some(user_id) = args.skip(1).next() {
                let _ = request.state().send.lock().unwrap().send(DbEvent::NewUser {
                    name: user_id.to_string(),
                });
            }
        }
        //{} {}" => New dinner option (pass (User ID, Shortname))
        a if a.starts_with("n") => {
            let mut args = a.split(' ').skip(1);
            if let Some((user, name)) = args.next()
                .and_then(|a| Some((a, args.next()?)))
            {
                let _ = request.state().send.lock().unwrap().send(DbEvent::NewDinner {
                    user: user.to_string(),
                    name: name.to_string(),
                });
            }
        }
        //{} {} {}" => Edit shortname (pass (User ID, index, Shortname))
        a if a.starts_with("s") => {
            let mut args = a.split(' ').skip(1);
            if let Some((user, index, name)) = args.next()
                .and_then(|a| Some((a, args.next()?)))
                .and_then(|(a, b)| Some((a, b, args.next()?)))
            {
                let _ = request.state().send.lock().unwrap().send(DbEvent::EditShortname {
                    user: user.to_string(),
                    index: index.to_string(),
                    name: name.to_string(),
                });
            }
        }
        //{} {} {}" => Edit title / longname (pass (User ID, index, Shortname))
        a if a.starts_with("t") => {
            let mut args = a.split(' ').skip(1);
            if let Some((user, index, name)) = args.next()
                .and_then(|a| Some((a, args.next()?)))
                .and_then(|(a, b)| Some((a, b, args.next()?)))
            {
                let _ = request.state().send.lock().unwrap().send(DbEvent::EditLongname {
                    user: user.to_string(),
                    index: index.to_string(),
                    name: name.to_string(),
                });
            }
        }
        //{} {} {}" => Edit More details (pass (User ID, index, Shortname))
        a if a.starts_with("m") => {
            let mut args = a.split(' ').skip(1);
            if let Some((user, index, name)) = args.next()
                .and_then(|a| Some((a, args.next()?)))
                .and_then(|(a, b)| Some((a, b, args.next()?)))
            {
                let _ = request.state().send.lock().unwrap().send(DbEvent::EditDetails {
                    user: user.to_string(),
                    index: index.to_string(),
                    name: name.to_string(),
                });
            }
        }
        //{} {} {}" => Edit picture (pass (User ID, index, Shortname))
        a if a.starts_with("p") => {
            let mut iter = a.bytes().enumerate().skip(1);
            if let Some((_, b' ')) = iter.next() {
            } else {
                return Ok("".to_string())
            }
            let user_id = a[2..a[2..].find(' ').unwrap()].to_string();
            let index = a[2+user_id.len()..a[2..].find(' ').unwrap()].to_string();
            let raster = a[3+user_id.len()+index.len()..].as_bytes().to_vec();
            let _ = request.state().send.lock().unwrap().send(DbEvent::EditPhoto {
                user: user_id,
                index: index,
                photo: raster,
            });
        }
        //{} {}" => Delete dinner option (pass (User ID, index))
        a if a.starts_with("d") => {
            let mut iter = a.bytes().enumerate().skip(1);
            if let Some((_, b' ')) = iter.next() {
            } else {
                return Ok("".to_string())
            }
            let user_id = a[2..a[2..].find(' ').unwrap()].to_string();
            let index = a[2+user_id.len()..].to_string();
            let _ = request.state().send.lock().unwrap().send(DbEvent::DeleteDinner {
                user: user_id,
                index: index,
            });
        }
        //{} {} {}" => Set rating (pass (User ID, index, rating))
        a if a.starts_with("r") => {
            let mut iter = a.bytes().enumerate().skip(1);
            if let Some((_, b' ')) = iter.next() {
            } else {
                return Ok("".to_string())
            }
            let user_id = a[2..a[2..].find(' ').unwrap()].to_string();
            let index = a[2+user_id.len()..a[2..].find(' ').unwrap()].to_string();
            let rating = a[3+user_id.len()+index.len()..].to_string();
            let _ = request.state().send.lock().unwrap().send(DbEvent::SetRating {
                user: user_id,
                index,
                rating,
            });
        }
        //{} {?}" => View analytics (pass (User ID, index?))
        a if a.starts_with("y") => {
            let mut iter = a.bytes().enumerate().skip(1);
            if let Some((_, b' ')) = iter.next() {
            } else {
                return Ok("".to_string())
            }
            if let Some(end) = a[2..].find(' ') {
                let user_id = a[2..end].to_string();
                let index = a[2+user_id.len()..].to_string();
                let _ = request.state().send.lock().unwrap().send(DbEvent::ViewAnalytics {
                    user: user_id,
                    index: Some(index),
                });
            } else {
                let user_id = a[2..].to_string();
                let _ = request.state().send.lock().unwrap().send(DbEvent::ViewAnalytics {
                    user: user_id,
                    index: None,
                });
            };

        }
        u => eprintln!("Unknown POST: {}", u),
    }

    Ok(out)
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let database = std::sync::Arc::new(Database::new());
    let (send, recv) = std::sync::mpsc::channel();
    let server = Server {
        send: std::sync::Mutex::new(send), database: database.clone()
    };
    std::thread::spawn(move || database_thread(database, recv));

    tide::log::start();
    let mut app = tide::with_state(server);
    app.at("/meal_vote").post(handle_event);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
