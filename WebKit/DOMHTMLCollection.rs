//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLCollection")]
    #[deprecated]
    pub struct DOMHTMLCollection;

    #[cfg(feature = "WebKit_DOMHTMLCollection")]
    unsafe impl ClassType for DOMHTMLCollection {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLCollection")]
unsafe impl NSCopying for DOMHTMLCollection {}

#[cfg(feature = "WebKit_DOMHTMLCollection")]
unsafe impl NSObjectProtocol for DOMHTMLCollection {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLCollection")]
    unsafe impl DOMHTMLCollection {
        #[deprecated]
        #[method(length)]
        pub unsafe fn length(&self) -> c_uint;

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other item:)]
        pub unsafe fn item(&self, index: c_uint) -> Option<Id<DOMNode>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMNode"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other namedItem:)]
        pub unsafe fn namedItem(&self, name: Option<&NSString>) -> Option<Id<DOMNode>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMNodeList"))]
        #[method_id(@__retain_semantics Other tags:)]
        pub unsafe fn tags(&self, name: Option<&NSString>) -> Option<Id<DOMNodeList>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLCollection")]
    unsafe impl DOMHTMLCollection {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLCollection")]
    unsafe impl DOMHTMLCollection {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
