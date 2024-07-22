#![deny(clippy::all)]

use epub::doc::EpubDoc;

#[cfg_attr(feature = "node", napi_derive::napi(object))]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
#[derive(Debug, Clone)]
pub struct Metadata {
	pub title: String,
}

#[cfg_attr(feature = "node", napi_derive::napi(object))]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
#[derive(Debug, Clone)]
pub struct Page {
	pub content: String,
	pub mime: String,
}


#[cfg_attr(feature = "node", napi_derive::napi(object))]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
#[derive(Debug, Clone)]
pub struct Epub {
	pub metadata: Metadata,
	pub pages: Vec<Page>,
}

pub fn read_epub(path: String) -> Epub {
	let doc = EpubDoc::new(path);
	let mut reader = doc.unwrap();
	let (content, mime) = reader.get_current_str().unwrap();
	let mut epub = Epub {
		metadata: Metadata {
			title: reader.mdata("title").unwrap()
		},
		pages: vec![],
	};
	epub.pages.push(Page {
		content,
		mime,
	});
	while reader.go_next() {
		let (content, mime) = reader.get_current_str().unwrap();
		epub.pages.push(Page {
			content,
			mime,
		});
	}
	epub
}
