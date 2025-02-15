//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKCloudPlayer")]
    #[deprecated = "GKGameSession is deprecated. Use GKPlayer for both real-time and turn-based matchmaking APIs."]
    pub struct GKCloudPlayer;

    #[cfg(feature = "GameKit_GKCloudPlayer")]
    unsafe impl ClassType for GKCloudPlayer {
        #[inherits(NSObject)]
        type Super = GKBasePlayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKCloudPlayer")]
unsafe impl NSObjectProtocol for GKCloudPlayer {}

extern_methods!(
    #[cfg(feature = "GameKit_GKCloudPlayer")]
    unsafe impl GKCloudPlayer {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[deprecated = "GKGameSession is deprecated. Use GKPlayer for both real-time and turn-based matchmaking APIs."]
        #[method(getCurrentSignedInPlayerForContainer:completionHandler:)]
        pub unsafe fn getCurrentSignedInPlayerForContainer_completionHandler(
            container_name: Option<&NSString>,
            handler: &Block<(*mut GKCloudPlayer, *mut NSError), ()>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameKit_GKCloudPlayer")]
    unsafe impl GKCloudPlayer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
