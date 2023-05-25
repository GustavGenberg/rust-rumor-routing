use crate::messages::agent_message::AgentMessage;
use crate::messages::request_message::RequestMessage;
use crate::messages::response_message::ResponseMessage;

pub enum Message {
    AgentMessage(AgentMessage),
    RequestMessage(RequestMessage),
    ResponseMessage(ResponseMessage)
}