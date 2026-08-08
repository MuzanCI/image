use crate::digest::Digest;
use crate::image::MediaType;

#[derive(Debug, Clone)]
pub struct BlobRef {
    pub domain: String,
    pub namespace: String,
    pub digest: Digest,
    pub size: usize,
    pub media_type: MediaType,
}

impl BlobRef {
    pub fn to_url(&self) -> String {
        format!(
            "https://{}/v2/{}/blobs/{}",
            self.domain, self.namespace, self.digest
        )
    }
}
