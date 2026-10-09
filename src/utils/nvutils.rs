use crate::{editor::bindhelp::BindhelpMode, nvcore::narraverse::{Narraverse, NarraverseState}, utils::cacheutils::rebuild_bindhelp_cache};

pub fn enter_bindhelp(nv: &mut Narraverse, mode: BindhelpMode) {
    nv.bindhelp_mode = mode;
    rebuild_bindhelp_cache(&mut nv.cache, mode);
    nv.state = NarraverseState::BindHelp;
}
