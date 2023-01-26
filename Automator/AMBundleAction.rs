//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Automator::*;
use crate::Foundation::*;
use crate::OSAKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Automator_AMBundleAction")]
    pub struct AMBundleAction;

    #[cfg(feature = "Automator_AMBundleAction")]
    unsafe impl ClassType for AMBundleAction {
        #[inherits(NSObject)]
        type Super = AMAction;
    }
);

#[cfg(feature = "Automator_AMBundleAction")]
unsafe impl NSSecureCoding for AMBundleAction {}

extern_methods!(
    #[cfg(feature = "Automator_AMBundleAction")]
    unsafe impl AMBundleAction {
        #[method(awakeFromBundle)]
        pub unsafe fn awakeFromBundle(&self);

        #[method(hasView)]
        pub unsafe fn hasView(&self) -> bool;

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;

        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Other bundle)]
        pub unsafe fn bundle(&self) -> Id<NSBundle, Shared>;

        #[cfg(all(
            feature = "Foundation_NSMutableDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other parameters)]
        pub unsafe fn parameters(&self)
            -> Option<Id<NSMutableDictionary<NSString, Object>, Owned>>;

        #[cfg(all(
            feature = "Foundation_NSMutableDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method(setParameters:)]
        pub unsafe fn setParameters(
            &self,
            parameters: Option<&NSMutableDictionary<NSString, Object>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `AMAction`
    #[cfg(feature = "Automator_AMBundleAction")]
    unsafe impl AMBundleAction {
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithDefinition:fromArchive:)]
        pub unsafe fn initWithDefinition_fromArchive(
            this: Option<Allocated<Self>>,
            dict: Option<&NSDictionary<NSString, Object>>,
            archived: bool,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:error:_)]
        pub unsafe fn initWithContentsOfURL_error(
            this: Option<Allocated<Self>>,
            file_url: &NSURL,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
    }
);
