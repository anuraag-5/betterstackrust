use std::sync::{Arc, Mutex};
use url::Url;

use poem::{
    handler,
    http::header,
    web::{Data, Json},
    Response,
};
use store::{models::app::PageVisit, store::Store};

use crate::{
    auth_middleware::UserIdFromHeader,
    request_input::TrackingInput,
    request_output::{User},
};

#[handler]
pub fn snippet() -> Response {
    let script = r#"
    
      (function() {

        function sendTrack(url) {
          let visitor_id = localStorage.getItem("visitor_id");
          if (!visitor_id) {
            visitor_id = crypto.randomUUID();
            localStorage.setItem("visitor_id", visitor_id);
          }

          const payload = {
            visitor_id,
            page_url: url || window.location.href,
            referrer: document.referrer || "",
            user_agent: navigator.userAgent || "",
            time_stamp: Date.now().toString(),
          };

          fetch("http://localhost:3001/api/track", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload),
          });
        }

        
        sendTrack(window.location.href);

        
        const pushState = history.pushState;
        history.pushState = function() {
          pushState.apply(this, arguments);
          sendTrack(window.location.href);
        };

        const replaceState = history.replaceState;
        history.replaceState = function() {
          replaceState.apply(this, arguments);
          sendTrack(window.location.href);
        };

        
        window.addEventListener("popstate", function() {
          sendTrack(window.location.href);
        });

      })();

    "#;

    Response::builder()
        .header(header::CONTENT_TYPE, "application/javascript")
        .body(script)
}

#[handler]
pub fn track(Json(data): Json<TrackingInput>, Data(s): Data<&Arc<Mutex<Store>>>) {
    let page_url = data.page_url;
    let visitor_id = data.visitor_id;
    let referrer = data.referrer;
    let user_agent = data.user_agent;

    let parsed_page_url = Url::parse(&page_url);

    match parsed_page_url {
        Ok(url) => {
            let domain = url.domain().unwrap();
            let current_path = url.path();

            let mut locked_s = s.lock().unwrap();
            let website = locked_s.search_website(domain);

            match website {
                Ok(w) => {
                    locked_s.update_website_snippet(domain);
                    let page_visit = PageVisit {
                        visitor_id,
                        referrer,
                        user_agent,
                        page_path: current_path.to_string(),
                        website: w.url,
                    };

                    let _inserted_data = locked_s.store_tracks(page_visit);
                }
                Err(e) => {
                    println!("{}", e.to_string())
                }
            }
        }
        Err(_) => {}
    }
}

// #[handler]
// pub fn total_views(
//     Data(s): Data<&Arc<Mutex<Store>>>,
// ) -> Json<TotalViewsOutput> {
//     let mut locked_s = s.lock().unwrap();
//     let res = locked_s.get_total_views(w_id);

//     match res {
//         Ok(count) => Json(TotalViewsOutput {
//             total_views: count,
//             success: true,
//         }),
//         Err(_) => Json(TotalViewsOutput {
//             total_views: 0,
//             success: false,
//         }),
//     }
// }

#[handler]
pub fn get_user(
    Data(s): Data<&Arc<Mutex<Store>>>,
    UserIdFromHeader(user_id): UserIdFromHeader,
) -> Json<User> {
    let mut locked_s = s.lock().unwrap();
    if user_id.len() <= 0 {
        print!("User id not found");
        return Json( User { id: "()".to_string(), name: "()".to_string(), email: "()".to_string(), plan_type: "".to_string(), success: false })
    }
    let res = locked_s.get_user(user_id);

    match res {
        Some(user) => Json(User {
            id: user.id.clone(),
            name: user.name.clone(),
            email: user.email.clone(),
            plan_type: user.plan_type.clone(),
            success: true,
        }),
        None => Json(User {
            id: "".to_string(),
            name: "".to_string(),
            email: "".to_string(),
            plan_type: "".to_string(),
            success: false,
        }),
    }
}
