//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLFrameElement")]
    #[deprecated]
    pub struct DOMHTMLFrameElement;

    #[cfg(feature = "WebKit_DOMHTMLFrameElement")]
    unsafe impl ClassType for DOMHTMLFrameElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLFrameElement")]
unsafe impl DOMEventTarget for DOMHTMLFrameElement {}

#[cfg(feature = "WebKit_DOMHTMLFrameElement")]
unsafe impl NSCopying for DOMHTMLFrameElement {}

#[cfg(feature = "WebKit_DOMHTMLFrameElement")]
unsafe impl NSObjectProtocol for DOMHTMLFrameElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLFrameElement")]
    unsafe impl DOMHTMLFrameElement {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other frameBorder)]
        pub unsafe fn frameBorder(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setFrameBorder:)]
        pub unsafe fn setFrameBorder(&self, frame_border: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other longDesc)]
        pub unsafe fn longDesc(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setLongDesc:)]
        pub unsafe fn setLongDesc(&self, long_desc: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other marginHeight)]
        pub unsafe fn marginHeight(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setMarginHeight:)]
        pub unsafe fn setMarginHeight(&self, margin_height: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other marginWidth)]
        pub unsafe fn marginWidth(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setMarginWidth:)]
        pub unsafe fn setMarginWidth(&self, margin_width: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[deprecated]
        #[method(noResize)]
        pub unsafe fn noResize(&self) -> bool;

        #[deprecated]
        #[method(setNoResize:)]
        pub unsafe fn setNoResize(&self, no_resize: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other scrolling)]
        pub unsafe fn scrolling(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setScrolling:)]
        pub unsafe fn setScrolling(&self, scrolling: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other src)]
        pub unsafe fn src(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setSrc:)]
        pub unsafe fn setSrc(&self, src: Option<&NSString>);

        #[cfg(feature = "WebKit_DOMDocument")]
        #[deprecated]
        #[method_id(@__retain_semantics Other contentDocument)]
        pub unsafe fn contentDocument(&self) -> Option<Id<DOMDocument>>;

        #[cfg(feature = "WebKit_DOMAbstractView")]
        #[method_id(@__retain_semantics Other contentWindow)]
        pub unsafe fn contentWindow(&self) -> Option<Id<DOMAbstractView>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLocation:)]
        pub unsafe fn setLocation(&self, location: Option<&NSString>);

        #[method(width)]
        pub unsafe fn width(&self) -> c_int;

        #[method(height)]
        pub unsafe fn height(&self) -> c_int;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLFrameElement")]
    unsafe impl DOMHTMLFrameElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLFrameElement")]
    unsafe impl DOMHTMLFrameElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
