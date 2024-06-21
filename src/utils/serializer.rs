use serde::Serializer;

use super::app::get_file_url;

/// add full file url Serializer method
pub fn file_url<S>(value: &Option<String>, slz: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  match value {
    Some(ref val) => slz.serialize_some(&get_file_url(val)),
    None => slz.serialize_none(),
  }
}
