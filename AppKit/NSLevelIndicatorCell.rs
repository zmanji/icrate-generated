//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSLevelIndicatorStyle {
        NSLevelIndicatorStyleRelevancy = 0,
        NSLevelIndicatorStyleContinuousCapacity = 1,
        NSLevelIndicatorStyleDiscreteCapacity = 2,
        NSLevelIndicatorStyleRating = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSLevelIndicatorCell")]
    pub struct NSLevelIndicatorCell;

    #[cfg(feature = "AppKit_NSLevelIndicatorCell")]
    unsafe impl ClassType for NSLevelIndicatorCell {
        #[inherits(NSCell, NSObject)]
        type Super = NSActionCell;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSLevelIndicatorCell")]
unsafe impl NSAccessibility for NSLevelIndicatorCell {}

#[cfg(feature = "AppKit_NSLevelIndicatorCell")]
unsafe impl NSAccessibilityElementProtocol for NSLevelIndicatorCell {}

#[cfg(feature = "AppKit_NSLevelIndicatorCell")]
unsafe impl NSCoding for NSLevelIndicatorCell {}

#[cfg(feature = "AppKit_NSLevelIndicatorCell")]
unsafe impl NSCopying for NSLevelIndicatorCell {}

#[cfg(feature = "AppKit_NSLevelIndicatorCell")]
unsafe impl NSObjectProtocol for NSLevelIndicatorCell {}

#[cfg(feature = "AppKit_NSLevelIndicatorCell")]
unsafe impl NSUserInterfaceItemIdentification for NSLevelIndicatorCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSLevelIndicatorCell")]
    unsafe impl NSLevelIndicatorCell {
        #[method_id(@__retain_semantics Init initWithLevelIndicatorStyle:)]
        pub unsafe fn initWithLevelIndicatorStyle(
            this: Option<Allocated<Self>>,
            level_indicator_style: NSLevelIndicatorStyle,
        ) -> Id<Self>;

        #[method(levelIndicatorStyle)]
        pub unsafe fn levelIndicatorStyle(&self) -> NSLevelIndicatorStyle;

        #[method(setLevelIndicatorStyle:)]
        pub unsafe fn setLevelIndicatorStyle(&self, level_indicator_style: NSLevelIndicatorStyle);

        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;

        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, min_value: c_double);

        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;

        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, max_value: c_double);

        #[method(warningValue)]
        pub unsafe fn warningValue(&self) -> c_double;

        #[method(setWarningValue:)]
        pub unsafe fn setWarningValue(&self, warning_value: c_double);

        #[method(criticalValue)]
        pub unsafe fn criticalValue(&self) -> c_double;

        #[method(setCriticalValue:)]
        pub unsafe fn setCriticalValue(&self, critical_value: c_double);

        #[method(tickMarkPosition)]
        pub unsafe fn tickMarkPosition(&self) -> NSTickMarkPosition;

        #[method(setTickMarkPosition:)]
        pub unsafe fn setTickMarkPosition(&self, tick_mark_position: NSTickMarkPosition);

        #[method(numberOfTickMarks)]
        pub unsafe fn numberOfTickMarks(&self) -> NSInteger;

        #[method(setNumberOfTickMarks:)]
        pub unsafe fn setNumberOfTickMarks(&self, number_of_tick_marks: NSInteger);

        #[method(numberOfMajorTickMarks)]
        pub unsafe fn numberOfMajorTickMarks(&self) -> NSInteger;

        #[method(setNumberOfMajorTickMarks:)]
        pub unsafe fn setNumberOfMajorTickMarks(&self, number_of_major_tick_marks: NSInteger);

        #[method(rectOfTickMarkAtIndex:)]
        pub unsafe fn rectOfTickMarkAtIndex(&self, index: NSInteger) -> NSRect;

        #[method(tickMarkValueAtIndex:)]
        pub unsafe fn tickMarkValueAtIndex(&self, index: NSInteger) -> c_double;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(feature = "AppKit_NSLevelIndicatorCell")]
    unsafe impl NSLevelIndicatorCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Option<Allocated<Self>>, string: &NSString) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSLevelIndicatorCell")]
    unsafe impl NSLevelIndicatorCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_static!(NSRelevancyLevelIndicatorStyle: NSLevelIndicatorStyle = NSLevelIndicatorStyleRelevancy);

extern_static!(NSContinuousCapacityLevelIndicatorStyle: NSLevelIndicatorStyle = NSLevelIndicatorStyleContinuousCapacity);

extern_static!(NSDiscreteCapacityLevelIndicatorStyle: NSLevelIndicatorStyle = NSLevelIndicatorStyleDiscreteCapacity);

extern_static!(NSRatingLevelIndicatorStyle: NSLevelIndicatorStyle = NSLevelIndicatorStyleRating);
