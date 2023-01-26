//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSFetchIndexDescription")]
    pub struct NSFetchIndexDescription;

    #[cfg(feature = "CoreData_NSFetchIndexDescription")]
    unsafe impl ClassType for NSFetchIndexDescription {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreData_NSFetchIndexDescription")]
unsafe impl NSCoding for NSFetchIndexDescription {}

extern_methods!(
    #[cfg(feature = "CoreData_NSFetchIndexDescription")]
    unsafe impl NSFetchIndexDescription {
        #[cfg(all(
            feature = "CoreData_NSFetchIndexElementDescription",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithName:elements:)]
        pub unsafe fn initWithName_elements(
            this: Option<Allocated<Self>>,
            name: &NSString,
            elements: Option<&NSArray<NSFetchIndexElementDescription>>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);

        #[cfg(all(
            feature = "CoreData_NSFetchIndexElementDescription",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other elements)]
        pub unsafe fn elements(&self) -> Id<NSArray<NSFetchIndexElementDescription>, Shared>;

        #[cfg(all(
            feature = "CoreData_NSFetchIndexElementDescription",
            feature = "Foundation_NSArray"
        ))]
        #[method(setElements:)]
        pub unsafe fn setElements(&self, elements: &NSArray<NSFetchIndexElementDescription>);

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Other entity)]
        pub unsafe fn entity(&self) -> Option<Id<NSEntityDescription, Shared>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other partialIndexPredicate)]
        pub unsafe fn partialIndexPredicate(&self) -> Option<Id<NSPredicate, Shared>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(setPartialIndexPredicate:)]
        pub unsafe fn setPartialIndexPredicate(
            &self,
            partial_index_predicate: Option<&NSPredicate>,
        );
    }
);
