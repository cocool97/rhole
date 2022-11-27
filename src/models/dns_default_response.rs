use trust_dns_client::{
    op::{MessageType, ResponseCode},
    rr::Record,
};
use trust_dns_server::{
    authority::{MessageResponse, MessageResponseBuilder},
    server::Request,
};

pub fn dns_default_response(
    request: &Request,
    response_code: ResponseCode,
) -> MessageResponse<
    impl Iterator<Item = &Record> + Send,
    impl Iterator<Item = &Record> + Send,
    impl Iterator<Item = &Record> + Send,
    impl Iterator<Item = &Record> + Send,
> {
    let response_builder = MessageResponseBuilder::from_message_request(request);
    let mut response = response_builder.error_msg(request.header(), response_code);

    response
        .header_mut()
        .set_message_type(MessageType::Response);

    response
}
