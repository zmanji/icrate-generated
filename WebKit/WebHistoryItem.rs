//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_static!(WebHistoryItemChangedNotification: Option<&'static NSString>);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WebHistoryItem")]
    #[deprecated]
    pub struct WebHistoryItem;

    #[cfg(feature = "WebKit_WebHistoryItem")]
    unsafe impl ClassType for WebHistoryItem {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_WebHistoryItem")]
unsafe impl NSCopying for WebHistoryItem {}

#[cfg(feature = "WebKit_WebHistoryItem")]
unsafe impl NSObjectProtocol for WebHistoryItem {}

extern_methods!(
    #[cfg(feature = "WebKit_WebHistoryItem")]
    unsafe impl WebHistoryItem {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithURLString:title:lastVisitedTimeInterval:)]
        pub unsafe fn initWithURLString_title_lastVisitedTimeInterval(
            this: Option<Allocated<Self>>,
            url_string: Option<&NSString>,
            title: Option<&NSString>,
            time: NSTimeInterval,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other originalURLString)]
        pub unsafe fn originalURLString(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other URLString)]
        pub unsafe fn URLString(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[deprecated]
        #[method(lastVisitedTimeInterval)]
        pub unsafe fn lastVisitedTimeInterval(&self) -> NSTimeInterval;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other alternateTitle)]
        pub unsafe fn alternateTitle(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setAlternateTitle:)]
        pub unsafe fn setAlternateTitle(&self, alternate_title: Option<&NSString>);

        #[cfg(feature = "AppKit_NSImage")]
        #[deprecated]
        #[method_id(@__retain_semantics Other icon)]
        pub unsafe fn icon(&self) -> Option<Id<NSImage>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_WebHistoryItem")]
    unsafe impl WebHistoryItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
