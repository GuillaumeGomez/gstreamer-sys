// This file was generated by gir (5c71144) from gir-files (???)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals)]

extern crate libc;
#[macro_use] extern crate bitflags;
extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
extern crate gstreamer_sys as gst;
extern crate gstreamer_tag_sys as gst_tag;
extern crate gstreamer_audio_sys as gst_audio;
extern crate gstreamer_video_sys as gst_video;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType, Volatile};

// Aliases
pub type GstDiscovererAudioInfoClass = gobject::GObjectClass;
pub type GstDiscovererContainerInfoClass = gobject::GObjectClass;
pub type GstDiscovererInfoClass = gobject::GObjectClass;
pub type GstDiscovererStreamInfoClass = gobject::GObjectClass;
pub type GstDiscovererSubtitleInfoClass = gobject::GObjectClass;
pub type GstDiscovererVideoInfoClass = gobject::GObjectClass;
pub type GstEncodingTargetClass = gobject::GObjectClass;

// Enums
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum GstAudioVisualizerShader {
    None = 0,
    Fade = 1,
    FadeAndMoveUp = 2,
    FadeAndMoveDown = 3,
    FadeAndMoveLeft = 4,
    FadeAndMoveRight = 5,
    FadeAndMoveHorizOut = 6,
    FadeAndMoveHorizIn = 7,
    FadeAndMoveVertOut = 8,
    FadeAndMoveVertIn = 9,
}
pub const GST_AUDIO_VISUALIZER_SHADER_NONE: GstAudioVisualizerShader = GstAudioVisualizerShader::None;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE: GstAudioVisualizerShader = GstAudioVisualizerShader::Fade;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_UP: GstAudioVisualizerShader = GstAudioVisualizerShader::FadeAndMoveUp;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_DOWN: GstAudioVisualizerShader = GstAudioVisualizerShader::FadeAndMoveDown;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_LEFT: GstAudioVisualizerShader = GstAudioVisualizerShader::FadeAndMoveLeft;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_RIGHT: GstAudioVisualizerShader = GstAudioVisualizerShader::FadeAndMoveRight;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_HORIZ_OUT: GstAudioVisualizerShader = GstAudioVisualizerShader::FadeAndMoveHorizOut;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_HORIZ_IN: GstAudioVisualizerShader = GstAudioVisualizerShader::FadeAndMoveHorizIn;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_VERT_OUT: GstAudioVisualizerShader = GstAudioVisualizerShader::FadeAndMoveVertOut;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_VERT_IN: GstAudioVisualizerShader = GstAudioVisualizerShader::FadeAndMoveVertIn;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum GstDiscovererResult {
    Ok = 0,
    UriInvalid = 1,
    Error = 2,
    Timeout = 3,
    Busy = 4,
    MissingPlugins = 5,
}
pub const GST_DISCOVERER_OK: GstDiscovererResult = GstDiscovererResult::Ok;
pub const GST_DISCOVERER_URI_INVALID: GstDiscovererResult = GstDiscovererResult::UriInvalid;
pub const GST_DISCOVERER_ERROR: GstDiscovererResult = GstDiscovererResult::Error;
pub const GST_DISCOVERER_TIMEOUT: GstDiscovererResult = GstDiscovererResult::Timeout;
pub const GST_DISCOVERER_BUSY: GstDiscovererResult = GstDiscovererResult::Busy;
pub const GST_DISCOVERER_MISSING_PLUGINS: GstDiscovererResult = GstDiscovererResult::MissingPlugins;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum GstInstallPluginsReturn {
    Success = 0,
    NotFound = 1,
    Error = 2,
    PartialSuccess = 3,
    UserAbort = 4,
    Crashed = 100,
    Invalid = 101,
    StartedOk = 200,
    InternalFailure = 201,
    HelperMissing = 202,
    InstallInProgress = 203,
}
pub const GST_INSTALL_PLUGINS_SUCCESS: GstInstallPluginsReturn = GstInstallPluginsReturn::Success;
pub const GST_INSTALL_PLUGINS_NOT_FOUND: GstInstallPluginsReturn = GstInstallPluginsReturn::NotFound;
pub const GST_INSTALL_PLUGINS_ERROR: GstInstallPluginsReturn = GstInstallPluginsReturn::Error;
pub const GST_INSTALL_PLUGINS_PARTIAL_SUCCESS: GstInstallPluginsReturn = GstInstallPluginsReturn::PartialSuccess;
pub const GST_INSTALL_PLUGINS_USER_ABORT: GstInstallPluginsReturn = GstInstallPluginsReturn::UserAbort;
pub const GST_INSTALL_PLUGINS_CRASHED: GstInstallPluginsReturn = GstInstallPluginsReturn::Crashed;
pub const GST_INSTALL_PLUGINS_INVALID: GstInstallPluginsReturn = GstInstallPluginsReturn::Invalid;
pub const GST_INSTALL_PLUGINS_STARTED_OK: GstInstallPluginsReturn = GstInstallPluginsReturn::StartedOk;
pub const GST_INSTALL_PLUGINS_INTERNAL_FAILURE: GstInstallPluginsReturn = GstInstallPluginsReturn::InternalFailure;
pub const GST_INSTALL_PLUGINS_HELPER_MISSING: GstInstallPluginsReturn = GstInstallPluginsReturn::HelperMissing;
pub const GST_INSTALL_PLUGINS_INSTALL_IN_PROGRESS: GstInstallPluginsReturn = GstInstallPluginsReturn::InstallInProgress;

// Constants
pub const GST_ENCODING_CATEGORY_CAPTURE: *const c_char = b"capture\0" as *const u8 as *const c_char;
pub const GST_ENCODING_CATEGORY_DEVICE: *const c_char = b"device\0" as *const u8 as *const c_char;
pub const GST_ENCODING_CATEGORY_FILE_EXTENSION: *const c_char = b"file-extension\0" as *const u8 as *const c_char;
pub const GST_ENCODING_CATEGORY_ONLINE_SERVICE: *const c_char = b"online-service\0" as *const u8 as *const c_char;
pub const GST_ENCODING_CATEGORY_STORAGE_EDITING: *const c_char = b"storage-editing\0" as *const u8 as *const c_char;
pub const GST_PLUGINS_BASE_VERSION_MAJOR: c_int = 1;
pub const GST_PLUGINS_BASE_VERSION_MICRO: c_int = 1;
pub const GST_PLUGINS_BASE_VERSION_MINOR: c_int = 12;
pub const GST_PLUGINS_BASE_VERSION_NANO: c_int = 0;

// Flags
bitflags! {
    #[repr(C)]
    pub struct GstDiscovererSerializeFlags: c_uint {
        const GST_DISCOVERER_SERIALIZE_BASIC = 0;
        const GST_DISCOVERER_SERIALIZE_CAPS = 1;
        const GST_DISCOVERER_SERIALIZE_TAGS = 2;
        const GST_DISCOVERER_SERIALIZE_MISC = 4;
        const GST_DISCOVERER_SERIALIZE_ALL = 7;
    }
}

// Callbacks
pub type GstAudioVisualizerShaderFunc = Option<unsafe extern "C" fn(*mut GstAudioVisualizer, *const gst_video::GstVideoFrame, *mut gst_video::GstVideoFrame)>;
pub type GstInstallPluginsResultFunc = Option<unsafe extern "C" fn(GstInstallPluginsReturn, gpointer)>;

// Records
#[repr(C)]
pub struct GstAudioVisualizerClass {
    pub parent_class: gst::GstElementClass,
    pub setup: Option<unsafe extern "C" fn(*mut GstAudioVisualizer) -> gboolean>,
    pub render: Option<unsafe extern "C" fn(*mut GstAudioVisualizer, *mut gst::GstBuffer, *mut gst_video::GstVideoFrame) -> gboolean>,
    pub decide_allocation: Option<unsafe extern "C" fn(*mut GstAudioVisualizer, *mut gst::GstQuery) -> gboolean>,
}

#[repr(C)]
pub struct GstAudioVisualizerPrivate(c_void);

#[repr(C)]
pub struct GstDiscovererClass {
    pub parentclass: gobject::GObjectClass,
    pub finished: Option<unsafe extern "C" fn(*mut GstDiscoverer)>,
    pub starting: Option<unsafe extern "C" fn(*mut GstDiscoverer)>,
    pub discovered: Option<unsafe extern "C" fn(*mut GstDiscoverer, *mut GstDiscovererInfo, *const glib::GError)>,
    pub source_setup: Option<unsafe extern "C" fn(*mut GstDiscoverer, *mut gst::GstElement)>,
    pub _reserved: [gpointer; 4],
}

#[repr(C)]
pub struct GstDiscovererPrivate(c_void);

#[repr(C)]
pub struct GstEncodingAudioProfileClass(c_void);

#[repr(C)]
pub struct GstEncodingContainerProfileClass(c_void);

#[repr(C)]
pub struct GstEncodingProfileClass(c_void);

#[repr(C)]
pub struct GstEncodingVideoProfileClass(c_void);

#[repr(C)]
pub struct GstInstallPluginsContext(c_void);

// Classes
#[repr(C)]
pub struct GstAudioVisualizer {
    pub parent: gst::GstElement,
    pub req_spf: c_uint,
    pub vinfo: gst_video::GstVideoInfo,
    pub ainfo: gst_audio::GstAudioInfo,
    pub priv_: *mut GstAudioVisualizerPrivate,
}

#[repr(C)]
pub struct GstDiscoverer {
    pub parent: gobject::GObject,
    pub priv_: *mut GstDiscovererPrivate,
    pub _reserved: [gpointer; 4],
}

#[repr(C)]
pub struct GstDiscovererAudioInfo(c_void);

#[repr(C)]
pub struct GstDiscovererContainerInfo(c_void);

#[repr(C)]
pub struct GstDiscovererInfo(c_void);

#[repr(C)]
pub struct GstDiscovererStreamInfo(c_void);

#[repr(C)]
pub struct GstDiscovererSubtitleInfo(c_void);

#[repr(C)]
pub struct GstDiscovererVideoInfo(c_void);

#[repr(C)]
pub struct GstEncodingAudioProfile(c_void);

#[repr(C)]
pub struct GstEncodingContainerProfile(c_void);

#[repr(C)]
pub struct GstEncodingProfile(c_void);

#[repr(C)]
pub struct GstEncodingTarget(c_void);

#[repr(C)]
pub struct GstEncodingVideoProfile(c_void);

extern "C" {

    //=========================================================================
    // GstAudioVisualizerShader
    //=========================================================================
    pub fn gst_audio_visualizer_shader_get_type() -> GType;

    //=========================================================================
    // GstDiscovererResult
    //=========================================================================
    pub fn gst_discoverer_result_get_type() -> GType;

    //=========================================================================
    // GstInstallPluginsReturn
    //=========================================================================
    pub fn gst_install_plugins_return_get_type() -> GType;
    pub fn gst_install_plugins_return_get_name(ret: GstInstallPluginsReturn) -> *const c_char;

    //=========================================================================
    // GstDiscovererSerializeFlags
    //=========================================================================
    pub fn gst_discoverer_serialize_flags_get_type() -> GType;

    //=========================================================================
    // GstInstallPluginsContext
    //=========================================================================
    pub fn gst_install_plugins_context_get_type() -> GType;
    pub fn gst_install_plugins_context_new() -> *mut GstInstallPluginsContext;
    pub fn gst_install_plugins_context_copy(ctx: *mut GstInstallPluginsContext) -> *mut GstInstallPluginsContext;
    pub fn gst_install_plugins_context_free(ctx: *mut GstInstallPluginsContext);
    #[cfg(feature = "v1_6")]
    pub fn gst_install_plugins_context_set_confirm_search(ctx: *mut GstInstallPluginsContext, confirm_search: gboolean);
    #[cfg(feature = "v1_6")]
    pub fn gst_install_plugins_context_set_desktop_id(ctx: *mut GstInstallPluginsContext, desktop_id: *const c_char);
    #[cfg(feature = "v1_6")]
    pub fn gst_install_plugins_context_set_startup_notification_id(ctx: *mut GstInstallPluginsContext, startup_id: *const c_char);
    pub fn gst_install_plugins_context_set_xid(ctx: *mut GstInstallPluginsContext, xid: c_uint);

    //=========================================================================
    // GstAudioVisualizer
    //=========================================================================
    pub fn gst_audio_visualizer_get_type() -> GType;

    //=========================================================================
    // GstDiscoverer
    //=========================================================================
    pub fn gst_discoverer_get_type() -> GType;
    pub fn gst_discoverer_new(timeout: gst::GstClockTime, error: *mut *mut glib::GError) -> *mut GstDiscoverer;
    pub fn gst_discoverer_discover_uri(discoverer: *mut GstDiscoverer, uri: *const c_char, error: *mut *mut glib::GError) -> *mut GstDiscovererInfo;
    pub fn gst_discoverer_discover_uri_async(discoverer: *mut GstDiscoverer, uri: *const c_char) -> gboolean;
    pub fn gst_discoverer_start(discoverer: *mut GstDiscoverer);
    pub fn gst_discoverer_stop(discoverer: *mut GstDiscoverer);

    //=========================================================================
    // GstDiscovererAudioInfo
    //=========================================================================
    pub fn gst_discoverer_audio_info_get_type() -> GType;
    pub fn gst_discoverer_audio_info_get_bitrate(info: *const GstDiscovererAudioInfo) -> c_uint;
    pub fn gst_discoverer_audio_info_get_channels(info: *const GstDiscovererAudioInfo) -> c_uint;
    pub fn gst_discoverer_audio_info_get_depth(info: *const GstDiscovererAudioInfo) -> c_uint;
    pub fn gst_discoverer_audio_info_get_language(info: *const GstDiscovererAudioInfo) -> *const c_char;
    pub fn gst_discoverer_audio_info_get_max_bitrate(info: *const GstDiscovererAudioInfo) -> c_uint;
    pub fn gst_discoverer_audio_info_get_sample_rate(info: *const GstDiscovererAudioInfo) -> c_uint;

    //=========================================================================
    // GstDiscovererContainerInfo
    //=========================================================================
    pub fn gst_discoverer_container_info_get_type() -> GType;
    pub fn gst_discoverer_container_info_get_streams(info: *mut GstDiscovererContainerInfo) -> *mut glib::GList;

    //=========================================================================
    // GstDiscovererInfo
    //=========================================================================
    pub fn gst_discoverer_info_get_type() -> GType;
    #[cfg(feature = "v1_6")]
    pub fn gst_discoverer_info_from_variant(variant: *mut glib::GVariant) -> *mut GstDiscovererInfo;
    pub fn gst_discoverer_info_copy(ptr: *mut GstDiscovererInfo) -> *mut GstDiscovererInfo;
    pub fn gst_discoverer_info_get_audio_streams(info: *mut GstDiscovererInfo) -> *mut glib::GList;
    pub fn gst_discoverer_info_get_container_streams(info: *mut GstDiscovererInfo) -> *mut glib::GList;
    pub fn gst_discoverer_info_get_duration(info: *const GstDiscovererInfo) -> gst::GstClockTime;
    pub fn gst_discoverer_info_get_misc(info: *const GstDiscovererInfo) -> *const gst::GstStructure;
    #[cfg(feature = "v1_4")]
    pub fn gst_discoverer_info_get_missing_elements_installer_details(info: *const GstDiscovererInfo) -> *mut *mut c_char;
    pub fn gst_discoverer_info_get_result(info: *const GstDiscovererInfo) -> GstDiscovererResult;
    pub fn gst_discoverer_info_get_seekable(info: *const GstDiscovererInfo) -> gboolean;
    pub fn gst_discoverer_info_get_stream_info(info: *mut GstDiscovererInfo) -> *mut GstDiscovererStreamInfo;
    pub fn gst_discoverer_info_get_stream_list(info: *mut GstDiscovererInfo) -> *mut glib::GList;
    pub fn gst_discoverer_info_get_streams(info: *mut GstDiscovererInfo, streamtype: GType) -> *mut glib::GList;
    pub fn gst_discoverer_info_get_subtitle_streams(info: *mut GstDiscovererInfo) -> *mut glib::GList;
    pub fn gst_discoverer_info_get_tags(info: *const GstDiscovererInfo) -> *const gst::GstTagList;
    pub fn gst_discoverer_info_get_toc(info: *const GstDiscovererInfo) -> *const gst::GstToc;
    pub fn gst_discoverer_info_get_uri(info: *const GstDiscovererInfo) -> *const c_char;
    pub fn gst_discoverer_info_get_video_streams(info: *mut GstDiscovererInfo) -> *mut glib::GList;
    #[cfg(feature = "v1_6")]
    pub fn gst_discoverer_info_to_variant(info: *mut GstDiscovererInfo, flags: GstDiscovererSerializeFlags) -> *mut glib::GVariant;

    //=========================================================================
    // GstDiscovererStreamInfo
    //=========================================================================
    pub fn gst_discoverer_stream_info_get_type() -> GType;
    pub fn gst_discoverer_stream_info_list_free(infos: *mut glib::GList);
    pub fn gst_discoverer_stream_info_get_caps(info: *mut GstDiscovererStreamInfo) -> *mut gst::GstCaps;
    pub fn gst_discoverer_stream_info_get_misc(info: *mut GstDiscovererStreamInfo) -> *const gst::GstStructure;
    pub fn gst_discoverer_stream_info_get_next(info: *mut GstDiscovererStreamInfo) -> *mut GstDiscovererStreamInfo;
    pub fn gst_discoverer_stream_info_get_previous(info: *mut GstDiscovererStreamInfo) -> *mut GstDiscovererStreamInfo;
    pub fn gst_discoverer_stream_info_get_stream_id(info: *mut GstDiscovererStreamInfo) -> *const c_char;
    pub fn gst_discoverer_stream_info_get_stream_type_nick(info: *mut GstDiscovererStreamInfo) -> *const c_char;
    pub fn gst_discoverer_stream_info_get_tags(info: *mut GstDiscovererStreamInfo) -> *const gst::GstTagList;
    pub fn gst_discoverer_stream_info_get_toc(info: *mut GstDiscovererStreamInfo) -> *const gst::GstToc;

    //=========================================================================
    // GstDiscovererSubtitleInfo
    //=========================================================================
    pub fn gst_discoverer_subtitle_info_get_type() -> GType;
    pub fn gst_discoverer_subtitle_info_get_language(info: *const GstDiscovererSubtitleInfo) -> *const c_char;

    //=========================================================================
    // GstDiscovererVideoInfo
    //=========================================================================
    pub fn gst_discoverer_video_info_get_type() -> GType;
    pub fn gst_discoverer_video_info_get_bitrate(info: *const GstDiscovererVideoInfo) -> c_uint;
    pub fn gst_discoverer_video_info_get_depth(info: *const GstDiscovererVideoInfo) -> c_uint;
    pub fn gst_discoverer_video_info_get_framerate_denom(info: *const GstDiscovererVideoInfo) -> c_uint;
    pub fn gst_discoverer_video_info_get_framerate_num(info: *const GstDiscovererVideoInfo) -> c_uint;
    pub fn gst_discoverer_video_info_get_height(info: *const GstDiscovererVideoInfo) -> c_uint;
    pub fn gst_discoverer_video_info_get_max_bitrate(info: *const GstDiscovererVideoInfo) -> c_uint;
    pub fn gst_discoverer_video_info_get_par_denom(info: *const GstDiscovererVideoInfo) -> c_uint;
    pub fn gst_discoverer_video_info_get_par_num(info: *const GstDiscovererVideoInfo) -> c_uint;
    pub fn gst_discoverer_video_info_get_width(info: *const GstDiscovererVideoInfo) -> c_uint;
    pub fn gst_discoverer_video_info_is_image(info: *const GstDiscovererVideoInfo) -> gboolean;
    pub fn gst_discoverer_video_info_is_interlaced(info: *const GstDiscovererVideoInfo) -> gboolean;

    //=========================================================================
    // GstEncodingAudioProfile
    //=========================================================================
    pub fn gst_encoding_audio_profile_get_type() -> GType;
    pub fn gst_encoding_audio_profile_new(format: *mut gst::GstCaps, preset: *const c_char, restriction: *mut gst::GstCaps, presence: c_uint) -> *mut GstEncodingAudioProfile;

    //=========================================================================
    // GstEncodingContainerProfile
    //=========================================================================
    pub fn gst_encoding_container_profile_get_type() -> GType;
    pub fn gst_encoding_container_profile_new(name: *const c_char, description: *const c_char, format: *mut gst::GstCaps, preset: *const c_char) -> *mut GstEncodingContainerProfile;
    pub fn gst_encoding_container_profile_add_profile(container: *mut GstEncodingContainerProfile, profile: *mut GstEncodingProfile) -> gboolean;
    pub fn gst_encoding_container_profile_contains_profile(container: *mut GstEncodingContainerProfile, profile: *mut GstEncodingProfile) -> gboolean;
    pub fn gst_encoding_container_profile_get_profiles(profile: *mut GstEncodingContainerProfile) -> *const glib::GList;

    //=========================================================================
    // GstEncodingProfile
    //=========================================================================
    pub fn gst_encoding_profile_get_type() -> GType;
    pub fn gst_encoding_profile_find(targetname: *const c_char, profilename: *const c_char, category: *const c_char) -> *mut GstEncodingProfile;
    pub fn gst_encoding_profile_from_discoverer(info: *mut GstDiscovererInfo) -> *mut GstEncodingProfile;
    pub fn gst_encoding_profile_copy(self_: *mut GstEncodingProfile) -> *mut GstEncodingProfile;
    pub fn gst_encoding_profile_get_allow_dynamic_output(profile: *mut GstEncodingProfile) -> gboolean;
    pub fn gst_encoding_profile_get_description(profile: *mut GstEncodingProfile) -> *const c_char;
    pub fn gst_encoding_profile_get_file_extension(profile: *mut GstEncodingProfile) -> *const c_char;
    pub fn gst_encoding_profile_get_format(profile: *mut GstEncodingProfile) -> *mut gst::GstCaps;
    pub fn gst_encoding_profile_get_input_caps(profile: *mut GstEncodingProfile) -> *mut gst::GstCaps;
    pub fn gst_encoding_profile_get_name(profile: *mut GstEncodingProfile) -> *const c_char;
    pub fn gst_encoding_profile_get_presence(profile: *mut GstEncodingProfile) -> c_uint;
    pub fn gst_encoding_profile_get_preset(profile: *mut GstEncodingProfile) -> *const c_char;
    pub fn gst_encoding_profile_get_preset_name(profile: *mut GstEncodingProfile) -> *const c_char;
    pub fn gst_encoding_profile_get_restriction(profile: *mut GstEncodingProfile) -> *mut gst::GstCaps;
    pub fn gst_encoding_profile_get_type_nick(profile: *mut GstEncodingProfile) -> *const c_char;
    pub fn gst_encoding_profile_is_enabled(profile: *mut GstEncodingProfile) -> gboolean;
    pub fn gst_encoding_profile_is_equal(a: *mut GstEncodingProfile, b: *mut GstEncodingProfile) -> gboolean;
    pub fn gst_encoding_profile_set_allow_dynamic_output(profile: *mut GstEncodingProfile, allow_dynamic_output: gboolean);
    pub fn gst_encoding_profile_set_description(profile: *mut GstEncodingProfile, description: *const c_char);
    pub fn gst_encoding_profile_set_enabled(profile: *mut GstEncodingProfile, enabled: gboolean);
    pub fn gst_encoding_profile_set_format(profile: *mut GstEncodingProfile, format: *mut gst::GstCaps);
    pub fn gst_encoding_profile_set_name(profile: *mut GstEncodingProfile, name: *const c_char);
    pub fn gst_encoding_profile_set_presence(profile: *mut GstEncodingProfile, presence: c_uint);
    pub fn gst_encoding_profile_set_preset(profile: *mut GstEncodingProfile, preset: *const c_char);
    pub fn gst_encoding_profile_set_preset_name(profile: *mut GstEncodingProfile, preset_name: *const c_char);
    pub fn gst_encoding_profile_set_restriction(profile: *mut GstEncodingProfile, restriction: *mut gst::GstCaps);

    //=========================================================================
    // GstEncodingTarget
    //=========================================================================
    pub fn gst_encoding_target_get_type() -> GType;
    pub fn gst_encoding_target_new(name: *const c_char, category: *const c_char, description: *const c_char, profiles: *const glib::GList) -> *mut GstEncodingTarget;
    pub fn gst_encoding_target_load(name: *const c_char, category: *const c_char, error: *mut *mut glib::GError) -> *mut GstEncodingTarget;
    pub fn gst_encoding_target_load_from_file(filepath: *const c_char, error: *mut *mut glib::GError) -> *mut GstEncodingTarget;
    pub fn gst_encoding_target_add_profile(target: *mut GstEncodingTarget, profile: *mut GstEncodingProfile) -> gboolean;
    pub fn gst_encoding_target_get_category(target: *mut GstEncodingTarget) -> *const c_char;
    pub fn gst_encoding_target_get_description(target: *mut GstEncodingTarget) -> *const c_char;
    pub fn gst_encoding_target_get_name(target: *mut GstEncodingTarget) -> *const c_char;
    pub fn gst_encoding_target_get_profile(target: *mut GstEncodingTarget, name: *const c_char) -> *mut GstEncodingProfile;
    pub fn gst_encoding_target_get_profiles(target: *mut GstEncodingTarget) -> *const glib::GList;
    pub fn gst_encoding_target_save(target: *mut GstEncodingTarget, error: *mut *mut glib::GError) -> gboolean;
    pub fn gst_encoding_target_save_to_file(target: *mut GstEncodingTarget, filepath: *const c_char, error: *mut *mut glib::GError) -> gboolean;

    //=========================================================================
    // GstEncodingVideoProfile
    //=========================================================================
    pub fn gst_encoding_video_profile_get_type() -> GType;
    pub fn gst_encoding_video_profile_new(format: *mut gst::GstCaps, preset: *const c_char, restriction: *mut gst::GstCaps, presence: c_uint) -> *mut GstEncodingVideoProfile;
    pub fn gst_encoding_video_profile_get_pass(prof: *mut GstEncodingVideoProfile) -> c_uint;
    pub fn gst_encoding_video_profile_get_variableframerate(prof: *mut GstEncodingVideoProfile) -> gboolean;
    pub fn gst_encoding_video_profile_set_pass(prof: *mut GstEncodingVideoProfile, pass: c_uint);
    pub fn gst_encoding_video_profile_set_variableframerate(prof: *mut GstEncodingVideoProfile, variableframerate: gboolean);

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn gst_codec_utils_aac_caps_set_level_and_profile(caps: *mut gst::GstCaps, audio_config: *const u8, len: c_uint) -> gboolean;
    pub fn gst_codec_utils_aac_get_channels(audio_config: *const u8, len: c_uint) -> c_uint;
    pub fn gst_codec_utils_aac_get_index_from_sample_rate(rate: c_uint) -> c_int;
    pub fn gst_codec_utils_aac_get_level(audio_config: *const u8, len: c_uint) -> *const c_char;
    pub fn gst_codec_utils_aac_get_profile(audio_config: *const u8, len: c_uint) -> *const c_char;
    pub fn gst_codec_utils_aac_get_sample_rate(audio_config: *const u8, len: c_uint) -> c_uint;
    pub fn gst_codec_utils_aac_get_sample_rate_from_index(sr_idx: c_uint) -> c_uint;
    pub fn gst_codec_utils_h264_caps_set_level_and_profile(caps: *mut gst::GstCaps, sps: *const u8, len: c_uint) -> gboolean;
    pub fn gst_codec_utils_h264_get_level(sps: *const u8, len: c_uint) -> *const c_char;
    pub fn gst_codec_utils_h264_get_level_idc(level: *const c_char) -> u8;
    pub fn gst_codec_utils_h264_get_profile(sps: *const u8, len: c_uint) -> *const c_char;
    pub fn gst_codec_utils_h265_caps_set_level_tier_and_profile(caps: *mut gst::GstCaps, profile_tier_level: *const u8, len: c_uint) -> gboolean;
    pub fn gst_codec_utils_h265_get_level(profile_tier_level: *const u8, len: c_uint) -> *const c_char;
    pub fn gst_codec_utils_h265_get_level_idc(level: *const c_char) -> u8;
    pub fn gst_codec_utils_h265_get_profile(profile_tier_level: *const u8, len: c_uint) -> *const c_char;
    pub fn gst_codec_utils_h265_get_tier(profile_tier_level: *const u8, len: c_uint) -> *const c_char;
    pub fn gst_codec_utils_mpeg4video_caps_set_level_and_profile(caps: *mut gst::GstCaps, vis_obj_seq: *const u8, len: c_uint) -> gboolean;
    pub fn gst_codec_utils_mpeg4video_get_level(vis_obj_seq: *const u8, len: c_uint) -> *const c_char;
    pub fn gst_codec_utils_mpeg4video_get_profile(vis_obj_seq: *const u8, len: c_uint) -> *const c_char;
    #[cfg(feature = "v1_8")]
    pub fn gst_codec_utils_opus_create_caps(rate: u32, channels: u8, channel_mapping_family: u8, stream_count: u8, coupled_count: u8, channel_mapping: *const u8) -> *mut gst::GstCaps;
    #[cfg(feature = "v1_8")]
    pub fn gst_codec_utils_opus_create_caps_from_header(header: *mut gst::GstBuffer, comments: *mut gst::GstBuffer) -> *mut gst::GstCaps;
    #[cfg(feature = "v1_8")]
    pub fn gst_codec_utils_opus_create_header(rate: u32, channels: u8, channel_mapping_family: u8, stream_count: u8, coupled_count: u8, channel_mapping: *const u8, pre_skip: u16, output_gain: i16) -> *mut gst::GstBuffer;
    #[cfg(feature = "v1_8")]
    pub fn gst_codec_utils_opus_parse_caps(caps: *mut gst::GstCaps, rate: *mut u32, channels: *mut u8, channel_mapping_family: *mut u8, stream_count: *mut u8, coupled_count: *mut u8, channel_mapping: u8) -> gboolean;
    #[cfg(feature = "v1_8")]
    pub fn gst_codec_utils_opus_parse_header(header: *mut gst::GstBuffer, rate: *mut u32, channels: *mut u8, channel_mapping_family: *mut u8, stream_count: *mut u8, coupled_count: *mut u8, channel_mapping: u8, pre_skip: *mut u16, output_gain: *mut i16) -> gboolean;
    pub fn gst_encoding_list_all_targets(categoryname: *const c_char) -> *mut glib::GList;
    pub fn gst_encoding_list_available_categories() -> *mut glib::GList;
    pub fn gst_install_plugins_async(details: *mut *mut c_char, ctx: *mut GstInstallPluginsContext, func: GstInstallPluginsResultFunc, user_data: gpointer) -> GstInstallPluginsReturn;
    pub fn gst_install_plugins_installation_in_progress() -> gboolean;
    pub fn gst_install_plugins_supported() -> gboolean;
    pub fn gst_install_plugins_sync(details: *mut *mut c_char, ctx: *mut GstInstallPluginsContext) -> GstInstallPluginsReturn;
    pub fn gst_is_missing_plugin_message(msg: *mut gst::GstMessage) -> gboolean;
    pub fn gst_missing_decoder_installer_detail_new(decode_caps: *const gst::GstCaps) -> *mut c_char;
    pub fn gst_missing_decoder_message_new(element: *mut gst::GstElement, decode_caps: *const gst::GstCaps) -> *mut gst::GstMessage;
    pub fn gst_missing_element_installer_detail_new(factory_name: *const c_char) -> *mut c_char;
    pub fn gst_missing_element_message_new(element: *mut gst::GstElement, factory_name: *const c_char) -> *mut gst::GstMessage;
    pub fn gst_missing_encoder_installer_detail_new(encode_caps: *const gst::GstCaps) -> *mut c_char;
    pub fn gst_missing_encoder_message_new(element: *mut gst::GstElement, encode_caps: *const gst::GstCaps) -> *mut gst::GstMessage;
    pub fn gst_missing_plugin_message_get_description(msg: *mut gst::GstMessage) -> *mut c_char;
    pub fn gst_missing_plugin_message_get_installer_detail(msg: *mut gst::GstMessage) -> *mut c_char;
    pub fn gst_missing_uri_sink_installer_detail_new(protocol: *const c_char) -> *mut c_char;
    pub fn gst_missing_uri_sink_message_new(element: *mut gst::GstElement, protocol: *const c_char) -> *mut gst::GstMessage;
    pub fn gst_missing_uri_source_installer_detail_new(protocol: *const c_char) -> *mut c_char;
    pub fn gst_missing_uri_source_message_new(element: *mut gst::GstElement, protocol: *const c_char) -> *mut gst::GstMessage;
    pub fn gst_pb_utils_add_codec_description_to_tag_list(taglist: *mut gst::GstTagList, codec_tag: *const c_char, caps: *const gst::GstCaps) -> gboolean;
    pub fn gst_pb_utils_get_codec_description(caps: *const gst::GstCaps) -> *mut c_char;
    pub fn gst_pb_utils_get_decoder_description(caps: *const gst::GstCaps) -> *mut c_char;
    pub fn gst_pb_utils_get_element_description(factory_name: *const c_char) -> *mut c_char;
    pub fn gst_pb_utils_get_encoder_description(caps: *const gst::GstCaps) -> *mut c_char;
    pub fn gst_pb_utils_get_sink_description(protocol: *const c_char) -> *mut c_char;
    pub fn gst_pb_utils_get_source_description(protocol: *const c_char) -> *mut c_char;
    pub fn gst_pb_utils_init();
    pub fn gst_plugins_base_version(major: *mut c_uint, minor: *mut c_uint, micro: *mut c_uint, nano: *mut c_uint);
    pub fn gst_plugins_base_version_string() -> *mut c_char;

}
