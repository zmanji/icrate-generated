//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait ASWebAuthenticationSessionRequestDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "AuthenticationServices_ASWebAuthenticationSessionRequest",
            feature = "Foundation_NSURL"
        ))]
        #[optional]
        #[method(authenticationSessionRequest:didCompleteWithCallbackURL:)]
        unsafe fn authenticationSessionRequest_didCompleteWithCallbackURL(
            &self,
            authentication_session_request: &ASWebAuthenticationSessionRequest,
            callback_url: &NSURL,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASWebAuthenticationSessionRequest",
            feature = "Foundation_NSError"
        ))]
        #[optional]
        #[method(authenticationSessionRequest:didCancelWithError:)]
        unsafe fn authenticationSessionRequest_didCancelWithError(
            &self,
            authentication_session_request: &ASWebAuthenticationSessionRequest,
            error: &NSError,
        );
    }

    unsafe impl ProtocolType for dyn ASWebAuthenticationSessionRequestDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSessionRequest")]
    pub struct ASWebAuthenticationSessionRequest;

    #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSessionRequest")]
    unsafe impl ClassType for ASWebAuthenticationSessionRequest {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSessionRequest")]
    unsafe impl ASWebAuthenticationSessionRequest {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Id<NSUUID, Shared>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Id<NSURL, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other callbackURLScheme)]
        pub unsafe fn callbackURLScheme(&self) -> Option<Id<NSString, Shared>>;

        #[method(shouldUseEphemeralSession)]
        pub unsafe fn shouldUseEphemeralSession(&self) -> bool;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn ASWebAuthenticationSessionRequestDelegate>, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn ASWebAuthenticationSessionRequestDelegate>>,
        );

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(cancelWithError:)]
        pub unsafe fn cancelWithError(&self, error: &NSError);

        #[cfg(feature = "Foundation_NSURL")]
        #[method(completeWithCallbackURL:)]
        pub unsafe fn completeWithCallbackURL(&self, url: &NSURL);
    }
);
