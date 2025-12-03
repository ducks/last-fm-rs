use std::collections::BTreeMap;

/// Generate Last.fm API signature
///
/// Steps:
/// 1. Sort parameters alphabetically (excluding 'format')
/// 2. Concatenate as name+value pairs
/// 3. Append secret
/// 4. MD5 hash the result
pub fn generate(params: &BTreeMap<String, String>, secret: &str) -> String {
  let mut sig_string = String::new();

  for (key, value) in params.iter() {
    if key != "format" {
      sig_string.push_str(key);
      sig_string.push_str(value);
    }
  }

  sig_string.push_str(secret);

  format!("{:x}", md5::compute(sig_string.as_bytes()))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_signature_generation() {
    let mut params = BTreeMap::new();
    params.insert("method".to_string(), "auth.getSession".to_string());
    params.insert("api_key".to_string(), "testkey".to_string());
    params.insert("token".to_string(), "testtoken".to_string());

    let sig = generate(&params, "testsecret");

    // Expected: MD5("api_keytestkeymethodauth.getSessiontokentesttokentestsecret")
    assert_eq!(sig.len(), 32);
  }

  #[test]
  fn test_signature_excludes_format() {
    let mut params = BTreeMap::new();
    params.insert("method".to_string(), "test".to_string());
    params.insert("format".to_string(), "json".to_string());

    let sig = generate(&params, "secret");

    // format should be excluded from signature
    let expected_input = "methodtestsecret";
    let expected = format!("{:x}", md5::compute(expected_input.as_bytes()));

    assert_eq!(sig, expected);
  }
}
