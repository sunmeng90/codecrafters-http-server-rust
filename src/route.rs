use std::collections::HashMap;

use regex::Regex;

use crate::handlers::{handle_404, handle_base};
use crate::req::Request;
use crate::resp::Response;

pub type Handler = fn(&Request, &HashMap<String, String>) -> Response;

struct Route {
    pattern: Regex,
    handler: Handler,
}
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: Vec::new(),
        }
    }
    pub fn add_route(&mut self, path: &str, handler: Handler) {
        let regex_pattern = Regex::new(&route_to_regex(path)).unwrap();
        self.routes.push(Route {
            pattern: regex_pattern,
            handler,
        });
    }

    pub fn handle_req(&self, req: &Request) -> Response {
        if req.uri == "/" || req.uri.is_empty() {
            let params_map = HashMap::new();
            return handle_base(req, &params_map);
        }

        for route in &self.routes {
            if let Some(captures) = route.pattern.captures(&req.uri) {
                let mut params_map = HashMap::new();
                println!("Req: {}, match pattern: {:?}, capture groups: {:?}", req.uri, route.pattern, route.pattern.capture_names());
                for (name, value) in route.pattern.capture_names().skip(1).zip(captures.iter().skip(1)) {
                    if let (Some(name), Some(value)) = (name, value) {
                        params_map.insert(name.to_string(), value.as_str().to_string());
                    }
                }
                return (route.handler)(&req, &params_map);
            }
        }

        handle_404(req)
    }
}


fn route_to_regex<S: Into<String>>(path: S) -> String {
    let mut regex = "^".to_string();
    path.into().split("/")
        .for_each(|part| {
            if part.starts_with(":") {
                regex.push_str(&format!(r"/(?P<{}>[^/]+)", part[1..].to_string()))
            } else if !part.is_empty() {
                regex.push_str(&format!(r"/{}", part))
            }
        });
    regex.push_str("?$");
    regex
}
