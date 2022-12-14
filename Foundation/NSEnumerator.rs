//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_struct!(
    pub struct NSFastEnumerationState {
        pub state: c_ulong,
        pub itemsPtr: *mut *mut Object,
        pub mutationsPtr: *mut c_ulong,
        pub extra: [c_ulong; 5],
    }
);

extern_protocol!(
    pub struct NSFastEnumeration;

    unsafe impl ProtocolType for NSFastEnumeration {
        #[method(countByEnumeratingWithState:objects:count:)]
        pub unsafe fn countByEnumeratingWithState_objects_count(
            &self,
            state: NonNull<NSFastEnumerationState>,
            buffer: NonNull<*mut Object>,
            len: NSUInteger,
        ) -> NSUInteger;
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSEnumerator<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared> {
        _inner0: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> ClassType
        for NSEnumerator<ObjectType, ObjectTypeOwnership>
    {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSEnumerator<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other nextObject)]
        pub unsafe fn nextObject(&self) -> Option<Id<ObjectType, Shared>>;
    }
);

extern_methods!(
    /// NSExtendedEnumerator
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSEnumerator<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other allObjects)]
        pub unsafe fn allObjects(&self) -> Id<NSArray<ObjectType>, Shared>;
    }
);
