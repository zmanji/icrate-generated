//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLTableCellElement")]
    #[deprecated]
    pub struct DOMHTMLTableCellElement;

    #[cfg(feature = "WebKit_DOMHTMLTableCellElement")]
    unsafe impl ClassType for DOMHTMLTableCellElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLTableCellElement")]
unsafe impl DOMEventTarget for DOMHTMLTableCellElement {}

#[cfg(feature = "WebKit_DOMHTMLTableCellElement")]
unsafe impl NSCopying for DOMHTMLTableCellElement {}

#[cfg(feature = "WebKit_DOMHTMLTableCellElement")]
unsafe impl NSObjectProtocol for DOMHTMLTableCellElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLTableCellElement")]
    unsafe impl DOMHTMLTableCellElement {
        #[deprecated]
        #[method(cellIndex)]
        pub unsafe fn cellIndex(&self) -> c_int;

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
        #[method_id(@__retain_semantics Other axis)]
        pub unsafe fn axis(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setAxis:)]
        pub unsafe fn setAxis(&self, axis: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other bgColor)]
        pub unsafe fn bgColor(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setBgColor:)]
        pub unsafe fn setBgColor(&self, bg_color: Option<&NSString>);

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
        #[method(colSpan)]
        pub unsafe fn colSpan(&self) -> c_int;

        #[deprecated]
        #[method(setColSpan:)]
        pub unsafe fn setColSpan(&self, col_span: c_int);

        #[deprecated]
        #[method(rowSpan)]
        pub unsafe fn rowSpan(&self) -> c_int;

        #[deprecated]
        #[method(setRowSpan:)]
        pub unsafe fn setRowSpan(&self, row_span: c_int);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other headers)]
        pub unsafe fn headers(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setHeaders:)]
        pub unsafe fn setHeaders(&self, headers: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other height)]
        pub unsafe fn height(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setHeight:)]
        pub unsafe fn setHeight(&self, height: Option<&NSString>);

        #[deprecated]
        #[method(noWrap)]
        pub unsafe fn noWrap(&self) -> bool;

        #[deprecated]
        #[method(setNoWrap:)]
        pub unsafe fn setNoWrap(&self, no_wrap: bool);

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

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other abbr)]
        pub unsafe fn abbr(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setAbbr:)]
        pub unsafe fn setAbbr(&self, abbr: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other scope)]
        pub unsafe fn scope(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setScope:)]
        pub unsafe fn setScope(&self, scope: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLTableCellElement")]
    unsafe impl DOMHTMLTableCellElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLTableCellElement")]
    unsafe impl DOMHTMLTableCellElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
