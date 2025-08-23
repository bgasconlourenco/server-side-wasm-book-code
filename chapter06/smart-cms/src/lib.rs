use std::io::Read;
use wasmcloud_component::http;
use wasmcloud_component::wasi::keyvalue::*;
 
struct Component;
 
http::export!(Component);
 
impl http::Server for Component {
    fn handle(
        request: http::IncomingRequest,
    ) -> http::Result<http::Response<impl http::OutgoingBody>> {
        let (parts, mut body) = request.into_parts();
 
        match parts.uri.path() {
            "/api/create" => {
                let bucket = store::open("default").unwrap();
 
                let mut buf = Vec::new();
                body.read_to_end(&mut buf).unwrap();
                let body = String::from_utf8(buf).unwrap();
 
                let mut lines = body.lines();
                let story_name = lines.next().unwrap_or("Unnamed Story").trim();
                let story_content = lines.collect::<Vec<&str>>().join("\n");
 
                bucket.set(story_name, story_content.as_bytes()).unwrap();
 
                Ok(http::Response::new(format!("Stored {}\n", story_name)))
            }
            "/api/retrieve" => {
                let bucket = store::open("default").unwrap();
 
                let mut buf = Vec::new();
                body.read_to_end(&mut buf).unwrap();
                let story_name = String::from_utf8(buf).unwrap().trim().to_string();
 
                match bucket.get(&story_name).unwrap() {
                    Some(content) => {
                        let story_content = String::from_utf8(content).unwrap();
                        Ok(http::Response::new(format!("{story_content}\n")))
                    }
                    None => {
                        Ok(http::Response::new("Story not found\n".to_string()))
                    }
                }
            }
            _ => {
                Ok(http::Response::new("Invalid route!\n".to_string()))
            }
        }
    }
}