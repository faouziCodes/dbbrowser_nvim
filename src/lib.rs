use nvim_oxi as oxi;
use oxi::Dictionary;

pub mod browsers;
pub mod config;
pub mod ui;

#[oxi::module]
pub fn dbbrowser() -> oxi::Result<oxi::Dictionary> {
    return Ok(Dictionary::from_iter([(
        "ui",
        oxi::Function::from_fn(move |()| ui::ui()),
    )]));
}
