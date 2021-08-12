use crate::entity::state::State;
pub mod setup;
pub mod sys;

pub fn api_route(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);
    api.at("/setup").post(setup::post_setup);
    api.at("/sys/volumes").get(sys::get_system_volumes);
    api.at("/sys/dirs/:dir*").get(sys::get_system_dirs);

    api
}
