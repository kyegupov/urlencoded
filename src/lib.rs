//! Url Encoded middleware for Iron
//! 
//! This middleware focuses on parsing the incoming url parameters from client requests.
#![crate_id = "urlencoded"]
#![license = "MIT"]

extern crate url;
extern crate iron;
extern crate serialize;

use iron::{Request, Response, Middleware, Alloy};
use iron::mixin::GetUrl;
use iron::middleware::{Status, Continue, Unwind};

use url::from_str;
use std::collections::HashMap;

/// Stores a `HashMap` of a `String` and a `Vec<Strings>` to address
/// client data that is sent with multiple values for a single key.
#[deriving(Clone)]
pub struct Encoded(pub HashMap<String, Vec<String>>);

/// This middleware is used for parsing url parameters and storing
/// the data as conveniently accessible data `insert`ed into an Alloy. 
#[deriving(Clone)]
pub struct UrlEncoded;

/// Creates a `UrlEncoded` instance to `link` to `server.chain`. Calling the
/// function will `insert` a new `HashMap` into the `alloy`.
impl UrlEncoded {
    pub fn new() -> UrlEncoded {
        UrlEncoded
    }
}

impl Middleware for UrlEncoded {
    fn enter(&mut self, req: &mut Request, res: &mut Response, alloy: &mut Alloy) -> Status {

        let query = match url::path_from_str(req.url().unwrap().as_slice()) {
            Ok(e) => {
                e.query
            },
            Err(e) => {
                return Continue;
            }
        };
        alloy.insert::<Encoded>(Encoded(createHash(query)));
        Continue
    }
}

fn createHash(q: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut hashStrVec: HashMap<String, Vec<String>> = HashMap::new();
 
    for (k, v) in q.move_iter() {
        hashStrVec.find_with_or_insert_with(
            k, v,
            |_, already, new| {
                already.push(new);
            },
            |_, v| vec![v]
        );
    }
    hashStrVec
}
