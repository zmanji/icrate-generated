//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_protocol!(
    pub unsafe trait WKURLSchemeTask: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSURLRequest")]
        #[method_id(@__retain_semantics Other request)]
        unsafe fn request(&self) -> Id<NSURLRequest, Shared>;

        #[cfg(feature = "Foundation_NSURLResponse")]
        #[method(didReceiveResponse:)]
        unsafe fn didReceiveResponse(&self, response: &NSURLResponse);

        #[cfg(feature = "Foundation_NSData")]
        #[method(didReceiveData:)]
        unsafe fn didReceiveData(&self, data: &NSData);

        #[method(didFinish)]
        unsafe fn didFinish(&self);

        #[cfg(feature = "Foundation_NSError")]
        #[method(didFailWithError:)]
        unsafe fn didFailWithError(&self, error: &NSError);
    }

    unsafe impl ProtocolType for dyn WKURLSchemeTask {}
);
