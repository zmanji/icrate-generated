//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSComboBoxCell")]
    pub struct NSComboBoxCell;

    #[cfg(feature = "AppKit_NSComboBoxCell")]
    unsafe impl ClassType for NSComboBoxCell {
        #[inherits(NSActionCell, NSCell, NSObject)]
        type Super = NSTextFieldCell;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSComboBoxCell")]
unsafe impl NSAccessibility for NSComboBoxCell {}

#[cfg(feature = "AppKit_NSComboBoxCell")]
unsafe impl NSAccessibilityElementProtocol for NSComboBoxCell {}

#[cfg(feature = "AppKit_NSComboBoxCell")]
unsafe impl NSCoding for NSComboBoxCell {}

#[cfg(feature = "AppKit_NSComboBoxCell")]
unsafe impl NSCopying for NSComboBoxCell {}

#[cfg(feature = "AppKit_NSComboBoxCell")]
unsafe impl NSObjectProtocol for NSComboBoxCell {}

#[cfg(feature = "AppKit_NSComboBoxCell")]
unsafe impl NSUserInterfaceItemIdentification for NSComboBoxCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSComboBoxCell")]
    unsafe impl NSComboBoxCell {
        #[method(hasVerticalScroller)]
        pub unsafe fn hasVerticalScroller(&self) -> bool;

        #[method(setHasVerticalScroller:)]
        pub unsafe fn setHasVerticalScroller(&self, has_vertical_scroller: bool);

        #[method(intercellSpacing)]
        pub unsafe fn intercellSpacing(&self) -> NSSize;

        #[method(setIntercellSpacing:)]
        pub unsafe fn setIntercellSpacing(&self, intercell_spacing: NSSize);

        #[method(itemHeight)]
        pub unsafe fn itemHeight(&self) -> CGFloat;

        #[method(setItemHeight:)]
        pub unsafe fn setItemHeight(&self, item_height: CGFloat);

        #[method(numberOfVisibleItems)]
        pub unsafe fn numberOfVisibleItems(&self) -> NSInteger;

        #[method(setNumberOfVisibleItems:)]
        pub unsafe fn setNumberOfVisibleItems(&self, number_of_visible_items: NSInteger);

        #[method(isButtonBordered)]
        pub unsafe fn isButtonBordered(&self) -> bool;

        #[method(setButtonBordered:)]
        pub unsafe fn setButtonBordered(&self, button_bordered: bool);

        #[method(reloadData)]
        pub unsafe fn reloadData(&self);

        #[method(noteNumberOfItemsChanged)]
        pub unsafe fn noteNumberOfItemsChanged(&self);

        #[method(usesDataSource)]
        pub unsafe fn usesDataSource(&self) -> bool;

        #[method(setUsesDataSource:)]
        pub unsafe fn setUsesDataSource(&self, uses_data_source: bool);

        #[method(scrollItemAtIndexToTop:)]
        pub unsafe fn scrollItemAtIndexToTop(&self, index: NSInteger);

        #[method(scrollItemAtIndexToVisible:)]
        pub unsafe fn scrollItemAtIndexToVisible(&self, index: NSInteger);

        #[method(selectItemAtIndex:)]
        pub unsafe fn selectItemAtIndex(&self, index: NSInteger);

        #[method(deselectItemAtIndex:)]
        pub unsafe fn deselectItemAtIndex(&self, index: NSInteger);

        #[method(indexOfSelectedItem)]
        pub unsafe fn indexOfSelectedItem(&self) -> NSInteger;

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[method(completes)]
        pub unsafe fn completes(&self) -> bool;

        #[method(setCompletes:)]
        pub unsafe fn setCompletes(&self, completes: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other completedString:)]
        pub unsafe fn completedString(&self, string: &NSString) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(&self)
            -> Option<Id<ProtocolObject<dyn NSComboBoxCellDataSource>>>;

        #[method(setDataSource:)]
        pub unsafe fn setDataSource(
            &self,
            data_source: Option<&ProtocolObject<dyn NSComboBoxCellDataSource>>,
        );

        #[method(addItemWithObjectValue:)]
        pub unsafe fn addItemWithObjectValue(&self, object: &AnyObject);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(addItemsWithObjectValues:)]
        pub unsafe fn addItemsWithObjectValues(&self, objects: &NSArray);

        #[method(insertItemWithObjectValue:atIndex:)]
        pub unsafe fn insertItemWithObjectValue_atIndex(
            &self,
            object: &AnyObject,
            index: NSInteger,
        );

        #[method(removeItemWithObjectValue:)]
        pub unsafe fn removeItemWithObjectValue(&self, object: &AnyObject);

        #[method(removeItemAtIndex:)]
        pub unsafe fn removeItemAtIndex(&self, index: NSInteger);

        #[method(removeAllItems)]
        pub unsafe fn removeAllItems(&self);

        #[method(selectItemWithObjectValue:)]
        pub unsafe fn selectItemWithObjectValue(&self, object: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other itemObjectValueAtIndex:)]
        pub unsafe fn itemObjectValueAtIndex(&self, index: NSInteger) -> Id<AnyObject>;

        #[method_id(@__retain_semantics Other objectValueOfSelectedItem)]
        pub unsafe fn objectValueOfSelectedItem(&self) -> Option<Id<AnyObject>>;

        #[method(indexOfItemWithObjectValue:)]
        pub unsafe fn indexOfItemWithObjectValue(&self, object: &AnyObject) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other objectValues)]
        pub unsafe fn objectValues(&self) -> Id<NSArray>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextFieldCell`
    #[cfg(feature = "AppKit_NSComboBoxCell")]
    unsafe impl NSComboBoxCell {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Option<Allocated<Self>>, string: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(feature = "AppKit_NSComboBoxCell")]
    unsafe impl NSComboBoxCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSComboBoxCell")]
    unsafe impl NSComboBoxCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSComboBoxCellDataSource: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSComboBoxCell")]
        #[optional]
        #[method(numberOfItemsInComboBoxCell:)]
        unsafe fn numberOfItemsInComboBoxCell(&self, combo_box_cell: &NSComboBoxCell) -> NSInteger;

        #[cfg(feature = "AppKit_NSComboBoxCell")]
        #[optional]
        #[method_id(@__retain_semantics Other comboBoxCell:objectValueForItemAtIndex:)]
        unsafe fn comboBoxCell_objectValueForItemAtIndex(
            &self,
            combo_box_cell: &NSComboBoxCell,
            index: NSInteger,
        ) -> Id<AnyObject>;

        #[cfg(all(feature = "AppKit_NSComboBoxCell", feature = "Foundation_NSString"))]
        #[optional]
        #[method(comboBoxCell:indexOfItemWithStringValue:)]
        unsafe fn comboBoxCell_indexOfItemWithStringValue(
            &self,
            combo_box_cell: &NSComboBoxCell,
            string: &NSString,
        ) -> NSUInteger;

        #[cfg(all(feature = "AppKit_NSComboBoxCell", feature = "Foundation_NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other comboBoxCell:completedString:)]
        unsafe fn comboBoxCell_completedString(
            &self,
            combo_box_cell: &NSComboBoxCell,
            uncompleted_string: &NSString,
        ) -> Option<Id<NSString>>;
    }

    unsafe impl ProtocolType for dyn NSComboBoxCellDataSource {}
);
