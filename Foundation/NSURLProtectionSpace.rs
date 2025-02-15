//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSURLProtectionSpaceHTTP: &'static NSString);

extern_static!(NSURLProtectionSpaceHTTPS: &'static NSString);

extern_static!(NSURLProtectionSpaceFTP: &'static NSString);

extern_static!(NSURLProtectionSpaceHTTPProxy: &'static NSString);

extern_static!(NSURLProtectionSpaceHTTPSProxy: &'static NSString);

extern_static!(NSURLProtectionSpaceFTPProxy: &'static NSString);

extern_static!(NSURLProtectionSpaceSOCKSProxy: &'static NSString);

extern_static!(NSURLAuthenticationMethodDefault: &'static NSString);

extern_static!(NSURLAuthenticationMethodHTTPBasic: &'static NSString);

extern_static!(NSURLAuthenticationMethodHTTPDigest: &'static NSString);

extern_static!(NSURLAuthenticationMethodHTMLForm: &'static NSString);

extern_static!(NSURLAuthenticationMethodNTLM: &'static NSString);

extern_static!(NSURLAuthenticationMethodNegotiate: &'static NSString);

extern_static!(NSURLAuthenticationMethodClientCertificate: &'static NSString);

extern_static!(NSURLAuthenticationMethodServerTrust: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSURLProtectionSpace")]
    pub struct NSURLProtectionSpace;

    #[cfg(feature = "Foundation_NSURLProtectionSpace")]
    unsafe impl ClassType for NSURLProtectionSpace {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSURLProtectionSpace")]
unsafe impl Send for NSURLProtectionSpace {}

#[cfg(feature = "Foundation_NSURLProtectionSpace")]
unsafe impl Sync for NSURLProtectionSpace {}

#[cfg(feature = "Foundation_NSURLProtectionSpace")]
unsafe impl NSCoding for NSURLProtectionSpace {}

#[cfg(feature = "Foundation_NSURLProtectionSpace")]
unsafe impl NSCopying for NSURLProtectionSpace {}

#[cfg(feature = "Foundation_NSURLProtectionSpace")]
unsafe impl NSObjectProtocol for NSURLProtectionSpace {}

#[cfg(feature = "Foundation_NSURLProtectionSpace")]
unsafe impl NSSecureCoding for NSURLProtectionSpace {}

extern_methods!(
    #[cfg(feature = "Foundation_NSURLProtectionSpace")]
    unsafe impl NSURLProtectionSpace {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithHost:port:protocol:realm:authenticationMethod:)]
        pub unsafe fn initWithHost_port_protocol_realm_authenticationMethod(
            this: Option<Allocated<Self>>,
            host: &NSString,
            port: NSInteger,
            protocol: Option<&NSString>,
            realm: Option<&NSString>,
            authentication_method: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithProxyHost:port:type:realm:authenticationMethod:)]
        pub unsafe fn initWithProxyHost_port_type_realm_authenticationMethod(
            this: Option<Allocated<Self>>,
            host: &NSString,
            port: NSInteger,
            r#type: Option<&NSString>,
            realm: Option<&NSString>,
            authentication_method: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other realm)]
        pub unsafe fn realm(&self) -> Option<Id<NSString>>;

        #[method(receivesCredentialSecurely)]
        pub unsafe fn receivesCredentialSecurely(&self) -> bool;

        #[method(isProxy)]
        pub unsafe fn isProxy(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other host)]
        pub unsafe fn host(&self) -> Id<NSString>;

        #[method(port)]
        pub unsafe fn port(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other proxyType)]
        pub unsafe fn proxyType(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other protocol)]
        pub unsafe fn protocol(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other authenticationMethod)]
        pub unsafe fn authenticationMethod(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSURLProtectionSpace")]
    unsafe impl NSURLProtectionSpace {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSClientCertificateSpace
    #[cfg(feature = "Foundation_NSURLProtectionSpace")]
    unsafe impl NSURLProtectionSpace {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSData"))]
        #[method_id(@__retain_semantics Other distinguishedNames)]
        pub unsafe fn distinguishedNames(&self) -> Option<Id<NSArray<NSData>>>;
    }
);

extern_methods!(
    /// NSServerTrustValidationSpace
    #[cfg(feature = "Foundation_NSURLProtectionSpace")]
    unsafe impl NSURLProtectionSpace {}
);
