//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSGroupTouchBarItem")]
    pub struct NSGroupTouchBarItem;

    #[cfg(feature = "AppKit_NSGroupTouchBarItem")]
    unsafe impl ClassType for NSGroupTouchBarItem {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSGroupTouchBarItem")]
unsafe impl NSCoding for NSGroupTouchBarItem {}

#[cfg(feature = "AppKit_NSGroupTouchBarItem")]
unsafe impl NSObjectProtocol for NSGroupTouchBarItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSGroupTouchBarItem")]
    unsafe impl NSGroupTouchBarItem {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other groupItemWithIdentifier:items:)]
        pub unsafe fn groupItemWithIdentifier_items(
            identifier: &NSTouchBarItemIdentifier,
            items: &NSArray<NSTouchBarItem>,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "AppKit_NSUserInterfaceCompressionOptions",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other groupItemWithIdentifier:items:allowedCompressionOptions:)]
        pub unsafe fn groupItemWithIdentifier_items_allowedCompressionOptions(
            identifier: &NSTouchBarItemIdentifier,
            items: &NSArray<NSTouchBarItem>,
            allowed_compression_options: &NSUserInterfaceCompressionOptions,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other alertStyleGroupItemWithIdentifier:)]
        pub unsafe fn alertStyleGroupItemWithIdentifier(
            identifier: &NSTouchBarItemIdentifier,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSTouchBar")]
        #[method_id(@__retain_semantics Other groupTouchBar)]
        pub unsafe fn groupTouchBar(&self) -> Id<NSTouchBar>;

        #[cfg(feature = "AppKit_NSTouchBar")]
        #[method(setGroupTouchBar:)]
        pub unsafe fn setGroupTouchBar(&self, group_touch_bar: &NSTouchBar);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customization_label: Option<&NSString>);

        #[method(groupUserInterfaceLayoutDirection)]
        pub unsafe fn groupUserInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;

        #[method(setGroupUserInterfaceLayoutDirection:)]
        pub unsafe fn setGroupUserInterfaceLayoutDirection(
            &self,
            group_user_interface_layout_direction: NSUserInterfaceLayoutDirection,
        );

        #[method(prefersEqualWidths)]
        pub unsafe fn prefersEqualWidths(&self) -> bool;

        #[method(setPrefersEqualWidths:)]
        pub unsafe fn setPrefersEqualWidths(&self, prefers_equal_widths: bool);

        #[method(preferredItemWidth)]
        pub unsafe fn preferredItemWidth(&self) -> CGFloat;

        #[method(setPreferredItemWidth:)]
        pub unsafe fn setPreferredItemWidth(&self, preferred_item_width: CGFloat);

        #[cfg(feature = "AppKit_NSUserInterfaceCompressionOptions")]
        #[method_id(@__retain_semantics Other effectiveCompressionOptions)]
        pub unsafe fn effectiveCompressionOptions(&self) -> Id<NSUserInterfaceCompressionOptions>;

        #[cfg(all(
            feature = "AppKit_NSUserInterfaceCompressionOptions",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other prioritizedCompressionOptions)]
        pub unsafe fn prioritizedCompressionOptions(
            &self,
        ) -> Id<NSArray<NSUserInterfaceCompressionOptions>>;

        #[cfg(all(
            feature = "AppKit_NSUserInterfaceCompressionOptions",
            feature = "Foundation_NSArray"
        ))]
        #[method(setPrioritizedCompressionOptions:)]
        pub unsafe fn setPrioritizedCompressionOptions(
            &self,
            prioritized_compression_options: &NSArray<NSUserInterfaceCompressionOptions>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "AppKit_NSGroupTouchBarItem")]
    unsafe impl NSGroupTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSGroupTouchBarItem")]
    unsafe impl NSGroupTouchBarItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
