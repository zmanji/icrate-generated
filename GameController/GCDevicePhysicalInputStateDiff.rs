//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GCDevicePhysicalInputElementChange {
        GCDevicePhysicalInputElementUnknownChange = -1,
        GCDevicePhysicalInputElementNoChange = 0,
        GCDevicePhysicalInputElementChanged = 1,
    }
);

extern_protocol!(
    pub unsafe trait GCDevicePhysicalInputStateDiff: NSObjectProtocol {
        #[method(changeForElement:)]
        unsafe fn changeForElement(
            &self,
            element: &ProtocolObject<dyn GCPhysicalInputElement>,
        ) -> GCDevicePhysicalInputElementChange;

        #[cfg(feature = "Foundation_NSEnumerator")]
        #[method_id(@__retain_semantics Other changedElements)]
        unsafe fn changedElements(
            &self,
        ) -> Option<Id<NSEnumerator<ProtocolObject<dyn GCPhysicalInputElement>>, Shared>>;
    }

    unsafe impl ProtocolType for dyn GCDevicePhysicalInputStateDiff {}
);
