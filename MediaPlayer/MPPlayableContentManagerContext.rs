//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPPlayableContentManagerContext")]
    #[deprecated = "Use CarPlay framework"]
    pub struct MPPlayableContentManagerContext;

    #[cfg(feature = "MediaPlayer_MPPlayableContentManagerContext")]
    unsafe impl ClassType for MPPlayableContentManagerContext {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPPlayableContentManagerContext")]
unsafe impl NSObjectProtocol for MPPlayableContentManagerContext {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPPlayableContentManagerContext")]
    unsafe impl MPPlayableContentManagerContext {
        #[deprecated = "Use CarPlay framework"]
        #[method(enforcedContentItemsCount)]
        pub unsafe fn enforcedContentItemsCount(&self) -> NSInteger;

        #[deprecated = "Use CarPlay framework"]
        #[method(enforcedContentTreeDepth)]
        pub unsafe fn enforcedContentTreeDepth(&self) -> NSInteger;

        #[deprecated = "Use CarPlay framework"]
        #[method(contentLimitsEnforced)]
        pub unsafe fn contentLimitsEnforced(&self) -> bool;

        #[deprecated]
        #[method(contentLimitsEnabled)]
        pub unsafe fn contentLimitsEnabled(&self) -> bool;

        #[deprecated = "Use CarPlay framework"]
        #[method(endpointAvailable)]
        pub unsafe fn endpointAvailable(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPPlayableContentManagerContext")]
    unsafe impl MPPlayableContentManagerContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
