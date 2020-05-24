use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

// A dinner option
#[derive(Serialize, Deserialize, Debug)]
struct Dinner {
    // Short description
    short: String,
    // Long description
    long: String,
}

// Database of dinners & votes
#[derive(Serialize, Deserialize, Debug)]
struct DatabaseData {
    // Key is dinner name,
    dinners: HashMap<String, Dinner>,
    // Key is person name
    people: HashSet<String>,
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
                people: HashSet::new(),
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
}

fn database_thread(database: std::sync::Arc<Database>, recv: std::sync::mpsc::Receiver<DbEvent>) {
    while let Ok(event) = recv.recv() {
        match event {
            DbEvent::NewUser { name } => {
                database.update(|db| {
                });
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

    match post {
        a if a.starts_with("l") => {} // Get entire list of dinner options
        a if a.starts_with("g") => {} // "{}" => Get details for a specific dinner option (pass index)
        a if a.starts_with("v") => {} //{}" => Vote (pass User ID)
        a if a.starts_with("r") => {} //{}" => Revoke Vote (pass User ID)
        a if a.starts_with("a") => {} //{}" => View all votes (pass User ID)
        a if a.starts_with("c") => {} //{}" => Create account (pass user's name)
        a if a.starts_with("n") => {} //{} {}" => New dinner option (pass (User ID, Shortname))
        a if a.starts_with("s") => {} //{} {} {}" => Edit shortname (pass (User ID, index, Shortname))
        a if a.starts_with("t") => {} //{} {} {}" => Edit title / longname (pass (User ID, index, Shortname))
        a if a.starts_with("m") => {} //{} {} {}" => Edit More details (pass (User ID, index, Shortname))
        a if a.starts_with("p") => {} //{} {} {}" => Edit picture (pass (User ID, index, Shortname))
        a if a.starts_with("d") => {} //{} {}" => Delete dinner option (pass (User ID, index))
        a if a.starts_with("r") => {} //{} {} {}" => Set rating (pass (User ID, index, rating))
        a if a.starts_with("y") => {} //{} {?}" => View analytics (pass (User ID, index?))
        u => eprintln!("Unknown POST: {}", u),
    }

    Ok("Edit".to_string())
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
