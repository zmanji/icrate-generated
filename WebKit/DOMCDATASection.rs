//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMCDATASection")]
    #[deprecated]
    pub struct DOMCDATASection;

    #[cfg(feature = "WebKit_DOMCDATASection")]
    unsafe impl ClassType for DOMCDATASection {
        #[inherits(DOMCharacterData, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMText;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMCDATASection")]
unsafe impl DOMEventTarget for DOMCDATASection {}

#[cfg(feature = "WebKit_DOMCDATASection")]
unsafe impl NSCopying for DOMCDATASection {}

#[cfg(feature = "WebKit_DOMCDATASection")]
unsafe impl NSObjectProtocol for DOMCDATASection {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMCDATASection")]
    unsafe impl DOMCDATASection {}
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMCDATASection")]
    unsafe impl DOMCDATASection {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMCDATASection")]
    unsafe impl DOMCDATASection {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
