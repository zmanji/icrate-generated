//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLMapElement")]
    #[deprecated]
    pub struct DOMHTMLMapElement;

    #[cfg(feature = "WebKit_DOMHTMLMapElement")]
    unsafe impl ClassType for DOMHTMLMapElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLMapElement")]
unsafe impl DOMEventTarget for DOMHTMLMapElement {}

#[cfg(feature = "WebKit_DOMHTMLMapElement")]
unsafe impl NSCopying for DOMHTMLMapElement {}

#[cfg(feature = "WebKit_DOMHTMLMapElement")]
unsafe impl NSObjectProtocol for DOMHTMLMapElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLMapElement")]
    unsafe impl DOMHTMLMapElement {
        #[cfg(feature = "WebKit_DOMHTMLCollection")]
        #[deprecated]
        #[method_id(@__retain_semantics Other areas)]
        pub unsafe fn areas(&self) -> Option<Id<DOMHTMLCollection>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLMapElement")]
    unsafe impl DOMHTMLMapElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLMapElement")]
    unsafe impl DOMHTMLMapElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
