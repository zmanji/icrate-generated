//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_protocol!(
    pub unsafe trait GCTouchedStateInput: NSObjectProtocol {
        #[method(touchedDidChangeHandler)]
        unsafe fn touchedDidChangeHandler(
            &self,
        ) -> *mut Block<
            (
                NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                NonNull<ProtocolObject<dyn GCTouchedStateInput>>,
                Bool,
            ),
            (),
        >;

        #[method(setTouchedDidChangeHandler:)]
        unsafe fn setTouchedDidChangeHandler(
            &self,
            touched_did_change_handler: Option<
                &Block<
                    (
                        NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                        NonNull<ProtocolObject<dyn GCTouchedStateInput>>,
                        Bool,
                    ),
                    (),
                >,
            >,
        );

        #[method(isTouched)]
        unsafe fn isTouched(&self) -> bool;

        #[method(lastTouchedStateTimestamp)]
        unsafe fn lastTouchedStateTimestamp(&self) -> NSTimeInterval;

        #[method(lastTouchedStateLatency)]
        unsafe fn lastTouchedStateLatency(&self) -> NSTimeInterval;
    }

    unsafe impl ProtocolType for dyn GCTouchedStateInput {}
);
