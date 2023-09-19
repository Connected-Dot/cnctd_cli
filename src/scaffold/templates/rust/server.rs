use genco::prelude::*;

fn main() -> genco::fmt::Result {
    let health_route = rust! {
        let health_route = warp::path!("health")
            .and_then(handlers::health_handler);
    };

    let post_route = rust! {
        let post_route = warp::path!("p")
            .and(warp::body::json())
            .and(warp::addr::remote())
            .and_then(handlers::post_handler);
    };

    let get_route = rust! {
        let get_route = warp::path!("g")
            .and(warp::get())
            .and(warp::query::<IncomingMessage>())
            .and(warp::addr::remote())
            .and_then(get_handler);
    };

    // ... more routes

    // Now, let's assume you have a user input that specifies which routes to include.
    let user_selected_routes = vec!["health", "post"]; // This would come from the user.

    let mut selected_route_tokens = vec![];

    for route in user_selected_routes {
        match route {
            "health" => selected_route_tokens.push(health_route.clone()),
            "post" => selected_route_tokens.push(post_route.clone()),
            "get" => selected_route_tokens.push(get_route.clone()),
            // ... more routes
            _ => {}
        }
    }

    let tokens: rust::Tokens = rust! {
        use std::fs;
        use std::collections::HashMap;
        use std::sync::Arc;
        use tokio::sync::{mpsc, RwLock};
        // ... other imports

        pub mod handlers;
        pub mod socket;
        mod routes;

        pub static CLIENTS: InitCell<Clients> = InitCell::new();
        // ... other static and type definitions

        pub async fn start_server() {
            let cors = cors();
            // ... other setup

            #(for route in &selected_route_tokens {
                #route
            })*

            let routes = health_route
                .or(post_route)
                .or(get_route)
                // ... more routes
                .with(cors);

            // ... rest of the function
        }
    };

    println!("{}", tokens.to_file_string()?);

    Ok(())
}
