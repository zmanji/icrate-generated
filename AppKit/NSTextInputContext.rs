//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSTextInputSourceIdentifier = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextInputContext")]
    pub struct NSTextInputContext;

    #[cfg(feature = "AppKit_NSTextInputContext")]
    unsafe impl ClassType for NSTextInputContext {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextInputContext")]
    unsafe impl NSTextInputContext {
        #[method_id(@__retain_semantics Other currentInputContext)]
        pub unsafe fn currentInputContext() -> Option<Id<NSTextInputContext, Shared>>;

        #[method_id(@__retain_semantics Init initWithClient:)]
        pub unsafe fn initWithClient(
            this: Option<Allocated<Self>>,
            client: &ProtocolObject<dyn NSTextInputClient>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Id<ProtocolObject<dyn NSTextInputClient>, Shared>;

        #[method(acceptsGlyphInfo)]
        pub unsafe fn acceptsGlyphInfo(&self) -> bool;

        #[method(setAcceptsGlyphInfo:)]
        pub unsafe fn setAcceptsGlyphInfo(&self, accepts_glyph_info: bool);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other allowedInputSourceLocales)]
        pub unsafe fn allowedInputSourceLocales(&self) -> Option<Id<NSArray<NSString>, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setAllowedInputSourceLocales:)]
        pub unsafe fn setAllowedInputSourceLocales(
            &self,
            allowed_input_source_locales: Option<&NSArray<NSString>>,
        );

        #[method(activate)]
        pub unsafe fn activate(&self);

        #[method(deactivate)]
        pub unsafe fn deactivate(&self);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(handleEvent:)]
        pub unsafe fn handleEvent(&self, event: &NSEvent) -> bool;

        #[method(discardMarkedText)]
        pub unsafe fn discardMarkedText(&self);

        #[method(invalidateCharacterCoordinates)]
        pub unsafe fn invalidateCharacterCoordinates(&self);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other keyboardInputSources)]
        pub unsafe fn keyboardInputSources(
            &self,
        ) -> Option<Id<NSArray<NSTextInputSourceIdentifier>, Shared>>;

        #[method_id(@__retain_semantics Other selectedKeyboardInputSource)]
        pub unsafe fn selectedKeyboardInputSource(
            &self,
        ) -> Option<Id<NSTextInputSourceIdentifier, Shared>>;

        #[method(setSelectedKeyboardInputSource:)]
        pub unsafe fn setSelectedKeyboardInputSource(
            &self,
            selected_keyboard_input_source: Option<&NSTextInputSourceIdentifier>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedNameForInputSource:)]
        pub unsafe fn localizedNameForInputSource(
            input_source_identifier: &NSTextInputSourceIdentifier,
        ) -> Option<Id<NSString, Shared>>;
    }
);

extern_static!(
    NSTextInputContextKeyboardSelectionDidChangeNotification: &'static NSNotificationName
);
