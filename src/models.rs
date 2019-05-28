use uuid::Uuid;
use chrono::{ DateTime, Utc };
use rusted_cypher::cypher::result::Row;

#[derive(juniper::GraphQLObject)]
/// A model produced by a manufacturer
pub struct Model {
    /// The unique id of the model
    pub id: Uuid,
    // The name of the model
    pub name: String,
    /// The id of the manufacturer the model belongs to
    pub manufacturer_id: Uuid,
}

#[derive(juniper::GraphQLObject)]
/// A manufacturer of equipment
pub struct Manufacturer {
    /// The unique id of the manufacturer
    pub id: Uuid,
    // The name of the manufacturer
    pub name: String,
}

impl Manufacturer {
    pub fn mapper(row: &Row) -> Manufacturer {
        let id: String = row.get("n.id").unwrap();
        return Manufacturer {
            id: Uuid::parse_str(&id).unwrap(),
            name: row.get("n.name").unwrap(),
        };
    }
    pub fn cypher_query_single(id: Uuid) -> String {
        return format!("MATCH (n:Manufacturer) WHERE n.id={:?} RETURN n.id, n.name", id.to_string());
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum Status {
    CheckedIn,
    CheckedOut,
    Broken,
    Dirty,
    Missing,
    Retired,
}

#[derive(juniper::GraphQLObject)]
/// A barcoded piece of equipment
pub struct Piece {
    /// The unique id of the piece
    pub id: Uuid,
    /// The barcode of the piece
    pub barcode: String,
    /// The name of the piece
    pub name: String,
    /// The status of the piece
    pub status: Status,
    /// The id of the equipment the piece belongs to
    pub equipment_id: Uuid,
}

#[derive(juniper::GraphQLObject)]
/// A specific type of equipment
pub struct Equipment {
    /// The unique id of the equipment
    pub id: Uuid,
    /// The manufacturer of the equipment
    pub manufacturer: Manufacturer,
    /// The model of the equipment
    pub model: Model,
    /// The barcoded pieces of the equipment
    pub pieces: Vec<Piece>,
    /// The id of the kit the equipment may belong to
    pub kit_id: Uuid,
    /// The id of the category the equipment may belong to
    pub category_id: Uuid,
}

#[derive(juniper::GraphQLObject)]
/// A kit of different types of equipment
pub struct Kit {
    /// The unique id of the kit
    pub id: Uuid,
    /// The name of the kit
    pub name: String,
    /// The equipment in the kit
    pub equipment: Vec<Equipment>,
    /// The id of the category the kit may belong to
    pub category_id: Uuid,
}

#[derive(juniper::GraphQLObject)]
/// A category of equipment and kits
pub struct Category {
    /// The unique id of the category
    pub id: Uuid,
    /// The name of the category
    pub name: String,
    /// The equipment in the category that is not in a kit
    pub kitless_equipment: Vec<Equipment>,
    /// The kits in the category
    pub kits: Vec<Kit>,
}

#[derive(juniper::GraphQLObject)]
/// A room that can be reserved
pub struct Room {
    /// The unique id of the room
    pub id: Uuid,
    /// The name of the room
    pub name: String,
    /// The location of the room
    pub location: String,
    /// The description of the room
    pub description: String,
}

#[derive(juniper::GraphQLEnum)]
pub enum Role {
    Admin,
    LabWorker,
    Instructor,
    Student,
}

#[derive(juniper::GraphQLObject)]
/// A person in the system
pub struct Person {
    /// The unique id of the person
    pub id: Uuid,
    /// The username of the perso
    pub username: String,
    /// The first name of the person
    pub first_name: String,
    /// The last name of the person
    pub last_name: String,
    /// The role of the person in the system
    pub role: Role,
}

#[derive(juniper::GraphQLEnum)]
pub enum Season {
    FA,
    SP,
    SUM,
}

#[derive(juniper::GraphQLObject)]
/// A semester within an academic year
pub struct Semester {
    /// The unique id of the semester
    pub id: Uuid,
    /// Either FA, SP, or SUM
    pub season: Season,
    /// The year of the semester
    pub year: i32,
}

#[derive(juniper::GraphQLObject)]
/// A project done within a course
pub struct Project {
    /// The unique id of the project
    pub id: Uuid,
    /// The name of the project
    pub name: String,
    /// The id of the course the project belongs to
    pub course_id: Uuid,
}

#[derive(juniper::GraphQLObject)]
/// An instructional course taken by students in the system
pub struct Course {
    /// The unique id of the course
    pub id: Uuid,
    /// The name of the course
    pub name: String,
    /// The code of the course
    pub code: String,
    /// The semester of the course
    pub semester: Semester,
    /// The instructor of the course
    pub instructor: Person,
    /// The students in the course
    pub students: Vec<Person>,
    /// The projects in the course
    pub projects: Vec<Project>,
    /// The equipment allowed for use in the course
    pub equipment: Vec<Equipment>,
    /// The kits allowed for use in the course
    pub kits: Vec<Kit>,
    /// The categories of kits and equipment allowed for use in the course
    pub categories: Vec<Category>,
    /// The rooms allowed for use in the course
    pub rooms: Vec<Room>,
}

#[derive(juniper::GraphQLObject)]
/// A reservation of equipment, kits, or rooms
pub struct Reservation {
    /// The unique id of the reservation
    pub id: Uuid,
    /// The person requesting the reservation
    pub requestor: Person,
    /// The person creating the reservation, could be the same as the requestor
    pub creator: Person,
    /// The course the reservation is for
    pub course: Course,
    /// The project the reservation is for
    pub project: Project,
    /// The equipment on the reservation
    pub equipment: Vec<Equipment>,
    /// The kits on the reservation
    pub kits: Vec<Kit>,
    /// The rooms on the reservation
    pub rooms: Vec<Room>,
    /// The start date and time of the reservation
    pub start: DateTime<Utc>,
    /// The end date and time of the reservation
    pub end: DateTime<Utc>,
}
