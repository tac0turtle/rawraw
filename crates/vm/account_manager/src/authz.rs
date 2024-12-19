//! Authorization API

use ixc_message_api::code::ErrorCode;
use ixc_message_api::code::ErrorCode::SystemCode;
use ixc_message_api::code::SystemCode::UnauthorizedCallerAccess;
use ixc_message_api::packet::MessagePacket;
use ixc_message_api::AccountID;

/// Defines authorization middleware.
pub trait AuthorizationMiddleware {
    /// Decides if the real caller can run the provided message packet.
    fn authorize(&self, real_caller: AccountID, msg: &MessagePacket) -> Result<(), ErrorCode>;
}

impl AuthorizationMiddleware for () {
    fn authorize(&self, _real_caller: AccountID, _msg: &MessagePacket) -> Result<(), ErrorCode> {
        Err(SystemCode(UnauthorizedCallerAccess))
    }
}
