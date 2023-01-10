//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation;
use crate::Foundation;

typed_enum!(
    pub type CAMediaTimingFillMode = Foundation::NSString;
);

extern_protocol!(
    pub struct CAMediaTiming;

    unsafe impl ProtocolType for CAMediaTiming {
        #[method(beginTime)]
        pub unsafe fn beginTime(&self) -> CFTimeInterval;

        #[method(setBeginTime:)]
        pub unsafe fn setBeginTime(&self, beginTime: CFTimeInterval);

        #[method(duration)]
        pub unsafe fn duration(&self) -> CFTimeInterval;

        #[method(setDuration:)]
        pub unsafe fn setDuration(&self, duration: CFTimeInterval);

        #[method(speed)]
        pub unsafe fn speed(&self) -> c_float;

        #[method(setSpeed:)]
        pub unsafe fn setSpeed(&self, speed: c_float);

        #[method(timeOffset)]
        pub unsafe fn timeOffset(&self) -> CFTimeInterval;

        #[method(setTimeOffset:)]
        pub unsafe fn setTimeOffset(&self, timeOffset: CFTimeInterval);

        #[method(repeatCount)]
        pub unsafe fn repeatCount(&self) -> c_float;

        #[method(setRepeatCount:)]
        pub unsafe fn setRepeatCount(&self, repeatCount: c_float);

        #[method(repeatDuration)]
        pub unsafe fn repeatDuration(&self) -> CFTimeInterval;

        #[method(setRepeatDuration:)]
        pub unsafe fn setRepeatDuration(&self, repeatDuration: CFTimeInterval);

        #[method(autoreverses)]
        pub unsafe fn autoreverses(&self) -> bool;

        #[method(setAutoreverses:)]
        pub unsafe fn setAutoreverses(&self, autoreverses: bool);

        #[method_id(@__retain_semantics Other fillMode)]
        pub unsafe fn fillMode(&self) -> Id<CoreAnimation::CAMediaTimingFillMode, Shared>;

        #[method(setFillMode:)]
        pub unsafe fn setFillMode(&self, fillMode: &CoreAnimation::CAMediaTimingFillMode);
    }
);

extern_static!(kCAFillModeForwards: &'static CoreAnimation::CAMediaTimingFillMode);

extern_static!(kCAFillModeBackwards: &'static CoreAnimation::CAMediaTimingFillMode);

extern_static!(kCAFillModeBoth: &'static CoreAnimation::CAMediaTimingFillMode);

extern_static!(kCAFillModeRemoved: &'static CoreAnimation::CAMediaTimingFillMode);
