use ixc_message_api::code::ErrorCode;
use ixc_message_api::message::{Message, Request, Response};
use ixc_message_api::AccountID;

pub trait Observer {
    fn before_invoke(&self, operation: InvokeType, caller: &AccountID, message: &Message);
    fn after_invoke(&self, res: &Result<Response, ErrorCode>);
    fn on_state_update(
        &self,
        account: &AccountID,
        req: &Request,
        res: &Result<Response, ErrorCode>,
    );
    fn on_state_query(&self, account: &AccountID, req: &Request, res: &Result<Response, ErrorCode>);
}

pub enum InvokeType {
    InvokeMsg,
    InvokeQuery,
    InvokeSystem,
}
