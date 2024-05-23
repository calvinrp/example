#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::http::incoming_handler::{Guest};
use bindings::wasi::http::types::{IncomingRequest, ResponseOutparam, OutgoingResponse, OutgoingBody, Fields};
use bindings::macovedj::hashimap::macovedj_shapes_hashimap::Hashimap;

struct Component;

impl Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let hdrs = Fields::new();
        let resp = OutgoingResponse::new(hdrs);
        let body = resp.body().expect("outgoing response");

        ResponseOutparam::set(response_out, Ok(resp));

        let map = Hashimap::new();
        let msg = format!("{:?}", map.keys());

        let out = body.write().expect("outgoing stream");
        out.blocking_write_and_flush(msg.as_bytes())
            .expect("writing response");

        drop(out);
        OutgoingBody::finish(body, None).unwrap();
    }
}

bindings::export!(Component with_types_in bindings);
