//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSController")]
    pub struct NSController;

    #[cfg(feature = "AppKit_NSController")]
    unsafe impl ClassType for NSController {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSController")]
unsafe impl NSCoding for NSController {}

#[cfg(feature = "AppKit_NSController")]
unsafe impl NSEditor for NSController {}

#[cfg(feature = "AppKit_NSController")]
unsafe impl NSEditorRegistration for NSController {}

extern_methods!(
    #[cfg(feature = "AppKit_NSController")]
    unsafe impl NSController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method(objectDidBeginEditing:)]
        pub unsafe fn objectDidBeginEditing(&self, editor: &ProtocolObject<dyn NSEditor>);

        #[method(objectDidEndEditing:)]
        pub unsafe fn objectDidEndEditing(&self, editor: &ProtocolObject<dyn NSEditor>);

        #[method(discardEditing)]
        pub unsafe fn discardEditing(&self);

        #[method(commitEditing)]
        pub unsafe fn commitEditing(&self) -> bool;

        #[method(commitEditingWithDelegate:didCommitSelector:contextInfo:)]
        pub unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo(
            &self,
            delegate: Option<&Object>,
            did_commit_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[method(isEditing)]
        pub unsafe fn isEditing(&self) -> bool;
    }
);
