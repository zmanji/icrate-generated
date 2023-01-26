//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSTokenFieldDelegate: NSTextFieldDelegate {
        #[cfg(all(
            feature = "AppKit_NSTokenField",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenField:completionsForSubstring:indexOfToken:indexOfSelectedItem:)]
        unsafe fn tokenField_completionsForSubstring_indexOfToken_indexOfSelectedItem(
            &self,
            token_field: &NSTokenField,
            substring: &NSString,
            token_index: NSInteger,
            selected_index: *mut NSInteger,
        ) -> Option<Id<NSArray, Shared>>;

        #[cfg(all(feature = "AppKit_NSTokenField", feature = "Foundation_NSArray"))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenField:shouldAddObjects:atIndex:)]
        unsafe fn tokenField_shouldAddObjects_atIndex(
            &self,
            token_field: &NSTokenField,
            tokens: &NSArray,
            index: NSUInteger,
        ) -> Id<NSArray, Shared>;

        #[cfg(all(feature = "AppKit_NSTokenField", feature = "Foundation_NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenField:displayStringForRepresentedObject:)]
        unsafe fn tokenField_displayStringForRepresentedObject(
            &self,
            token_field: &NSTokenField,
            represented_object: &Object,
        ) -> Option<Id<NSString, Shared>>;

        #[cfg(all(feature = "AppKit_NSTokenField", feature = "Foundation_NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenField:editingStringForRepresentedObject:)]
        unsafe fn tokenField_editingStringForRepresentedObject(
            &self,
            token_field: &NSTokenField,
            represented_object: &Object,
        ) -> Option<Id<NSString, Shared>>;

        #[cfg(all(feature = "AppKit_NSTokenField", feature = "Foundation_NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenField:representedObjectForEditingString:)]
        unsafe fn tokenField_representedObjectForEditingString(
            &self,
            token_field: &NSTokenField,
            editing_string: &NSString,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSPasteboard",
            feature = "AppKit_NSTokenField",
            feature = "Foundation_NSArray"
        ))]
        #[optional]
        #[method(tokenField:writeRepresentedObjects:toPasteboard:)]
        unsafe fn tokenField_writeRepresentedObjects_toPasteboard(
            &self,
            token_field: &NSTokenField,
            objects: &NSArray,
            pboard: &NSPasteboard,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSPasteboard",
            feature = "AppKit_NSTokenField",
            feature = "Foundation_NSArray"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenField:readFromPasteboard:)]
        unsafe fn tokenField_readFromPasteboard(
            &self,
            token_field: &NSTokenField,
            pboard: &NSPasteboard,
        ) -> Option<Id<NSArray, Shared>>;

        #[cfg(all(feature = "AppKit_NSMenu", feature = "AppKit_NSTokenField"))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenField:menuForRepresentedObject:)]
        unsafe fn tokenField_menuForRepresentedObject(
            &self,
            token_field: &NSTokenField,
            represented_object: &Object,
        ) -> Option<Id<NSMenu, Shared>>;

        #[cfg(feature = "AppKit_NSTokenField")]
        #[optional]
        #[method(tokenField:hasMenuForRepresentedObject:)]
        unsafe fn tokenField_hasMenuForRepresentedObject(
            &self,
            token_field: &NSTokenField,
            represented_object: &Object,
        ) -> bool;

        #[cfg(feature = "AppKit_NSTokenField")]
        #[optional]
        #[method(tokenField:styleForRepresentedObject:)]
        unsafe fn tokenField_styleForRepresentedObject(
            &self,
            token_field: &NSTokenField,
            represented_object: &Object,
        ) -> NSTokenStyle;
    }

    unsafe impl ProtocolType for dyn NSTokenFieldDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTokenField")]
    pub struct NSTokenField;

    #[cfg(feature = "AppKit_NSTokenField")]
    unsafe impl ClassType for NSTokenField {
        #[inherits(NSControl, NSView, NSResponder, NSObject)]
        type Super = NSTextField;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTokenField")]
    unsafe impl NSTokenField {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSTokenFieldDelegate>, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTokenFieldDelegate>>,
        );

        #[method(tokenStyle)]
        pub unsafe fn tokenStyle(&self) -> NSTokenStyle;

        #[method(setTokenStyle:)]
        pub unsafe fn setTokenStyle(&self, token_style: NSTokenStyle);

        #[method(completionDelay)]
        pub unsafe fn completionDelay(&self) -> NSTimeInterval;

        #[method(setCompletionDelay:)]
        pub unsafe fn setCompletionDelay(&self, completion_delay: NSTimeInterval);

        #[method(defaultCompletionDelay)]
        pub unsafe fn defaultCompletionDelay() -> NSTimeInterval;

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method_id(@__retain_semantics Other tokenizingCharacterSet)]
        pub unsafe fn tokenizingCharacterSet(&self) -> Id<NSCharacterSet, Shared>;

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method(setTokenizingCharacterSet:)]
        pub unsafe fn setTokenizingCharacterSet(
            &self,
            tokenizing_character_set: Option<&NSCharacterSet>,
        );

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method_id(@__retain_semantics Other defaultTokenizingCharacterSet)]
        pub unsafe fn defaultTokenizingCharacterSet() -> Id<NSCharacterSet, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextField`
    ///
    /// NSTextFieldConvenience
    #[cfg(feature = "AppKit_NSTokenField")]
    unsafe impl NSTokenField {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other labelWithString:)]
        pub unsafe fn labelWithString(string_value: &NSString) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other wrappingLabelWithString:)]
        pub unsafe fn wrappingLabelWithString(string_value: &NSString) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other labelWithAttributedString:)]
        pub unsafe fn labelWithAttributedString(
            attributed_string_value: &NSAttributedString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other textFieldWithString:)]
        pub unsafe fn textFieldWithString(string_value: &NSString) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSTokenField")]
    unsafe impl NSTokenField {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frame_rect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
