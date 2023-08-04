mod toast;
pub use toast::{Toast, ToastType};

/// Result for querying data without an OK toast vector
pub type DataResult<T> = std::result::Result<T, Vec<Toast>>;

/// Result returning toast vector results
pub type ToastResult = std::result::Result<Vec<Toast>, Vec<Toast>>;

/// A list of Toasts to return to the frontend
pub type ToastList = Vec<Toast>;
