#![deny(clippy::all)]
use napi_derive::napi;

use llamareader;

#[napi]
pub fn read_epub(path: String) -> llamareader::Epub {
	llamareader::read_epub(path)
}
