//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLUListElement")]
    #[deprecated]
    pub struct DOMHTMLUListElement;

    #[cfg(feature = "WebKit_DOMHTMLUListElement")]
    unsafe impl ClassType for DOMHTMLUListElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLUListElement")]
unsafe impl DOMEventTarget for DOMHTMLUListElement {}

#[cfg(feature = "WebKit_DOMHTMLUListElement")]
unsafe impl NSCopying for DOMHTMLUListElement {}

#[cfg(feature = "WebKit_DOMHTMLUListElement")]
unsafe impl NSObjectProtocol for DOMHTMLUListElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLUListElement")]
    unsafe impl DOMHTMLUListElement {
        #[deprecated]
        #[method(compact)]
        pub unsafe fn compact(&self) -> bool;

        #[deprecated]
        #[method(setCompact:)]
        pub unsafe fn setCompact(&self, compact: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLUListElement")]
    unsafe impl DOMHTMLUListElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLUListElement")]
    unsafe impl DOMHTMLUListElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
