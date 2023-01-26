//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGestureRecognizerState {
        NSGestureRecognizerStatePossible = 0,
        NSGestureRecognizerStateBegan = 1,
        NSGestureRecognizerStateChanged = 2,
        NSGestureRecognizerStateEnded = 3,
        NSGestureRecognizerStateCancelled = 4,
        NSGestureRecognizerStateFailed = 5,
        NSGestureRecognizerStateRecognized = NSGestureRecognizerStateEnded,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSGestureRecognizer")]
    pub struct NSGestureRecognizer;

    #[cfg(feature = "AppKit_NSGestureRecognizer")]
    unsafe impl ClassType for NSGestureRecognizer {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSGestureRecognizer")]
    unsafe impl NSGestureRecognizer {
        #[method_id(@__retain_semantics Init initWithTarget:action:)]
        pub unsafe fn initWithTarget_action(
            this: Option<Allocated<Self>>,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSGestureRecognizerDelegate>, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSGestureRecognizerDelegate>>,
        );

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;

        #[cfg(feature = "AppKit_NSPressureConfiguration")]
        #[method_id(@__retain_semantics Other pressureConfiguration)]
        pub unsafe fn pressureConfiguration(&self) -> Id<NSPressureConfiguration, Shared>;

        #[cfg(feature = "AppKit_NSPressureConfiguration")]
        #[method(setPressureConfiguration:)]
        pub unsafe fn setPressureConfiguration(
            &self,
            pressure_configuration: &NSPressureConfiguration,
        );

        #[method(delaysPrimaryMouseButtonEvents)]
        pub unsafe fn delaysPrimaryMouseButtonEvents(&self) -> bool;

        #[method(setDelaysPrimaryMouseButtonEvents:)]
        pub unsafe fn setDelaysPrimaryMouseButtonEvents(
            &self,
            delays_primary_mouse_button_events: bool,
        );

        #[method(delaysSecondaryMouseButtonEvents)]
        pub unsafe fn delaysSecondaryMouseButtonEvents(&self) -> bool;

        #[method(setDelaysSecondaryMouseButtonEvents:)]
        pub unsafe fn setDelaysSecondaryMouseButtonEvents(
            &self,
            delays_secondary_mouse_button_events: bool,
        );

        #[method(delaysOtherMouseButtonEvents)]
        pub unsafe fn delaysOtherMouseButtonEvents(&self) -> bool;

        #[method(setDelaysOtherMouseButtonEvents:)]
        pub unsafe fn setDelaysOtherMouseButtonEvents(
            &self,
            delays_other_mouse_button_events: bool,
        );

        #[method(delaysKeyEvents)]
        pub unsafe fn delaysKeyEvents(&self) -> bool;

        #[method(setDelaysKeyEvents:)]
        pub unsafe fn setDelaysKeyEvents(&self, delays_key_events: bool);

        #[method(delaysMagnificationEvents)]
        pub unsafe fn delaysMagnificationEvents(&self) -> bool;

        #[method(setDelaysMagnificationEvents:)]
        pub unsafe fn setDelaysMagnificationEvents(&self, delays_magnification_events: bool);

        #[method(delaysRotationEvents)]
        pub unsafe fn delaysRotationEvents(&self) -> bool;

        #[method(setDelaysRotationEvents:)]
        pub unsafe fn setDelaysRotationEvents(&self, delays_rotation_events: bool);

        #[cfg(feature = "AppKit_NSView")]
        #[method(locationInView:)]
        pub unsafe fn locationInView(&self, view: Option<&NSView>) -> NSPoint;
    }
);

extern_methods!(
    /// NSTouchBar
    #[cfg(feature = "AppKit_NSGestureRecognizer")]
    unsafe impl NSGestureRecognizer {
        #[method(allowedTouchTypes)]
        pub unsafe fn allowedTouchTypes(&self) -> NSTouchTypeMask;

        #[method(setAllowedTouchTypes:)]
        pub unsafe fn setAllowedTouchTypes(&self, allowed_touch_types: NSTouchTypeMask);
    }
);

extern_protocol!(
    pub unsafe trait NSGestureRecognizerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSEvent", feature = "AppKit_NSGestureRecognizer"))]
        #[optional]
        #[method(gestureRecognizer:shouldAttemptToRecognizeWithEvent:)]
        unsafe fn gestureRecognizer_shouldAttemptToRecognizeWithEvent(
            &self,
            gesture_recognizer: &NSGestureRecognizer,
            event: &NSEvent,
        ) -> bool;

        #[cfg(feature = "AppKit_NSGestureRecognizer")]
        #[optional]
        #[method(gestureRecognizerShouldBegin:)]
        unsafe fn gestureRecognizerShouldBegin(
            &self,
            gesture_recognizer: &NSGestureRecognizer,
        ) -> bool;

        #[cfg(feature = "AppKit_NSGestureRecognizer")]
        #[optional]
        #[method(gestureRecognizer:shouldRecognizeSimultaneouslyWithGestureRecognizer:)]
        unsafe fn gestureRecognizer_shouldRecognizeSimultaneouslyWithGestureRecognizer(
            &self,
            gesture_recognizer: &NSGestureRecognizer,
            other_gesture_recognizer: &NSGestureRecognizer,
        ) -> bool;

        #[cfg(feature = "AppKit_NSGestureRecognizer")]
        #[optional]
        #[method(gestureRecognizer:shouldRequireFailureOfGestureRecognizer:)]
        unsafe fn gestureRecognizer_shouldRequireFailureOfGestureRecognizer(
            &self,
            gesture_recognizer: &NSGestureRecognizer,
            other_gesture_recognizer: &NSGestureRecognizer,
        ) -> bool;

        #[cfg(feature = "AppKit_NSGestureRecognizer")]
        #[optional]
        #[method(gestureRecognizer:shouldBeRequiredToFailByGestureRecognizer:)]
        unsafe fn gestureRecognizer_shouldBeRequiredToFailByGestureRecognizer(
            &self,
            gesture_recognizer: &NSGestureRecognizer,
            other_gesture_recognizer: &NSGestureRecognizer,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSGestureRecognizer", feature = "AppKit_NSTouch"))]
        #[optional]
        #[method(gestureRecognizer:shouldReceiveTouch:)]
        unsafe fn gestureRecognizer_shouldReceiveTouch(
            &self,
            gesture_recognizer: &NSGestureRecognizer,
            touch: &NSTouch,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn NSGestureRecognizerDelegate {}
);

extern_methods!(
    /// NSSubclassUse
    #[cfg(feature = "AppKit_NSGestureRecognizer")]
    unsafe impl NSGestureRecognizer {
        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSGestureRecognizerState);

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[method(canPreventGestureRecognizer:)]
        pub unsafe fn canPreventGestureRecognizer(
            &self,
            prevented_gesture_recognizer: &NSGestureRecognizer,
        ) -> bool;

        #[method(canBePreventedByGestureRecognizer:)]
        pub unsafe fn canBePreventedByGestureRecognizer(
            &self,
            preventing_gesture_recognizer: &NSGestureRecognizer,
        ) -> bool;

        #[method(shouldRequireFailureOfGestureRecognizer:)]
        pub unsafe fn shouldRequireFailureOfGestureRecognizer(
            &self,
            other_gesture_recognizer: &NSGestureRecognizer,
        ) -> bool;

        #[method(shouldBeRequiredToFailByGestureRecognizer:)]
        pub unsafe fn shouldBeRequiredToFailByGestureRecognizer(
            &self,
            other_gesture_recognizer: &NSGestureRecognizer,
        ) -> bool;

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(mouseDown:)]
        pub unsafe fn mouseDown(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(rightMouseDown:)]
        pub unsafe fn rightMouseDown(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(otherMouseDown:)]
        pub unsafe fn otherMouseDown(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(mouseUp:)]
        pub unsafe fn mouseUp(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(rightMouseUp:)]
        pub unsafe fn rightMouseUp(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(otherMouseUp:)]
        pub unsafe fn otherMouseUp(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(mouseDragged:)]
        pub unsafe fn mouseDragged(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(rightMouseDragged:)]
        pub unsafe fn rightMouseDragged(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(otherMouseDragged:)]
        pub unsafe fn otherMouseDragged(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(keyDown:)]
        pub unsafe fn keyDown(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(keyUp:)]
        pub unsafe fn keyUp(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(flagsChanged:)]
        pub unsafe fn flagsChanged(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(tabletPoint:)]
        pub unsafe fn tabletPoint(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(magnifyWithEvent:)]
        pub unsafe fn magnifyWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(rotateWithEvent:)]
        pub unsafe fn rotateWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(pressureChangeWithEvent:)]
        pub unsafe fn pressureChangeWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(touchesBeganWithEvent:)]
        pub unsafe fn touchesBeganWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(touchesMovedWithEvent:)]
        pub unsafe fn touchesMovedWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(touchesEndedWithEvent:)]
        pub unsafe fn touchesEndedWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(touchesCancelledWithEvent:)]
        pub unsafe fn touchesCancelledWithEvent(&self, event: &NSEvent);
    }
);
