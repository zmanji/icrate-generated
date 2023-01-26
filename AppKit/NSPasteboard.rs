//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSPasteboardType = NSString;
);

extern_static!(NSPasteboardTypeString: &'static NSPasteboardType);

extern_static!(NSPasteboardTypePDF: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeTIFF: &'static NSPasteboardType);

extern_static!(NSPasteboardTypePNG: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeRTF: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeRTFD: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeHTML: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeTabularText: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeFont: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeRuler: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeColor: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeSound: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeMultipleTextSelection: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeTextFinderOptions: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeURL: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeFileURL: &'static NSPasteboardType);

typed_extensible_enum!(
    pub type NSPasteboardName = NSString;
);

extern_static!(NSPasteboardNameGeneral: &'static NSPasteboardName);

extern_static!(NSPasteboardNameFont: &'static NSPasteboardName);

extern_static!(NSPasteboardNameRuler: &'static NSPasteboardName);

extern_static!(NSPasteboardNameFind: &'static NSPasteboardName);

extern_static!(NSPasteboardNameDrag: &'static NSPasteboardName);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSPasteboardContentsOptions {
        NSPasteboardContentsCurrentHostOnly = 1 << 0,
    }
);

typed_enum!(
    pub type NSPasteboardReadingOptionKey = NSString;
);

extern_static!(NSPasteboardURLReadingFileURLsOnlyKey: &'static NSPasteboardReadingOptionKey);

extern_static!(
    NSPasteboardURLReadingContentsConformToTypesKey: &'static NSPasteboardReadingOptionKey
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPasteboard")]
    pub struct NSPasteboard;

    #[cfg(feature = "AppKit_NSPasteboard")]
    unsafe impl ClassType for NSPasteboard {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSPasteboard")]
    unsafe impl NSPasteboard {
        #[method_id(@__retain_semantics Other generalPasteboard)]
        pub unsafe fn generalPasteboard() -> Id<NSPasteboard, Shared>;

        #[method_id(@__retain_semantics Other pasteboardWithName:)]
        pub unsafe fn pasteboardWithName(name: &NSPasteboardName) -> Id<NSPasteboard, Shared>;

        #[method_id(@__retain_semantics Other pasteboardWithUniqueName)]
        pub unsafe fn pasteboardWithUniqueName() -> Id<NSPasteboard, Shared>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSPasteboardName, Shared>;

        #[method(changeCount)]
        pub unsafe fn changeCount(&self) -> NSInteger;

        #[method(prepareForNewContentsWithOptions:)]
        pub unsafe fn prepareForNewContentsWithOptions(
            &self,
            options: NSPasteboardContentsOptions,
        ) -> NSInteger;

        #[method(clearContents)]
        pub unsafe fn clearContents(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(writeObjects:)]
        pub unsafe fn writeObjects(
            &self,
            objects: &NSArray<ProtocolObject<dyn NSPasteboardWriting>>,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSDictionary"))]
        #[method_id(@__retain_semantics Other readObjectsForClasses:options:)]
        pub unsafe fn readObjectsForClasses_options(
            &self,
            class_array: &NSArray<TodoClass>,
            options: Option<&NSDictionary<NSPasteboardReadingOptionKey, Object>>,
        ) -> Option<Id<NSArray, Shared>>;

        #[cfg(all(feature = "AppKit_NSPasteboardItem", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other pasteboardItems)]
        pub unsafe fn pasteboardItems(&self) -> Option<Id<NSArray<NSPasteboardItem>, Shared>>;

        #[cfg(feature = "AppKit_NSPasteboardItem")]
        #[method(indexOfPasteboardItem:)]
        pub unsafe fn indexOfPasteboardItem(
            &self,
            pasteboard_item: &NSPasteboardItem,
        ) -> NSUInteger;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(canReadItemWithDataConformingToTypes:)]
        pub unsafe fn canReadItemWithDataConformingToTypes(
            &self,
            types: &NSArray<NSString>,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSDictionary"))]
        #[method(canReadObjectForClasses:options:)]
        pub unsafe fn canReadObjectForClasses_options(
            &self,
            class_array: &NSArray<TodoClass>,
            options: Option<&NSDictionary<NSPasteboardReadingOptionKey, Object>>,
        ) -> bool;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(declareTypes:owner:)]
        pub unsafe fn declareTypes_owner(
            &self,
            new_types: &NSArray<NSPasteboardType>,
            new_owner: Option<&Object>,
        ) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(addTypes:owner:)]
        pub unsafe fn addTypes_owner(
            &self,
            new_types: &NSArray<NSPasteboardType>,
            new_owner: Option<&Object>,
        ) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other types)]
        pub unsafe fn types(&self) -> Option<Id<NSArray<NSPasteboardType>, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other availableTypeFromArray:)]
        pub unsafe fn availableTypeFromArray(
            &self,
            types: &NSArray<NSPasteboardType>,
        ) -> Option<Id<NSPasteboardType, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setData:forType:)]
        pub unsafe fn setData_forType(
            &self,
            data: Option<&NSData>,
            data_type: &NSPasteboardType,
        ) -> bool;

        #[method(setPropertyList:forType:)]
        pub unsafe fn setPropertyList_forType(
            &self,
            plist: &Object,
            data_type: &NSPasteboardType,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setString:forType:)]
        pub unsafe fn setString_forType(
            &self,
            string: &NSString,
            data_type: &NSPasteboardType,
        ) -> bool;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other dataForType:)]
        pub unsafe fn dataForType(
            &self,
            data_type: &NSPasteboardType,
        ) -> Option<Id<NSData, Shared>>;

        #[method_id(@__retain_semantics Other propertyListForType:)]
        pub unsafe fn propertyListForType(
            &self,
            data_type: &NSPasteboardType,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringForType:)]
        pub unsafe fn stringForType(
            &self,
            data_type: &NSPasteboardType,
        ) -> Option<Id<NSString, Shared>>;
    }
);

extern_methods!(
    /// FilterServices
    #[cfg(feature = "AppKit_NSPasteboard")]
    unsafe impl NSPasteboard {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other typesFilterableTo:)]
        pub unsafe fn typesFilterableTo(
            r#type: &NSPasteboardType,
        ) -> Id<NSArray<NSPasteboardType>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pasteboardByFilteringFile:)]
        pub unsafe fn pasteboardByFilteringFile(filename: &NSString) -> Id<NSPasteboard, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other pasteboardByFilteringData:ofType:)]
        pub unsafe fn pasteboardByFilteringData_ofType(
            data: &NSData,
            r#type: &NSPasteboardType,
        ) -> Id<NSPasteboard, Shared>;

        #[method_id(@__retain_semantics Other pasteboardByFilteringTypesInPasteboard:)]
        pub unsafe fn pasteboardByFilteringTypesInPasteboard(
            pboard: &NSPasteboard,
        ) -> Id<NSPasteboard, Shared>;
    }
);

extern_protocol!(
    pub unsafe trait NSPasteboardTypeOwner: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method(pasteboard:provideDataForType:)]
        unsafe fn pasteboard_provideDataForType(
            &self,
            sender: &NSPasteboard,
            r#type: &NSPasteboardType,
        );

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[optional]
        #[method(pasteboardChangedOwner:)]
        unsafe fn pasteboardChangedOwner(&self, sender: &NSPasteboard);
    }

    unsafe impl ProtocolType for dyn NSPasteboardTypeOwner {}
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSPasteboardWritingOptions {
        NSPasteboardWritingPromised = 1 << 9,
    }
);

extern_protocol!(
    pub unsafe trait NSPasteboardWriting: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSPasteboard", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other writableTypesForPasteboard:)]
        unsafe fn writableTypesForPasteboard(
            &self,
            pasteboard: &NSPasteboard,
        ) -> Id<NSArray<NSPasteboardType>, Shared>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[optional]
        #[method(writingOptionsForType:pasteboard:)]
        unsafe fn writingOptionsForType_pasteboard(
            &self,
            r#type: &NSPasteboardType,
            pasteboard: &NSPasteboard,
        ) -> NSPasteboardWritingOptions;

        #[method_id(@__retain_semantics Other pasteboardPropertyListForType:)]
        unsafe fn pasteboardPropertyListForType(
            &self,
            r#type: &NSPasteboardType,
        ) -> Option<Id<Object, Shared>>;
    }

    unsafe impl ProtocolType for dyn NSPasteboardWriting {}
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSPasteboardReadingOptions {
        NSPasteboardReadingAsData = 0,
        NSPasteboardReadingAsString = 1 << 0,
        NSPasteboardReadingAsPropertyList = 1 << 1,
        NSPasteboardReadingAsKeyedArchive = 1 << 2,
    }
);

extern_protocol!(
    pub unsafe trait NSPasteboardReading: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSPasteboard", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other readableTypesForPasteboard:)]
        unsafe fn readableTypesForPasteboard(
            pasteboard: &NSPasteboard,
        ) -> Id<NSArray<NSPasteboardType>, Shared>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[optional]
        #[method(readingOptionsForType:pasteboard:)]
        unsafe fn readingOptionsForType_pasteboard(
            r#type: &NSPasteboardType,
            pasteboard: &NSPasteboard,
        ) -> NSPasteboardReadingOptions;

        #[optional]
        #[method_id(@__retain_semantics Init initWithPasteboardPropertyList:ofType:)]
        unsafe fn initWithPasteboardPropertyList_ofType(
            this: Option<Allocated<Self>>,
            property_list: &Object,
            r#type: &NSPasteboardType,
        ) -> Option<Id<Self, Shared>>;
    }

    unsafe impl ProtocolType for dyn NSPasteboardReading {}
);

extern_methods!(
    /// NSPasteboardSupport
    #[cfg(feature = "Foundation_NSURL")]
    unsafe impl NSURL {
        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method_id(@__retain_semantics Other URLFromPasteboard:)]
        pub unsafe fn URLFromPasteboard(paste_board: &NSPasteboard) -> Option<Id<NSURL, Shared>>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method(writeToPasteboard:)]
        pub unsafe fn writeToPasteboard(&self, paste_board: &NSPasteboard);
    }
);

extern_methods!(
    /// NSPasteboardSupport
    #[cfg(feature = "Foundation_NSString")]
    unsafe impl NSString {}
);

extern_methods!(
    /// NSFileContents
    #[cfg(feature = "AppKit_NSPasteboard")]
    unsafe impl NSPasteboard {
        #[cfg(feature = "Foundation_NSString")]
        #[method(writeFileContents:)]
        pub unsafe fn writeFileContents(&self, filename: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other readFileContentsType:toFile:)]
        pub unsafe fn readFileContentsType_toFile(
            &self,
            r#type: Option<&NSPasteboardType>,
            filename: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSFileWrapper")]
        #[method(writeFileWrapper:)]
        pub unsafe fn writeFileWrapper(&self, wrapper: &NSFileWrapper) -> bool;

        #[cfg(feature = "Foundation_NSFileWrapper")]
        #[method_id(@__retain_semantics Other readFileWrapper)]
        pub unsafe fn readFileWrapper(&self) -> Option<Id<NSFileWrapper, Shared>>;
    }
);

extern_static!(NSFileContentsPboardType: &'static NSPasteboardType);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSCreateFilenamePboardType(file_type: &NSString) -> *mut NSPasteboardType;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSCreateFileContentsPboardType(file_type: &NSString) -> *mut NSPasteboardType;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSGetFileType(pboard_type: &NSPasteboardType) -> *mut NSString;
);

extern_fn!(
    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    pub unsafe fn NSGetFileTypes(
        pboard_types: &NSArray<NSPasteboardType>,
    ) -> *mut NSArray<NSString>;
);

extern_static!(NSStringPboardType: &'static NSPasteboardType);

extern_static!(NSFilenamesPboardType: &'static NSPasteboardType);

extern_static!(NSTIFFPboardType: &'static NSPasteboardType);

extern_static!(NSRTFPboardType: &'static NSPasteboardType);

extern_static!(NSTabularTextPboardType: &'static NSPasteboardType);

extern_static!(NSFontPboardType: &'static NSPasteboardType);

extern_static!(NSRulerPboardType: &'static NSPasteboardType);

extern_static!(NSColorPboardType: &'static NSPasteboardType);

extern_static!(NSRTFDPboardType: &'static NSPasteboardType);

extern_static!(NSHTMLPboardType: &'static NSPasteboardType);

extern_static!(NSURLPboardType: &'static NSPasteboardType);

extern_static!(NSPDFPboardType: &'static NSPasteboardType);

extern_static!(NSMultipleTextSelectionPboardType: &'static NSPasteboardType);

extern_static!(NSPostScriptPboardType: &'static NSPasteboardType);

extern_static!(NSVCardPboardType: &'static NSPasteboardType);

extern_static!(NSInkTextPboardType: &'static NSPasteboardType);

extern_static!(NSFilesPromisePboardType: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeFindPanelSearchOptions: &'static NSPasteboardType);

extern_static!(NSGeneralPboard: &'static NSPasteboardName);

extern_static!(NSFontPboard: &'static NSPasteboardName);

extern_static!(NSRulerPboard: &'static NSPasteboardName);

extern_static!(NSFindPboard: &'static NSPasteboardName);

extern_static!(NSDragPboard: &'static NSPasteboardName);

extern_static!(NSPICTPboardType: &'static NSPasteboardType);
