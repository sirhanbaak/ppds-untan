use yew_router::prelude::*;

#[derive(Clone, Copy, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Dashboard,
    #[at("/formulir-pendaftaran")]
    Formulir,
    #[at("/unggah-berkas")]
    UnggahBerkas,
    #[at("/hasil-pengumuman")]
    Pengumuman,
    #[not_found]
    #[at("/404")]
    NotFound,
}
