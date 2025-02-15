//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSColorWellStyle {
        NSColorWellStyleDefault = 0,
        NSColorWellStyleMinimal = 1,
        NSColorWellStyleExpanded = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSColorWell")]
    pub struct NSColorWell;

    #[cfg(feature = "AppKit_NSColorWell")]
    unsafe impl ClassType for NSColorWell {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSColorWell")]
unsafe impl NSAccessibility for NSColorWell {}

#[cfg(feature = "AppKit_NSColorWell")]
unsafe impl NSAccessibilityElementProtocol for NSColorWell {}

#[cfg(feature = "AppKit_NSColorWell")]
unsafe impl NSAnimatablePropertyContainer for NSColorWell {}

#[cfg(feature = "AppKit_NSColorWell")]
unsafe impl NSAppearanceCustomization for NSColorWell {}

#[cfg(feature = "AppKit_NSColorWell")]
unsafe impl NSCoding for NSColorWell {}

#[cfg(feature = "AppKit_NSColorWell")]
unsafe impl NSDraggingDestination for NSColorWell {}

#[cfg(feature = "AppKit_NSColorWell")]
unsafe impl NSObjectProtocol for NSColorWell {}

#[cfg(feature = "AppKit_NSColorWell")]
unsafe impl NSUserInterfaceItemIdentification for NSColorWell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSColorWell")]
    unsafe impl NSColorWell {
        #[method_id(@__retain_semantics Other colorWellWithStyle:)]
        pub unsafe fn colorWellWithStyle(
            style: NSColorWellStyle,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[method(deactivate)]
        pub unsafe fn deactivate(&self);

        #[method(activate:)]
        pub unsafe fn activate(&self, exclusive: bool);

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[method(drawWellInside:)]
        pub unsafe fn drawWellInside(&self, inside_rect: NSRect);

        #[deprecated = "This property will be deprecated in a future release."]
        #[method(isBordered)]
        pub unsafe fn isBordered(&self) -> bool;

        #[deprecated = "This property will be deprecated in a future release."]
        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, bordered: bool);

        #[method(takeColorFrom:)]
        pub unsafe fn takeColorFrom(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &NSColor);

        #[method(colorWellStyle)]
        pub unsafe fn colorWellStyle(&self) -> NSColorWellStyle;

        #[method(setColorWellStyle:)]
        pub unsafe fn setColorWellStyle(&self, color_well_style: NSColorWellStyle);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method_id(@__retain_semantics Other pulldownTarget)]
        pub unsafe fn pulldownTarget(&self) -> Option<Id<AnyObject>>;

        #[method(setPulldownTarget:)]
        pub unsafe fn setPulldownTarget(&self, pulldown_target: Option<&AnyObject>);

        #[method(pulldownAction)]
        pub unsafe fn pulldownAction(&self) -> Option<Sel>;

        #[method(setPulldownAction:)]
        pub unsafe fn setPulldownAction(&self, pulldown_action: Option<Sel>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSColorWell")]
    unsafe impl NSColorWell {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSColorWell")]
    unsafe impl NSColorWell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSColorWell")]
    unsafe impl NSColorWell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
