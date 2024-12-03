use ixc_message_api::AccountID;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::code::ErrorCode::SystemCode;
use ixc_message_api::code::SystemCode::UnauthorizedCallerAccess;
use ixc_message_api::packet::MessagePacket;

pub trait AuthorizationMiddleware {
    fn authorize(&self, real_caller: AccountID, msg: &MessagePacket) -> Result<(), ErrorCode>;
}

impl AuthorizationMiddleware for () {
    fn authorize(&self, _real_caller: AccountID, _msg: &MessagePacket) -> Result<(),ErrorCode> {
        Err(SystemCode(UnauthorizedCallerAccess))
    }
}