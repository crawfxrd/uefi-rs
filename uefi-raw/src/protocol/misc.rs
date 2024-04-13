use crate::{guid, Guid, Status};
use crate::table::runtime;

#[derive(Debug)]
#[repr(C)]
pub struct TimestampProtocol {
    pub get_timestamp: unsafe extern "efiapi" fn() -> u64,
    pub get_properties: unsafe extern "efiapi" fn(*mut TimestampProperties) -> Status,
}

impl TimestampProtocol {
    pub const GUID: Guid = guid!("afbfde41-2e6e-4262-ba65-62b9236e5495");
}

/// Properties of the timestamp counter.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct TimestampProperties {
    /// Timestamp counter frequency, in Hz.
    pub frequency: u64,

    /// The maximum value of the timestamp counter before it rolls over. For
    /// example, a 24-bit counter would have an end value of `0xff_ffff`.
    pub end_value: u64,
}

/// Properties of Reset Notification.
#[derive(Debug)]
#[repr(C)]
pub struct ResetNotificationProtocol {
    pub register_reset_notify:
        unsafe extern "efiapi" fn(this: *mut Self, reset_function: Option<ResetSystemFn>) -> Status,
    pub unregister_reset_notify:
        unsafe extern "efiapi" fn(this: *mut Self, reset_function: Option<ResetSystemFn>) -> Status,
}

impl ResetNotificationProtocol {
    pub const GUID: Guid = guid!("9da34ae0-eaf9-4bbf-8ec3-fd60226c44be");
}

/// Raw reset notification function, to be called if you register it when a RestSystem() is executed.
// similar to uefi-raw/src/table/runtime.rs:53  at commit@6093205c3eb27b2e78be4c003c04d46679bff420
pub type ResetSystemFn = unsafe extern "efiapi" fn(
    rt: runtime::ResetType,
    status: Status,
    data_size: usize,
    data: *const u8,
);
