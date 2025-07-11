//! This module provides a function to create a Last.fm API method signature.

use std::collections::BTreeMap;

/// Generates a Last.fm API method signature.
///
/// According to the [Last.fm API authentication spec](https://www.last.fm/api/authspec#_8-signing-calls),
/// a method signature is an MD5 hash of a string constructed as follows:
///
/// 1.  Take all API call parameters (except `format` and `callback`).
/// 2.  Order them alphabetically by parameter name.
/// 3.  Concatenate each parameter's name and value into a single string (e.g., `keyvalue`).
/// 4.  Concatenate all these `keyvalue` strings into one long string.
/// 5.  Append your API secret to the end of this long string.
/// 6.  The final signature is the MD5 hash of this string.
///
/// This function expects `params` to be a `BTreeMap`, which automatically handles
/// the alphabetical ordering of parameters (step 2). The caller is responsible for
/// ensuring `params` contains all required parameters for the signature (e.g., `api_key`)
/// and excludes any that should not be signed (like `format` or `callback`).
///
/// # Arguments
///
/// * `params` - A map of API method parameters, sorted alphabetically by key.
/// * `api_secret` - Your Last.fm API secret.
///
/// # Returns
///
/// A string containing the 32-character lowercase hex-encoded MD5 signature.
pub fn create_sig(params: &BTreeMap<String, String>, api_secret: &str) -> String {
    // The signature base string is created by concatenating key-value pairs
    // from the alphabetically sorted `params` map.
    let mut sig_base = params.iter().fold(
        // Pre-allocate a string with a reasonable capacity to reduce reallocations.
        String::with_capacity(256),
        |mut acc, (key, value)| {
            acc.push_str(key);
            acc.push_str(value);
            acc
        },
    );

    // The API secret is appended to the end of the concatenated parameters.
    sig_base.push_str(api_secret);

    // The final signature is the MD5 hash of the base string, formatted as a
    // lowercase hexadecimal string.
    let digest = md5::compute(sig_base.as_bytes());
    format!("{:x}", digest)
}
