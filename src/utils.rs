pub(crate) fn prefix_key(prefix: &Vec<u8>, key: &[u8]) -> Vec<u8> {
  [prefix as &[u8], key].concat()
}
