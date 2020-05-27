use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// A dinner option
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Dinner {
    // Short description
    short: String,
    // Long description
    long: String,
    // Photo of the dinner option.
    photo: Option<Vec<u8>>,
    // Who voted for this one, if anyone
    vote: Option<String>,
}

// A person
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Person {
    // Number of votes.
    votes: u16,
    // Admin can delete, add and edit dinners.
    admin: bool,
}

// Database of dinners & votes
struct DatabaseData {
    // Key is dinner name,
    dinners: HashMap<String, Dinner>,
    // Key is person name
    people: HashMap<String, Person>,
}

impl DatabaseData {
    fn from_serde(database_data: DatabaseDataSerde) -> Self {
        let mut dinners = HashMap::new();
        let mut people = HashMap::new();
        
        for dinner in database_data.dinners {
            dinners.insert(dinner.key, dinner.value);
        }
        
        for person in database_data.people {
            people.insert(person.key, person.value);
        }

        Self {
            dinners,
            people,
        }
    }

    fn to_serde(&self) -> DatabaseDataSerde {
        let mut dinners = Vec::new();
        let mut people = Vec::new();
        
        for (key, value) in self.dinners.clone() {
            dinners.push(DinnerKV { key, value });
        }
        
        for (key, value) in self.people.clone() {
            people.push(PersonKV { key, value });
        }
        
        DatabaseDataSerde {
            dinners, people
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DinnerKV {
    key: String,
    value: Dinner,
}

#[derive(Serialize, Deserialize, Debug)]
struct PersonKV {
    key: String,
    value: Person,
}

#[derive(Serialize, Deserialize, Debug)]
struct DatabaseDataSerde {
    dinners: Vec<DinnerKV>,
    people: Vec<PersonKV>,
}

// A "database"
struct Database {
    data: std::sync::Mutex<DatabaseData>,
}

impl Database {
    fn new() -> Self {
        let data: std::sync::Mutex<DatabaseData> = if std::path::Path::new("database").exists() {
            std::sync::Mutex::new(DatabaseData::from_serde(muon_rs::from_slice(&std::fs::read("database").unwrap()).unwrap()))
        } else {
            std::sync::Mutex::new(DatabaseData {
                dinners: HashMap::new(),
                people: HashMap::new(),
            })
        };

        Database { data }
    }

    fn update<F: FnOnce(&mut DatabaseData)>(&self, closure: F) {
        println!("Locking…");
        let data = &mut self.data.lock().unwrap();
        println!("Running…");
        closure(data);
        println!("Ran");
        let data = DatabaseData::to_serde(data);

        let encoded: Vec<u8> = muon_rs::to_vec(&data).unwrap();

        // Create temp file
        std::fs::write("temp", encoded).unwrap();

        // Move temp file onto old file, deleting old file
        std::fs::rename("temp", "database").unwrap();
        println!("Releaseing…");
    }
}

enum DbEvent {
    NewUser {
        name: String,
    },
    Vote {
        user: String,
        index: String,
    },
    Unvote {
        user: String,
        index: String,
    },
    ViewVotes {
        name: String,
    },
    NewDinner {
        user: String,
        name: String,
    },
    EditShortname {
        user: String,
        index: String,
        name: String,
    },
    EditLongname {
        user: String,
        index: String,
        name: String,
    },
    EditDetails {
        user: String,
        index: String,
        name: String,
    },
    EditPhoto {
        user: String,
        index: String,
        photo: Vec<u8>,
    },
    DeleteDinner {
        user: String,
        index: String,
    },
    SetRating {
        user: String,
        index: String,
        rating: String,
    },
    ViewAnalytics {
        user: String,
        index: Option<String>,
    },
    SetVotes {
        user: String,
        votes: String,
    }
}

fn database_thread(database: std::sync::Arc<Database>, recv: std::sync::mpsc::Receiver<DbEvent>) {
    while let Ok(event) = recv.recv() {
        match event {
            DbEvent::NewUser { name } => {
                database.update(|db| {
                    // Add person if they're not already in the system.
                    if db.people.get(&name).is_none() {
                        db.people.insert(
                            name,
                            Person {
                                votes: 0,
                                admin: false,
                            },
                        );
                    }
                });
            }
            DbEvent::Vote { user, index } => {
                database.update(|db| {
                    if let Some(person) = db.people.get_mut(&user) {
                        if let Some(dinner) = db.dinners.get_mut(&index) {
                            if dinner.vote.is_none() {
                                if person.votes != 0 {
                                    dinner.vote = Some(user);
                                    if !person.admin {
                                        person.votes -= 1;
                                    }
                                }
                            }
                        }
                    }
                });
            }
            DbEvent::Unvote { user, index } => {
                println!("Unvote {} {}", user, index);
                database.update(|db| {
                    if let Some(person) = db.people.get_mut(&user) {
                        if let Some(dinner) = db.dinners.get_mut(&index) {
                            if dinner.vote == Some(user) || person.admin {
                                dinner.vote = None;
                                if !person.admin {
                                    person.votes += 1;
                                }
                            }
                        }
                    }
                });
            }
            DbEvent::ViewVotes { name } => {
                // FIXME
                let _ = name;
            }
            DbEvent::NewDinner { user, name } => {
                database.update(|db| {
                    // Add dinner if it's not already in the system.
                    if let Some(person) = db.people.get(&user) {
                        if person.admin {
                            if db.dinners.get(&name).is_none() {
                                db.dinners.insert(
                                    name,
                                    Dinner {
                                        short: "-".to_string(),
                                        long: "-".to_string(),
                                        photo: None,
                                        vote: None,
                                    },
                                );
                            }
                        }
                    }
                });
            }
            DbEvent::EditShortname { user, index, name } => {
                database.update(|db| {
                    if let Some(person) = db.people.get(&user) {
                        if person.admin {
                            if let Some(value) = db.dinners.remove(&index) {
                                db.dinners.insert(name, value);
                            }
                        }
                    }
                });
            }
            DbEvent::EditLongname { user, index, name } => {
                database.update(|db| {
                    if let Some(person) = db.people.get(&user) {
                        if person.admin {
                            if let Some(dinner) = db.dinners.get_mut(&index) {
                                dinner.short = name;
                            }
                        }
                    }
                });
            }
            DbEvent::EditDetails { user, index, name } => {
                database.update(|db| {
                    if let Some(person) = db.people.get(&user) {
                        if person.admin {
                            if let Some(dinner) = db.dinners.get_mut(&index) {
                                dinner.long = name;
                            }
                        }
                    }
                });
            }
            DbEvent::EditPhoto { user, index, photo } => {
                let _ = user;
                let _ = index;
                let _ = photo;
                database.update(|db| {
                    // FIXME
                    let _ = db;
                });
            }
            DbEvent::DeleteDinner { user, index } => {
                database.update(|db| {
                    if let Some(person) = db.people.get(&user) {
                        if person.admin {
                            db.dinners.remove(&index);
                        }
                    }
                });
            }
            DbEvent::SetRating {
                user,
                index,
                rating,
            } => {
                let _ = user;
                let _ = index;
                let _ = rating;
                database.update(|db| {
                    // FIXME
                    let _ = db;
                });
            }
            DbEvent::ViewAnalytics { user, index } => {
                let _ = user;
                let _ = index;
            }
            DbEvent::SetVotes { user, votes } => {
                println!("SETVOTE '{}' '{}'", user, votes);
                if let Ok(votes) = votes.parse() {
                    database.update(|db| {
                        for person in db.people.values_mut() {
                            person.votes = votes;
                        }
                    });
                }
            }
        }
    }
}

struct Server {
    send: std::sync::Mutex<std::sync::mpsc::Sender<DbEvent>>,
    database: std::sync::Arc<Database>,
}

async fn handle_event(mut request: tide::Request<Server>) -> Result<String, tide::Error> {
    let post = request
        .body_string()
        .await
        .unwrap_or_else(|_| "".to_string());
    let mut out = String::new();

    match post {
        // Get entire list of dinner options
        a if a.starts_with("l") => {
            for (key, value) in (*request.state().database.data.lock().unwrap())
                .dinners.iter()
            {
                out.push_str(&key);
                out.push('\\');
                out.push_str(&value.short);
                if let Some(ref user) = value.vote {
                    out.push('\\');
                    out.push_str(user);
                }
                out.push('\n');
            }
            out.pop();
        }
        // Get details for a specific dinner option (pass index)
        a if a.starts_with("g") => {
            if a.chars().nth(1).unwrap() == ' ' {
                if let Some(details) = (*request.state().database.data.lock().unwrap())
                    .dinners
                    .get(&a[2..])
                {
                    out.push_str(&details.short);
                    out.push('\r');
                    out.push_str(&details.long);
                    out.push_str("\r\r");
                }
            }
        }
        // Vote (pass (User ID, index))
        a if a.starts_with("v") => {
            let mut args = a[2..].split('\\');
            if let Some((user, index)) = args.next().and_then(|a| Some((a, args.next()?))) {
                let _ = request.state().send.lock().unwrap().send(DbEvent::Vote {
                    user: user.to_string(),
                    index: index.to_string(),
                });
            }
        }
        // Revoke Vote (pass (User ID, index))
        a if a.starts_with("u") => {
            println!("UVNOTE:");
            let mut args = a[2..].split('\\');
            if let Some((user, index)) = args.next().and_then(|a| Some((a, args.next()?))) {
                println!("UNvote {} {}", user, index);
                let _ = request.state().send.lock().unwrap().send(DbEvent::Unvote {
                    user: user.to_string(),
                    index: index.to_string(),
                });
            }
        }
        //{}" => View all votes (pass User ID)
        a if a.starts_with("a") => {
            let args = a.split(' ');
            if let Some(user_id) = args.skip(1).next() {
                let _ = request
                    .state()
                    .send
                    .lock()
                    .unwrap()
                    .send(DbEvent::ViewVotes {
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
            println!("NEW DINNER OPTION");
            if let Some(split_index) = a[2..].find('\\') {
                let user = a[2..2+split_index].to_string();
                let name = a[(2+split_index + 1)..].to_string();

                println!("SENDING: {} {}", user, name);

                let _ = request
                    .state()
                    .send
                    .lock()
                    .unwrap()
                    .send(DbEvent::NewDinner { user, name });
            }
        }
        //{} {} {}" => Edit shortname (pass (User ID, index, Shortname))
        a if a.starts_with("s") => {
            let mut args = a[2..].split('\\');
            if let Some((user, index, name)) = args
                .next()
                .and_then(|a| Some((a, args.next()?)))
                .and_then(|(a, b)| Some((a, b, args.next()?)))
            {
                let _ = request
                    .state()
                    .send
                    .lock()
                    .unwrap()
                    .send(DbEvent::EditShortname {
                        user: user.to_string(),
                        index: index.to_string(),
                        name: name.to_string(),
                    });
            }
        }
        //{} {} {}" => Edit title / longname (pass (User ID, index, Shortname))
        a if a.starts_with("t") => {
            let mut args = a[2..].split('\\');
            if let Some((user, index, name)) = args
                .next()
                .and_then(|a| Some((a, args.next()?)))
                .and_then(|(a, b)| Some((a, b, args.next()?)))
            {
                let _ = request
                    .state()
                    .send
                    .lock()
                    .unwrap()
                    .send(DbEvent::EditLongname {
                        user: user.to_string(),
                        index: index.to_string(),
                        name: name.to_string(),
                    });
            }
        }
        //{} {} {}" => Edit More details (pass (User ID, index, Shortname))
        a if a.starts_with("m") => {
            let mut args = a.split(' ').skip(1);
            if let Some((user, index, name)) = args
                .next()
                .and_then(|a| Some((a, args.next()?)))
                .and_then(|(a, b)| Some((a, b, args.next()?)))
            {
                let _ = request
                    .state()
                    .send
                    .lock()
                    .unwrap()
                    .send(DbEvent::EditDetails {
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
                return Ok("".to_string());
            }
            let user_id = a[2..a[2..].find(' ').unwrap()].to_string();
            let index = a[2 + user_id.len()..a[2..].find(' ').unwrap()].to_string();
            let raster = a[3 + user_id.len() + index.len()..].as_bytes().to_vec();
            let _ = request
                .state()
                .send
                .lock()
                .unwrap()
                .send(DbEvent::EditPhoto {
                    user: user_id,
                    index: index,
                    photo: raster,
                });
        }
        //{} {}" => Delete dinner option (pass (User ID, index))
        a if a.starts_with("d") => {
            println!("DELETE DINNER OPTION");
            if let Some(split_index) = a[2..].find('\\') {
                let user = a[2..2+split_index].to_string();
                let index = a[(2+split_index + 1)..].to_string();

                println!("SENDING: {} {}", user, index);

                let _ = request
                    .state()
                    .send
                    .lock()
                    .unwrap()
                    .send(DbEvent::DeleteDinner { user, index });
            }
        }
        //{} {} {}" => Set rating (pass (User ID, index, rating))
        a if a.starts_with("r") => {
            let mut iter = a.bytes().enumerate().skip(1);
            if let Some((_, b' ')) = iter.next() {
            } else {
                return Ok("".to_string());
            }
            let user_id = a[2..a[2..].find(' ').unwrap()].to_string();
            let index = a[2 + user_id.len()..a[2..].find(' ').unwrap()].to_string();
            let rating = a[3 + user_id.len() + index.len()..].to_string();
            let _ = request
                .state()
                .send
                .lock()
                .unwrap()
                .send(DbEvent::SetRating {
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
                return Ok("".to_string());
            }
            if let Some(end) = a[2..].find(' ') {
                let user_id = a[2..end].to_string();
                let index = a[2 + user_id.len()..].to_string();
                let _ = request
                    .state()
                    .send
                    .lock()
                    .unwrap()
                    .send(DbEvent::ViewAnalytics {
                        user: user_id,
                        index: Some(index),
                    });
            } else {
                let user_id = a[2..].to_string();
                let _ = request
                    .state()
                    .send
                    .lock()
                    .unwrap()
                    .send(DbEvent::ViewAnalytics {
                        user: user_id,
                        index: None,
                    });
            };
        }
        //{}" => View all votes (pass User ID)
        a if a.starts_with("h") => {
            let args = a.split(' ');
            if let Some(user_id) = args.skip(1).next() {
                println!("{}", user_id);
                if let Some(person) = request
                    .state()
                    .database
                    .data
                    .lock()
                    .unwrap()
                    .people
                    .get(user_id)
                {
                    let string = format!("{}", person.votes);
                    println!("Get #VOTES {}", person.votes);
                    out.push_str(&string);
                    out.push('\\');
                    out.push_str(if person.admin {
                        "TRUE"
                    } else {
                        "FALSE"
                    });
                }
            }
        }
        //{} {}" => Set all votes (pass User ID)
        a if a.starts_with("z") => {
            let mut args = a[2..].split('\\');
            if let Some(user_id) = args.next() {
                if let Some(votes) = args.next() {
                    println!("ZZZ {} {}", user_id, votes);
                    if let Some(person) = request
                        .state()
                        .database
                        .data
                        .lock()
                        .unwrap()
                        .people
                        .get(user_id)
                    {
                        println!("STAGE 1");
                        if person.admin {
                            println!("STAGE 2");
                            let _ = request
                                .state()
                                .send
                                .lock()
                                .unwrap()
                                .send(DbEvent::SetVotes {
                                    user: user_id.to_string(),
                                    votes: votes.to_string(),
                                });
                        }
                    }
                }
            }
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
        send: std::sync::Mutex::new(send),
        database: database.clone(),
    };
    std::thread::spawn(move || database_thread(database, recv));

    tide::log::start();
    let mut app = tide::with_state(server);
    app.at("/meal_vote").post(handle_event);
    app.listen("192.168.0.111:8080" /*"127.0.0.1:8080"*/).await?;
    Ok(())
}
