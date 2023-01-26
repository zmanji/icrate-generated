//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKMultiPolyline")]
    pub struct MKMultiPolyline;

    #[cfg(feature = "MapKit_MKMultiPolyline")]
    unsafe impl ClassType for MKMultiPolyline {
        #[inherits(NSObject)]
        type Super = MKShape;
    }
);

#[cfg(feature = "MapKit_MKMultiPolyline")]
unsafe impl MKOverlay for MKMultiPolyline {}

extern_methods!(
    #[cfg(feature = "MapKit_MKMultiPolyline")]
    unsafe impl MKMultiPolyline {
        #[cfg(all(feature = "Foundation_NSArray", feature = "MapKit_MKPolyline"))]
        #[method_id(@__retain_semantics Init initWithPolylines:)]
        pub unsafe fn initWithPolylines(
            this: Option<Allocated<Self>>,
            polylines: &NSArray<MKPolyline>,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MapKit_MKPolyline"))]
        #[method_id(@__retain_semantics Other polylines)]
        pub unsafe fn polylines(&self) -> Id<NSArray<MKPolyline>, Shared>;
    }
);
