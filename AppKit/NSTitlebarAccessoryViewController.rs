//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTitlebarAccessoryViewController")]
    pub struct NSTitlebarAccessoryViewController;

    #[cfg(feature = "AppKit_NSTitlebarAccessoryViewController")]
    unsafe impl ClassType for NSTitlebarAccessoryViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
    }
);

#[cfg(feature = "AppKit_NSTitlebarAccessoryViewController")]
unsafe impl NSAnimationDelegate for NSTitlebarAccessoryViewController {}

#[cfg(feature = "AppKit_NSTitlebarAccessoryViewController")]
unsafe impl NSAnimatablePropertyContainer for NSTitlebarAccessoryViewController {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTitlebarAccessoryViewController")]
    unsafe impl NSTitlebarAccessoryViewController {
        #[method(layoutAttribute)]
        pub unsafe fn layoutAttribute(&self) -> NSLayoutAttribute;

        #[method(setLayoutAttribute:)]
        pub unsafe fn setLayoutAttribute(&self, layout_attribute: NSLayoutAttribute);

        #[method(fullScreenMinHeight)]
        pub unsafe fn fullScreenMinHeight(&self) -> CGFloat;

        #[method(setFullScreenMinHeight:)]
        pub unsafe fn setFullScreenMinHeight(&self, full_screen_min_height: CGFloat);

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[method(automaticallyAdjustsSize)]
        pub unsafe fn automaticallyAdjustsSize(&self) -> bool;

        #[method(setAutomaticallyAdjustsSize:)]
        pub unsafe fn setAutomaticallyAdjustsSize(&self, automatically_adjusts_size: bool);

        #[method(viewWillAppear)]
        pub unsafe fn viewWillAppear(&self);

        #[method(viewDidAppear)]
        pub unsafe fn viewDidAppear(&self);

        #[method(viewDidDisappear)]
        pub unsafe fn viewDidDisappear(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "AppKit_NSTitlebarAccessoryViewController")]
    unsafe impl NSTitlebarAccessoryViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self, Shared>;
    }
);
