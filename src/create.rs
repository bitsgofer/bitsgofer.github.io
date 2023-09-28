use crate::DocumentType;

/// Create a document.
pub fn create(doctype: DocumentType, name: &str, slug: &str) {
    println!(
        "Creating new {:?} with name={} (slug={})",
        doctype, name, slug
    );
}
