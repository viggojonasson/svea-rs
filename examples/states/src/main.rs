// State and cookie example.
// An example of keeping track of visit counts using cookies and mutable states.
// A user goes to any page -> they get a unique user_id cookie if not present
// The global visit count also gets incremented if its the first time the user visits the page.
// The user specific visit count gets incremented everytime the user visits any page.
// This is made possible using server states, global services and cookie mangement.

use async_trait::async_trait;
use hex::encode;
use sha2::{Digest, Sha512};
use std::sync::Arc;
use svea::prelude::*;
use tokio::sync::{Mutex, MutexGuard};

pub const SALT: &'static str = "insert-salt-here";

pub struct UserSpecificVisitCount(Mutex<Vec<(String, usize)>>);
pub struct GlobalVisitCount(Mutex<usize>);

pub struct VisitCounterService;

pub fn generate_user_id(user_id: usize, ip_address: String) -> String {
    let mut hasher = Sha512::new();

    hasher.update(format!("{user_id}{ip_address}{SALT}"));

    let result = hasher.finalize();

    let hash_string = encode(result);

    hash_string
}

#[async_trait]
impl GlobalService for VisitCounterService {
    async fn on_request(&self, server: Arc<Server>, request: &Request, response: &mut Response) {
        // Get potential user_id cookie.
        let user_id_cookie = request.cookies.get_by_key("user_id");

        let user_spec_vis_count = server.get_state::<UserSpecificVisitCount>().unwrap();
        let mut user_spec_vis_count_lock = user_spec_vis_count.0.lock().await;
        let global_visit_count = server.get_state::<GlobalVisitCount>().unwrap();
        let mut global_visit_count = global_visit_count.0.lock().await;

        let user_id: String;
        let mut count: usize = 1;

        // Function to generate a new id for a user.
        // Uses the user count and ip address + a salt to get an id.
        // This is quite a brainless "solution" that doesn't make sense in this case because it doesnt need to be secure
        // and because it uses a memory based count which changes on a restart.
        let get_new_id = |visit_count: &MutexGuard<Vec<(String, usize)>>,
                          ip_address: &Option<String>| {
            let user_count = visit_count.len() + 1;

            generate_user_id(
                user_count,
                ip_address.clone().unwrap_or("127.0.0.1".to_string()),
            )
        };

        // There is no user_id cookie present.
        if user_id_cookie.is_none() {
            // Lets increment the global visit count because this is a new user.
            *global_visit_count += 1;

            // Now lets create a specific user for this request.
            let new_user_id = get_new_id(&user_spec_vis_count_lock, &request.ip_address);

            user_spec_vis_count_lock.push((new_user_id.clone(), 1));

            response.set_cookies.push(Cookie {
                name: "user_id".to_string(),
                value: new_user_id.to_string(),
                options: None,
            });
            user_id = new_user_id;
        } else {
            let user_id_value = user_id_cookie.unwrap().value.clone();

            user_id = user_id_value;

            if user_spec_vis_count_lock
                .iter()
                .find(|(id, _count)| *id == user_id)
                .is_none()
            {
                user_spec_vis_count_lock.push((user_id.clone(), 0));
            } else {
                user_spec_vis_count_lock
                    .iter_mut()
                    .find(|(id, _count)| *id == user_id)
                    .map(|(_id, user_count)| {
                        count = user_count.clone() + 1;
                        *user_count += 1;
                    });
            }
        }

        response.body += format!(
            " Your user id is: {} You have visited this page {} times. There have been {} unique visitors.",
            user_id,
            count,
            global_visit_count
        )
        .as_str();
    }
}

async fn index(_server: Arc<Server>, _request: Request) -> String {
    String::from("Hello world!")
}

#[tokio::main]
async fn main() {
    let router = Router::new().route(Route::new().path("/").handler(index));

    Server::new()
        .port(3000)
        .address("localhost")
        .global_service(VisitCounterService)
        .state(UserSpecificVisitCount(Mutex::new(vec![])))
        .state(GlobalVisitCount(Mutex::new(0)))
        .router(router)
        .no_default_404(true)
        .run()
        .await;
}
