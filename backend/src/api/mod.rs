mod file;
mod site;
mod sys;
mod user;
use crate::service::state::State;
use tide::Server;

pub fn mount_api(mut app: Server<State>) -> Server<State> {
    app.at("/api/setup").post(site::post_setup);
    app.at("/api/login").post(user::login);

    app.at("/api/sys/volumes").get(sys::get_system_volumes);
    app.at("api/sys/dirs/:dir").get(sys::get_system_dirs);

    app.at("/api/dir").get(file::get_dir_contents);
    app.at("/api/dir").post(file::post_dir);
    app.at("/api/dir/:dir_path").get(file::get_dir_contents);

    // app.at("/api/upload/prepare")
    //     .post(upload::post_prepare_upload);
    // app.at("/api/upload/progress/:upload_id")
    //     .post(upload::post_upload);
    // app.at("/api/upload/finish")
    //     .post(upload::post_finish_upload);

    // app.at("/api/file/:file_id").get(file::get_file);
    // app.at("/api/dir").post(file::post_create_dir);

    // app.at("/api/file/:file_id").put(file::put_rename_file);
    // app.at("/api/file/:file_id").delete(file::delete_file);

    app
}
