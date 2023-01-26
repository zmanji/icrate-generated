//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSImageView")]
    pub struct NSImageView;

    #[cfg(feature = "AppKit_NSImageView")]
    unsafe impl ClassType for NSImageView {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
    }
);

#[cfg(feature = "AppKit_NSImageView")]
unsafe impl NSAccessibilityImage for NSImageView {}

#[cfg(feature = "AppKit_NSImageView")]
unsafe impl NSMenuItemValidation for NSImageView {}

extern_methods!(
    #[cfg(feature = "AppKit_NSImageView")]
    unsafe impl NSImageView {
        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other imageViewWithImage:)]
        pub unsafe fn imageViewWithImage(image: &NSImage) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(imageAlignment)]
        pub unsafe fn imageAlignment(&self) -> NSImageAlignment;

        #[method(setImageAlignment:)]
        pub unsafe fn setImageAlignment(&self, image_alignment: NSImageAlignment);

        #[method(imageScaling)]
        pub unsafe fn imageScaling(&self) -> NSImageScaling;

        #[method(setImageScaling:)]
        pub unsafe fn setImageScaling(&self, image_scaling: NSImageScaling);

        #[method(imageFrameStyle)]
        pub unsafe fn imageFrameStyle(&self) -> NSImageFrameStyle;

        #[method(setImageFrameStyle:)]
        pub unsafe fn setImageFrameStyle(&self, image_frame_style: NSImageFrameStyle);

        #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
        #[method_id(@__retain_semantics Other symbolConfiguration)]
        pub unsafe fn symbolConfiguration(&self) -> Option<Id<NSImageSymbolConfiguration, Shared>>;

        #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
        #[method(setSymbolConfiguration:)]
        pub unsafe fn setSymbolConfiguration(
            &self,
            symbol_configuration: Option<&NSImageSymbolConfiguration>,
        );

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other contentTintColor)]
        pub unsafe fn contentTintColor(&self) -> Option<Id<NSColor, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setContentTintColor:)]
        pub unsafe fn setContentTintColor(&self, content_tint_color: Option<&NSColor>);

        #[method(animates)]
        pub unsafe fn animates(&self) -> bool;

        #[method(setAnimates:)]
        pub unsafe fn setAnimates(&self, animates: bool);

        #[method(allowsCutCopyPaste)]
        pub unsafe fn allowsCutCopyPaste(&self) -> bool;

        #[method(setAllowsCutCopyPaste:)]
        pub unsafe fn setAllowsCutCopyPaste(&self, allows_cut_copy_paste: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSImageView")]
    unsafe impl NSImageView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frame_rect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
