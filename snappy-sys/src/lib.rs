use libc::{c_int, size_t};

#[link(name = "snappy")]
extern "C" {
    /// Takes the data stored in "input[0..input_length-1]" and stores
    /// it in the array pointed to by "compressed".
    ///
    /// <compressed_length> signals the space available in "compressed".
    /// If it is not at least equal to "snappy_max_compressed_length(input_length)",
    /// SNAPPY_BUFFER_TOO_SMALL is returned. After successful compression,
    /// <compressed_length> contains the true length of the compressed output,
    /// and SNAPPY_OK is returned.
    ///
    /// Example:
    /// ```
    ///   size_t output_length = snappy_max_compressed_length(input_length);
    ///   char* output = (char*)malloc(output_length);
    ///   if (snappy_compress(input, input_length, output, &output_length)
    ///       == SNAPPY_OK) {
    ///     ... Process(output, output_length) ...
    ///   }
    ///   free(output);
    /// ```
    pub fn snappy_compress(
        input: *const u8,
        input_length: size_t,
        compressed: *mut u8,
        compressed_length: *mut size_t,
    ) -> c_int;

    /// Given data in "compressed[0..compressed_length-1]" generated by
    /// calling the snappy_compress routine, this routine stores
    /// the uncompressed data to
    ///   uncompressed[0..uncompressed_length-1].
    /// Returns failure (a value not equal to SNAPPY_OK) if the message
    /// is corrupted and could not be decrypted.
    ///
    /// <uncompressed_length> signals the space available in "uncompressed".
    /// If it is not at least equal to the value returned by
    /// snappy_uncompressed_length for this stream, SNAPPY_BUFFER_TOO_SMALL
    /// is returned. After successful decompression, <uncompressed_length>
    /// contains the true length of the decompressed output.
    ///
    /// Example:
    /// ```
    ///   size_t output_length;
    ///   if (snappy_uncompressed_length(input, input_length, &output_length)
    ///       != SNAPPY_OK) {
    ///     ... fail ...
    ///   }
    ///   char* output = (char*)malloc(output_length);
    ///   if (snappy_uncompress(input, input_length, output, &output_length)
    ///       == SNAPPY_OK) {
    ///     ... Process(output, output_length) ...
    ///   }
    ///   free(output);
    /// ```
    pub fn snappy_uncompress(
        compressed: *const u8,
        compressed_length: size_t,
        uncompressed: *mut u8,
        uncompressed_length: *mut size_t,
    ) -> c_int;

    /// Returns the maximal size of the compressed representation of
    /// input data that is "source_length" bytes in length.
    pub fn snappy_max_compressed_length(source_length: size_t) -> size_t;

    /// REQUIRES: "compressed[]" was produced by snappy_compress()
    /// Returns SNAPPY_OK and stores the length of the uncompressed data in
    /// *result normally. Returns SNAPPY_INVALID_INPUT on parsing error.
    /// This operation takes O(1) time.
    pub fn snappy_uncompressed_length(
        compressed: *const u8,
        compressed_length: size_t,
        result: *mut size_t,
    ) -> c_int;

    /// Check if the contents of "compressed[]" can be uncompressed successfully.
    /// Does not return the uncompressed data; if so, returns SNAPPY_OK,
    /// or if not, returns SNAPPY_INVALID_INPUT.
    /// Takes time proportional to compressed_length, but is usually at least a
    /// factor of four faster than actual decompression.
    pub fn snappy_validate_compressed_buffer(
        compressed: *const u8,
        compressed_length: size_t,
    ) -> c_int;
}
