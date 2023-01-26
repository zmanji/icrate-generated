//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSWindowRestoration: NSObjectProtocol {
        #[cfg(all(
            feature = "AppKit_NSWindow",
            feature = "Foundation_NSCoder",
            feature = "Foundation_NSError"
        ))]
        #[method(restoreWindowWithIdentifier:state:completionHandler:)]
        unsafe fn restoreWindowWithIdentifier_state_completionHandler(
            identifier: &NSUserInterfaceItemIdentifier,
            state: &NSCoder,
            completion_handler: &Block<(*mut NSWindow, *mut NSError), ()>,
        );
    }

    unsafe impl ProtocolType for dyn NSWindowRestoration {}
);

extern_methods!(
    /// NSWindowRestoration
    #[cfg(feature = "AppKit_NSDocumentController")]
    unsafe impl NSDocumentController {}
);

extern_methods!(
    /// NSWindowRestoration
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[cfg(all(
            feature = "AppKit_NSWindow",
            feature = "Foundation_NSCoder",
            feature = "Foundation_NSError"
        ))]
        #[method(restoreWindowWithIdentifier:state:completionHandler:)]
        pub unsafe fn restoreWindowWithIdentifier_state_completionHandler(
            &self,
            identifier: &NSUserInterfaceItemIdentifier,
            state: &NSCoder,
            completion_handler: &Block<(*mut NSWindow, *mut NSError), ()>,
        ) -> bool;
    }
);

extern_static!(NSApplicationDidFinishRestoringWindowsNotification: &'static NSNotificationName);

extern_methods!(
    /// NSUserInterfaceRestoration
    #[cfg(feature = "AppKit_NSWindow")]
    unsafe impl NSWindow {
        #[method(isRestorable)]
        pub unsafe fn isRestorable(&self) -> bool;

        #[method(setRestorable:)]
        pub unsafe fn setRestorable(&self, restorable: bool);

        #[method_id(@__retain_semantics Other restorationClass)]
        pub unsafe fn restorationClass(&self) -> Option<Id<TodoClass, Shared>>;

        #[method(setRestorationClass:)]
        pub unsafe fn setRestorationClass(&self, restoration_class: Option<&TodoClass>);

        #[method(disableSnapshotRestoration)]
        pub unsafe fn disableSnapshotRestoration(&self);

        #[method(enableSnapshotRestoration)]
        pub unsafe fn enableSnapshotRestoration(&self);
    }
);

extern_methods!(
    /// NSRestorableState
    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl NSResponder {
        #[cfg(feature = "Foundation_NSCoder")]
        #[method(encodeRestorableStateWithCoder:)]
        pub unsafe fn encodeRestorableStateWithCoder(&self, coder: &NSCoder);

        #[cfg(all(
            feature = "Foundation_NSCoder",
            feature = "Foundation_NSOperationQueue"
        ))]
        #[method(encodeRestorableStateWithCoder:backgroundQueue:)]
        pub unsafe fn encodeRestorableStateWithCoder_backgroundQueue(
            &self,
            coder: &NSCoder,
            queue: &NSOperationQueue,
        );

        #[cfg(feature = "Foundation_NSCoder")]
        #[method(restoreStateWithCoder:)]
        pub unsafe fn restoreStateWithCoder(&self, coder: &NSCoder);

        #[method(invalidateRestorableState)]
        pub unsafe fn invalidateRestorableState(&self);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other restorableStateKeyPaths)]
        pub unsafe fn restorableStateKeyPaths() -> Id<NSArray<NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other allowedClassesForRestorableStateKeyPath:)]
        pub unsafe fn allowedClassesForRestorableStateKeyPath(
            key_path: &NSString,
        ) -> Id<NSArray<TodoClass>, Shared>;
    }
);

extern_methods!(
    /// NSRestorableStateExtension
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[method(extendStateRestoration)]
        pub unsafe fn extendStateRestoration(&self);

        #[method(completeStateRestoration)]
        pub unsafe fn completeStateRestoration(&self);
    }
);

extern_methods!(
    /// NSRestorableState
    #[cfg(feature = "AppKit_NSDocument")]
    unsafe impl NSDocument {
        #[cfg(all(
            feature = "AppKit_NSWindow",
            feature = "Foundation_NSCoder",
            feature = "Foundation_NSError"
        ))]
        #[method(restoreDocumentWindowWithIdentifier:state:completionHandler:)]
        pub unsafe fn restoreDocumentWindowWithIdentifier_state_completionHandler(
            &self,
            identifier: &NSUserInterfaceItemIdentifier,
            state: &NSCoder,
            completion_handler: &Block<(*mut NSWindow, *mut NSError), ()>,
        );

        #[cfg(feature = "Foundation_NSCoder")]
        #[method(encodeRestorableStateWithCoder:)]
        pub unsafe fn encodeRestorableStateWithCoder(&self, coder: &NSCoder);

        #[cfg(all(
            feature = "Foundation_NSCoder",
            feature = "Foundation_NSOperationQueue"
        ))]
        #[method(encodeRestorableStateWithCoder:backgroundQueue:)]
        pub unsafe fn encodeRestorableStateWithCoder_backgroundQueue(
            &self,
            coder: &NSCoder,
            queue: &NSOperationQueue,
        );

        #[cfg(feature = "Foundation_NSCoder")]
        #[method(restoreStateWithCoder:)]
        pub unsafe fn restoreStateWithCoder(&self, coder: &NSCoder);

        #[method(invalidateRestorableState)]
        pub unsafe fn invalidateRestorableState(&self);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other restorableStateKeyPaths)]
        pub unsafe fn restorableStateKeyPaths() -> Id<NSArray<NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other allowedClassesForRestorableStateKeyPath:)]
        pub unsafe fn allowedClassesForRestorableStateKeyPath(
            key_path: &NSString,
        ) -> Id<NSArray<TodoClass>, Shared>;
    }
);
