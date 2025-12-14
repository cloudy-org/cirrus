pub mod config;
pub mod error;

/// # ⚠️ WARNING
/// The root of the path and `self.` is consumed.
/// For example `config_key_path!(self.config.field_a.field_b)` will result in `field_a.field_b`.
/// 
/// ## Example A
/// ```ignore
/// let path = config_key_path!(config.misc.enable_custom_folder);
/// 
/// assert!(path, "misc.enable_custom_folder");
/// ```
/// 
/// ## Example B
/// ```ignore
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
        // parameter, in turn tricking the rust compiler to do real strict type 
        // checking on the key path to avoid developers from inserting 
        // incorrect or non-existence config paths, pretty cool.
        let _ = &$base.$($rest)*;

        let config_key_path = stringify!($base.$($rest)*);

        let formatted_key_path = config_key_path
            .replace("self.", "");

        let mut split_key_path = formatted_key_path.split(".");

        // we're consuming the root to get rid of the path's prefix ("config.").
        split_key_path.next(); 

        split_key_path.collect::<Vec<&str>>().join(".")
    }};
}