use serde::Serialize;

/// The type of toast
/// # Variants:
/// - `INFO` - For toasts detailing a successful operation
/// - `WARNING` - For toasts describing a warning message
/// - `ERROR` - For toasts describing an error message
#[derive(Serialize)]
pub enum ToastType {
    INFO,
    WARNING,
    ERROR,
}

/// A toast message to be returned to frontend
/// # Properties:
/// - `toast_type`: `ToastType` - The type of toast message being returned
/// - `category`: `String` - The toast message category
/// - `message`: `String` - The main toast message to be displayed
/// - `details`: `Option<String>` - Further information about the toast message
/// # Constructors:
/// - `Toast::new` - Used to create a new toast
/// - `Toast::new_info_toast` - Creates a `ToastType::INFO` toast
/// - `Toast::new_warning_toast` - Creates a `ToastType::WARNING` toast
/// - `Toast::new_error_toast` - Creates a `ToastType::ERROR` toast
/// # Example:
/// ```
/// fn toast_example() -> Result<Vec<Toast>, Toast> {
///   match some_condition {
///     Ok(_) => Ok(vec![
///       Toast::new_info_toast(
///         "Example",
///         "This is an INFO toast",
///         None
///       ),
///       Toast::new_warning_toast(
///         "Example",
///         "This is a WARNING toast",
///         Some("Caused from 'toast_example'")
///       )
///     ]),
///     Err(e) => Err(
///       Toast::new_error_toast(
///         "Example",
///         "This is an ERROR toast",
///         Some(e)
///       )
///     )
///   }
/// }
/// ```
#[derive(Serialize)]
pub struct Toast {
    pub toast_type: ToastType,
    pub category: String,
    pub message: String,
    pub details: Option<String>,
}
impl Toast {
    /// Creates a new toast
    /// # Arguments:
    /// - `toast_type` - The type of toast message being returned
    /// - `category` - The toast message category
    /// - `message` - The main toast message to be displayed
    /// - `details` - Further information about the toast message
    /// # Example:
    /// ```
    /// let toast = Toast::new(
    ///   ToastType::WARNING,
    ///   "Example",
    ///   "This is an example 'WARNING' toast",
    ///   Some("Further details")
    /// );
    ///
    /// assert_eq!(
    ///   toast.toast_type,
    ///   ToastType::WARNING
    /// );
    /// assert_eq!(
    ///   toast.category,
    ///   String::from("Example")
    /// );
    /// assert_eq!(
    ///   toast.message,
    ///   String::from("This is an example 'WARNING' toast")
    /// );
    /// assert_eq!(
    ///   toast.details,
    ///   String::from("Further details")
    /// );
    /// ```
    pub fn new<S: Into<String>>(
        toast_type: ToastType,
        category: S,
        description: S,
        details: Option<S>,
    ) -> Self {
        Toast {
            toast_type,
            category: category.into(),
            message: description.into(),
            details: match details {
                Some(details) => Some(details.into()),
                None => None,
            },
        }
    }

    /// Creates a new `ToastType::INFO` toast
    /// # Arguments:
    /// - `category` - The toast message category
    /// - `message` - The main toast message to be displayed
    /// - `details` - Further information about the toast message
    /// # Example:
    /// ```
    /// let toast = Toast::new_info_toast(
    ///   "Example",
    ///   "This is an example 'INFO' toast",
    ///   Some("Further details")
    /// );
    ///
    /// assert_eq!(
    ///   toast.toast_type,
    ///   ToastType::INFO
    /// );
    /// assert_eq!(
    ///   toast.category,
    ///   String::from("Example")
    /// );
    /// assert_eq!(
    ///   toast.message,
    ///   String::from("This is an example 'INFO' toast")
    /// );
    /// assert_eq!(
    ///   toast.details,
    ///   String::from("Further details")
    /// );
    /// ```
    pub fn new_info_toast<S: Into<String>>(
        category: S,
        description: S,
        details: Option<S>,
    ) -> Self {
        Toast::new(ToastType::INFO, category, description, details)
    }

    /// Creates a new `ToastType::WARNING` toast
    /// # Arguments:
    /// - `category` - The toast message category
    /// - `message` - The main toast message to be displayed
    /// - `details` - Further information about the toast message
    /// # Example:
    /// ```
    /// let toast = Toast::new_warning_toast(
    ///   "Example",
    ///   "This is an example 'WARNING' toast",
    ///   Some("Further details")
    /// );
    ///
    /// assert_eq!(
    ///   toast.toast_type,
    ///   ToastType::WARNING
    /// );
    /// assert_eq!(
    ///   toast.category,
    ///   String::from("Example")
    /// );
    /// assert_eq!(
    ///   toast.message,
    ///   String::from("This is an example 'WARNING' toast")
    /// );
    /// assert_eq!(
    ///   toast.details,
    ///   String::from("Further details")
    /// );
    /// ```
    pub fn new_warning_toast<S: Into<String>>(
        category: S,
        description: S,
        details: Option<S>,
    ) -> Self {
        Toast::new(ToastType::WARNING, category, description, details)
    }

    /// Creates a new `ToastType::ERROR` toast
    /// # Arguments:
    /// - `category` - The toast message category
    /// - `message` - The main toast message to be displayed
    /// - `details` - Further information about the toast message
    /// # Example:
    /// ```
    /// let toast = Toast::new_error_toast(
    ///   "Example",
    ///   "This is an example 'ERROR' toast",
    ///   Some("Further details")
    /// );
    ///
    /// assert_eq!(
    ///   toast.toast_type,
    ///   ToastType::ERROR
    /// );
    /// assert_eq!(
    ///   toast.category,
    ///   String::from("Example")
    /// );
    /// assert_eq!(
    ///   toast.message,
    ///   String::from("This is an example 'ERROR' toast")
    /// );
    /// assert_eq!(
    ///   toast.details,
    ///   String::from("Further details")
    /// );
    /// ```
    pub fn new_error_toast<S: Into<String>>(
        category: S,
        description: S,
        details: Option<S>,
    ) -> Self {
        Toast::new(ToastType::ERROR, category, description, details)
    }

    /// Creates a new `ToastType::ERROR` toast enclosed in a Vector
    /// # Arguments:
    /// - `category` - The toast message category
    /// - `message` - The main toast message to be displayed
    /// - `details` - Further information about the toast message
    /// # Example:
    /// ```
    /// let toast_vec = Toast::new_error_toast_vec(
    ///   "Example",
    ///   "This is an example 'ERROR' toast",
    ///   Some("Further details")
    /// );
    ///
    /// assert_eq!(
    ///   toast_vec[0].toast_type,
    ///   ToastType::ERROR
    /// );
    /// assert_eq!(
    ///   toast_vec[0].category,
    ///   String::from("Example")
    /// );
    /// assert_eq!(
    ///   toast_vec[0].message,
    ///   String::from("This is an example 'ERROR' toast")
    /// );
    /// assert_eq!(
    ///   toast_vec[0].details,
    ///   String::from("Further details")
    /// );
    /// ```
    pub fn new_error_toast_vec<S: Into<String>>(
        category: S,
        description: S,
        details: Option<S>,
    ) -> Vec<Self> {
        vec![Toast::new(ToastType::ERROR, category, description, details)]
    }
}
