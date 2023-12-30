use uuid::Uuid;

pub struct Player {
    pub name: String,
    pub uuid: Uuid,
}

impl Player {
    pub fn new(name: String, uuid: Uuid) -> Self {
        Self { name, uuid }
    }
}
