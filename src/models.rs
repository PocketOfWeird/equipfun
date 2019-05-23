use uuid::Uuid;
use chrono::{ DateTime, Utc };

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
#[graphql(description="A barcoded piece of equipment")]
pub struct Piece {
    #[graphql(description="The unique id of the piece")]
    pub id: Uuid,
    #[graphql(description="The barcode of the piece")]
    pub barcode: String,
    #[graphql(description="The name of the piece")]
    pub name: String,
    #[graphql(description="The status of the piece")]
    pub status: Status,
    #[graphql(description="The id of the equipment the piece belongs to")]
    pub equipment_id: Uuid,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description="A specific type of equipment")]
pub struct Equipment {
    #[graphql(description="The unique id of the equipment")]
    pub id: Uuid,
    #[graphql(description="The manufacturer of the equipment")]
    pub manufacturer: Manufacturer,
    #[graphql(description="The model of the equipment")]
    pub model: Model,
    #[graphql(description="The barcoded pieces of the equipment")]
    pub pieces: Vec<Piece>,
    #[graphql(description="The id of the kit the equipment may belong to")]
    pub kit_id: Uuid,
    #[graphql(description="The id of the category the equipment may belong to")]
    pub category_id: Uuid,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description="A kit of different types of equipment")]
pub struct Kit {
    #[graphql(description="The unique id of the kit")]
    pub id: Uuid,
    #[graphql(description="The name of the kit")]
    pub name: String,
    #[graphql(description="The equipment in the kit")]
    pub equipment: Vec<Equipment>,
    #[graphql(description="The id of the category the kit may belong to")]
    pub category_id: Uuid,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description="A category of equipment and kits")]
pub struct Category {
    #[graphql(description="The unique id of the category")]
    pub id: Uuid,
    #[graphql(description="The name of the category")]
    pub name: String,
    #[graphql(description="The equipment in the category that is not in a kit")]
    pub kitless_equipment: Vec<Equipment>,
    #[graphql(description="The kits in the category")]
    pub kits: Vec<Kit>,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description="A room that can be reserved")]
pub struct Room {
    #[graphql(description="The unique id of the room")]
    pub id: Uuid,
    #[graphql(description="The name of the room")]
    pub name: String,
    #[graphql(description="The location of the room")]
    pub location: String,
    #[graphql(description="The description of the room")]
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
#[graphql(description="A person in the system")]
struct Person {
    #[graphql(description="The unique id of the person")]
    pub id: Uuid,
    #[graphql(description="The username of the person")]
    pub username: String,
    #[graphql(description="The first name of the person")]
    pub first_name: String,
    #[graphql(description="The last name of the person")]
    pub last_name: String,
    #[graphql(description="The role of the person in the system")]
    pub role: Role,
}

#[derive(juniper::GraphQLEnum)]
pub enum Season {
    FA,
    SP,
    SUM,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description="A semester within an academic year")]
pub struct Semester {
    #[graphql(description="The unique id of the semester")]
    pub id: Uuid,
    #[graphql(description="Either FA, SP, or SUM")]
    pub season: Season,
    pub year: i32,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description="A project done within a course")]
pub struct Project {
    #[graphql(description="The unique id of the project")]
    pub id: Uuid,
    #[graphql(description="The name of the project")]
    pub name: String,
    #[graphql(description="The id of the course the project belongs to")]
    pub course_id: Uuid,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description="An instructional course taken by students in the system")]
pub struct Course {
    #[graphql(description="The unique id of the course")]
    pub id: Uuid,
    #[graphql(description="The name of the course")]
    pub name: String,
    #[graphql(description="The code of the course")]
    pub code: String,
    #[graphql(description="The semester of the course")]
    pub semester: Semester,
    #[graphql(description="The instructor of the course")]
    pub instructor: Person,
    #[graphql(description="The students in the course")]
    pub students: Vec<Person>,
    #[graphql(description="The projects in the course")]
    pub projects: Vec<Project>,
    #[graphql(description="The equipment allowed for use in the course")]
    pub equipment: Vec<Equipment>,
    #[graphql(description="The kits allowed for use in the course")]
    pub kits: Vec<Kit>,
    #[graphql(description="The categories of kits and equipment allowed for use in the course")]
    pub categories: Vec<Category>,
    #[graphql(description="The rooms allowed for use in the course")]
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
