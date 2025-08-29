use std::sync::{Arc, Mutex};
use url::Url;

use poem::{handler, http::header, web::{Data, Json}, Response};
use store::{models::app::PageVisit, store::Store};

use crate::request_input::TrackingInput;

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
pub fn track(
  Json(data): Json<TrackingInput>,
  Data(s):Data<&Arc<Mutex<Store>>>
){
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
              let page_visit = PageVisit {
                visitor_id,
                referrer,
                user_agent,
                page_url: current_path.to_string(),
                website_id: w.id
              };

              let _inserted_data = locked_s.store_tracks(page_visit);
              
            },
            Err(e) => {
              println!("{}",e.to_string())
            }
        }

      },
      Err(_) => {}
  }
}