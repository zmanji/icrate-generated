//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSFetchIndexElementType {
        NSFetchIndexElementTypeBinary = 0,
        NSFetchIndexElementTypeRTree = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSFetchIndexElementDescription")]
    pub struct NSFetchIndexElementDescription;

    #[cfg(feature = "CoreData_NSFetchIndexElementDescription")]
    unsafe impl ClassType for NSFetchIndexElementDescription {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreData_NSFetchIndexElementDescription")]
unsafe impl NSCoding for NSFetchIndexElementDescription {}

extern_methods!(
    #[cfg(feature = "CoreData_NSFetchIndexElementDescription")]
    unsafe impl NSFetchIndexElementDescription {
        #[cfg(feature = "CoreData_NSPropertyDescription")]
        #[method_id(@__retain_semantics Init initWithProperty:collationType:)]
        pub unsafe fn initWithProperty_collationType(
            this: Option<Allocated<Self>>,
            property: &NSPropertyDescription,
            collation_type: NSFetchIndexElementType,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "CoreData_NSPropertyDescription")]
        #[method_id(@__retain_semantics Other property)]
        pub unsafe fn property(&self) -> Option<Id<NSPropertyDescription, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other propertyName)]
        pub unsafe fn propertyName(&self) -> Option<Id<NSString, Shared>>;

        #[method(collationType)]
        pub unsafe fn collationType(&self) -> NSFetchIndexElementType;

        #[method(setCollationType:)]
        pub unsafe fn setCollationType(&self, collation_type: NSFetchIndexElementType);

        #[method(isAscending)]
        pub unsafe fn isAscending(&self) -> bool;

        #[method(setAscending:)]
        pub unsafe fn setAscending(&self, ascending: bool);

        #[cfg(feature = "CoreData_NSFetchIndexDescription")]
        #[method_id(@__retain_semantics Other indexDescription)]
        pub unsafe fn indexDescription(&self) -> Option<Id<NSFetchIndexDescription, Shared>>;
    }
);
