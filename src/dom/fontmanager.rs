// src/dom/fontmanager.rs

use once_cell::sync::Lazy;
use std::cell::RefCell;
use skia_safe::FontMgr;

thread_local! {
    static THREAD_FONT_MGR: RefCell<Option<FontMgr>> = RefCell::new(None);
}

pub fn get_thread_local_font_mgr() -> FontMgr {
    THREAD_FONT_MGR.with(|cell| {
        let mut mgr = cell.borrow_mut();
        if mgr.is_none() {
            *mgr = Some(FontMgr::new());
        }
        mgr.as_ref().unwrap().clone()
    })
}