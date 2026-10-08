use crate::nvcore::{
    cacheutils::rebuild_bindhelp_cache,
    editorbindhelp::BindhelpMode,
    narraverse::{Narraverse, NarraverseState},
};

pub fn enter_bindhelp(nv: &mut Narraverse, mode: BindhelpMode) {
    nv.bindhelp_mode = mode;
    rebuild_bindhelp_cache(nv);
    nv.state = NarraverseState::BindHelp;
}
