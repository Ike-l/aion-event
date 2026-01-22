

pub mod prelude {
    #[derive(Hash, PartialEq, Eq)]
    pub struct Event;

    // double buffer
    pub struct CurrentEvents;
    pub struct NextEvents;
}