//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CKShareParticipantAcceptanceStatus {
        CKShareParticipantAcceptanceStatusUnknown = 0,
        CKShareParticipantAcceptanceStatusPending = 1,
        CKShareParticipantAcceptanceStatusAccepted = 2,
        CKShareParticipantAcceptanceStatusRemoved = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CKShareParticipantPermission {
        CKShareParticipantPermissionUnknown = 0,
        CKShareParticipantPermissionNone = 1,
        CKShareParticipantPermissionReadOnly = 2,
        CKShareParticipantPermissionReadWrite = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CKShareParticipantRole {
        CKShareParticipantRoleUnknown = 0,
        CKShareParticipantRoleOwner = 1,
        CKShareParticipantRolePrivateUser = 3,
        CKShareParticipantRolePublicUser = 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[deprecated]
    pub enum CKShareParticipantType {
        #[deprecated]
        CKShareParticipantTypeUnknown = 0,
        #[deprecated]
        CKShareParticipantTypeOwner = 1,
        #[deprecated]
        CKShareParticipantTypePrivateUser = 3,
        #[deprecated]
        CKShareParticipantTypePublicUser = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKShareParticipant")]
    pub struct CKShareParticipant;

    #[cfg(feature = "CloudKit_CKShareParticipant")]
    unsafe impl ClassType for CKShareParticipant {
        type Super = NSObject;
    }
);

#[cfg(feature = "CloudKit_CKShareParticipant")]
unsafe impl NSSecureCoding for CKShareParticipant {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKShareParticipant")]
    unsafe impl CKShareParticipant {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[cfg(feature = "CloudKit_CKUserIdentity")]
        #[method_id(@__retain_semantics Other userIdentity)]
        pub unsafe fn userIdentity(&self) -> Id<CKUserIdentity, Shared>;

        #[method(role)]
        pub unsafe fn role(&self) -> CKShareParticipantRole;

        #[method(setRole:)]
        pub unsafe fn setRole(&self, role: CKShareParticipantRole);

        #[deprecated]
        #[method(type)]
        pub unsafe fn r#type(&self) -> CKShareParticipantType;

        #[deprecated]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: CKShareParticipantType);

        #[method(acceptanceStatus)]
        pub unsafe fn acceptanceStatus(&self) -> CKShareParticipantAcceptanceStatus;

        #[method(permission)]
        pub unsafe fn permission(&self) -> CKShareParticipantPermission;

        #[method(setPermission:)]
        pub unsafe fn setPermission(&self, permission: CKShareParticipantPermission);
    }
);
