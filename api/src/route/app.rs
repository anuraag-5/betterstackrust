use std::sync::Arc;
use url::Url;

use poem::{
    Response, handler, http::{StatusCode, header}, web::{Data, Json}
};
use store::{models::app::PageVisit, store::Store};

use crate::{
    auth_middleware::UserIdFromHeader,
    request_input::{GetViewsPerPageInput, TrackingInput},
    request_output::{GetTotalUniqueUsersOutput, GetTotalViewsOutput, GetViewsPerPageOutput, User},
};

#[handler]
pub async fn snippet() -> Response {
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

          fetch("https://api.nexus.speeedops.com/api/track", {
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
pub async fn track(Json(data): Json<TrackingInput>, Data(s): Data<&Arc<Store>>) {
    let page_url = data.page_url;
    let visitor_id = data.visitor_id;
    let referrer = data.referrer;
    let user_agent = data.user_agent;

    let parsed_page_url = Url::parse(&page_url);

    match parsed_page_url {
        Ok(url) => {
            let domain = url.domain().unwrap();
            let current_path = url.path();

    
            let website = s.search_website(domain).await;

            match website {
                Ok(w) => {
                    let _ = s.update_website_snippet(domain).await;
                    let page_visit = PageVisit {
                        visitor_id,
                        referrer,
                        user_agent,
                        page_path: current_path.to_string(),
                        website: w.url,
                    };

                    let _inserted_data = s.store_tracks(page_visit).await;
                }
                Err(e) => {
                    println!("{}", e.to_string())
                }
            }
        }
        Err(_) => {}
    }
}

#[handler]
pub async fn total_views_per_page(
    Data(s): Data<&Arc<Store>>,
    Json(data): Json<GetViewsPerPageInput>,
) -> Json<GetViewsPerPageOutput> {
    let res = s.get_per_page_views(data.website).await;

    match res {
        Ok(d) => Json(GetViewsPerPageOutput {
            data: Some(d),
            success: true,
        }),
        Err(_) => Json(GetViewsPerPageOutput {
            data: None,
            success: false,
        }),
    }
}

#[handler]
pub async fn total_unique_users(
    Data(s): Data<&Arc<Store>>,
    Json(data): Json<GetViewsPerPageInput>,
) -> Json<GetTotalUniqueUsersOutput> {
    let res = s.get_total_unique_users(data.website).await;

    match res {
        Ok(d) => Json(GetTotalUniqueUsersOutput {
            data: Some(d),
            success: true,
        }),
        Err(_) => Json(GetTotalUniqueUsersOutput {
            data: None,
            success: false,
        }),
    }
}
#[handler]
pub async fn total_views(
    Data(s): Data<&Arc<Store>>,
    Json(data): Json<GetViewsPerPageInput>,
) -> Json<GetTotalViewsOutput> {
    let res = s.get_total_views(data.website).await;

    match res {
        Ok(d) => Json(GetTotalViewsOutput {
            data: Some(d),
            success: true,
        }),
        Err(_) => Json(GetTotalViewsOutput {
            data: None,
            success: false,
        }),
    }
}

#[handler]
pub async fn get_user(
    Data(s): Data<&Arc<Store>>,
    UserIdFromHeader(user_id): UserIdFromHeader,
) -> Json<User> {
    if user_id.len() <= 0 {
        print!("User id not found");
        return Json(User {
            id: "()".to_string(),
            name: "()".to_string(),
            email: "()".to_string(),
            plan_type: "".to_string(),
            success: false,
        });
    }
    let res = s.get_user(user_id).await;

    match res {
        Ok(user) => match user {
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
        },
        Err(_) => Json(User {
            id: "".to_string(),
            name: "".to_string(),
            email: "".to_string(),
            plan_type: "".to_string(),
            success: false,
        }),
    }
}

#[handler]
pub async fn get_health() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .body("OK")
}