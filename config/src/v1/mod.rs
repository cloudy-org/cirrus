pub mod config;
pub mod manager;
pub mod error;

/// # ⚠️ WARNING
/// The root of the path and `self.` is consumed.
/// For example `config_key_path!(self.config.field_a.field_b)` will result in `field_a.field_b`.
/// 
/// ## Example A
/// ```rust
/// let path: &str = config_key_path!(config.misc.enable_custom_folder);
/// 
/// assert!(path, "misc.enable_custom_folder");
/// ```
/// 
/// ## Example B
/// ```rust
/// Settings::new(&self.config_template_string)
///     .add_section::<bool>(
///         Section::new(
///             config_key_path!(self.config.misc.enable_custom_folder),
///             &mut self.config.misc.enable_custom_folder
///         ).into()
///     )
///     .show(ctx, ui, &self.theme);
/// ```
#[macro_export] macro_rules! config_key_path {
    ($base:ident . $($rest:tt)*) => {{
        // The only reason we're doing this below is so we reference the 
        // parameter, in turn tricking the rust compiler to do actual strict type 
        // checking on the key path to avoid incorrect or non-existence config paths.
        let _ = &$base.$($rest)*;

        // Step 2: return the path as string
        stringify!($base.$($rest)*)
    }};
}