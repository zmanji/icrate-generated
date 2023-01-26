//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextFinderAction {
        NSTextFinderActionShowFindInterface = 1,
        NSTextFinderActionNextMatch = 2,
        NSTextFinderActionPreviousMatch = 3,
        NSTextFinderActionReplaceAll = 4,
        NSTextFinderActionReplace = 5,
        NSTextFinderActionReplaceAndFind = 6,
        NSTextFinderActionSetSearchString = 7,
        NSTextFinderActionReplaceAllInSelection = 8,
        NSTextFinderActionSelectAll = 9,
        NSTextFinderActionSelectAllInSelection = 10,
        NSTextFinderActionHideFindInterface = 11,
        NSTextFinderActionShowReplaceInterface = 12,
        NSTextFinderActionHideReplaceInterface = 13,
    }
);

typed_enum!(
    pub type NSPasteboardTypeTextFinderOptionKey = NSString;
);

extern_static!(NSTextFinderCaseInsensitiveKey: &'static NSPasteboardTypeTextFinderOptionKey);

extern_static!(NSTextFinderMatchingTypeKey: &'static NSPasteboardTypeTextFinderOptionKey);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextFinderMatchingType {
        NSTextFinderMatchingTypeContains = 0,
        NSTextFinderMatchingTypeStartsWith = 1,
        NSTextFinderMatchingTypeFullWord = 2,
        NSTextFinderMatchingTypeEndsWith = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextFinder")]
    pub struct NSTextFinder;

    #[cfg(feature = "AppKit_NSTextFinder")]
    unsafe impl ClassType for NSTextFinder {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextFinder")]
    unsafe impl NSTextFinder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Option<Id<ProtocolObject<dyn NSTextFinderClient>, Shared>>;

        #[method(setClient:)]
        pub unsafe fn setClient(&self, client: Option<&ProtocolObject<dyn NSTextFinderClient>>);

        #[method(performAction:)]
        pub unsafe fn performAction(&self, op: NSTextFinderAction);

        #[method(validateAction:)]
        pub unsafe fn validateAction(&self, op: NSTextFinderAction) -> bool;

        #[method_id(@__retain_semantics Other findBarContainer)]
        pub unsafe fn findBarContainer(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSTextFinderBarContainer>, Shared>>;

        #[method(setFindBarContainer:)]
        pub unsafe fn setFindBarContainer(
            &self,
            find_bar_container: Option<&ProtocolObject<dyn NSTextFinderBarContainer>>,
        );

        #[method(cancelFindIndicator)]
        pub unsafe fn cancelFindIndicator(&self);

        #[method(findIndicatorNeedsUpdate)]
        pub unsafe fn findIndicatorNeedsUpdate(&self) -> bool;

        #[method(setFindIndicatorNeedsUpdate:)]
        pub unsafe fn setFindIndicatorNeedsUpdate(&self, find_indicator_needs_update: bool);

        #[method(isIncrementalSearchingEnabled)]
        pub unsafe fn isIncrementalSearchingEnabled(&self) -> bool;

        #[method(setIncrementalSearchingEnabled:)]
        pub unsafe fn setIncrementalSearchingEnabled(&self, incremental_searching_enabled: bool);

        #[method(incrementalSearchingShouldDimContentView)]
        pub unsafe fn incrementalSearchingShouldDimContentView(&self) -> bool;

        #[method(setIncrementalSearchingShouldDimContentView:)]
        pub unsafe fn setIncrementalSearchingShouldDimContentView(
            &self,
            incremental_searching_should_dim_content_view: bool,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other incrementalMatchRanges)]
        pub unsafe fn incrementalMatchRanges(&self) -> Id<NSArray<NSValue>, Shared>;

        #[method(drawIncrementalMatchHighlightInRect:)]
        pub unsafe fn drawIncrementalMatchHighlightInRect(rect: NSRect);

        #[method(noteClientStringWillChange)]
        pub unsafe fn noteClientStringWillChange(&self);
    }
);

extern_protocol!(
    pub unsafe trait NSTextFinderClient: NSObjectProtocol {
        #[optional]
        #[method(isSelectable)]
        unsafe fn isSelectable(&self) -> bool;

        #[optional]
        #[method(allowsMultipleSelection)]
        unsafe fn allowsMultipleSelection(&self) -> bool;

        #[optional]
        #[method(isEditable)]
        unsafe fn isEditable(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method_id(@__retain_semantics Other string)]
        unsafe fn string(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method_id(@__retain_semantics Other stringAtIndex:effectiveRange:endsWithSearchBoundary:)]
        unsafe fn stringAtIndex_effectiveRange_endsWithSearchBoundary(
            &self,
            character_index: NSUInteger,
            out_range: NSRangePointer,
            out_flag: NonNull<Bool>,
        ) -> Id<NSString, Shared>;

        #[optional]
        #[method(stringLength)]
        unsafe fn stringLength(&self) -> NSUInteger;

        #[optional]
        #[method(firstSelectedRange)]
        unsafe fn firstSelectedRange(&self) -> NSRange;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[optional]
        #[method_id(@__retain_semantics Other selectedRanges)]
        unsafe fn selectedRanges(&self) -> Id<NSArray<NSValue>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[optional]
        #[method(setSelectedRanges:)]
        unsafe fn setSelectedRanges(&self, selected_ranges: &NSArray<NSValue>);

        #[optional]
        #[method(scrollRangeToVisible:)]
        unsafe fn scrollRangeToVisible(&self, range: NSRange);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "Foundation_NSValue"
        ))]
        #[optional]
        #[method(shouldReplaceCharactersInRanges:withStrings:)]
        unsafe fn shouldReplaceCharactersInRanges_withStrings(
            &self,
            ranges: &NSArray<NSValue>,
            strings: &NSArray<NSString>,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(replaceCharactersInRange:withString:)]
        unsafe fn replaceCharactersInRange_withString(&self, range: NSRange, string: &NSString);

        #[optional]
        #[method(didReplaceCharacters)]
        unsafe fn didReplaceCharacters(&self);

        #[cfg(feature = "AppKit_NSView")]
        #[optional]
        #[method_id(@__retain_semantics Other contentViewAtIndex:effectiveCharacterRange:)]
        unsafe fn contentViewAtIndex_effectiveCharacterRange(
            &self,
            index: NSUInteger,
            out_range: NSRangePointer,
        ) -> Id<NSView, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[optional]
        #[method_id(@__retain_semantics Other rectsForCharacterRange:)]
        unsafe fn rectsForCharacterRange(
            &self,
            range: NSRange,
        ) -> Option<Id<NSArray<NSValue>, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[optional]
        #[method_id(@__retain_semantics Other visibleCharacterRanges)]
        unsafe fn visibleCharacterRanges(&self) -> Id<NSArray<NSValue>, Shared>;

        #[cfg(feature = "AppKit_NSView")]
        #[optional]
        #[method(drawCharactersInRange:forContentView:)]
        unsafe fn drawCharactersInRange_forContentView(&self, range: NSRange, view: &NSView);
    }

    unsafe impl ProtocolType for dyn NSTextFinderClient {}
);

extern_protocol!(
    pub unsafe trait NSTextFinderBarContainer: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other findBarView)]
        unsafe fn findBarView(&self) -> Option<Id<NSView, Shared>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setFindBarView:)]
        unsafe fn setFindBarView(&self, find_bar_view: Option<&NSView>);

        #[method(isFindBarVisible)]
        unsafe fn isFindBarVisible(&self) -> bool;

        #[method(setFindBarVisible:)]
        unsafe fn setFindBarVisible(&self, find_bar_visible: bool);

        #[method(findBarViewDidChangeHeight)]
        unsafe fn findBarViewDidChangeHeight(&self);

        #[cfg(feature = "AppKit_NSView")]
        #[optional]
        #[method_id(@__retain_semantics Other contentView)]
        unsafe fn contentView(&self) -> Option<Id<NSView, Shared>>;
    }

    unsafe impl ProtocolType for dyn NSTextFinderBarContainer {}
);
