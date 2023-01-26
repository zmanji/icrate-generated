//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_static!(WebArchivePboardType: Option<&'static NSString>);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WebArchive")]
    #[deprecated]
    pub struct WebArchive;

    #[cfg(feature = "WebKit_WebArchive")]
    unsafe impl ClassType for WebArchive {
        type Super = NSObject;
    }
);

#[cfg(feature = "WebKit_WebArchive")]
unsafe impl NSCoding for WebArchive {}

extern_methods!(
    #[cfg(feature = "WebKit_WebArchive")]
    unsafe impl WebArchive {
        #[cfg(all(feature = "Foundation_NSArray", feature = "WebKit_WebResource"))]
        #[method_id(@__retain_semantics Init initWithMainResource:subresources:subframeArchives:)]
        pub unsafe fn initWithMainResource_subresources_subframeArchives(
            this: Option<Allocated<Self>>,
            main_resource: Option<&WebResource>,
            subresources: Option<&NSArray>,
            subframe_archives: Option<&NSArray>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Option<Allocated<Self>>,
            data: Option<&NSData>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "WebKit_WebResource")]
        #[method_id(@__retain_semantics Other mainResource)]
        pub unsafe fn mainResource(&self) -> Option<Id<WebResource, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other subresources)]
        pub unsafe fn subresources(&self) -> Id<NSArray, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other subframeArchives)]
        pub unsafe fn subframeArchives(&self) -> Id<NSArray, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Id<NSData, Shared>;
    }
);
