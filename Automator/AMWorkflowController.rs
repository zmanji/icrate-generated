//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Automator::*;
use crate::Foundation::*;
use crate::OSAKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Automator_AMWorkflowController")]
    pub struct AMWorkflowController;

    #[cfg(feature = "Automator_AMWorkflowController")]
    unsafe impl ClassType for AMWorkflowController {
        #[inherits(NSObject)]
        type Super = NSController;
    }
);

extern_methods!(
    #[cfg(feature = "Automator_AMWorkflowController")]
    unsafe impl AMWorkflowController {
        #[cfg(feature = "Automator_AMWorkflow")]
        #[method_id(@__retain_semantics Other workflow)]
        pub unsafe fn workflow(&self) -> Option<Id<AMWorkflow, Shared>>;

        #[cfg(feature = "Automator_AMWorkflow")]
        #[method(setWorkflow:)]
        pub unsafe fn setWorkflow(&self, workflow: Option<&AMWorkflow>);

        #[cfg(feature = "Automator_AMWorkflowView")]
        #[method_id(@__retain_semantics Other workflowView)]
        pub unsafe fn workflowView(&self) -> Option<Id<AMWorkflowView, Shared>>;

        #[cfg(feature = "Automator_AMWorkflowView")]
        #[method(setWorkflowView:)]
        pub unsafe fn setWorkflowView(&self, workflow_view: Option<&AMWorkflowView>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn AMWorkflowControllerDelegate>, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn AMWorkflowControllerDelegate>>,
        );

        #[method(canRun)]
        pub unsafe fn canRun(&self) -> bool;

        #[method(isRunning)]
        pub unsafe fn isRunning(&self) -> bool;

        #[method(isPaused)]
        pub unsafe fn isPaused(&self) -> bool;

        #[method(run:)]
        pub unsafe fn run(&self, sender: &Object);

        #[method(stop:)]
        pub unsafe fn stop(&self, sender: &Object);

        #[method(pause:)]
        pub unsafe fn pause(&self, sender: &Object);

        #[method(step:)]
        pub unsafe fn step(&self, sender: &Object);

        #[method(reset:)]
        pub unsafe fn reset(&self, sender: &Object);
    }
);

extern_protocol!(
    pub unsafe trait AMWorkflowControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "Automator_AMWorkflowController")]
        #[optional]
        #[method(workflowControllerWillRun:)]
        unsafe fn workflowControllerWillRun(&self, controller: &AMWorkflowController);

        #[cfg(feature = "Automator_AMWorkflowController")]
        #[optional]
        #[method(workflowControllerWillStop:)]
        unsafe fn workflowControllerWillStop(&self, controller: &AMWorkflowController);

        #[cfg(feature = "Automator_AMWorkflowController")]
        #[optional]
        #[method(workflowControllerDidRun:)]
        unsafe fn workflowControllerDidRun(&self, controller: &AMWorkflowController);

        #[cfg(feature = "Automator_AMWorkflowController")]
        #[optional]
        #[method(workflowControllerDidStop:)]
        unsafe fn workflowControllerDidStop(&self, controller: &AMWorkflowController);

        #[cfg(all(
            feature = "Automator_AMAction",
            feature = "Automator_AMWorkflowController"
        ))]
        #[optional]
        #[method(workflowController:willRunAction:)]
        unsafe fn workflowController_willRunAction(
            &self,
            controller: &AMWorkflowController,
            action: &AMAction,
        );

        #[cfg(all(
            feature = "Automator_AMAction",
            feature = "Automator_AMWorkflowController"
        ))]
        #[optional]
        #[method(workflowController:didRunAction:)]
        unsafe fn workflowController_didRunAction(
            &self,
            controller: &AMWorkflowController,
            action: &AMAction,
        );

        #[cfg(all(
            feature = "Automator_AMWorkflowController",
            feature = "Foundation_NSError"
        ))]
        #[optional]
        #[method(workflowController:didError:)]
        unsafe fn workflowController_didError(
            &self,
            controller: &AMWorkflowController,
            error: &NSError,
        );
    }

    unsafe impl ProtocolType for dyn AMWorkflowControllerDelegate {}
);
