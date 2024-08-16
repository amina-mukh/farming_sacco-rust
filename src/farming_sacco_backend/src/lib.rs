#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use regex::Regex;
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Farmer {
    farmer_id: String,
    owner: String,
    name: String,
    email: String,
    phone_number: String,
    created_at: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct FarmPlot {
    id: String,
    farmer_id: String,
    size: String,
    location: String,
    reserved_until: String,
    created_at: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct FarmingActivity {
    id: String,
    plot_id: String,
    description: String,
    date: String,
    created_at: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Resource {
    id: String,
    name: String,
    quantity: u64,
    available: bool,
    created_at: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Event {
    id: String,
    title: String,
    description: String,
    date: String,
    location: String,
    created_at: String,
}

impl Storable for Farmer {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Farmer {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for FarmPlot {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for FarmPlot {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for FarmingActivity {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for FarmingActivity {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Resource {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Resource {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Event {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Event {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static FARMERS_STORAGE: RefCell<StableBTreeMap<u64, Farmer, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static PLOTS_STORAGE: RefCell<StableBTreeMap<u64, FarmPlot, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static ACTIVITIES_STORAGE: RefCell<StableBTreeMap<u64, FarmingActivity, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static RESOURCES_STORAGE: RefCell<StableBTreeMap<u64, Resource, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static EVENTS_STORAGE: RefCell<StableBTreeMap<u64, Event, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct FarmerPayload {
    name: String,
    email: String,
    phone_number: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct PlotPayload {
    farmer_id: String,
    size: String,
    location: String,
    reserved_until: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct ActivityPayload {
    plot_id: String,
    description: String,
    date: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct ResourcePayload {
    name: String,
    quantity: u64,
    available: bool,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct EventPayload {
    title: String,
    description: String,
    date: String,
    location: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

// Function to create a new farmer profile
#[ic_cdk::update]
fn create_farmer_profile(payload: FarmerPayload) -> Result<Farmer, Message> {
    if payload.name.is_empty() || payload.email.is_empty() || payload.phone_number.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'name', 'email', and 'phone_number' are provided.".to_string(),
        ));
    }

    // Validate email
    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    if !email_regex.is_match(&payload.email) {
        return Err(Message::InvalidPayload(
            "Invalid email format: Ensure the email is in the correct format.".to_string(),
        ));
    }

    // Validate the email address to make it unique
    let email = payload.email.clone();
    let email_exists = FARMERS_STORAGE.with(|storage| {
        storage.borrow().iter().any(|(_, farmer)| farmer.email == email)
    });
    if email_exists {
        return Err(Message::InvalidPayload(
            "Email already exists: Ensure the email address is unique.".to_string(),
        ));
    }

    // Validate phone number
    let phone_number_regex = Regex::new(r"^\d{10}$").unwrap();
    if !phone_number_regex.is_match(&payload.phone_number) {
        return Err(Message::InvalidPayload(
            "Invalid phone number: Ensure the phone number is in the correct format.".to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let farmer = Farmer {
        farmer_id: id.to_string(),
        owner: ic_cdk::caller().to_string(),
        name: payload.name,
        email: payload.email,
        phone_number: payload.phone_number,
        created_at: time().to_string(),
    };

    FARMERS_STORAGE.with(|storage| storage.borrow_mut().insert(id, farmer.clone()));

    Ok(farmer)
}

// Function to update a farmer profile
#[ic_cdk::update]
fn update_farmer_profile(farmer_id: String, payload: FarmerPayload) -> Result<Farmer, Message> {
    if payload.name.is_empty() || payload.email.is_empty() || payload.phone_number.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'name', 'email', and 'phone_number' are provided.".to_string(),
        ));
    }

    // Validate email
    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    if !email_regex.is_match(&payload.email) {
        return Err(Message::InvalidPayload(
            "Invalid email format: Ensure the email is in the correct format.".to_string(),
        ));
    }

    // Validate the email address to make it unique
    let email = payload.email.clone();
    let email_exists = FARMERS_STORAGE.with(|storage| {
        storage.borrow().iter().any(|(_, farmer)| farmer.email == email)
    });
    if email_exists {
        return Err(Message::InvalidPayload(
            "Email already exists: Ensure the email address is unique.".to_string(),
        ));
    }

    // Validate phone number
    let phone_number_regex = Regex::new(r"^\d{10}$").unwrap();
    if !phone_number_regex.is_match(&payload.phone_number) {
        return Err(Message::InvalidPayload(
            "Invalid phone number: Ensure the phone number is in the correct format.".to_string(),
        ));
    }

    let farmer_id = farmer_id.parse::<u64>().map_err(|_| Message::InvalidPayload(
        "Invalid farmer_id: Ensure the farmer ID is valid.".to_string(),
    ))?;

    let mut updated_farmer = FARMERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&farmer_id)
            .map(|farmer| farmer.clone())
            .ok_or(Message::NotFound(
                "Farmer not found: Ensure the farmer ID is correct.".to_string(),
            ))
    })?;

    updated_farmer.name = payload.name;
    updated_farmer.email = payload.email;
    updated_farmer.phone_number = payload.phone_number;

    FARMERS_STORAGE.with(|storage| storage.borrow_mut().insert(farmer_id, updated_farmer.clone()));

    Ok(updated_farmer)
}

// Function to update a farm plot
#[ic_cdk::update]
fn update_farm_plot(plot_id: String, payload: PlotPayload) -> Result<FarmPlot, Message> {
    if payload.farmer_id.is_empty() || payload.size.is_empty() || payload.location.is_empty() || payload.reserved_until.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'farmer_id', 'size', 'location', and 'reserved_until' are provided.".to_string(),
        ));
    }

    // Validate the farmer_id to ensure it exists
    let farmer_id = payload.farmer_id.parse::<u64>().map_err(|_| Message::InvalidPayload(
        "Invalid farmer_id: Ensure the farmer ID is valid.".to_string(),
    ))?;
    let farmer_exists = FARMERS_STORAGE.with(|storage| storage.borrow().contains_key(&farmer_id));
    if !farmer_exists {
        return Err(Message::InvalidPayload(
            "Farmer ID does not exist: Ensure the farmer ID is valid.".to_string(),
        ));
    }

    let plot_id = plot_id.parse::<u64>().map_err(|_| Message::InvalidPayload(
        "Invalid plot_id: Ensure the plot ID is valid.".to_string(),
    ))?;

    let mut updated_plot = PLOTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&plot_id)
            .map(|plot| plot.clone())
            .ok_or(Message::NotFound(
                "Plot not found: Ensure the plot ID is correct.".to_string(),
            ))
    })?;

    updated_plot.farmer_id = payload.farmer_id;
    updated_plot.size = payload.size;
    updated_plot.location = payload.location;
    updated_plot.reserved_until = payload.reserved_until;

    PLOTS_STORAGE.with(|storage| storage.borrow_mut().insert(plot_id, updated_plot.clone()));

    Ok(updated_plot)
}

// Function to update a farming activity
#[ic_cdk::update]
fn update_farming_activity(activity_id: String, payload: ActivityPayload) -> Result<FarmingActivity, Message> {
    if payload.plot_id.is_empty() || payload.description.is_empty() || payload.date.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'plot_id', 'description', and 'date' are provided.".to_string(),
        ));
    }

    // Validate the plot_id to ensure it exists
    let plot_id = payload.plot_id.parse::<u64>().map_err(|_| Message::InvalidPayload(
        "Invalid plot_id: Ensure the plot ID is valid.".to_string(),
    ))?;
    let plot_exists = PLOTS_STORAGE.with(|storage| storage.borrow().contains_key(&plot_id));
    if !plot_exists {
        return Err(Message::InvalidPayload(
            "Plot ID does not exist: Ensure the plot ID is valid.".to_string(),
        ));
    }

    let activity_id = activity_id.parse::<u64>().map_err(|_| Message::InvalidPayload(
        "Invalid activity_id: Ensure the activity ID is valid.".to_string(),
    ))?;

    let mut updated_activity = ACTIVITIES_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&activity_id)
            .map(|activity| activity.clone())
            .ok_or(Message::NotFound(
                "Activity not found: Ensure the activity ID is correct.".to_string(),
            ))
    })?;

    updated_activity.plot_id = payload.plot_id;
    updated_activity.description = payload.description;
    updated_activity.date = payload.date;

    ACTIVITIES_STORAGE.with(|storage| storage.borrow_mut().insert(activity_id, updated_activity.clone()));

    Ok(updated_activity)
}

// Function to create or update a resource
#[ic_cdk::update]
fn upsert_resource(resource_id: String, payload: ResourcePayload) -> Result<Resource, Message> {
    if payload.name.is_empty() || payload.quantity == 0 {
        return Err(Message::InvalidPayload(
            "Ensure 'name' and 'quantity' are provided and quantity is greater than 0.".to_string(),
        ));
    }

    let id = resource_id.parse::<u64>().map_err(|_| Message::InvalidPayload(
        "Invalid resource_id: Ensure the resource ID is valid.".to_string(),
    ))?;

    let resource = Resource {
        id: id.to_string(),
        name: payload.name,
        quantity: payload.quantity,
        available: payload.available,
        created_at: time().to_string(),
    };

    RESOURCES_STORAGE.with(|storage| storage.borrow_mut().insert(id, resource.clone()));

    Ok(resource)
}

// Function to create or update an event
#[ic_cdk::update]
fn upsert_event(event_id: String, payload: EventPayload) -> Result<Event, Message> {
    if payload.title.is_empty() || payload.description.is_empty() || payload.date.is_empty() || payload.location.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'title', 'description', 'date', and 'location' are provided.".to_string(),
        ));
    }

    let id = event_id.parse::<u64>().map_err(|_| Message::InvalidPayload(
        "Invalid event_id: Ensure the event ID is valid.".to_string(),
    ))?;

    let event = Event {
        id: id.to_string(),
        title: payload.title,
        description: payload.description,
        date: payload.date,
        location: payload.location,
        created_at: time().to_string(),
    };

    EVENTS_STORAGE.with(|storage| storage.borrow_mut().insert(id, event.clone()));

    Ok(event)
}
ic_cdk::export_candid!();