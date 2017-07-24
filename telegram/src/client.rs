use tokio::reactor::Handle;
use futures::{future, Stream, Future};
use ser::Serialize;
use hyper::{self, Body};
use hyper::client::HttpConnector;
use request::Request;
use error;

pub struct Client {
    http_client: hyper::Client<HttpConnector, Body>,
}

impl Client {
    /// Create a new Telegram client.
    #[inline]
    pub fn new(handle: &Handle) -> Client {
        Client {
            http_client: hyper::Client::new(handle),
        }
    }

    // Send a constructed request using this Client.
    pub fn request<T: Serialize>(
        &self,
        req: Request<T>,
    ) -> Box<Future<Item = Vec<u8>, Error = error::Error>> {
        let http_request = match req.to_http_request() {
            Ok(req) => req,
            Err(error) => return Box::new(future::err(error)),
        };

        Box::new(
            self.http_client
                .request(http_request)
                .and_then(|res| res.body().concat2())
                .map(|data| data.to_vec())
                .map_err(|err| err.into())
        )
    }
}
