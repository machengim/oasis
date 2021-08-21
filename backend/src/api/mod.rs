use crate::service::state::State;
use tide::Server;
mod file;
mod site;
mod sys;
mod upload;
mod user;

pub fn mount_api(mut app: Server<State>) -> Server<State> {
    app.at("/api/setup").post(site::post_setup);
    app.at("/api/login").post(user::login);

    app.at("/api/sys/volumes").get(sys::get_system_volumes);
    app.at("api/sys/dirs/:dir").get(sys::get_system_dirs);

    app.at("/api/upload/prepare")
        .post(upload::post_prepare_upload);
    app.at("/api/upload/progress/:upload_id")
        .post(upload::post_upload);
    app.at("/api/upload/finish")
        .post(upload::post_finish_upload);

    app.at("/api/dir/:dir_id").get(file::get_dir_list);
    app.at("/api/dir").post(file::post_create_dir);

    app
}
