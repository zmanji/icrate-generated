//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CNContainerType {
        CNContainerTypeUnassigned = 0,
        CNContainerTypeLocal = 1,
        CNContainerTypeExchange = 2,
        CNContainerTypeCardDAV = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNContainer")]
    pub struct CNContainer;

    #[cfg(feature = "Contacts_CNContainer")]
    unsafe impl ClassType for CNContainer {
        type Super = NSObject;
    }
);

#[cfg(feature = "Contacts_CNContainer")]
unsafe impl NSSecureCoding for CNContainer {}

extern_methods!(
    #[cfg(feature = "Contacts_CNContainer")]
    unsafe impl CNContainer {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> CNContainerType;
    }
);

extern_static!(CNContainerIdentifierKey: &'static NSString);

extern_static!(CNContainerNameKey: &'static NSString);

extern_static!(CNContainerTypeKey: &'static NSString);
