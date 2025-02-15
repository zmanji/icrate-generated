// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `MetricKit` framework

#[path = "../../fixes/MetricKit/mod.rs"]
mod fixes;
pub use self::fixes::*;

#[cfg_attr(feature = "apple", link(name = "MetricKit", kind = "framework"))]
extern "C" {}

#[path = "MXAnimationMetric.rs"]
mod __MXAnimationMetric;
#[path = "MXAppExitMetric.rs"]
mod __MXAppExitMetric;
#[path = "MXAppLaunchDiagnostic.rs"]
mod __MXAppLaunchDiagnostic;
#[path = "MXAppLaunchMetric.rs"]
mod __MXAppLaunchMetric;
#[path = "MXAppResponsivenessMetric.rs"]
mod __MXAppResponsivenessMetric;
#[path = "MXAppRunTimeMetric.rs"]
mod __MXAppRunTimeMetric;
#[path = "MXAverage.rs"]
mod __MXAverage;
#[path = "MXCPUExceptionDiagnostic.rs"]
mod __MXCPUExceptionDiagnostic;
#[path = "MXCPUMetric.rs"]
mod __MXCPUMetric;
#[path = "MXCallStackTree.rs"]
mod __MXCallStackTree;
#[path = "MXCellularConditionMetric.rs"]
mod __MXCellularConditionMetric;
#[path = "MXCrashDiagnostic.rs"]
mod __MXCrashDiagnostic;
#[path = "MXDiagnostic.rs"]
mod __MXDiagnostic;
#[path = "MXDiagnosticPayload.rs"]
mod __MXDiagnosticPayload;
#[path = "MXDiskIOMetric.rs"]
mod __MXDiskIOMetric;
#[path = "MXDiskWriteExceptionDiagnostic.rs"]
mod __MXDiskWriteExceptionDiagnostic;
#[path = "MXDisplayMetric.rs"]
mod __MXDisplayMetric;
#[path = "MXError.rs"]
mod __MXError;
#[path = "MXGPUMetric.rs"]
mod __MXGPUMetric;
#[path = "MXHangDiagnostic.rs"]
mod __MXHangDiagnostic;
#[path = "MXHistogram.rs"]
mod __MXHistogram;
#[path = "MXLocationActivityMetric.rs"]
mod __MXLocationActivityMetric;
#[path = "MXMemoryMetric.rs"]
mod __MXMemoryMetric;
#[path = "MXMetaData.rs"]
mod __MXMetaData;
#[path = "MXMetric.rs"]
mod __MXMetric;
#[path = "MXMetricManager.rs"]
mod __MXMetricManager;
#[path = "MXMetricPayload.rs"]
mod __MXMetricPayload;
#[path = "MXNetworkTransferMetric.rs"]
mod __MXNetworkTransferMetric;
#[path = "MXSignpost.rs"]
mod __MXSignpost;
#[path = "MXSignpostMetric.rs"]
mod __MXSignpostMetric;
#[path = "MXSignpost_Private.rs"]
mod __MXSignpost_Private;
#[path = "MXUnit.rs"]
mod __MXUnit;

#[cfg(feature = "MetricKit_MXAnimationMetric")]
pub use self::__MXAnimationMetric::MXAnimationMetric;
#[cfg(feature = "MetricKit_MXAppExitMetric")]
pub use self::__MXAppExitMetric::MXAppExitMetric;
#[cfg(feature = "MetricKit_MXBackgroundExitData")]
pub use self::__MXAppExitMetric::MXBackgroundExitData;
#[cfg(feature = "MetricKit_MXForegroundExitData")]
pub use self::__MXAppExitMetric::MXForegroundExitData;
#[cfg(feature = "MetricKit_MXAppLaunchDiagnostic")]
pub use self::__MXAppLaunchDiagnostic::MXAppLaunchDiagnostic;
#[cfg(feature = "MetricKit_MXAppLaunchMetric")]
pub use self::__MXAppLaunchMetric::MXAppLaunchMetric;
#[cfg(feature = "MetricKit_MXAppResponsivenessMetric")]
pub use self::__MXAppResponsivenessMetric::MXAppResponsivenessMetric;
#[cfg(feature = "MetricKit_MXAppRunTimeMetric")]
pub use self::__MXAppRunTimeMetric::MXAppRunTimeMetric;
#[cfg(feature = "MetricKit_MXAverage")]
pub use self::__MXAverage::MXAverage;
#[cfg(feature = "MetricKit_MXCPUExceptionDiagnostic")]
pub use self::__MXCPUExceptionDiagnostic::MXCPUExceptionDiagnostic;
#[cfg(feature = "MetricKit_MXCPUMetric")]
pub use self::__MXCPUMetric::MXCPUMetric;
#[cfg(feature = "MetricKit_MXCallStackTree")]
pub use self::__MXCallStackTree::MXCallStackTree;
#[cfg(feature = "MetricKit_MXCellularConditionMetric")]
pub use self::__MXCellularConditionMetric::MXCellularConditionMetric;
#[cfg(feature = "MetricKit_MXCrashDiagnostic")]
pub use self::__MXCrashDiagnostic::MXCrashDiagnostic;
#[cfg(feature = "MetricKit_MXDiagnostic")]
pub use self::__MXDiagnostic::MXDiagnostic;
#[cfg(feature = "MetricKit_MXDiagnosticPayload")]
pub use self::__MXDiagnosticPayload::MXDiagnosticPayload;
#[cfg(feature = "MetricKit_MXDiskIOMetric")]
pub use self::__MXDiskIOMetric::MXDiskIOMetric;
#[cfg(feature = "MetricKit_MXDiskWriteExceptionDiagnostic")]
pub use self::__MXDiskWriteExceptionDiagnostic::MXDiskWriteExceptionDiagnostic;
#[cfg(feature = "MetricKit_MXDisplayMetric")]
pub use self::__MXDisplayMetric::MXDisplayMetric;
pub use self::__MXError::MXErrorDomain;
pub use self::__MXError::{
    MXErrorCode, MXErrorLaunchTaskDuplicated, MXErrorLaunchTaskInternalFailure,
    MXErrorLaunchTaskInvalidID, MXErrorLaunchTaskMaxCount, MXErrorLaunchTaskPastDeadline,
    MXErrorLaunchTaskUnknown,
};
#[cfg(feature = "MetricKit_MXGPUMetric")]
pub use self::__MXGPUMetric::MXGPUMetric;
#[cfg(feature = "MetricKit_MXHangDiagnostic")]
pub use self::__MXHangDiagnostic::MXHangDiagnostic;
#[cfg(feature = "MetricKit_MXHistogram")]
pub use self::__MXHistogram::MXHistogram;
#[cfg(feature = "MetricKit_MXHistogramBucket")]
pub use self::__MXHistogram::MXHistogramBucket;
#[cfg(feature = "MetricKit_MXLocationActivityMetric")]
pub use self::__MXLocationActivityMetric::MXLocationActivityMetric;
#[cfg(feature = "MetricKit_MXMemoryMetric")]
pub use self::__MXMemoryMetric::MXMemoryMetric;
#[cfg(feature = "MetricKit_MXMetaData")]
pub use self::__MXMetaData::MXMetaData;
#[cfg(feature = "MetricKit_MXMetric")]
pub use self::__MXMetric::MXMetric;
#[cfg(feature = "MetricKit_MXMetricManager")]
pub use self::__MXMetricManager::MXMetricManager;
pub use self::__MXMetricManager::MXMetricManagerSubscriber;
#[cfg(feature = "MetricKit_MXMetricPayload")]
pub use self::__MXMetricPayload::MXMetricPayload;
#[cfg(feature = "MetricKit_MXNetworkTransferMetric")]
pub use self::__MXNetworkTransferMetric::MXNetworkTransferMetric;
#[cfg(feature = "MetricKit_MXSignpostIntervalData")]
pub use self::__MXSignpostMetric::MXSignpostIntervalData;
#[cfg(feature = "MetricKit_MXSignpostMetric")]
pub use self::__MXSignpostMetric::MXSignpostMetric;
pub use self::__MXSignpost_Private::_MXSignpostMetricsSnapshot;
#[cfg(feature = "MetricKit_MXUnitAveragePixelLuminance")]
pub use self::__MXUnit::MXUnitAveragePixelLuminance;
#[cfg(feature = "MetricKit_MXUnitSignalBars")]
pub use self::__MXUnit::MXUnitSignalBars;
