pub type ChromaprintContext = ::std::os::raw::c_void;

pub const CHROMAPRINT_ALGORITHM_TEST1: ChromaprintAlgorithm = 0;
pub const CHROMAPRINT_ALGORITHM_TEST2: ChromaprintAlgorithm = 1;
pub const CHROMAPRINT_ALGORITHM_TEST3: ChromaprintAlgorithm = 2;
pub const CHROMAPRINT_ALGORITHM_TEST4: ChromaprintAlgorithm = 3;
pub const CHROMAPRINT_ALGORITHM_TEST5: ChromaprintAlgorithm = 4;
pub const CHROMAPRINT_ALGORITHM_DEFAULT: ChromaprintAlgorithm = CHROMAPRINT_ALGORITHM_TEST2;

pub type ChromaprintAlgorithm = ::std::os::raw::c_int;

extern "C" {
    #[doc = " Return the version number of Chromaprint."]
    pub fn chromaprint_get_version() -> *const ::std::os::raw::c_char;

    #[doc = " Allocate and initialize the Chromaprint context."]
    #[doc = ""]
    #[doc = " Note that when Chromaprint is compiled with FFTW, this function is"]
    #[doc = " not reentrant and you need to call it only from one thread at a time."]
    #[doc = " This is not a problem when using FFmpeg or vDSP."]
    #[doc = ""]
    #[doc = " @param algorithm the fingerprint algorithm version you want to use, or"]
    #[doc = "\t\tCHROMAPRINT_ALGORITHM_DEFAULT for the default algorithm"]
    #[doc = ""]
    #[doc = " @return ctx Chromaprint context pointer"]
    pub fn chromaprint_new(algorithm: ::std::os::raw::c_int) -> *mut ChromaprintContext;

    #[doc = " Deallocate the Chromaprint context."]
    #[doc = ""]
    #[doc = " Note that when Chromaprint is compiled with FFTW, this function is"]
    #[doc = " not reentrant and you need to call it only from one thread at a time."]
    #[doc = " This is not a problem when using FFmpeg or vDSP."]
    #[doc = ""]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    pub fn chromaprint_free(ctx: *mut ChromaprintContext);

    #[doc = " Return the fingerprint algorithm this context is configured to use."]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    #[doc = " @return current algorithm version"]
    pub fn chromaprint_get_algorithm(ctx: *mut ChromaprintContext) -> ::std::os::raw::c_int;

    #[doc = " Set a configuration option for the selected fingerprint algorithm."]
    #[doc = ""]
    #[doc = " DO NOT USE THIS FUNCTION IF YOU ARE PLANNING TO USE"]
    #[doc = " THE GENERATED FINGERPRINTS WITH THE ACOUSTID SERVICE."]
    #[doc = ""]
    #[doc = " Possible options:"]
    #[doc = "  - silence_threshold: threshold for detecting silence, 0-32767"]
    #[doc = ""]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    #[doc = " @param[in] name option name"]
    #[doc = " @param[in] value option value"]
    #[doc = ""]
    #[doc = " @return 0 on error, 1 on success"]
    pub fn chromaprint_set_option(
        ctx: *mut ChromaprintContext,
        name: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = " Get the number of channels that is internally used for fingerprinting."]
    #[doc = ""]
    #[doc = " @note You normally don\'t need this. Just set the audio\'s actual number of channels"]
    #[doc = " when calling chromaprint_start() and everything will work. This is only used for"]
    #[doc = " certain optimized cases to control the audio source."]
    #[doc = ""]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    #[doc = ""]
    #[doc = " @return number of channels"]
    pub fn chromaprint_get_num_channels(ctx: *mut ChromaprintContext) -> ::std::os::raw::c_int;

    #[doc = " Get the sampling rate that is internally used for fingerprinting."]
    #[doc = ""]
    #[doc = " @note You normally don\'t need this. Just set the audio\'s actual number of channels"]
    #[doc = " when calling chromaprint_start() and everything will work. This is only used for"]
    #[doc = " certain optimized cases to control the audio source."]
    #[doc = ""]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    #[doc = ""]
    #[doc = " @return sampling rate"]
    pub fn chromaprint_get_sample_rate(ctx: *mut ChromaprintContext) -> ::std::os::raw::c_int;

    #[doc = " Get the duration of one item in the raw fingerprint in samples."]
    #[doc = ""]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    #[doc = ""]
    #[doc = " @return duration in samples"]
    pub fn chromaprint_get_item_duration(ctx: *mut ChromaprintContext) -> ::std::os::raw::c_int;

    #[doc = " Get the duration of one item in the raw fingerprint in milliseconds."]
    #[doc = ""]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    #[doc = ""]
    #[doc = " @return duration in milliseconds"]
    pub fn chromaprint_get_item_duration_ms(ctx: *mut ChromaprintContext) -> ::std::os::raw::c_int;

    #[doc = " Get the duration of internal buffers that the fingerprinting algorithm uses."]
    #[doc = ""]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    #[doc = ""]
    #[doc = " @return duration in samples"]
    pub fn chromaprint_get_delay(ctx: *mut ChromaprintContext) -> ::std::os::raw::c_int;

    #[doc = " Get the duration of internal buffers that the fingerprinting algorithm uses."]
    #[doc = ""]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    #[doc = ""]
    #[doc = " @return duration in milliseconds"]
    pub fn chromaprint_get_delay_ms(ctx: *mut ChromaprintContext) -> ::std::os::raw::c_int;

    #[doc = " Restart the computation of a fingerprint with a new audio stream."]
    #[doc = ""]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    #[doc = " @param[in] sample_rate sample rate of the audio stream (in Hz)"]
    #[doc = " @param[in] num_channels numbers of channels in the audio stream (1 or 2)"]
    #[doc = ""]
    #[doc = " @return 0 on error, 1 on success"]
    pub fn chromaprint_start(
        ctx: *mut ChromaprintContext,
        sample_rate: ::std::os::raw::c_int,
        num_channels: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = " Send audio data to the fingerprint calculator."]
    #[doc = ""]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    #[doc = " @param[in] data raw audio data, should point to an array of 16-bit signed"]
    #[doc = "          integers in native byte-order"]
    #[doc = " @param[in] size size of the data buffer (in samples)"]
    #[doc = ""]
    #[doc = " @return 0 on error, 1 on success"]
    pub fn chromaprint_feed(
        ctx: *mut ChromaprintContext,
        data: *const i16,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = " Process any remaining buffered audio data."]
    #[doc = ""]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    #[doc = ""]
    #[doc = " @return 0 on error, 1 on success"]
    pub fn chromaprint_finish(ctx: *mut ChromaprintContext) -> ::std::os::raw::c_int;

    #[doc = " Return the calculated fingerprint as a compressed string."]
    #[doc = ""]
    #[doc = " The caller is responsible for freeing the returned pointer using"]
    #[doc = " chromaprint_dealloc()."]
    #[doc = ""]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    #[doc = " @param[out] fingerprint pointer to a pointer, where a pointer to the allocated array"]
    #[doc = "                 will be stored"]
    #[doc = ""]
    #[doc = " @return 0 on error, 1 on success"]
    pub fn chromaprint_get_fingerprint(
        ctx: *mut ChromaprintContext,
        fingerprint: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    #[doc = " Return the calculated fingerprint as an array of 32-bit integers."]
    #[doc = ""]
    #[doc = " The caller is responsible for freeing the returned pointer using"]
    #[doc = " chromaprint_dealloc()."]
    #[doc = ""]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    #[doc = " @param[out] fingerprint pointer to a pointer, where a pointer to the allocated array"]
    #[doc = "                 will be stored"]
    #[doc = " @param[out] size number of items in the returned raw fingerprint"]
    #[doc = ""]
    #[doc = " @return 0 on error, 1 on success"]
    pub fn chromaprint_get_raw_fingerprint(
        ctx: *mut ChromaprintContext,
        fingerprint: *mut *mut u32,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = " Return the length of the current raw fingerprint."]
    #[doc = ""]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    #[doc = " @param[out] size number of items in the current raw fingerprint"]
    #[doc = ""]
    #[doc = " @return 0 on error, 1 on success"]
    pub fn chromaprint_get_raw_fingerprint_size(
        ctx: *mut ChromaprintContext,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = " Return 32-bit hash of the calculated fingerprint."]
    #[doc = ""]
    #[doc = " See chromaprint_hash_fingerprint() for details on how to use the hash."]
    #[doc = ""]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    #[doc = " @param[out] hash pointer to a 32-bit integer where the hash will be stored"]
    #[doc = ""]
    #[doc = " @return 0 on error, 1 on success"]
    pub fn chromaprint_get_fingerprint_hash(
        ctx: *mut ChromaprintContext,
        hash: *mut u32,
    ) -> ::std::os::raw::c_int;

    #[doc = " Clear the current fingerprint, but allow more data to be processed."]
    #[doc = ""]
    #[doc = " This is useful if you are processing a long stream and want to many"]
    #[doc = " smaller fingerprints, instead of waiting for the entire stream to be"]
    #[doc = " processed."]
    #[doc = ""]
    #[doc = " @param[in] ctx Chromaprint context pointer"]
    #[doc = ""]
    #[doc = " @return 0 on error, 1 on success"]
    pub fn chromaprint_clear_fingerprint(ctx: *mut ChromaprintContext) -> ::std::os::raw::c_int;

    #[doc = " Compress and optionally base64-encode a raw fingerprint"]
    #[doc = ""]
    #[doc = " The caller is responsible for freeing the returned pointer using"]
    #[doc = " chromaprint_dealloc()."]
    #[doc = ""]
    #[doc = " @param[in] fp pointer to an array of 32-bit integers representing the raw"]
    #[doc = "        fingerprint to be encoded"]
    #[doc = " @param[in] size number of items in the raw fingerprint"]
    #[doc = " @param[in] algorithm Chromaprint algorithm version which was used to generate the"]
    #[doc = "               raw fingerprint"]
    #[doc = " @param[out] encoded_fp pointer to a pointer, where the encoded fingerprint will be"]
    #[doc = "                stored"]
    #[doc = " @param[out] encoded_size size of the encoded fingerprint in bytes"]
    #[doc = " @param[in] base64 Whether to return binary data or base64-encoded ASCII data. The"]
    #[doc = "            compressed fingerprint will be encoded using base64 with the"]
    #[doc = "            URL-safe scheme if you set this parameter to 1. It will return"]
    #[doc = "            binary data if it\'s 0."]
    #[doc = ""]
    #[doc = " @return 0 on error, 1 on success"]
    pub fn chromaprint_encode_fingerprint(
        fp: *const u32,
        size: ::std::os::raw::c_int,
        algorithm: ::std::os::raw::c_int,
        encoded_fp: *mut *mut ::std::os::raw::c_char,
        encoded_size: *mut ::std::os::raw::c_int,
        base64: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = " Uncompress and optionally base64-decode an encoded fingerprint"]
    #[doc = ""]
    #[doc = " The caller is responsible for freeing the returned pointer using"]
    #[doc = " chromaprint_dealloc()."]
    #[doc = ""]
    #[doc = " @param[in] encoded_fp pointer to an encoded fingerprint"]
    #[doc = " @param[in] encoded_size size of the encoded fingerprint in bytes"]
    #[doc = " @param[out] fp pointer to a pointer, where the decoded raw fingerprint (array"]
    #[doc = "        of 32-bit integers) will be stored"]
    #[doc = " @param[out] size Number of items in the returned raw fingerprint"]
    #[doc = " @param[out] algorithm Chromaprint algorithm version which was used to generate the"]
    #[doc = "               raw fingerprint"]
    #[doc = " @param[in] base64 Whether the encoded_fp parameter contains binary data or"]
    #[doc = "            base64-encoded ASCII data. If 1, it will base64-decode the data"]
    #[doc = "            before uncompressing the fingerprint."]
    #[doc = ""]
    #[doc = " @return 0 on error, 1 on success"]
    pub fn chromaprint_decode_fingerprint(
        encoded_fp: *const ::std::os::raw::c_char,
        encoded_size: ::std::os::raw::c_int,
        fp: *mut *mut u32,
        size: *mut ::std::os::raw::c_int,
        algorithm: *mut ::std::os::raw::c_int,
        base64: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = " Generate a single 32-bit hash for a raw fingerprint."]
    #[doc = ""]
    #[doc = " If two fingerprints are similar, their hashes generated by this function"]
    #[doc = " will also be similar. If they are significantly different, their hashes"]
    #[doc = " will most likely be significantly different as well, but you can\'t rely"]
    #[doc = " on that."]
    #[doc = ""]
    #[doc = " You compare two hashes by counting the bits in which they differ. Normally"]
    #[doc = " that would be something like POPCNT(hash1 XOR hash2), which returns a"]
    #[doc = " number between 0 and 32. Anthing above 15 means the hashes are"]
    #[doc = " completely different."]
    #[doc = ""]
    #[doc = " @param[in] fp pointer to an array of 32-bit integers representing the raw"]
    #[doc = "        fingerprint to be hashed"]
    #[doc = " @param[in] size number of items in the raw fingerprint"]
    #[doc = " @param[out] hash pointer to a 32-bit integer where the hash will be stored"]
    #[doc = ""]
    #[doc = " @return 0 on error, 1 on success"]
    pub fn chromaprint_hash_fingerprint(
        fp: *const u32,
        size: ::std::os::raw::c_int,
        hash: *mut u32,
    ) -> ::std::os::raw::c_int;

    #[doc = " Free memory allocated by any function from the Chromaprint API."]
    #[doc = ""]
    #[doc = " @param ptr pointer to be deallocated"]
    pub fn chromaprint_dealloc(ptr: *mut ::std::os::raw::c_void);
}
