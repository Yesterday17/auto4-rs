use crate::ass::{ProjectProperties, KeyFrames, Style};

// TODO: remove placeholder type F
type F = String;

pub trait AegisubAutomation: Sized {
    fn lua_automation_version() -> u8 { 4 }

    /// Macro Registration Function
    ///
    /// This is a function called from top-level of an Automation script to register
    /// a new Macro Feature.
    ///
    /// @name (string)
    ///   The displayed name of the menu item this macro will generate.
    ///
    /// @description (string)
    ///
    ///   A longer description of the function of this macro. This will appear
    ///   on the status bar when hovering over the menu item.
    ///
    /// @processing_function (function)
    ///   The actual function called for the macro execution.
    ///   This function must be an instance of the Macro Processing Function
    ///   described below.
    ///
    /// @validation_function (function)
    ///   Optional. A function called when it is to be determined whether the
    ///   macro can act on the current subtitles.
    ///   This function, if provided, must execute very quickly to avoid lag
    ///   in the GUI.
    ///   This function must be an instance of the Macro Validation Function
    ///   described below.
    ///
    /// Returns: nothing.
    fn register_macro(&self, name: String, description: String, processing_function: F, validation_function: Option<F>, is_active_function: Option<F>);

    /// Filter Registration Function
    ///
    /// This is a function called from top level of an Automation script to register
    /// a new Export Filter Feature.
    ///
    /// @name (string)
    ///   The name of the filter, as presented to the user.
    ///
    /// @description (string)
    ///   A longer description of the filter presented to the user.
    ///
    /// @priority (number)
    ///   A number determining the default order the enabled filters will be
    ///   processed. The order can be overridden by the user.
    ///   Priorities of some built-in filters:
    ///    o Clean Script Info = 0
    ///    o Fix Styles = -5000
    ///    o Transform Framerate = 1000
    ///   Filters with higher priority will be executed earlier by default.
    ///
    /// @processing_function (function)
    ///   The function called to do the actual filter processing.
    ///   This function must be an instance of the Filter Processing Function
    ///   described below.
    ///
    /// @options_window_provider (function)
    ///   Optional. A function providing a dialog template for setting options
    ///   prior to filter processing.
    ///   This function must be an instance of the Filter Options Window Provider
    ///   function described below.
    ///
    /// Returns: nothing.
    fn register_filter(&self, name: String, description: String, priority: i32, processing_function: F, configuration_panel_provider: Option<F>);

    // return width, height, descent, ext_lead
    fn text_extents(&self, style: Style, text: String) -> (i32, i32, i32, i32);
    fn gettext(&self, untranslated: String) -> String { untranslated }
    fn frame_from_ms(&self, ms: i32) -> i32;
    fn ms_from_frame(&self, frame: i32) -> i32;
    // x_res, y_res, ar, ar_type
    fn video_size(&self) -> (i32, i32, i32, i32);
    fn keyframes(&self) -> KeyFrames;
    fn decode_path(&self, encoded_path: String) -> String;
    fn project_properties(&self) -> ProjectProperties;
}