//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSExtensionItem")]
    pub struct NSExtensionItem;

    #[cfg(feature = "Foundation_NSExtensionItem")]
    unsafe impl ClassType for NSExtensionItem {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSExtensionItem")]
unsafe impl NSSecureCoding for NSExtensionItem {}

extern_methods!(
    #[cfg(feature = "Foundation_NSExtensionItem")]
    unsafe impl NSExtensionItem {
        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Option<Id<NSAttributedString, Shared>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: Option<&NSAttributedString>);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedContentText)]
        pub unsafe fn attributedContentText(&self) -> Option<Id<NSAttributedString, Shared>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedContentText:)]
        pub unsafe fn setAttributedContentText(
            &self,
            attributed_content_text: Option<&NSAttributedString>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSItemProvider"))]
        #[method_id(@__retain_semantics Other attachments)]
        pub unsafe fn attachments(&self) -> Option<Id<NSArray<NSItemProvider>, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSItemProvider"))]
        #[method(setAttachments:)]
        pub unsafe fn setAttachments(&self, attachments: Option<&NSArray<NSItemProvider>>);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary>);
    }
);

extern_static!(NSExtensionItemAttributedTitleKey: Option<&'static NSString>);

extern_static!(NSExtensionItemAttributedContentTextKey: Option<&'static NSString>);

extern_static!(NSExtensionItemAttachmentsKey: Option<&'static NSString>);
