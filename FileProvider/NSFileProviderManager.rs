//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::FileProvider::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSFileProviderDomainRemovalMode {
        NSFileProviderDomainRemovalModeRemoveAll = 0,
        NSFileProviderDomainRemovalModePreserveDirtyUserData = 1,
        NSFileProviderDomainRemovalModePreserveDownloadedUserData = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "FileProvider_NSFileProviderManager")]
    pub struct NSFileProviderManager;

    #[cfg(feature = "FileProvider_NSFileProviderManager")]
    unsafe impl ClassType for NSFileProviderManager {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "FileProvider_NSFileProviderManager")]
    unsafe impl NSFileProviderManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other defaultManager)]
        pub unsafe fn defaultManager() -> Id<NSFileProviderManager, Shared>;

        #[cfg(feature = "FileProvider_NSFileProviderDomain")]
        #[method_id(@__retain_semantics Other managerForDomain:)]
        pub unsafe fn managerForDomain(domain: &NSFileProviderDomain) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(signalEnumeratorForContainerItemIdentifier:completionHandler:)]
        pub unsafe fn signalEnumeratorForContainerItemIdentifier_completionHandler(
            &self,
            container_item_identifier: &NSFileProviderItemIdentifier,
            completion: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(getUserVisibleURLForItemIdentifier:completionHandler:)]
        pub unsafe fn getUserVisibleURLForItemIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<(*mut NSURL, *mut NSError), ()>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(getIdentifierForUserVisibleFileAtURL:completionHandler:)]
        pub unsafe fn getIdentifierForUserVisibleFileAtURL_completionHandler(
            url: &NSURL,
            completion_handler: &Block<
                (
                    *mut NSFileProviderItemIdentifier,
                    *mut NSFileProviderDomainIdentifier,
                    *mut NSError,
                ),
                (),
            >,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSURLSessionTask"
        ))]
        #[method(registerURLSessionTask:forItemWithIdentifier:completionHandler:)]
        pub unsafe fn registerURLSessionTask_forItemWithIdentifier_completionHandler(
            &self,
            task: &NSURLSessionTask,
            identifier: &NSFileProviderItemIdentifier,
            completion: &Block<(*mut NSError,), ()>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other providerIdentifier)]
        pub unsafe fn providerIdentifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other documentStorageURL)]
        pub unsafe fn documentStorageURL(&self) -> Id<NSURL, Shared>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other temporaryDirectoryURLWithError:_)]
        pub unsafe fn temporaryDirectoryURLWithError(
            &self,
        ) -> Result<Id<NSURL, Shared>, Id<NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(writePlaceholderAtURL:withMetadata:error:_)]
        pub unsafe fn writePlaceholderAtURL_withMetadata_error(
            placeholder_url: &NSURL,
            metadata: &NSFileProviderItem,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other placeholderURLForURL:)]
        pub unsafe fn placeholderURLForURL(url: &NSURL) -> Id<NSURL, Shared>;

        #[cfg(all(
            feature = "FileProvider_NSFileProviderDomain",
            feature = "Foundation_NSError"
        ))]
        #[method(addDomain:completionHandler:)]
        pub unsafe fn addDomain_completionHandler(
            domain: &NSFileProviderDomain,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(
            feature = "FileProvider_NSFileProviderDomain",
            feature = "Foundation_NSError"
        ))]
        #[method(removeDomain:completionHandler:)]
        pub unsafe fn removeDomain_completionHandler(
            domain: &NSFileProviderDomain,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(
            feature = "FileProvider_NSFileProviderDomain",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method(removeDomain:mode:completionHandler:)]
        pub unsafe fn removeDomain_mode_completionHandler(
            domain: &NSFileProviderDomain,
            mode: NSFileProviderDomainRemovalMode,
            completion_handler: &Block<(*mut NSURL, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "FileProvider_NSFileProviderDomain",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(getDomainsWithCompletionHandler:)]
        pub unsafe fn getDomainsWithCompletionHandler(
            completion_handler: &Block<(NonNull<NSArray<NSFileProviderDomain>>, *mut NSError), ()>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(removeAllDomainsWithCompletionHandler:)]
        pub unsafe fn removeAllDomainsWithCompletionHandler(
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(signalErrorResolved:completionHandler:)]
        pub unsafe fn signalErrorResolved_completionHandler(
            &self,
            error: &NSError,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(feature = "Foundation_NSProgress")]
        #[method_id(@__retain_semantics Other globalProgressForKind:)]
        pub unsafe fn globalProgressForKind(
            &self,
            kind: &NSProgressFileOperationKind,
        ) -> Id<NSProgress, Shared>;
    }
);

extern_static!(NSFileProviderMaterializedSetDidChange: &'static NSNotificationName);

extern_methods!(
    /// MaterializedSet
    #[cfg(feature = "FileProvider_NSFileProviderManager")]
    unsafe impl NSFileProviderManager {
        #[method_id(@__retain_semantics Other enumeratorForMaterializedItems)]
        pub unsafe fn enumeratorForMaterializedItems(&self)
            -> Id<NSFileProviderEnumerator, Shared>;
    }
);

extern_static!(NSFileProviderPendingSetDidChange: &'static NSNotificationName);

extern_protocol!(
    pub struct NSFileProviderPendingSetEnumerator;

    unsafe impl ProtocolType for NSFileProviderPendingSetEnumerator {
        #[cfg(feature = "FileProvider_NSFileProviderDomainVersion")]
        #[method_id(@__retain_semantics Other domainVersion)]
        pub unsafe fn domainVersion(&self) -> Option<Id<NSFileProviderDomainVersion, Shared>>;

        #[method(refreshInterval)]
        pub unsafe fn refreshInterval(&self) -> NSTimeInterval;

        #[method(isMaximumSizeReached)]
        pub unsafe fn isMaximumSizeReached(&self) -> bool;
    }
);

extern_methods!(
    /// PendingSet
    #[cfg(feature = "FileProvider_NSFileProviderManager")]
    unsafe impl NSFileProviderManager {
        #[method_id(@__retain_semantics Other enumeratorForPendingItems)]
        pub unsafe fn enumeratorForPendingItems(
            &self,
        ) -> Id<NSFileProviderPendingSetEnumerator, Shared>;
    }
);

extern_methods!(
    /// Import
    #[cfg(feature = "FileProvider_NSFileProviderManager")]
    unsafe impl NSFileProviderManager {
        #[cfg(all(
            feature = "FileProvider_NSFileProviderDomain",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method(importDomain:fromDirectoryAtURL:completionHandler:)]
        pub unsafe fn importDomain_fromDirectoryAtURL_completionHandler(
            domain: &NSFileProviderDomain,
            url: &NSURL,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(reimportItemsBelowItemWithIdentifier:completionHandler:)]
        pub unsafe fn reimportItemsBelowItemWithIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(requestModificationOfFields:forItemWithIdentifier:options:completionHandler:)]
        pub unsafe fn requestModificationOfFields_forItemWithIdentifier_options_completionHandler(
            &self,
            fields: NSFileProviderItemFields,
            item_identifier: &NSFileProviderItemIdentifier,
            options: NSFileProviderModifyItemOptions,
            completion_handler: &Block<(*mut NSError,), ()>,
        );
    }
);

extern_methods!(
    /// Eviction
    #[cfg(feature = "FileProvider_NSFileProviderManager")]
    unsafe impl NSFileProviderManager {
        #[cfg(feature = "Foundation_NSError")]
        #[method(evictItemWithIdentifier:completionHandler:)]
        pub unsafe fn evictItemWithIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<(*mut NSError,), ()>,
        );
    }
);

extern_methods!(
    /// Barrier
    #[cfg(feature = "FileProvider_NSFileProviderManager")]
    unsafe impl NSFileProviderManager {
        #[cfg(feature = "Foundation_NSError")]
        #[method(waitForChangesOnItemsBelowItemWithIdentifier:completionHandler:)]
        pub unsafe fn waitForChangesOnItemsBelowItemWithIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<(*mut NSError,), ()>,
        );
    }
);

extern_methods!(
    /// Stabilization
    #[cfg(feature = "FileProvider_NSFileProviderManager")]
    unsafe impl NSFileProviderManager {
        #[cfg(feature = "Foundation_NSError")]
        #[method(waitForStabilizationWithCompletionHandler:)]
        pub unsafe fn waitForStabilizationWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSError,), ()>,
        );
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileProviderManagerDisconnectionOptions {
        NSFileProviderManagerDisconnectionOptionsTemporary = 1 << 0,
    }
);

extern_methods!(
    /// Disconnection
    #[cfg(feature = "FileProvider_NSFileProviderManager")]
    unsafe impl NSFileProviderManager {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(disconnectWithReason:options:completionHandler:)]
        pub unsafe fn disconnectWithReason_options_completionHandler(
            &self,
            localized_reason: &NSString,
            options: NSFileProviderManagerDisconnectionOptions,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(reconnectWithCompletionHandler:)]
        pub unsafe fn reconnectWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSError,), ()>,
        );
    }
);

extern_methods!(
    /// Materialize
    #[cfg(feature = "FileProvider_NSFileProviderManager")]
    unsafe impl NSFileProviderManager {
        #[cfg(feature = "Foundation_NSError")]
        #[method(requestDownloadForItemWithIdentifier:requestedRange:completionHandler:)]
        pub unsafe fn requestDownloadForItemWithIdentifier_requestedRange_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            range_to_materialize: NSRange,
            completion_handler: &Block<(*mut NSError,), ()>,
        );
    }
);
