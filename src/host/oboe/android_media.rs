use std::sync::Arc;

// extern crate jni;
// extern crate ndk_glue;

// use self::jni::Executor;
// use self::jni::{errors::Result as JResult, objects::JObject, JNIEnv, JavaVM};

// constants from android.media.AudioFormat
pub const ENCODING_PCM_16BIT: i32 = 2;
pub const ENCODING_PCM_FLOAT: i32 = 4;
pub const CHANNEL_OUT_MONO: i32 = 4;
pub const CHANNEL_OUT_STEREO: i32 = 12;

// fn with_attached<F, R>(closure: F) -> JResult<R>
// where
//     F: FnOnce(&JNIEnv, JObject) -> JResult<R>,
// {
//     let activity = ndk_glue::native_activity();
//     let vm = Arc::new(unsafe { JavaVM::from_raw(activity.vm())? });
//     let activity = activity.activity();
//     Executor::new(vm).with_attached(|env| closure(env, activity.into()))
// }

fn get_min_buffer_size(
    class: &'static str,
    sample_rate: i32,
    channel_mask: i32,
    format: i32,
) -> i32 {
    // Unwrapping everything because these operations are not expected to fail
    // or throw exceptions. Android returns negative values for invalid parameters,
    // which is what we expect.
    // with_attached(|env, _activity| {
    //     let class = env.find_class(class).unwrap();
    //     env.call_static_method(
    //         class,
    //         "getMinBufferSize",
    //         "(III)I",
    //         &[sample_rate.into(), channel_mask.into(), format.into()],
    //     )
    //     .unwrap()
    //     .i()
    // })
    // .unwrap()
    0
}

pub fn get_audio_track_min_buffer_size(sample_rate: i32, channel_mask: i32, format: i32) -> i32 {
    get_min_buffer_size(
        "android/media/AudioTrack",
        sample_rate,
        channel_mask,
        format,
    )
}

pub fn get_audio_record_min_buffer_size(sample_rate: i32, channel_mask: i32, format: i32) -> i32 {
    get_min_buffer_size(
        "android/media/AudioRecord",
        sample_rate,
        channel_mask,
        format,
    )
}
