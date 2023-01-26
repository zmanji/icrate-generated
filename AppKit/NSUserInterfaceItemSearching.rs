//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSUserInterfaceItemSearching: NSObjectProtocol {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(searchForItemsWithSearchString:resultLimit:matchedItemHandler:)]
        unsafe fn searchForItemsWithSearchString_resultLimit_matchedItemHandler(
            &self,
            search_string: &NSString,
            result_limit: NSInteger,
            handle_matched_items: &Block<(NonNull<NSArray>,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other localizedTitlesForItem:)]
        unsafe fn localizedTitlesForItem(&self, item: &Object) -> Id<NSArray<NSString>, Shared>;

        #[optional]
        #[method(performActionForItem:)]
        unsafe fn performActionForItem(&self, item: &Object);

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(showAllHelpTopicsForSearchString:)]
        unsafe fn showAllHelpTopicsForSearchString(&self, search_string: &NSString);
    }

    unsafe impl ProtocolType for dyn NSUserInterfaceItemSearching {}
);

extern_methods!(
    /// NSUserInterfaceItemSearching
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[method(registerUserInterfaceItemSearchHandler:)]
        pub unsafe fn registerUserInterfaceItemSearchHandler(
            &self,
            handler: &ProtocolObject<dyn NSUserInterfaceItemSearching>,
        );

        #[method(unregisterUserInterfaceItemSearchHandler:)]
        pub unsafe fn unregisterUserInterfaceItemSearchHandler(
            &self,
            handler: &ProtocolObject<dyn NSUserInterfaceItemSearching>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(searchString:inUserInterfaceItemString:searchRange:foundRange:)]
        pub unsafe fn searchString_inUserInterfaceItemString_searchRange_foundRange(
            &self,
            search_string: &NSString,
            string_to_search: &NSString,
            search_range: NSRange,
            found_range: *mut NSRange,
        ) -> bool;
    }
);
