/*
* Rust-FMOD - Copyright (c) 2014 Gomez Guillaume.
*
* The Original software, FmodEx library, is provided by FIRELIGHT TECHNOLOGIES.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

use libc::{c_void, c_uint, c_int, c_char, c_float, c_short};
use ffi;
use types::*;
use enums::*;
use sound;
use sound::Sound;
use sound_group;
use std::mem;
use channel_group;
use channel;
use dsp;
use dsp::Dsp;
use vector;
use reverb_properties;
use geometry;
use reverb;
use dsp_connection;
use std::default::Default;
use callbacks::*;
use std::c_vec::CVec;
use std::string;

extern "C" fn pcm_read_callback(sound: *mut ffi::FMOD_SOUND, data: *mut c_void, data_len: c_uint) -> fmod::Result {
    unsafe {
        if sound.is_not_null() {
            let mut tmp = ::std::ptr::mut_null();

            ffi::FMOD_Sound_GetUserData(sound, &mut tmp);
            if tmp.is_not_null() {
                let callbacks : &mut ffi::SoundData = ::std::mem::transmute(tmp);

                match callbacks.pcm_read {
                    Some(p) => {
                        let max = data_len as int >> 2;
                        let mut data_vec = CVec::new(data as *mut c_short, max as uint * 2);

                        let ret = p(&sound::from_ptr(sound), data_vec.as_mut_slice());
                        ret
                    },
                    None => fmod::Ok
                }
            } else {
                fmod::Ok
            }
        } else {
            fmod::Ok
        }
    }
}

extern "C" fn non_block_callback(sound: *mut ffi::FMOD_SOUND, result: fmod::Result) -> fmod::Result {
    unsafe {
        if sound.is_not_null() {
            let mut tmp = ::std::ptr::mut_null();

            ffi::FMOD_Sound_GetUserData(sound, &mut tmp);
            if tmp.is_not_null() {
                let callbacks : &mut ffi::SoundData = ::std::mem::transmute(tmp);

                match callbacks.non_block {
                    Some(p) => p(&sound::from_ptr(sound), result),
                    None => fmod::Ok
                }
            } else {
                fmod::Ok
            }
        } else {
            fmod::Ok
        }
    }
}

extern "C" fn pcm_set_pos_callback(sound: *mut ffi::FMOD_SOUND, sub_sound: c_int, position: c_uint, postype: ffi::FMOD_TIMEUNIT) -> fmod::Result {
    unsafe {
        if sound.is_not_null() {
            let mut tmp = ::std::ptr::mut_null();

            ffi::FMOD_Sound_GetUserData(sound, &mut tmp);
            if tmp.is_not_null() {
                let callbacks : &mut ffi::SoundData = ::std::mem::transmute(tmp);

                match callbacks.pcm_set_pos {
                    Some(p) => p(&sound::from_ptr(sound), sub_sound, position, FmodTimeUnit(postype)),
                    None => fmod::Ok
                }
            } else {
                fmod::Ok
            }
        } else {
            fmod::Ok
        }
    }
}

/// Structure describing a globally unique identifier.
pub struct FmodGuid
{
    /// Specifies the first 8 hexadecimal digits of the GUID
    pub data1: u32,
    /// Specifies the first group of 4 hexadecimal digits.
    pub data2: u16,
    /// Specifies the second group of 4 hexadecimal digits.
    pub data3: u16,
    /// Array of 8 bytes. The first 2 bytes contain the third group of 4 hexadecimal digits. The remaining 6 bytes contain the final 12 hexadecimal digits.
    pub data4: [u8, ..8]
}

impl Default for FmodGuid {
    fn default() -> FmodGuid {
        FmodGuid {
            data1: 0u32,
            data2: 0u16,
            data3: 0u16,
            data4: [0u8, ..8]
        }
    }
}

/// Wrapper for arguments of [`FmodSys::set_software_format`](struct.FmodSys.html#method.set_software_format) and [`FmodSys::get_software_format`](struct.FmodSys.html#method.get_software_format)
pub struct FmodSoftwareFormat
{
    pub sample_rate        : i32,
    pub format             : fmod::SoundFormat,
    pub num_output_channels: i32,
    pub max_input_channels : i32,
    pub resample_method    : fmod::DSPResampler,
    pub bits               : i32
}

impl Default for FmodSoftwareFormat {
    fn default() -> FmodSoftwareFormat {
        FmodSoftwareFormat {
            sample_rate: 0i32,
            format: fmod::SoundFormatNone,
            num_output_channels: 0i32,
            max_input_channels: 0i32,
            resample_method: fmod::DSPResamplerNoInterp,
            bits: 0i32
        }
    }
}

/// Settings for advanced features like configuring memory and cpu usage for the FMOD_CREATECOMPRESSEDSAMPLE feature.
pub struct FmodAdvancedSettings
{
    /// [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only. Mpeg codecs consume 21,684 bytes per instance and this number will determine how many mpeg channels can be played simultaneously. Default = 32.
    pub max_MPEG_codecs               : i32,
    /// [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only. ADPCM codecs consume 2,136 bytes per instance and this number will determine how many ADPCM channels can be played simultaneously. Default = 32.
    pub max_ADPCM_codecs              : i32,
    /// [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only. XMA codecs consume 14,836 bytes per instance and this number will determine how many XMA channels can be played simultaneously. Default = 32.
    pub max_XMA_codecs                : i32,
    /// [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only. CELT codecs consume 11,500 bytes per instance and this number will determine how many CELT channels can be played simultaneously. Default = 32.
    pub max_CELT_codecs               : i32,
    /// [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only. Vorbis codecs consume 12,000 bytes per instance and this number will determine how many Vorbis channels can be played simultaneously. Default = 32.
    pub max_VORBIS_codecs             : i32,
    /// [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only. AT9 codecs consume 8,720 bytes per instance and this number will determine how many AT9 channels can be played simultaneously. Default = 32.
    pub max_AT9_codecs                : i32,
    /// [r/w] Optional. Specify 0 to ignore. For use with PS3 only. PCM codecs consume 12,672 bytes per instance and this number will determine how many streams and PCM voices can be played simultaneously. Default = 16.
    pub max_PCM_codecs                : i32,
    /// [r/w] Optional. Specify 0 to ignore. Number of channels available on the ASIO device.
    pub ASIO_num_channels             : i32,
    /// [r/w] Optional. Specify 0 to ignore. Pointer to an array of strings (number of entries defined by ASIONumChannels) with ASIO channel names.
    pub ASIO_channel_list             : Vec<String>,
    /// [r/w] Optional. Specify 0 to ignore. Pointer to a list of speakers that the ASIO channels map to. This can be called after [`FmodSys::init`](doc/rfmod/struct.FmodSys.html#method.init) to remap ASIO output.
    pub ASIO_speaker_list             : Vec<fmod::Speaker>,
    /// [r/w] Optional. Specify 0 to ignore. The max number of 3d reverb DSP's in the system. (NOTE: CURRENTLY DISABLED / UNUSED)
    pub max_3D_reverb_DSPs            : i32,
    /// [r/w] Optional. For use with FMOD_INIT_HRTF_LOWPASS. The angle range (0-360) of a 3D sound in relation to the listener, at which the HRTF function begins to have an effect. 0 = in front of the listener. 180 = from 90 degrees to the left of the listener to 90 degrees to the right. 360 = behind the listener. Default = 180.0.
    pub HRTF_min_angle                : f32,
    /// [r/w] Optional. For use with FMOD_INIT_HRTF_LOWPASS. The angle range (0-360) of a 3D sound in relation to the listener, at which the HRTF function has maximum effect. 0 = front of the listener. 180 = from 90 degrees to the left of the listener to 90 degrees to the right. 360 = behind the listener. Default = 360.0.
    pub HRTF_max_angle                : f32,
    /// [r/w] Optional. Specify 0 to ignore. For use with FMOD_INIT_HRTF_LOWPASS. The cutoff frequency of the HRTF's lowpass filter function when at maximum effect. (i.e. at HRTFMaxAngle). Default = 4000.0.
    pub HRTF_freq                     : f32,
    /// [r/w] Optional. Specify 0 to ignore. For use with FMOD_INIT_VOL0_BECOMES_VIRTUAL. If this flag is used, and the volume is 0.0, then the sound will become virtual. Use this value to raise the threshold to a different point where a sound goes virtual.
    pub vol0_virtual_vol              : f32,
    /// [r/w] Optional. Specify 0 to ignore. For use with FMOD Event system only. Specifies the number of slots available for simultaneous non blocking loads, across all threads. Default = 32.
    pub event_queue_size              : i32,
    /// [r/w] Optional. Specify 0 to ignore. For streams. This determines the default size of the double buffer (in milliseconds) that a stream uses. Default = 400ms
    pub default_decode_buffer_size    : u32,
    /// [r/w] Optional. Specify 0 to ignore. Gives fmod's logging system a path/filename. Normally the log is placed in the same directory as the executable and called fmod.log. When using [`FmodSys::get_advanced_settings`](doc/rfmod/struct.FmodSys.html#method.get_advanced_settings), provide at least 256 bytes of memory to copy into.
    pub debug_log_filename            : String,
    /// [r/w] Optional. Specify 0 to ignore. For use with FMOD_INIT_ENABLE_PROFILE. Specify the port to listen on for connections by the profiler application.
    pub profile_port                  : u16,
    /// [r/w] Optional. Specify 0 to ignore. The maximum time in miliseconds it takes for a channel to fade to the new level when its occlusion changes.
    pub geometry_max_fade_time        : u32,
    /// [r/w] Optional. Specify 0 to ignore. Tells [`FmodSys::init`](doc/rfmod/struct.FmodSys.html#method.init) to allocate a pool of wavedata/spectrum buffers to prevent memory fragmentation, any additional buffers will be allocated normally.
    pub max_spectrum_wave_data_buffers: u32,
    /// [r/w] Optional. Specify 0 to ignore. The delay the music system should allow for loading a sample from disk (in milliseconds). Default = 400 ms.
    pub music_system_cache_delay      : u32,
    /// [r/w] Optional. Specify 0 to ignore. For use with FMOD_INIT_DISTANCE_FILTERING. The default center frequency in Hz for the distance filtering effect. Default = 1500.0.
    pub distance_filter_center_freq   : f32,
    /// [r/w] Optional. Specify 0 to ignore. Specify the stack size for the FMOD Stream thread in bytes. Useful for custom codecs that use excess stack. Default 49,152 (48kb)
    pub stack_size_stream             : u32,
    /// [r/w] Optional. Specify 0 to ignore. Specify the stack size for the FMOD_NONBLOCKING loading thread. Useful for custom codecs that use excess stack. Default 65,536 (64kb)
    pub stack_size_non_blocking       : u32,
    /// [r/w] Optional. Specify 0 to ignore. Specify the stack size for the FMOD mixer thread. Useful for custom dsps that use excess stack. Default 49,152 (48kb)
    pub stack_size_mixer              : u32
}

impl Default for FmodAdvancedSettings {
    fn default() -> FmodAdvancedSettings {
        FmodAdvancedSettings {
            max_MPEG_codecs: 32i32,
            max_ADPCM_codecs: 32i32,
            max_XMA_codecs: 32i32,
            max_CELT_codecs: 32i32,
            max_VORBIS_codecs: 32i32,
            max_AT9_codecs: 32i32,
            max_PCM_codecs: 16i32,
            ASIO_num_channels: 0i32,
            ASIO_channel_list: Vec::new(),
            ASIO_speaker_list: Vec::new(),
            max_3D_reverb_DSPs: 0i32,
            HRTF_min_angle: 180f32,
            HRTF_max_angle: 360f32,
            HRTF_freq: 4000f32,
            vol0_virtual_vol: 0f32,
            event_queue_size: 32i32,
            default_decode_buffer_size: 400u32,
            debug_log_filename: String::new(),
            profile_port: 0u16,
            geometry_max_fade_time: 0u32,
            max_spectrum_wave_data_buffers: 0u32,
            music_system_cache_delay: 400u32,
            distance_filter_center_freq: 1500f32,
            stack_size_stream: 49152u32,
            stack_size_non_blocking: 65536u32,
            stack_size_mixer: 49152u32
        }
    }
}

/// Use this structure with [`FmodSys::create_sound`](struct.FmodSys.html#method.create_sound) when more control is needed over loading.
/// The possible reasons to use this with [`FmodSys::create_sound`](struct.FmodSys.html#method.create_sound) are:
///
/// * Loading a file from memory.
/// * Loading a file from within another larger (possibly wad/pak) file, by giving the loader an offset and length.
/// * To create a user created / non file based sound.
/// * To specify a starting subsound to seek to within a multi-sample sounds (ie FSB/DLS/SF2) when created as a stream.
/// * To specify which subsounds to load for multi-sample sounds (ie FSB/DLS/SF2) so that memory is saved and only a subset is actually loaded/read from disk.
/// * To specify 'piggyback' read and seek callbacks for capture of sound data as fmod reads and decodes it. Useful for ripping decoded PCM data from sounds as they are loaded / played.
/// * To specify a MIDI DLS/SF2 sample set file to load when opening a MIDI file.
///
/// See below on what members to fill for each of the above types of sound you want to create.
pub struct FmodCreateSoundexInfo
{
    /// [w] Optional. Specify 0 to ignore. Size in bytes of file to load, or sound to create (in this case only if FMOD_OPENUSER is used). Required if loading from memory. If 0 is specified, then it will use the size of the file (unless loading from memory then an error will be returned).
    pub length                 : u32,
    /// [w] Optional. Specify 0 to ignore. Offset from start of the file to start loading from. This is useful for loading files from inside big data files.
    pub file_offset            : u32,
    /// [w] Optional. Specify 0 to ignore. Number of channels in a sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used.
    pub num_channels           : i32,
    /// [w] Optional. Specify 0 to ignore. Default frequency of sound in a sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used. Other formats use the frequency determined by the file format.
    pub default_frequency      : i32,
    /// [w] Optional. Specify 0 or fmod::SoundFormatNone to ignore. Format of the sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used. Other formats use the format determined by the file format.
    pub format                 : fmod::SoundFormat,
    /// [w] Optional. Specify 0 to ignore. For streams. This determines the size of the double buffer (in PCM samples) that a stream uses. Use this for user created streams if you want to determine the size of the callback buffer passed to you. Specify 0 to use FMOD's default size which is currently equivalent to 400ms of the sound format created/loaded.
    pub decode_buffer_size     : u32,
    /// [w] Optional. Specify 0 to ignore. In a multi-sample file format such as .FSB/.DLS/.SF2, specify the initial subsound to seek to, only if FMOD_CREATESTREAM is used.
    pub initial_subsound       : i32,
    /// [w] Optional. Specify 0 to ignore or have no subsounds. In a sound created with FMOD_OPENUSER, specify the number of subsounds that are accessable with [`Sound::get_sub_sound`](doc/rfmod/struct.Sound.html#method.get_sub_sound). If not created with FMOD_OPENUSER, this will limit the number of subsounds loaded within a multi-subsound file. If using FSB, then if FMOD_CREATESOUNDEXINFO::inclusionlist is used, this will shuffle subsounds down so that there are not any gaps. It will mean that the indices of the sounds will be different.
    pub num_subsounds          : i32,
    /// [w] Optional. Specify 0 to ignore. In a multi-sample format such as .FSB/.DLS/.SF2 it may be desirable to specify only a subset of sounds to be loaded out of the whole file. This is an array of subsound indices to load into memory when created.
    pub inclusion_list         : Vec<i32>,
    /// [w] Optional. Specify 0 to ignore. Callback to 'piggyback' on FMOD's read functions and accept or even write PCM data while FMOD is opening the sound. Used for user sounds created with FMOD_OPENUSER or for capturing decoded data as FMOD reads it.
    pub pcm_read_callback      : SoundPcmReadCallback,
    /// [w] Optional. Specify 0 to ignore. Callback for when the user calls a seeking function such as [`Channel::set_time`](doc/rfmod/struct.Channel.html#method.set_time) or [`Channel::set_position`](doc/rfmod/struct.Channel.html#method.set_position) within a multi-sample sound, and for when it is opened.
    pub pcm_set_pos_callback   : SoundPcmSetPosCallback,
    /// [w] Optional. Specify 0 to ignore. Callback for successful completion, or error while loading a sound that used the FMOD_NONBLOCKING flag. Also called duing seeking, when setPosition is called or a stream is restarted.
    pub non_block_callback     : SoundNonBlockCallback,
    /// [w] Optional. Specify 0 to ignore. Filename for a DLS or SF2 sample set when loading a MIDI file. If not specified, on Windows it will attempt to open /windows/system32/drivers/gm.dls or /windows/system32/drivers/etc/gm.dls, on Mac it will attempt to load /System/Library/Components/CoreAudio.component/Contents/Resources/gs_instruments.dls, otherwise the MIDI will fail to open. Current DLS support is for level 1 of the specification.
    pub dls_name               : String,
    /// [w] Optional. Specify 0 to ignore. Key for encrypted FSB file. Without this key an encrypted FSB file will not load.
    pub encryption_key         : String,
    /// [w] Optional. Specify 0 to ignore. For sequenced formats with dynamic channel allocation such as .MID and .IT, this specifies the maximum voice count allowed while playing. .IT defaults to 64. .MID defaults to 32.
    pub max_polyphony          : i32,
    /// [w] Optional. Specify 0 to ignore. This is user data to be attached to the sound during creation. Access via [`Sound::get_user_data`](doc/rfmod/struct.Sound.html#method.get_user_data). Note: This is not passed to FMOD_FILE_OPENCALLBACK, that is a different userdata that is file specific.
    user_data                  : Box<ffi::SoundData>,
    /// [w] Optional. Specify 0 or fmod::SoundTypeUnknown to ignore. Instead of scanning all codec types, use this to speed up loading by making it jump straight to this codec.
    pub suggested_sound_type   : fmod::SoundType,
    /// [w] Optional. Specify 0 to ignore. Callback for opening this file.
    user_open                  : ffi::FMOD_FILE_OPENCALLBACK,
    /// [w] Optional. Specify 0 to ignore. Callback for closing this file.
    user_close                 : ffi::FMOD_FILE_CLOSECALLBACK,
    /// [w] Optional. Specify 0 to ignore. Callback for reading from this file.
    user_read                  : ffi::FMOD_FILE_READCALLBACK,
    /// [w] Optional. Specify 0 to ignore. Callback for seeking within this file.
    user_seek                  : ffi::FMOD_FILE_SEEKCALLBACK,
    /// [w] Optional. Specify 0 to ignore. Callback for seeking within this file.
    user_async_read            : ffi::FMOD_FILE_ASYNCREADCALLBACK,
    /// [w] Optional. Specify 0 to ignore. Callback for seeking within this file.
    user_async_cancel          : ffi::FMOD_FILE_ASYNCCANCELCALLBACK,
    /// [w] Optional. Specify 0 to ignore. Use this to differ the way fmod maps multichannel sounds to speakers. See fmod::SpeakerMapType for more.
    pub speaker_map            : fmod::SpeakerMapType,
    /// [w] Optional. Specify 0 to ignore. Specify a sound group if required, to put sound in as it is created.
    pub initial_sound_group    : sound_group::SoundGroup,
    /// [w] Optional. Specify 0 to ignore. For streams. Specify an initial position to seek the stream to.
    pub initial_seek_position  : u32,
    /// [w] Optional. Specify 0 to ignore. For streams. Specify the time unit for the position set in initialseekposition.
    pub initial_seek_pos_type  : FmodTimeUnit,
    /// [w] Optional. Specify 0 to ignore. Set to 1 to use fmod's built in file system. Ignores setFileSystem callbacks and also FMOD_CREATESOUNEXINFO file callbacks. Useful for specific cases where you don't want to use your own file system but want to use fmod's file system (ie net streaming).
    pub ignore_set_file_system : i32,
    /// [w] Optional. Specify 0 to ignore. For CDDA sounds only - if non-zero use ASPI instead of NTSCSI to access the specified CD/DVD device.
    pub cdda_force_aspi        : i32,
    /// [w] Optional. Specify 0 or FMOD_AUDIOQUEUE_CODECPOLICY_DEFAULT to ignore. Policy used to determine whether hardware or software is used for decoding, see FMOD_AUDIOQUEUE_CODECPOLICY for options (iOS >= 3.0 required, otherwise only hardware is available)
    pub audio_queue_policy     : u32,
    /// [w] Optional. Specify 0 to ignore. Allows you to set a minimum desired MIDI mixer granularity. Values smaller than 512 give greater than default accuracy at the cost of more CPU and vice versa. Specify 0 for default (512 samples).
    pub min_midi_granularity   : u32,
    /// [w] Optional. Specify 0 to ignore. Specifies a thread index to execute non blocking load on. Allows for up to 5 threads to be used for loading at once. This is to avoid one load blocking another. Maximum value = 4.
    pub non_block_thread_id    : i32
}

impl Default for FmodCreateSoundexInfo {
    fn default() -> FmodCreateSoundexInfo {
        FmodCreateSoundexInfo {
            length: 0u32,
            file_offset: 0u32,
            num_channels: 0i32,
            default_frequency: 0i32,
            format: fmod::SoundFormatNone,
            decode_buffer_size: 0u32,
            initial_subsound: 0i32,
            num_subsounds: 0i32,
            inclusion_list: Vec::new(),
            pcm_read_callback: None,
            pcm_set_pos_callback: None,
            non_block_callback: None,
            dls_name: String::new(),
            encryption_key: String::new(),
            max_polyphony: 0i32,
            user_data: box ffi::SoundData::new(),
            suggested_sound_type: fmod::SoundTypeUnknown,
            user_open: None,
            user_close: None,
            user_read: None,
            user_seek: None,
            user_async_read: None,
            user_async_cancel: None,
            speaker_map: fmod::SpeakerMapTypeDefault,
            initial_sound_group: sound_group::from_ptr(::std::ptr::mut_null()),
            initial_seek_position: 0u32,
            initial_seek_pos_type: FmodTimeUnit(0u32),
            ignore_set_file_system: 0i32,
            cdda_force_aspi: 0i32,
            audio_queue_policy: 0u32,
            min_midi_granularity: 0u32,
            non_block_thread_id: 0i32
        }
    }
}

impl FmodCreateSoundexInfo {
    fn convert_to_c(&mut self) -> ffi::FMOD_CREATESOUNDEXINFO {
        ffi::FMOD_CREATESOUNDEXINFO{
            cbsize: mem::size_of::<ffi::FMOD_CREATESOUNDEXINFO>() as i32,
            length: self.length,
            fileoffset: self.file_offset,
            numchannels: self.num_channels,
            defaultfrequency: self.default_frequency,
            format: self.format,
            decodebuffersize: self.decode_buffer_size,
            initialsubsound: self.initial_subsound,
            numsubsounds: self.num_subsounds,
            inclusionlist: self.inclusion_list.as_mut_ptr(),
            inclusionlistnum: self.inclusion_list.len() as i32,
            pcmreadcallback: match self.pcm_read_callback {
                Some(_) => Some(pcm_read_callback),
                None => None
            },
            pcmsetposcallback: match self.pcm_set_pos_callback {
                Some(_) => Some(pcm_set_pos_callback),
                None => None
            },
            nonblockcallback: match self.non_block_callback {
                Some(_) => Some(non_block_callback),
                None => None
            },
            dlsname: self.dls_name.clone().with_c_str(|c_str|{c_str as *mut c_char}),
            encryptionkey: self.encryption_key.clone().with_c_str(|c_str|{c_str as *mut c_char}),
            maxpolyphony: self.max_polyphony,
            userdata: {
                self.user_data.non_block = self.non_block_callback;
                self.user_data.pcm_read = self.pcm_read_callback;
                self.user_data.pcm_set_pos = self.pcm_set_pos_callback;
                unsafe { ::std::mem::transmute::<&mut ffi::SoundData, *mut c_void>(&mut *self.user_data) }
            },
            suggestedsoundtype: self.suggested_sound_type,
            useropen: self.user_open,
            userclose: self.user_close,
            userread: self.user_read,
            userseek: self.user_seek,
            userasynccancel: self.user_async_cancel,
            userasyncread: self.user_async_read,
            speakermap: self.speaker_map,
            initialsoundgroup: sound_group::get_ffi(&self.initial_sound_group),
            initialseekposition: self.initial_seek_position,
            initialseekpostype: match self.initial_seek_pos_type {FmodTimeUnit(v) => v},
            ignoresetfilesystem: self.ignore_set_file_system,
            cddaforceaspi: self.cdda_force_aspi,
            audioqueuepolicy: self.audio_queue_policy,
            minmidigranularity: self.min_midi_granularity,
            nonblockthreadid: self.non_block_thread_id
        }
    }
}

/// When creating a codec, declare one of these and provide the relevant callbacks and name for FMOD to use when it opens and reads a file.
pub struct FmodCodecDescription {
    /// [in] Name of the codec.
    pub name             : String,
    /// [in] Plugin writer's version number.
    pub version          : u32,
    /// [in] Tells FMOD to open the file as a stream when calling [`FmodSys::create_sound`](doc/rfmod/struct.FmodSys.html#method.create_sound), and not a static sample. Should normally be 0 (FALSE), because generally the user wants to decode the file into memory when using [`FmodSys::create_sound`](doc/rfmod/struct.FmodSys.html#method.create_sound). Mainly used for formats that decode for a very long time, or could use large amounts of memory when decoded.
    /// Usually sequenced formats such as mod/s3m/xm/it/midi fall into this category. It is mainly to stop users that don't know what they're doing from getting FMOD_ERR_MEMORY returned from createSound when they should have in fact called System::createStream or used FMOD_CREATESTREAM in [`FmodSys::create_sound`](doc/rfmod/struct.FmodSys.html#method.create_sound).
    pub default_as_stream: i32,
    /// [in] When setposition codec is called, only these time formats will be passed to the codec. Use bitwise OR to accumulate different types.
    pub time_units       : FmodTimeUnit,
    /// [in] Open callback for the codec for when FMOD tries to open a sound using this codec.
    open                 : ffi::FMOD_CODEC_OPENCALLBACK,
    /// [in] Close callback for the codec for when FMOD tries to close a sound using this codec.
    close                : ffi::FMOD_CODEC_CLOSECALLBACK,
    /// [in] Read callback for the codec for when FMOD tries to read some data from the file to the destination format (specified in the open callback).
    read                 : ffi::FMOD_CODEC_READCALLBACK,
    /// [in] Callback to return the length of the song in whatever format required when [`Sound::get_length`](doc/rfmod/struct.Sound.html#method.get_length) is called.
    get_length           : ffi::FMOD_CODEC_GETLENGTHCALLBACK,
    /// [in] Seek callback for the codec for when FMOD tries to seek within the file with [`Channel::set_position`](doc/rfmod/struct.Channel.html#method.set_position).
    set_position         : ffi::FMOD_CODEC_SETPOSITIONCALLBACK,
    /// [in] Tell callback for the codec for when FMOD tries to get the current position within the with [`Channel::get_position`](doc/rfmod/struct.Channel.html#method.get_position).
    get_position         : ffi::FMOD_CODEC_GETPOSITIONCALLBACK,
    /// [in] Sound creation callback for the codec when FMOD finishes creating the sound. (So the codec can set more parameters for the related created sound, ie loop points/mode or 3D attributes etc).
    sound_create         : ffi::FMOD_CODEC_SOUNDCREATECALLBACK,
    /// [in] Callback to tell FMOD about the waveformat of a particular subsound. This is to save memory, rather than saving 1000 FMOD_CODEC_WAVEFORMAT structures in the codec, the codec might have a more optimal way of storing this information.
    get_wave_format      : ffi::FMOD_CODEC_GETWAVEFORMAT
}

impl Default for FmodCodecDescription {
    fn default() -> FmodCodecDescription {
        FmodCodecDescription {
            name: String::new(),
            version: 0u32,
            default_as_stream: 0i32,
            time_units: FmodTimeUnit(0u32),
            open: None,
            close: None,
            read: None,
            get_length: None,
            set_position: None,
            get_position: None,
            sound_create: None,
            get_wave_format: None
        }
    }
}

/// Wrapper for OutputHandle
pub struct FmodOutputHandle {
    handle: *mut c_void
}

/// Structure to be filled with detailed memory usage information of a FMOD object
pub struct FmodMemoryUsageDetails
{
    /// [out] Memory not accounted for by other types
    pub other                  : u32,
    /// [out] String data
    pub string                 : u32,
    /// [out] System object and various internals
    pub system                 : u32,
    /// [out] Plugin objects and internals
    pub plugins                : u32,
    /// [out] Output module object and internals
    pub output                 : u32,
    /// [out] Channel related memory
    pub channel                : u32,
    /// [out] ChannelGroup objects and internals
    pub channel_group          : u32,
    /// [out] Codecs allocated for streaming
    pub codec                  : u32,
    /// [out] File buffers and structures
    pub file                   : u32,
    /// [out] Sound objects and internals
    pub sound                  : u32,
    /// [out] Sound data stored in secondary RAM
    pub secondary_ram          : u32,
    /// [out] SoundGroup objects and internals
    pub sound_group            : u32,
    /// [out] Stream buffer memory
    pub stream_buffer          : u32,
    /// [out] DSPConnection objects and internals
    pub dsp_connection         : u32,
    /// [out] DSP implementation objects
    pub dsp                    : u32,
    /// [out] Realtime file format decoding DSP objects
    pub dsp_codec              : u32,
    /// [out] Profiler memory footprint.
    pub profile                : u32,
    /// [out] Buffer used to store recorded data from microphone
    pub record_buffer          : u32,
    /// [out] Reverb implementation objects
    pub reverb                 : u32,
    /// [out] Reverb channel properties structs
    pub reverb_channel_props   : u32,
    /// [out] Geometry objects and internals
    pub geometry               : u32,
    /// [out] Sync point memory.
    pub sync_point             : u32,
    /// [out] EventSystem and various internals
    pub event_system           : u32,
    /// [out] MusicSystem and various internals
    pub music_system           : u32,
    /// [out] Definition of objects contained in all loaded projects e.g. events, groups, categories
    pub fev                    : u32,
    /// [out] Data loaded with preloadFSB
    pub memory_fsb             : u32,
    /// [out] EventProject objects and internals
    pub event_project          : u32,
    /// [out] EventGroup objects and internals
    pub event_group_i          : u32,
    /// [out] Objects used to manage wave banks
    pub sound_bank_class       : u32,
    /// [out] Data used to manage lists of wave bank usage
    pub sound_bank_list        : u32,
    /// [out] Stream objects and internals
    pub stream_instance        : u32,
    /// [out] Sound definition objects
    pub sound_def_class        : u32,
    /// [out] Sound definition static data objects
    pub sound_def_def_class    : u32,
    /// [out] Sound definition pool data
    pub sound_def_pool         : u32,
    /// [out] Reverb definition objects
    pub reverb_def             : u32,
    /// [out] Reverb objects
    pub event_reverb           : u32,
    /// [out] User property objects
    pub user_property          : u32,
    /// [out] Event instance base objects
    pub event_instance         : u32,
    /// [out] Complex event instance objects
    pub event_instance_complex : u32,
    /// [out] Simple event instance objects
    pub event_instance_simple  : u32,
    /// [out] Event layer instance objects
    pub event_instance_layer   : u32,
    /// [out] Event sound instance objects
    pub event_instance_sound   : u32,
    /// [out] Event envelope objects
    pub event_envelope         : u32,
    /// [out] Event envelope definition objects
    pub event_envelope_def     : u32,
    /// [out] Event parameter objects
    pub event_parameter        : u32,
    /// [out] Event category objects
    pub event_category         : u32,
    /// [out] Event envelope point objects
    pub event_envelope_point   : u32,
    /// [out] Event instance pool memory
    pub event_instance_pool    : u32
}

impl Default for FmodMemoryUsageDetails {
    fn default() -> FmodMemoryUsageDetails {
        FmodMemoryUsageDetails {
            other: 0u32,
            string: 0u32,
            system: 0u32,
            plugins: 0u32,
            output: 0u32,
            channel: 0u32,
            channel_group: 0u32,
            codec: 0u32,
            file: 0u32,
            sound: 0u32,
            secondary_ram: 0u32,
            sound_group: 0u32,
            stream_buffer: 0u32,
            dsp_connection: 0u32,
            dsp: 0u32,
            dsp_codec: 0u32,
            profile: 0u32,
            record_buffer: 0u32,
            reverb: 0u32,
            reverb_channel_props: 0u32,
            geometry: 0u32,
            sync_point: 0u32,
            event_system: 0u32,
            music_system: 0u32,
            fev: 0u32,
            memory_fsb: 0u32,
            event_project: 0u32,
            event_group_i: 0u32,
            sound_bank_class: 0u32,
            sound_bank_list: 0u32,
            stream_instance: 0u32,
            sound_def_class: 0u32,
            sound_def_def_class: 0u32,
            sound_def_pool: 0u32,
            reverb_def: 0u32,
            event_reverb: 0u32,
            user_property: 0u32,
            event_instance: 0u32,
            event_instance_complex: 0u32,
            event_instance_simple: 0u32,
            event_instance_layer: 0u32,
            event_instance_sound: 0u32,
            event_envelope: 0u32,
            event_envelope_def: 0u32,
            event_parameter: 0u32,
            event_category: 0u32,
            event_envelope_point: 0u32,
            event_instance_pool: 0u32
        }
    }
}

pub fn get_memory_usage_details_ffi(details: FmodMemoryUsageDetails) -> ffi::FMOD_MEMORY_USAGE_DETAILS {
    ffi::FMOD_MEMORY_USAGE_DETAILS {
        other: details.other,
        string: details.string,
        system: details.system,
        plugins: details.plugins,
        output: details.output,
        channel: details.channel,
        channel_group: details.channel_group,
        codec: details.codec,
        file: details.file,
        sound: details.sound,
        secondary_ram: details.secondary_ram,
        sound_group: details.sound_group,
        stream_buffer: details.stream_buffer,
        dsp_connection: details.dsp_connection,
        dsp: details.dsp,
        dsp_codec: details.dsp_codec,
        profile: details.profile,
        record_buffer: details.record_buffer,
        reverb: details.reverb,
        reverb_channel_props: details.reverb_channel_props,
        geometry: details.geometry,
        sync_point: details.sync_point,
        event_system: details.event_system,
        music_system: details.music_system,
        fev: details.fev,
        memory_fsb: details.memory_fsb,
        event_project: details.event_project,
        event_group_i: details.event_group_i,
        sound_bank_class: details.sound_bank_class,
        sound_bank_list: details.sound_bank_list,
        stream_instance: details.stream_instance,
        sound_def_class: details.sound_def_class,
        sound_def_def_class: details.sound_def_def_class,
        sound_def_pool: details.sound_def_pool,
        reverb_def: details.reverb_def,
        event_reverb: details.event_reverb,
        user_property: details.user_property,
        event_instance: details.event_instance,
        event_instance_complex: details.event_instance_complex,
        event_instance_simple: details.event_instance_simple,
        event_instance_layer: details.event_instance_layer,
        event_instance_sound: details.event_instance_sound,
        event_envelope: details.event_envelope,
        event_envelope_def: details.event_envelope_def,
        event_parameter: details.event_parameter,
        event_category: details.event_category,
        event_envelope_point: details.event_envelope_point,
        event_instance_pool: details.event_instance_pool
    }
}

pub fn from_memory_usage_details_ptr(details: ffi::FMOD_MEMORY_USAGE_DETAILS) -> FmodMemoryUsageDetails {
    FmodMemoryUsageDetails {
        other: details.other,
        string: details.string,
        system: details.system,
        plugins: details.plugins,
        output: details.output,
        channel: details.channel,
        channel_group: details.channel_group,
        codec: details.codec,
        file: details.file,
        sound: details.sound,
        secondary_ram: details.secondary_ram,
        sound_group: details.sound_group,
        stream_buffer: details.stream_buffer,
        dsp_connection: details.dsp_connection,
        dsp: details.dsp,
        dsp_codec: details.dsp_codec,
        profile: details.profile,
        record_buffer: details.record_buffer,
        reverb: details.reverb,
        reverb_channel_props: details.reverb_channel_props,
        geometry: details.geometry,
        sync_point: details.sync_point,
        event_system: details.event_system,
        music_system: details.music_system,
        fev: details.fev,
        memory_fsb: details.memory_fsb,
        event_project: details.event_project,
        event_group_i: details.event_group_i,
        sound_bank_class: details.sound_bank_class,
        sound_bank_list: details.sound_bank_list,
        stream_instance: details.stream_instance,
        sound_def_class: details.sound_def_class,
        sound_def_def_class: details.sound_def_def_class,
        sound_def_pool: details.sound_def_pool,
        reverb_def: details.reverb_def,
        event_reverb: details.event_reverb,
        user_property: details.user_property,
        event_instance: details.event_instance,
        event_instance_complex: details.event_instance_complex,
        event_instance_simple: details.event_instance_simple,
        event_instance_layer: details.event_instance_layer,
        event_instance_sound: details.event_instance_sound,
        event_envelope: details.event_envelope,
        event_envelope_def: details.event_envelope_def,
        event_parameter: details.event_parameter,
        event_category: details.event_category,
        event_envelope_point: details.event_envelope_point,
        event_instance_pool: details.event_instance_pool
    }
}

pub fn from_ptr(system: *mut ffi::FMOD_SYSTEM) -> FmodSys {
    FmodSys{system: system, is_first: false}
}

pub fn get_ffi(system: &FmodSys) -> *mut ffi::FMOD_SYSTEM {
    system.system
}

/// FMOD System Object
pub struct FmodSys {
    system: *mut ffi::FMOD_SYSTEM,
    is_first: bool
}

impl Drop for FmodSys {
    fn drop(&mut self) {
        self.release();
    }
}

impl FmodSys {
    /* the first one created has to be the last one released */
    pub fn new() -> Result<FmodSys, fmod::Result> {
        let mut tmp = ::std::ptr::mut_null();

        match unsafe { ffi::FMOD_System_Create(&mut tmp) } {
            fmod::Ok => Ok(FmodSys{system: tmp, is_first: true}),
            err => Err(err)
        }
    }

    pub fn init(&self) -> fmod::Result {
        unsafe { ffi::FMOD_System_Init(self.system, 1, FMOD_INIT_NORMAL, ::std::ptr::mut_null()) }
    }

    pub fn init_with_parameters(&self, max_channels: i32, FmodInitFlag(flag): FmodInitFlag) -> fmod::Result {
        unsafe { ffi::FMOD_System_Init(self.system, max_channels, flag, ::std::ptr::mut_null()) }
    }

    pub fn update(&self) -> fmod::Result {
        unsafe { ffi::FMOD_System_Update(self.system) }
    }

    pub fn release(&mut self) -> fmod::Result {
        if self.is_first && self.system.is_not_null() {
            unsafe {
                match match ffi::FMOD_System_Close(self.system) {
                    fmod::Ok => ffi::FMOD_System_Release(self.system),
                    e => e
                } {
                    fmod::Ok => {
                        self.system = ::std::ptr::mut_null();
                        fmod::Ok
                    }
                    e => e
                }
                
            }
        } else {
            fmod::Ok
        }
    }

    /// If music is empty, null is sent
    pub fn create_sound(&self, music: &str, options: Option<FmodMode>, exinfo: Option<&mut FmodCreateSoundexInfo>) -> Result<Sound, fmod::Result> {
        let mut sound = sound::from_ptr_first(::std::ptr::mut_null());
        let op = match options {
            Some(FmodMode(t)) => t,
            None => FMOD_SOFTWARE | FMOD_LOOP_OFF | FMOD_2D | FMOD_CREATESTREAM
        };
        let ex = match exinfo {
            Some(e) => {
                let user_data = sound::get_user_data(&mut sound);
                user_data.non_block = e.non_block_callback;
                user_data.pcm_read = e.pcm_read_callback;
                user_data.pcm_set_pos = e.pcm_set_pos_callback;
                unsafe {
                    user_data.user_data = ::std::mem::transmute::<&mut ffi::SoundData, *mut c_void>(&mut *e.user_data);
                }
                &mut e.convert_to_c() as *mut ffi::FMOD_CREATESOUNDEXINFO
            },
            None => ::std::ptr::mut_null()
        };

        match if music.len() > 0 {
            music.with_c_str(|c_str|{
               unsafe { ffi::FMOD_System_CreateSound(self.system, c_str, op, ex, sound::get_fffi(&mut sound)) }
            })
        } else {
            unsafe { ffi::FMOD_System_CreateSound(self.system, ::std::ptr::null(), op, ex, sound::get_fffi(&mut sound)) }
        } {
            fmod::Ok => {
                Ok(sound)
            },
            e => Err(e)
        }
    }

    pub fn create_stream(&self, music: &str, options: Option<FmodMode>, exinfo: Option<&mut FmodCreateSoundexInfo>) -> Result<Sound, fmod::Result> {
        let mut sound = sound::from_ptr_first(::std::ptr::mut_null());
        let op = match options {
            Some(FmodMode(t)) => t,
            None => FMOD_SOFTWARE | FMOD_LOOP_OFF | FMOD_2D | FMOD_CREATESTREAM
        };
        let ex = match exinfo {
            Some(e) => {
                let user_data = sound::get_user_data(&mut sound);
                user_data.non_block = e.non_block_callback;
                user_data.pcm_read = e.pcm_read_callback;
                user_data.pcm_set_pos = e.pcm_set_pos_callback;
                unsafe {
                    user_data.user_data = ::std::mem::transmute::<&mut ffi::SoundData, *mut c_void>(&mut *e.user_data);
                }
                &mut e.convert_to_c() as *mut ffi::FMOD_CREATESOUNDEXINFO
            },
            None => ::std::ptr::mut_null()
        };

        match if music.len() > 0 {
            music.with_c_str(|c_str|{
                unsafe { ffi::FMOD_System_CreateStream(self.system, c_str, op, ex, sound::get_fffi(&mut sound)) }
            })
        } else {
            unsafe { ffi::FMOD_System_CreateStream(self.system, ::std::ptr::null(), op, ex, sound::get_fffi(&mut sound)) }
        } {
            fmod::Ok => Ok(sound),
            err => Err(err)
        }
    }

    pub fn create_channel_group(&self, group_name: String) -> Result<channel_group::ChannelGroup, fmod::Result> {
        let t_group_name = group_name.clone();
        let mut channel_group = ::std::ptr::mut_null();

        t_group_name.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_CreateChannelGroup(self.system, c_str, &mut channel_group) } {
                fmod::Ok => Ok(channel_group::from_ptr(channel_group)),
                e => Err(e)
            }
        })
    }

    pub fn create_sound_group(&self, group_name: String) -> Result<sound_group::SoundGroup, fmod::Result> {
        let t_group_name = group_name.clone();
        let mut sound_group = ::std::ptr::mut_null();

        t_group_name.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_CreateSoundGroup(self.system, c_str, &mut sound_group) } {
                fmod::Ok => Ok(sound_group::from_ptr(sound_group)),
                e => Err(e)
            }
        })
    }

    pub fn create_reverb(&self) -> Result<reverb::Reverb, fmod::Result>{
        let mut t_reverb = ::std::ptr::mut_null();

        match unsafe { ffi::FMOD_System_CreateReverb(self.system, &mut t_reverb) } {
            fmod::Ok => Ok(reverb::from_ptr(t_reverb)),
            e => Err(e)
        }
    }

    pub fn create_DSP(&self) -> Result<dsp::Dsp, fmod::Result> {
        let mut t_dsp = ::std::ptr::mut_null();

        match unsafe { ffi::FMOD_System_CreateDSP(self.system, ::std::ptr::mut_null(), &mut t_dsp) } {
            fmod::Ok => Ok(dsp::from_ptr_first(t_dsp)),
            e => Err(e)
        }
    }

    pub fn create_DSP_with_description(&self, description: &mut dsp::DspDescription) -> Result<dsp::Dsp, fmod::Result> {
        let mut t_dsp = ::std::ptr::mut_null();
        let mut t_description = dsp::get_description_ffi(description);

        match unsafe { ffi::FMOD_System_CreateDSP(self.system, &mut t_description, &mut t_dsp) } {
            fmod::Ok => Ok(dsp::from_ptr_first(t_dsp)),
            e => Err(e)
        }
    }

    pub fn create_DSP_by_type(&self, _type: fmod::DspType) -> Result<dsp::Dsp, fmod::Result> {
        let mut t_dsp = ::std::ptr::mut_null();

        match unsafe { ffi::FMOD_System_CreateDSPByType(self.system, _type, &mut t_dsp) } {
            fmod::Ok => Ok(dsp::from_ptr_first(t_dsp)),
            e => Err(e)
        }
    }

    pub fn set_output(&self, output_type: fmod::OutputType) -> fmod::Result {
        unsafe { ffi::FMOD_System_SetOutput(self.system, output_type) }
    }

    pub fn get_output(&self) -> Result<fmod::OutputType, fmod::Result> {
        let mut output_type = fmod::OutputTypeAutoDetect;
        
        match unsafe { ffi::FMOD_System_GetOutput(self.system, &mut output_type) } {
            fmod::Ok => Ok(output_type),
            e => Err(e)
        }
    }

    pub fn get_num_drivers(&self) -> Result<i32, fmod::Result> {
        let mut num_drivers = 0i32;

        match unsafe { ffi::FMOD_System_GetNumDrivers(self.system, &mut num_drivers) } {
            fmod::Ok => Ok(num_drivers),
            e => Err(e)
        }
    }

    pub fn get_driver_info(&self, id: i32, name_len: uint) -> Result<(FmodGuid, String), fmod::Result> {
        let tmp_v = String::with_capacity(name_len);
        let mut guid = ffi::FMOD_GUID{Data1: 0, Data2: 0, Data3: 0, Data4: [0, 0, 0, 0, 0, 0, 0, 0]};

        tmp_v.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_GetDriverInfo(self.system, id, c_str as *mut c_char, name_len as i32, &mut guid) } {
                fmod::Ok => Ok((FmodGuid{data1: guid.Data1, data2: guid.Data2, data3: guid.Data3, data4: guid.Data4},
                    unsafe {string::raw::from_buf(c_str as *const u8).clone() })),
                e => Err(e)
            }
        })
    }

    pub fn get_driver_caps(&self, id: i32) -> Result<(FmodCaps, i32, fmod::SpeakerMode), fmod::Result> {
        let mut fmod_caps = 0u32;
        let mut speaker_mode = fmod::SpeakerModeRaw;
        let mut control_panel_output_rate = 0i32;

        match unsafe { ffi::FMOD_System_GetDriverCaps(self.system, id, &mut fmod_caps, &mut control_panel_output_rate, &mut speaker_mode) } {
            fmod::Ok => Ok((FmodCaps(fmod_caps), control_panel_output_rate, speaker_mode)),
            e => Err(e)
        }
    }

    pub fn set_driver(&self, driver: i32) -> fmod::Result {
        unsafe { ffi::FMOD_System_SetDriver(self.system, driver) }
    }

    pub fn get_driver(&self) -> Result<i32, fmod::Result> {
        let mut driver = 0i32;

        match unsafe { ffi::FMOD_System_GetDriver(self.system, &mut driver) } {
            fmod::Ok => Ok(driver),
            e => Err(e)
        }
    }

    pub fn set_hardware_channels(&self, num_hardware_channels: i32) -> fmod::Result {
        unsafe { ffi::FMOD_System_SetHardwareChannels(self.system, num_hardware_channels) }
    }

    pub fn get_hardware_channels(&self) -> Result<i32, fmod::Result> {
        let mut num_hardware_channels = 0i32;

        match unsafe { ffi::FMOD_System_GetHardwareChannels(self.system, &mut num_hardware_channels) } {
            fmod::Ok => Ok(num_hardware_channels),
            e => Err(e)
        }
    }

    pub fn set_software_channels(&self, num_software_channels: i32) -> fmod::Result {
        unsafe { ffi::FMOD_System_SetSoftwareChannels(self.system, num_software_channels) }
    }

    pub fn get_software_channels(&self) -> Result<i32, fmod::Result> {
        let mut num_software_channels = 0i32;

        match unsafe { ffi::FMOD_System_GetSoftwareChannels(self.system, &mut num_software_channels) } {
            fmod::Ok => Ok(num_software_channels),
            e => Err(e)
        }
    }

    pub fn set_software_format(&self, sample_rate: i32, format: fmod::SoundFormat, num_output_channels: i32,
        max_input_channels: i32, resample_method: fmod::DSPResampler) -> fmod::Result {
        unsafe { ffi::FMOD_System_SetSoftwareFormat(self.system, sample_rate, format, num_output_channels,
            max_input_channels, resample_method) }
    }

    pub fn get_software_format(&self) -> Result<FmodSoftwareFormat, fmod::Result> {
        let mut t = FmodSoftwareFormat{sample_rate: 0, format: fmod::SoundFormatNone, num_output_channels: 0,
            max_input_channels: 0, resample_method: fmod::DSPResamplerNoInterp, bits: 0};

        match unsafe { ffi::FMOD_System_GetSoftwareFormat(self.system, &mut t.sample_rate, &mut t.format,
            &mut t.num_output_channels, &mut t.max_input_channels, &mut t.resample_method, &mut t.bits) } {
            fmod::Ok => Ok(t),
            e => Err(e)
        }
    }

    pub fn set_DSP_buffer_size(&self, buffer_length: u32, num_buffers: i32) -> fmod::Result {
        unsafe { ffi::FMOD_System_SetDSPBufferSize(self.system, buffer_length, num_buffers) }
    }

    pub fn get_DSP_buffer_size(&self) -> Result<(u32, i32), fmod::Result> {
        let mut buffer_length = 0u32;
        let mut num_buffers = 0i32;

        match unsafe { ffi::FMOD_System_GetDSPBufferSize(self.system, &mut buffer_length, &mut num_buffers) } {
            fmod::Ok => Ok((buffer_length, num_buffers)),
            e => Err(e)
        }
    }

    pub fn set_advanced_settings(&self, settings: &mut FmodAdvancedSettings) -> fmod::Result {
        let mut converted_c_char = Vec::from_fn(settings.ASIO_channel_list.len(), |pos| {
            settings.ASIO_channel_list[pos].clone().with_c_str(|c_str| c_str)
        });
        let deb_log_filename = settings.debug_log_filename.clone();
        let mut advanced_settings = ffi::FMOD_ADVANCEDSETTINGS{
            cbsize: mem::size_of::<ffi::FMOD_ADVANCEDSETTINGS>() as i32,
            maxMPEGcodecs: settings.max_MPEG_codecs,
            maxADPCMcodecs: settings.max_ADPCM_codecs,
            maxXMAcodecs: settings.max_XMA_codecs,
            maxCELTcodecs: settings.max_CELT_codecs,
            maxVORBIScodecs: settings.max_VORBIS_codecs,
            maxAT9Codecs: settings.max_AT9_codecs,
            maxPCMcodecs: settings.max_PCM_codecs,
            ASIONumChannels: settings.ASIO_num_channels,
            ASIOChannelList: converted_c_char.as_mut_ptr() as *mut *mut c_char,
            ASIOSpeakerList: settings.ASIO_speaker_list.as_mut_ptr(),
            max3DReverbDSPs: settings.max_3D_reverb_DSPs,
            HRTFMinAngle: settings.HRTF_min_angle,
            HRTFMaxAngle: settings.HRTF_max_angle,
            HRTFFreq: settings.HRTF_freq,
            vol0virtualvol: settings.vol0_virtual_vol,
            eventqueuesize: settings.event_queue_size,
            defaultDecodeBufferSize: settings.default_decode_buffer_size,
            debugLogFilename: deb_log_filename.with_c_str(|c_str| c_str as *mut c_char),
            profileport: settings.profile_port,
            geometryMaxFadeTime: settings.geometry_max_fade_time,
            maxSpectrumWaveDataBuffers: settings.max_spectrum_wave_data_buffers,
            musicSystemCacheDelay: settings.music_system_cache_delay,
            distanceFilterCenterFreq: settings.distance_filter_center_freq,
            stackSizeStream: settings.stack_size_stream,
            stackSizeNonBlocking: settings.stack_size_non_blocking,
            stackSizeMixer: settings.stack_size_mixer
        };

        unsafe { ffi::FMOD_System_SetAdvancedSettings(self.system, &mut advanced_settings) }
    }

    pub fn get_advanced_settings(&self) -> Result<FmodAdvancedSettings, fmod::Result> {
        let mut advanced_settings = ffi::FMOD_ADVANCEDSETTINGS{
            cbsize: mem::size_of::<ffi::FMOD_ADVANCEDSETTINGS>() as i32,
            maxMPEGcodecs: 0,
            maxADPCMcodecs: 0,
            maxXMAcodecs: 0,
            maxCELTcodecs: 0,
            maxVORBIScodecs: 0,
            maxAT9Codecs: 0,
            maxPCMcodecs: 0,
            ASIONumChannels: 0,
            ASIOChannelList: ::std::ptr::mut_null(),
            ASIOSpeakerList: ::std::ptr::mut_null(),
            max3DReverbDSPs: 0,
            HRTFMinAngle: 0f32,
            HRTFMaxAngle: 0f32,
            HRTFFreq: 0f32,
            vol0virtualvol: 0f32,
            eventqueuesize: 0,
            defaultDecodeBufferSize: 0,
            debugLogFilename: ::std::ptr::mut_null(),
            profileport: 0,
            geometryMaxFadeTime: 0,
            maxSpectrumWaveDataBuffers: 0,
            musicSystemCacheDelay: 0,
            distanceFilterCenterFreq: 0f32,
            stackSizeStream: 0,
            stackSizeNonBlocking: 0,
            stackSizeMixer: 0
        };

        match unsafe { ffi::FMOD_System_GetAdvancedSettings(self.system, &mut advanced_settings) } {
            fmod::Ok => {
                let mut converted_ASIO_channel_vec = Vec::new();
                let mut converted_ASIO_speaker_vec = Vec::new();

                unsafe {
                    if advanced_settings.ASIOChannelList.is_not_null() {
                        let mut it = 0;
                        loop {
                            let tmp = advanced_settings.ASIOChannelList.offset(it);

                            if tmp.is_null() {
                                break;
                            }
                            converted_ASIO_channel_vec.push(string::raw::from_buf(*tmp as *const u8));
                            it += 1;
                        }
                    }
                    if advanced_settings.ASIOSpeakerList.is_not_null() {
                        let mut it = 0;
                        loop {
                            let tmp = advanced_settings.ASIOSpeakerList.offset(it);

                            if *tmp == fmod::SpeakerNull {
                                break;
                            }
                            converted_ASIO_speaker_vec.push(*tmp);
                            it += 1;
                        }
                    }
                }
                Ok(FmodAdvancedSettings{
                    max_MPEG_codecs: advanced_settings.maxMPEGcodecs,
                    max_ADPCM_codecs: advanced_settings.maxADPCMcodecs,
                    max_XMA_codecs: advanced_settings.maxXMAcodecs,
                    max_CELT_codecs: advanced_settings.maxCELTcodecs,
                    max_VORBIS_codecs: advanced_settings.maxVORBIScodecs,
                    max_AT9_codecs: advanced_settings.maxAT9Codecs,
                    max_PCM_codecs: advanced_settings.maxPCMcodecs,
                    ASIO_num_channels: advanced_settings.ASIONumChannels,
                    ASIO_channel_list: converted_ASIO_channel_vec.clone(),
                    ASIO_speaker_list: converted_ASIO_speaker_vec,
                    max_3D_reverb_DSPs: advanced_settings.max3DReverbDSPs,
                    HRTF_min_angle: advanced_settings.HRTFMinAngle,
                    HRTF_max_angle: advanced_settings.HRTFMaxAngle,
                    HRTF_freq: advanced_settings.HRTFFreq,
                    vol0_virtual_vol: advanced_settings.vol0virtualvol,
                    event_queue_size: advanced_settings.eventqueuesize,
                    default_decode_buffer_size: advanced_settings.defaultDecodeBufferSize,
                    debug_log_filename: {
                        if advanced_settings.debugLogFilename.is_not_null() {
                            unsafe {string::raw::from_buf(advanced_settings.debugLogFilename as *const u8).clone() }
                        } else {
                            String::new()
                        }
                    },
                    profile_port: advanced_settings.profileport,
                    geometry_max_fade_time: advanced_settings.geometryMaxFadeTime,
                    max_spectrum_wave_data_buffers: advanced_settings.maxSpectrumWaveDataBuffers,
                    music_system_cache_delay: advanced_settings.musicSystemCacheDelay,
                    distance_filter_center_freq: advanced_settings.distanceFilterCenterFreq,
                    stack_size_stream: advanced_settings.stackSizeStream,
                    stack_size_non_blocking: advanced_settings.stackSizeNonBlocking,
                    stack_size_mixer: advanced_settings.stackSizeMixer
                })
            }
            e => Err(e)
        }
    }

    pub fn set_speaker_mode(&self, speaker_mode: fmod::SpeakerMode) -> fmod::Result {
        unsafe { ffi::FMOD_System_SetSpeakerMode(self.system, speaker_mode) }
    }

    pub fn get_speaker_mode(&self) -> Result<fmod::SpeakerMode, fmod::Result> {
        let mut speaker_mode = fmod::SpeakerModeRaw;

        match unsafe { ffi::FMOD_System_GetSpeakerMode(self.system, &mut speaker_mode) } {
            fmod::Ok => Ok(speaker_mode),
            e => Err(e)
        }
    }

    pub fn set_plugin_path(&self, path: String) -> fmod::Result {
        let tmp_v = path.clone();

        tmp_v.with_c_str(|c_str|{
            unsafe { ffi::FMOD_System_SetPluginPath(self.system, c_str) }
        })
    }

    pub fn load_plugin(&self, filename: String, priority: u32) -> Result<FmodPluginHandle, fmod::Result> {
        let tmp_v = filename.clone();
        let mut handle = 0u32;

        tmp_v.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_LoadPlugin(self.system, c_str, &mut handle, priority) } {
                fmod::Ok => Ok(FmodPluginHandle(handle)),
                e => Err(e)
            }
        })
    }

    pub fn unload_plugin(&self, FmodPluginHandle(handle): FmodPluginHandle) -> fmod::Result {
        unsafe { ffi::FMOD_System_UnloadPlugin(self.system, handle) }
    }

    pub fn get_num_plugins(&self, plugin_type: fmod::PluginType) -> Result<i32, fmod::Result> {
        let mut num_plugins = 0i32;

        match unsafe { ffi::FMOD_System_GetNumPlugins(self.system, plugin_type, &mut num_plugins) } {
            fmod::Ok => Ok(num_plugins),
            e => Err(e)
        }
    }

    pub fn get_plugin_handle(&self, plugin_type: fmod::PluginType, index: i32) ->Result<FmodPluginHandle, fmod::Result> {
        let mut handle = 0u32;

        match unsafe { ffi::FMOD_System_GetPluginHandle(self.system, plugin_type, index, &mut handle) } {
            fmod::Ok => Ok(FmodPluginHandle(handle)),
            e => Err(e)
        }
    }

    pub fn get_plugin_info(&self, FmodPluginHandle(handle): FmodPluginHandle, name_len: u32) -> Result<(String, fmod::PluginType, u32), fmod::Result> {
        let name = String::with_capacity(name_len as uint);
        let mut plugin_type = fmod::PluginTypeOutput;
        let mut version = 0u32;

        name.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_GetPluginInfo(self.system, handle, &mut plugin_type, c_str as *mut c_char, name_len as i32, &mut version) } {
                fmod::Ok => Ok((unsafe {string::raw::from_buf(c_str as *const u8).clone() }, plugin_type, version)),
                e => Err(e)
            }
        })
    }

    pub fn set_output_by_plugin(&self, FmodPluginHandle(handle): FmodPluginHandle) -> fmod::Result {
        unsafe { ffi::FMOD_System_SetOutputByPlugin(self.system, handle) }
    }

    pub fn get_output_by_plugin(&self) -> Result<FmodPluginHandle, fmod::Result> {
        let mut handle = 0u32;

        match unsafe { ffi::FMOD_System_GetOutputByPlugin(self.system, &mut handle) } {
            fmod::Ok => Ok(FmodPluginHandle(handle)),
            e => Err(e)
        }
    }

    pub fn create_DSP_by_plugin(&self, FmodPluginHandle(handle): FmodPluginHandle) -> Result<Dsp, fmod::Result> {
        let mut dsp = ::std::ptr::mut_null();

        match unsafe { ffi::FMOD_System_CreateDSPByPlugin(self.system, handle, &mut dsp) } {
            fmod::Ok => Ok(dsp::from_ptr_first(dsp)),
            e => Err(e)
        }
    }

    pub fn set_3D_num_listeners(&self, num_listeners: i32) -> fmod::Result {
        unsafe { ffi::FMOD_System_Set3DNumListeners(self.system, num_listeners) }
    }

    pub fn get_3D_num_listeners(&self) -> Result<i32, fmod::Result> {
        let mut num_listeners = 0i32;

        match unsafe { ffi::FMOD_System_Get3DNumListeners(self.system, &mut num_listeners) } {
            fmod::Ok => Ok(num_listeners),
            e => Err(e)
        }
    }

    pub fn set_3D_listener_attributes(&self, listener: i32, pos: &vector::FmodVector, vel: &vector::FmodVector, forward: &vector::FmodVector,
        up: &vector::FmodVector) -> fmod::Result {
        let c_p = vector::get_ffi(pos);
        let c_v = vector::get_ffi(vel);
        let c_f = vector::get_ffi(forward);
        let c_u = vector::get_ffi(up);

        unsafe { ffi::FMOD_System_Set3DListenerAttributes(self.system, listener, &c_p, &c_v, &c_f, &c_u) }
    }

    pub fn get_3D_listener_attributes(&self, listener: i32) -> Result<(vector::FmodVector, vector::FmodVector, vector::FmodVector, vector::FmodVector), fmod::Result> {
        let mut pos = vector::get_ffi(&vector::FmodVector::new());
        let mut vel = vector::get_ffi(&vector::FmodVector::new());
        let mut forward = vector::get_ffi(&vector::FmodVector::new());
        let mut up = vector::get_ffi(&vector::FmodVector::new());

        match unsafe { ffi::FMOD_System_Get3DListenerAttributes(self.system, listener, &mut pos, &mut vel, &mut forward, &mut up) } {
            fmod::Ok => Ok((vector::from_ptr(pos), vector::from_ptr(vel), vector::from_ptr(forward), vector::from_ptr(up))),
            e => Err(e)
        }
    }

    pub fn set_3D_speaker_position(&self, speaker: fmod::Speaker, x: f32, y: f32, active: bool) -> fmod::Result {
        let t_active = match active {
            true => 1i32,
            false => 0i32
        };
        unsafe { ffi::FMOD_System_Set3DSpeakerPosition(self.system, speaker, x, y, t_active) }
    }

    pub fn get_3D_speaker_position(&self, speaker: fmod::Speaker) -> Result<(f32, f32, bool), fmod::Result> {
        let mut x = 0f32;
        let mut y = 0f32;
        let mut active = 0i32;

        match unsafe { ffi::FMOD_System_Get3DSpeakerPosition(self.system, speaker, &mut x, &mut y, &mut active) } {
            fmod::Ok => Ok((x, y, match active {
                1 => true,
                _ => false
            })),
            e => Err(e)
        }
    }

    pub fn set_3D_settings(&self, doppler_scale: f32, distance_factor: f32, roll_off_scale: f32) -> fmod::Result {
        unsafe { ffi::FMOD_System_Set3DSettings(self.system, doppler_scale, distance_factor, roll_off_scale) }
    }

    pub fn get_3D_settings(&self) -> Result<(f32, f32, f32), fmod::Result> {
        let mut doppler_scale = 0f32;
        let mut distance_factor = 0f32;
        let mut roll_off_scale = 0f32;

        match unsafe { ffi::FMOD_System_Get3DSettings(self.system, &mut doppler_scale, &mut distance_factor, &mut roll_off_scale) } {
            fmod::Ok => Ok((doppler_scale, distance_factor, roll_off_scale)),
            e => Err(e)
        }
    }

    pub fn set_stream_buffer_size(&self, file_buffer_size: u32, FmodTimeUnit(file_buffer_size_type): FmodTimeUnit) -> fmod::Result {
        unsafe { ffi::FMOD_System_SetStreamBufferSize(self.system, file_buffer_size, file_buffer_size_type) }
    }

    pub fn get_stream_buffer_size(&self) -> Result<(u32, FmodTimeUnit), fmod::Result> {
        let mut file_buffer_size = 0u32;
        let mut file_buffer_size_type = 0u32;

        match unsafe { ffi::FMOD_System_GetStreamBufferSize(self.system, &mut file_buffer_size, &mut file_buffer_size_type) } {
            fmod::Ok => Ok((file_buffer_size, FmodTimeUnit(file_buffer_size_type))),
            e => Err(e)
        }
    }

    pub fn get_version(&self) -> Result<u32, fmod::Result> {
        let mut version = 0u32;

        match unsafe { ffi::FMOD_System_GetVersion(self.system, &mut version) } {
            fmod::Ok => Ok(version),
            e => Err(e)
        }
    }

    pub fn get_output_handle(&self) -> Result<FmodOutputHandle, fmod::Result> {
        let mut output_h = ::std::ptr::mut_null();

        match unsafe { ffi::FMOD_System_GetOutputHandle(self.system, &mut output_h) } {
            fmod::Ok => Ok(FmodOutputHandle{handle: output_h}),
            e => Err(e)
        }
    }

    pub fn get_channels_playing(&self) -> Result<i32, fmod::Result> {
        let mut playing_chans = 0i32;

        match unsafe { ffi::FMOD_System_GetChannelsPlaying(self.system, &mut playing_chans) } {
            fmod::Ok => Ok(playing_chans),
            e => Err(e)
        }
    }

    pub fn get_CPU_usage(&self) -> Result<(f32, f32, f32, f32, f32), fmod::Result> {
        let mut dsp = 0f32;
        let mut stream = 0f32;
        let mut geometry = 0f32;
        let mut update = 0f32;
        let mut total = 0f32;

        match unsafe { ffi::FMOD_System_GetCPUUsage(self.system, &mut dsp, &mut stream, &mut geometry, &mut update, &mut total) } {
            fmod::Ok => Ok((dsp, stream, geometry, update, total)),
            e => Err(e)
        }
    }

    pub fn get_sound_RAM(&self) -> Result<(i32, i32, i32), fmod::Result> {
        let mut current_alloced = 0i32;
        let mut max_alloced = 0i32;
        let mut total = 0i32;

        match unsafe { ffi::FMOD_System_GetSoundRAM(self.system, &mut current_alloced, &mut max_alloced, &mut total) } {
            fmod::Ok => Ok((current_alloced, max_alloced, total)),
            e => Err(e)
        }
    }

    pub fn get_num_CDROM_drives(&self) -> Result<i32, fmod::Result> {
        let mut num_drives = 0i32;

        match unsafe { ffi::FMOD_System_GetNumCDROMDrives(self.system, &mut num_drives) } {
            fmod::Ok => Ok(num_drives),
            e => Err(e)
        }
    }

    pub fn get_CDROM_drive_name(&self, drive: i32, drive_name_len: u32, scsi_name_len: u32, device_name_len: u32) -> Result<(String, String, String), fmod::Result> {
        let drive_name = String::with_capacity(drive_name_len as uint);
        let scsi_name = String::with_capacity(scsi_name_len as uint);
        let device_name = String::with_capacity(device_name_len as uint);

        drive_name.with_c_str(|c_drive_name|{
            scsi_name.with_c_str(|c_scsi_name|{
                device_name.with_c_str(|c_device_name|{
                    match unsafe { ffi::FMOD_System_GetCDROMDriveName(self.system, drive, c_drive_name as *mut c_char, drive_name_len as i32, c_scsi_name as *mut c_char,
                        scsi_name_len as i32, c_device_name as *mut c_char, device_name_len as i32) } {
                        fmod::Ok => Ok((unsafe {string::raw::from_buf(c_drive_name as *const u8).clone() },
                                        unsafe {string::raw::from_buf(c_scsi_name as *const u8).clone() },
                                        unsafe {string::raw::from_buf(c_device_name as *const u8).clone() })),
                        e => Err(e)
                    }
                })
            })
        })
    }

    pub fn get_spectrum(&self, spectrum_size: uint, channel_offset: Option<i32>, window_type: Option<fmod::DSP_FFT_Window>) -> Result<Vec<f32>, fmod::Result> {
        let mut ptr = Vec::from_elem(spectrum_size, 0f32);
        let c_window_type = match window_type {
            Some(wt) => wt,
            None => fmod::DSP_FFT_WindowRect
        };
        let c_channel_offset = match channel_offset {
            Some(co) => co,
            None => 0i32
        };

        match unsafe { ffi::FMOD_System_GetSpectrum(self.system, ptr.as_mut_ptr(), spectrum_size as c_int, c_channel_offset, c_window_type) } {
            fmod::Ok => Ok(ptr),
            e => Err(e),
        }
    }

    pub fn get_wave_data(&self, wave_size: uint, channel_offset: i32) -> Result<Vec<f32>, fmod::Result> {
        let mut ptr = Vec::from_elem(wave_size, 0f32);

        match unsafe { ffi::FMOD_System_GetWaveData(self.system, ptr.as_mut_ptr(), wave_size as c_int, channel_offset) } {
            fmod::Ok => Ok(ptr),
            e => Err(e)
        }
    }
    
    pub fn get_channel(&self, channel_id: i32) -> Result<channel::Channel, fmod::Result> {
        let mut channel = ::std::ptr::mut_null();

        match unsafe { ffi::FMOD_System_GetChannel(self.system, channel_id, &mut channel) } {
            fmod::Ok => Ok(channel::from_ptr(channel)),
            e => Err(e)
        }
    }

    pub fn get_master_channel_group(&self) -> Result<channel_group::ChannelGroup, fmod::Result> {
        let mut channel_group = ::std::ptr::mut_null();

        match unsafe { ffi::FMOD_System_GetMasterChannelGroup(self.system, &mut channel_group) } {
            fmod::Ok => Ok(channel_group::from_ptr(channel_group)),
            e => Err(e)
        }
    }

    pub fn get_master_sound_group(&self) -> Result<sound_group::SoundGroup, fmod::Result> {
        let mut sound_group = ::std::ptr::mut_null();

        match unsafe { ffi::FMOD_System_GetMasterSoundGroup(self.system, &mut sound_group) } {
            fmod::Ok => Ok(sound_group::from_ptr(sound_group)),
            e => Err(e)
        }
    }

    pub fn set_reverb_properties(&self, properties: reverb_properties::ReverbProperties) -> fmod::Result {
        let t_properties = reverb_properties::get_ffi(properties);

        unsafe { ffi::FMOD_System_SetReverbProperties(self.system, &t_properties) }
    }

    pub fn get_reverb_properties(&self) -> Result<reverb_properties::ReverbProperties, fmod::Result> {
        let mut properties = reverb_properties::get_ffi(Default::default());

        match unsafe { ffi::FMOD_System_GetReverbProperties(self.system, &mut properties) } {
            fmod::Ok => Ok(reverb_properties::from_ptr(properties)),
            e => Err(e)
        }
    }

    pub fn set_reverb_ambient_properties(&self, properties: reverb_properties::ReverbProperties) -> fmod::Result {
        let mut t_properties = reverb_properties::get_ffi(properties);

        unsafe { ffi::FMOD_System_SetReverbAmbientProperties(self.system, &mut t_properties) }
    }

    pub fn get_reverb_ambient_properties(&self) -> Result<reverb_properties::ReverbProperties, fmod::Result> {
        let mut properties = reverb_properties::get_ffi(Default::default());

        match unsafe { ffi::FMOD_System_GetReverbAmbientProperties(self.system, &mut properties) } {
            fmod::Ok => Ok(reverb_properties::from_ptr(properties)),
            e => Err(e)
        }
    }

    pub fn get_DSP_head(&self) -> Result<Dsp, fmod::Result> {
        let mut head = ::std::ptr::mut_null();

        match unsafe { ffi::FMOD_System_GetDSPHead(self.system, &mut head) } {
            fmod::Ok => Ok(dsp::from_ptr(head)),
            e => Err(e)
        }
    }

    pub fn add_DSP(&self, dsp: &dsp::Dsp) -> Result<dsp_connection::DspConnection, fmod::Result> {
        let mut t_connection = ::std::ptr::mut_null();

        match unsafe { ffi::FMOD_System_AddDSP(self.system, dsp::get_ffi(dsp), &mut t_connection) } {
            fmod::Ok => Ok(dsp_connection::from_ptr(t_connection)),
            e => Err(e)
        }
    }

    pub fn lock_DSP(&self) -> fmod::Result {
        unsafe { ffi::FMOD_System_LockDSP(self.system) }
    }

    pub fn unlock_DSP(&self) -> fmod::Result {
        unsafe { ffi::FMOD_System_UnlockDSP(self.system) }
    }

    pub fn get_DSP_clock(&self) -> Result<(u32, u32), fmod::Result> {
        let mut hi = 0u32;
        let mut lo = 0u32;

        match unsafe { ffi::FMOD_System_GetDSPClock(self.system, &mut hi, &mut lo) } {
            fmod::Ok => Ok((hi, lo)),
            e => Err(e)
        }
    }

    pub fn get_record_num_drivers(&self) -> Result<i32, fmod::Result> {
        let mut num_drivers = 0i32;

        match unsafe { ffi::FMOD_System_GetRecordNumDrivers(self.system, &mut num_drivers) } {
            fmod::Ok => Ok(num_drivers),
            e => Err(e)
        }
    }

    pub fn get_record_driver_info(&self, id: i32, name_len: uint) -> Result<(FmodGuid, String), fmod::Result> {
        let tmp_v = String::with_capacity(name_len);
        let mut guid = ffi::FMOD_GUID{Data1: 0, Data2: 0, Data3: 0, Data4: [0, 0, 0, 0, 0, 0, 0, 0]};

        tmp_v.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_GetRecordDriverInfo(self.system, id, c_str as *mut c_char, name_len as i32, &mut guid) } {
                fmod::Ok => Ok((FmodGuid{data1: guid.Data1, data2: guid.Data2, data3: guid.Data3, data4: guid.Data4},
                    unsafe {string::raw::from_buf(c_str as *const u8).clone() })),
                e => Err(e)
            }
        })
    }

    pub fn get_record_driver_caps(&self, id: i32) -> Result<(FmodCaps, i32, i32), fmod::Result> {
        let mut fmod_caps = 0u32;
        let mut min_frequency = 0i32;
        let mut max_frequency = 0i32;

        match unsafe { ffi::FMOD_System_GetRecordDriverCaps(self.system, id, &mut fmod_caps, &mut min_frequency, &mut max_frequency) } {
            fmod::Ok => Ok((FmodCaps(fmod_caps), min_frequency, max_frequency)),
            e => Err(e)
        }
    }

    pub fn get_record_position(&self, id: i32) -> Result<u32, fmod::Result> {
        let mut position = 0u32;

        match unsafe { ffi::FMOD_System_GetRecordPosition(self.system, id, &mut position) } {
            fmod::Ok => Ok(position),
            e => Err(e)
        }
    }

    pub fn start_record(&self, id: i32, sound: &sound::Sound, _loop: bool) -> fmod::Result {
        let t_loop = match _loop {
            true => 1,
            _ => 0
        };

        unsafe { ffi::FMOD_System_RecordStart(self.system, id, sound::get_ffi(sound), t_loop) }
    }

    pub fn stop_record(&self, id: i32) -> fmod::Result {
        unsafe { ffi::FMOD_System_RecordStop(self.system, id) }
    }

    pub fn is_recording(&self, id: i32) -> Result<bool, fmod::Result> {
        let mut is_recording = 0i32;
        
        match unsafe { ffi::FMOD_System_IsRecording(self.system, id, &mut is_recording) } {
            fmod::Ok => Ok(is_recording == 1),
            e => Err(e)
        }
    }

    pub fn create_geometry(&self, max_polygons: i32, max_vertices: i32) -> Result<geometry::Geometry, fmod::Result> {
        let mut geometry = ::std::ptr::mut_null();

        match unsafe { ffi::FMOD_System_CreateGeometry(self.system, max_polygons, max_vertices, &mut geometry) } {
            fmod::Ok => Ok(geometry::from_ptr(geometry)),
            e => Err(e)
        }
    }

    pub fn set_geometry_settings(&self, max_world_size: f32) -> fmod::Result {
        unsafe { ffi::FMOD_System_SetGeometrySettings(self.system, max_world_size) }
    }

    pub fn get_geometry_settings(&self) -> Result<f32, fmod::Result> {
        let mut max_world_size = 0f32;

        match unsafe { ffi::FMOD_System_GetGeometrySettings(self.system, &mut max_world_size) } {
            fmod::Ok => Ok(max_world_size),
            e => Err(e)
        }
    }

    pub fn get_geometry_occlusion(&self) -> Result<(vector::FmodVector, vector::FmodVector, f32, f32), fmod::Result> {
        let listener = vector::get_ffi(&vector::FmodVector::new());
        let source = vector::get_ffi(&vector::FmodVector::new());
        let mut direct = 0f32;
        let mut reverb = 0f32;

        match unsafe { ffi::FMOD_System_GetGeometryOcclusion(self.system, &listener, &source, &mut direct, &mut reverb) } {
            fmod::Ok => Ok((vector::from_ptr(listener), vector::from_ptr(source), direct, reverb)),
            e => Err(e)
        }
    }

    pub fn get_memory_info(&self, FmodMemoryBits(memory_bits): FmodMemoryBits,
        FmodEventMemoryBits(event_memory_bits): FmodEventMemoryBits) -> Result<(u32, FmodMemoryUsageDetails), fmod::Result> {
        let mut details = get_memory_usage_details_ffi(Default::default());
        let mut memory_used = 0u32;

        match unsafe { ffi::FMOD_System_GetMemoryInfo(self.system, memory_bits, event_memory_bits, &mut memory_used, &mut details) } {
            fmod::Ok => Ok((memory_used, from_memory_usage_details_ptr(details))),
            e => Err(e)
        }
    }

    /*pub fn set_file_system(&self, user_open: ffi::FMOD_FILE_OPENCALLBACK, user_close: ffi::FMOD_FILE_CLOSECALLBACK,
        user_read: ffi::FMOD_FILE_READCALLBACK, user_seek: ffi::FMOD_FILE_SEEKCALLBACK,
        user_async_read: ffi::FMOD_FILE_ASYNCREADCALLBACK, user_async_cancel: ffi::FMOD_FILE_ASYNCCANCELCALLBACK,
        block_align: i32) -> fmod::Result {
        unsafe { ffi::FMOD_System_SetFileSystem(self.system,
            user_open,
            user_close,
            user_read,
            user_seek,
            user_async_read,
            user_async_cancel,
            block_align)
        }
    }*/
}