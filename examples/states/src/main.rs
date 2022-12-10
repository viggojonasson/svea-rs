// State and cookie example.
// An example of keeping track of visit counts using cookies and mutable states.
// A user goes to any page -> they get a unique user_id cookie if not present
// The global visit count also gets incremented if its the first time the user visits the page.
// The user specific visit count gets incremented everytime the user visits any page.
// This is made possible using server states, global services and cookie mangement.

use async_trait::async_trait;
use std::sync::Arc;
use svea::prelude::*;
use tokio::sync::{Mutex, MutexGuard};

pub struct UserSpecificVisitCount(Mutex<Vec<(usize, usize)>>);
pub struct GlobalVisitCount(Mutex<usize>);

pub struct VisitCounterService;

#[async_trait]
impl GlobalService for VisitCounterService {
    async fn on_request(&self, server: Arc<Server>, request: &Request, response: &mut Response) {
        // Get potential user_id cookie.
        let user_id_cookie = request.cookies.get_by_key("user_id");

        let user_spec_vis_count = server.get_state::<UserSpecificVisitCount>().unwrap();
        let mut user_spec_vis_count_lock = user_spec_vis_count.0.lock().await;
        let global_visit_count = server.get_state::<GlobalVisitCount>().unwrap();
        let mut global_visit_count = global_visit_count.0.lock().await;

        let user_id;

        // Function to generate a new id for a user.
        // Basically just incrementing the count based on the user list.
        let get_new_id = |visit_count: &MutexGuard<Vec<(usize, usize)>>| {
            let last = visit_count.last().unwrap_or_else(|| &(0, 0));

            last.1 + 1
        };

        // There is no user_id cookie present.
        if user_id_cookie.is_none() {
            // Lets increment the global visit count because this is a new user.
            *global_visit_count += 1;

            // Now lets create a specific user for this request.
            let new_user_id = get_new_id(&user_spec_vis_count_lock);

            user_spec_vis_count_lock.push((new_user_id, 1));

            response.set_cookies.push(Cookie {
                name: "user_id".to_string(),
                value: new_user_id.to_string(),
                options: None,
            });
            user_id = new_user_id;
        } else {
            let user_id_value = user_id_cookie.unwrap().value.clone();

            // Lets check if the user_id is valid.
            let new_user_id = user_id_value.parse::<usize>().unwrap_or_else(|_| {
                // If the user_id is not valid, lets create a new one.
                get_new_id(&user_spec_vis_count_lock)
            });

            user_id = new_user_id;

            if user_spec_vis_count_lock
                .iter()
                .find(|(id, _count)| *id == user_id)
                .is_none()
            {
                user_spec_vis_count_lock.push((new_user_id, 0));
            } else {
                user_spec_vis_count_lock
                    .iter_mut()
                    .find(|(id, _count)| *id == user_id)
                    .map(|(_id, count)| {
                        *count += 1;
                    });
            }
        }

        response.body += format!(
            " Your user id is: {} You have visited this page {} times. There have been {} unique visitors.",
            user_id,
            user_spec_vis_count_lock.len(),
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
        .run()
        .await;
}
