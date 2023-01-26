//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_protocol!(
    pub unsafe trait GCPhysicalInputElement: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sfSymbolsName)]
        unsafe fn sfSymbolsName(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedName)]
        unsafe fn localizedName(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other aliases)]
        unsafe fn aliases(&self) -> Id<NSSet<NSString>, Shared>;
    }

    unsafe impl ProtocolType for dyn GCPhysicalInputElement {}
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCPhysicalInputElementCollection")]
    pub struct GCPhysicalInputElementCollection<
        Key: Message = Object,
        Element: Message = Object,
        KeyOwnership: Ownership = Shared,
        ElementOwnership: Ownership = Shared,
    > {
        _inner0: PhantomData<*mut (Key, KeyOwnership)>,
        _inner1: PhantomData<*mut (Element, ElementOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "GameController_GCPhysicalInputElementCollection")]
    unsafe impl<Key: Message, Element: Message, KeyOwnership: Ownership, ElementOwnership: Ownership>
        ClassType
        for GCPhysicalInputElementCollection<Key, Element, KeyOwnership, ElementOwnership>
    {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "GameController_GCPhysicalInputElementCollection")]
    unsafe impl<Key: Message, Element: Message, KeyOwnership: Ownership, ElementOwnership: Ownership>
        GCPhysicalInputElementCollection<Key, Element, KeyOwnership, ElementOwnership>
    {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other elementForAlias:)]
        pub unsafe fn elementForAlias(&self, alias: &Key) -> Option<Id<Element, ElementOwnership>>;

        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        pub unsafe fn objectForKeyedSubscript(
            &self,
            key: &Key,
        ) -> Option<Id<Element, ElementOwnership>>;

        #[cfg(feature = "Foundation_NSEnumerator")]
        #[method_id(@__retain_semantics Other elementEnumerator)]
        pub unsafe fn elementEnumerator(&self) -> Id<NSEnumerator<Element>, Shared>;
    }
);
