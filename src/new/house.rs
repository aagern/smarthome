#![allow(unused)]
use crate::InputError;
use crate::new::room::Room;
use std::collections::HashMap;
use std::fmt;
use tracing::debug;

// House implemetation with HashMap
pub struct House {
    rooms: HashMap<String, Room>,
}

impl House {
    // Constructor
    pub fn new(room_name: String, room: Room) -> Self {
        let mut rooms = HashMap::new();
        rooms.insert(room_name, room);
        House { rooms }
    }

    // Add room. Returns added room
    pub fn add_room(&mut self, room_name: String, room: Room) -> Option<Room> {
        self.rooms.insert(room_name, room)
    }

    // Remove room. Returns removed room. If there is only 1 room, returns error
    pub fn remove_room(&mut self, room_name: &str) -> Result<Room, InputError> {
        if self.rooms.len() == 1 && self.rooms.contains_key(room_name) {
            return Err(InputError::RoomNotFound(room_name.to_string()));
        }
        self.rooms
            .remove(room_name)
            .ok_or(InputError::RoomNotFound(room_name.to_string()))
    }

    // Get immutable link to room
    pub fn get_room(&self, room_name: &str) -> Option<&Room> {
        self.rooms.get(room_name)
    }

    // Get mutable link to room
    pub fn get_room_mut(&mut self, room_name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(room_name)
    }

    // Check if house has room
    pub fn has_room(&self, room_name: &str) -> bool {
        self.rooms.contains_key(room_name)
    }

    // Get number of rooms
    pub fn room_count(&self) -> usize {
        self.rooms.len()
    }

    // Iterate over rooms
    pub fn iter_rooms(&self) -> impl Iterator<Item = (&String, &Room)> {
        self.rooms.iter()
    }
}

// House formatting
impl fmt::Display for House {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "==== House Report ====")?;
        for (room_name, room) in &self.rooms {
            writeln!(f, "Room name: {} | Room: {}", room_name, room)?;
        }
        debug!("Creted a report for a house.");
        Ok(())
    }
}

// House debug data
impl fmt::Debug for House {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug!("House {{ rooms: {:?} }}", self.rooms);
        write!(f, "House {{ rooms: {:?} }}", self.rooms)
    }
}
