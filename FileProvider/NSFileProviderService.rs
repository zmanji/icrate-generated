//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::FileProvider::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct NSFileProviderServiceSource;

    unsafe impl ProtocolType for NSFileProviderServiceSource {
        #[method_id(@__retain_semantics Other serviceName)]
        pub unsafe fn serviceName(&self) -> Id<NSFileProviderServiceName, Shared>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSXPCListenerEndpoint"
        ))]
        #[method_id(@__retain_semantics Other makeListenerEndpointAndReturnError:_)]
        pub unsafe fn makeListenerEndpointAndReturnError(
            &self,
        ) -> Result<Id<NSXPCListenerEndpoint, Shared>, Id<NSError, Shared>>;

        #[optional]
        #[method(isRestricted)]
        pub unsafe fn isRestricted(&self) -> bool;
    }
);

extern_methods!(
    /// NSFileProviderService
    #[cfg(feature = "FileProvider_NSFileProviderExtension")]
    unsafe impl NSFileProviderExtension {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Other supportedServiceSourcesForItemIdentifier:error:_)]
        pub unsafe fn supportedServiceSourcesForItemIdentifier_error(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
        ) -> Result<Id<NSArray<NSFileProviderServiceSource>, Shared>, Id<NSError, Shared>>;
    }
);

extern_methods!(
    /// NSFileProviderService
    #[cfg(feature = "FileProvider_NSFileProviderManager")]
    unsafe impl NSFileProviderManager {
        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSFileProviderService"
        ))]
        #[method(getServiceWithName:itemIdentifier:completionHandler:)]
        pub unsafe fn getServiceWithName_itemIdentifier_completionHandler(
            &self,
            service_name: &NSFileProviderServiceName,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<(*mut NSFileProviderService, *mut NSError), ()>,
        );
    }
);
