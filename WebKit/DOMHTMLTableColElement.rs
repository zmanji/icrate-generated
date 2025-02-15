//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLTableColElement")]
    #[deprecated]
    pub struct DOMHTMLTableColElement;

    #[cfg(feature = "WebKit_DOMHTMLTableColElement")]
    unsafe impl ClassType for DOMHTMLTableColElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLTableColElement")]
unsafe impl DOMEventTarget for DOMHTMLTableColElement {}

#[cfg(feature = "WebKit_DOMHTMLTableColElement")]
unsafe impl NSCopying for DOMHTMLTableColElement {}

#[cfg(feature = "WebKit_DOMHTMLTableColElement")]
unsafe impl NSObjectProtocol for DOMHTMLTableColElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLTableColElement")]
    unsafe impl DOMHTMLTableColElement {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other ch)]
        pub unsafe fn ch(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setCh:)]
        pub unsafe fn setCh(&self, ch: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other chOff)]
        pub unsafe fn chOff(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setChOff:)]
        pub unsafe fn setChOff(&self, ch_off: Option<&NSString>);

        #[deprecated]
        #[method(span)]
        pub unsafe fn span(&self) -> c_int;

        #[deprecated]
        #[method(setSpan:)]
        pub unsafe fn setSpan(&self, span: c_int);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other vAlign)]
        pub unsafe fn vAlign(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setVAlign:)]
        pub unsafe fn setVAlign(&self, v_align: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other width)]
        pub unsafe fn width(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLTableColElement")]
    unsafe impl DOMHTMLTableColElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLTableColElement")]
    unsafe impl DOMHTMLTableColElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
