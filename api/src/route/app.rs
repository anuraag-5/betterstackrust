use std::sync::{Arc, Mutex};

use poem::{handler, http::header, web::{Data, Json}, Response};
use store::store::Store;

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
            referrer: document.referrer,
            user_agent: navigator.userAgent,
            time_stamp: Date.now(),
          };

          fetch("http://localhost:3000/api/track", {
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

}