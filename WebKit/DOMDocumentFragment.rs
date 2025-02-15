//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMDocumentFragment")]
    #[deprecated]
    pub struct DOMDocumentFragment;

    #[cfg(feature = "WebKit_DOMDocumentFragment")]
    unsafe impl ClassType for DOMDocumentFragment {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMNode;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMDocumentFragment")]
unsafe impl DOMEventTarget for DOMDocumentFragment {}

#[cfg(feature = "WebKit_DOMDocumentFragment")]
unsafe impl NSCopying for DOMDocumentFragment {}

#[cfg(feature = "WebKit_DOMDocumentFragment")]
unsafe impl NSObjectProtocol for DOMDocumentFragment {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMDocumentFragment")]
    unsafe impl DOMDocumentFragment {}
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMDocumentFragment")]
    unsafe impl DOMDocumentFragment {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMDocumentFragment")]
    unsafe impl DOMDocumentFragment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
