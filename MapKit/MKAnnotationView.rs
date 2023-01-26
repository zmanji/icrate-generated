//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_static!(MKAnnotationCalloutInfoDidChangeNotification: &'static NSString);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MKAnnotationViewDragState {
        MKAnnotationViewDragStateNone = 0,
        MKAnnotationViewDragStateStarting = 1,
        MKAnnotationViewDragStateDragging = 2,
        MKAnnotationViewDragStateCanceling = 3,
        MKAnnotationViewDragStateEnding = 4,
    }
);

typed_extensible_enum!(
    pub type MKFeatureDisplayPriority = c_float;
);

typed_extensible_enum!(
    pub type MKAnnotationViewZPriority = c_float;
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MKAnnotationViewCollisionMode {
        MKAnnotationViewCollisionModeRectangle = 0,
        MKAnnotationViewCollisionModeCircle = 1,
        MKAnnotationViewCollisionModeNone = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKAnnotationView")]
    pub struct MKAnnotationView;

    #[cfg(feature = "MapKit_MKAnnotationView")]
    unsafe impl ClassType for MKAnnotationView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

extern_methods!(
    #[cfg(feature = "MapKit_MKAnnotationView")]
    unsafe impl MKAnnotationView {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithAnnotation:reuseIdentifier:)]
        pub unsafe fn initWithAnnotation_reuseIdentifier(
            this: Option<Allocated<Self>>,
            annotation: Option<&ProtocolObject<dyn MKAnnotation>>,
            reuse_identifier: Option<&NSString>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            a_decoder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other reuseIdentifier)]
        pub unsafe fn reuseIdentifier(&self) -> Option<Id<NSString, Shared>>;

        #[method(prepareForReuse)]
        pub unsafe fn prepareForReuse(&self);

        #[method(prepareForDisplay)]
        pub unsafe fn prepareForDisplay(&self);

        #[method_id(@__retain_semantics Other annotation)]
        pub unsafe fn annotation(&self) -> Option<Id<ProtocolObject<dyn MKAnnotation>, Shared>>;

        #[method(setAnnotation:)]
        pub unsafe fn setAnnotation(&self, annotation: Option<&ProtocolObject<dyn MKAnnotation>>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method(centerOffset)]
        pub unsafe fn centerOffset(&self) -> CGPoint;

        #[method(setCenterOffset:)]
        pub unsafe fn setCenterOffset(&self, center_offset: CGPoint);

        #[method(calloutOffset)]
        pub unsafe fn calloutOffset(&self) -> CGPoint;

        #[method(setCalloutOffset:)]
        pub unsafe fn setCalloutOffset(&self, callout_offset: CGPoint);

        #[method(leftCalloutOffset)]
        pub unsafe fn leftCalloutOffset(&self) -> CGPoint;

        #[method(setLeftCalloutOffset:)]
        pub unsafe fn setLeftCalloutOffset(&self, left_callout_offset: CGPoint);

        #[method(rightCalloutOffset)]
        pub unsafe fn rightCalloutOffset(&self) -> CGPoint;

        #[method(setRightCalloutOffset:)]
        pub unsafe fn setRightCalloutOffset(&self, right_callout_offset: CGPoint);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;

        #[method(setHighlighted:)]
        pub unsafe fn setHighlighted(&self, highlighted: bool);

        #[method(isSelected)]
        pub unsafe fn isSelected(&self) -> bool;

        #[method(setSelected:)]
        pub unsafe fn setSelected(&self, selected: bool);

        #[method(setSelected:animated:)]
        pub unsafe fn setSelected_animated(&self, selected: bool, animated: bool);

        #[method(canShowCallout)]
        pub unsafe fn canShowCallout(&self) -> bool;

        #[method(setCanShowCallout:)]
        pub unsafe fn setCanShowCallout(&self, can_show_callout: bool);

        #[method_id(@__retain_semantics Other leftCalloutAccessoryView)]
        pub unsafe fn leftCalloutAccessoryView(&self) -> Option<Id<NSView, Shared>>;

        #[method(setLeftCalloutAccessoryView:)]
        pub unsafe fn setLeftCalloutAccessoryView(
            &self,
            left_callout_accessory_view: Option<&NSView>,
        );

        #[method_id(@__retain_semantics Other rightCalloutAccessoryView)]
        pub unsafe fn rightCalloutAccessoryView(&self) -> Option<Id<NSView, Shared>>;

        #[method(setRightCalloutAccessoryView:)]
        pub unsafe fn setRightCalloutAccessoryView(
            &self,
            right_callout_accessory_view: Option<&NSView>,
        );

        #[method_id(@__retain_semantics Other detailCalloutAccessoryView)]
        pub unsafe fn detailCalloutAccessoryView(&self) -> Option<Id<NSView, Shared>>;

        #[method(setDetailCalloutAccessoryView:)]
        pub unsafe fn setDetailCalloutAccessoryView(
            &self,
            detail_callout_accessory_view: Option<&NSView>,
        );

        #[method(isDraggable)]
        pub unsafe fn isDraggable(&self) -> bool;

        #[method(setDraggable:)]
        pub unsafe fn setDraggable(&self, draggable: bool);

        #[method(dragState)]
        pub unsafe fn dragState(&self) -> MKAnnotationViewDragState;

        #[method(setDragState:)]
        pub unsafe fn setDragState(&self, drag_state: MKAnnotationViewDragState);

        #[method(setDragState:animated:)]
        pub unsafe fn setDragState_animated(
            &self,
            new_drag_state: MKAnnotationViewDragState,
            animated: bool,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other clusteringIdentifier)]
        pub unsafe fn clusteringIdentifier(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setClusteringIdentifier:)]
        pub unsafe fn setClusteringIdentifier(&self, clustering_identifier: Option<&NSString>);

        #[method_id(@__retain_semantics Other clusterAnnotationView)]
        pub unsafe fn clusterAnnotationView(&self) -> Option<Id<MKAnnotationView, Shared>>;

        #[method(displayPriority)]
        pub unsafe fn displayPriority(&self) -> MKFeatureDisplayPriority;

        #[method(setDisplayPriority:)]
        pub unsafe fn setDisplayPriority(&self, display_priority: MKFeatureDisplayPriority);

        #[method(zPriority)]
        pub unsafe fn zPriority(&self) -> MKAnnotationViewZPriority;

        #[method(setZPriority:)]
        pub unsafe fn setZPriority(&self, z_priority: MKAnnotationViewZPriority);

        #[method(selectedZPriority)]
        pub unsafe fn selectedZPriority(&self) -> MKAnnotationViewZPriority;

        #[method(setSelectedZPriority:)]
        pub unsafe fn setSelectedZPriority(&self, selected_z_priority: MKAnnotationViewZPriority);

        #[method(collisionMode)]
        pub unsafe fn collisionMode(&self) -> MKAnnotationViewCollisionMode;

        #[method(setCollisionMode:)]
        pub unsafe fn setCollisionMode(&self, collision_mode: MKAnnotationViewCollisionMode);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "MapKit_MKAnnotationView")]
    unsafe impl MKAnnotationView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frame_rect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
