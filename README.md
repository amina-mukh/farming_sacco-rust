# Farming Sacco Backend

This project is a backend service for managing a farming cooperative society (Sacco). It provides APIs to manage farmers, farm plots, activities, resources, and events using an IC (Internet Computer) canister.

## Overview

The backend service allows you to perform CRUD (Create, Read, Update, Delete) operations on the following entities:

- **Farmers**: Manage farmer profiles.
- **Farm Plots**: Manage plots of land within the cooperative.
- **Farming Activities**: Record activities related to farm plots.
- **Resources**: Track resources available within the cooperative.
- **Events**: Manage events associated with the cooperative.

## Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)
- [IC Canister SDK](https://internetcomputer.org/docs/current/developers-guide/cdks/cdks-overview/)

## Setup

1. **Clone the Repository:**

   ```bash
   git clone <repository-url>
   cd farming_sacco_backend
   ```

2. **Update the Cargo Configuration:**

   Rename the deprecated `.cargo/config` to `.cargo/config.toml` if applicable:

   ```bash
   mv ~/.cargo/config ~/.cargo/config.toml
   ```

3. **Build the Canister:**

   To build the WebAssembly (WASM) binary for the canister, run:

   ```bash
   cargo build --release --target wasm32-unknown-unknown --package farming_sacco_backend
   ```

4. **Generate the Candid Interface Definition:**

   Extract the Candid Interface Definition (DID) from the built WASM binary:

   ```bash
   candid-extractor target/wasm32-unknown-unknown/release/farming_sacco_backend.wasm > farming_sacco_backend.did
   ```

## Code Overview

- **src/lib.rs**: Contains the core logic and API implementations for managing farmers, plots, activities, resources, and events.

### Core Structures

- `Farmer`: Represents a farmer's profile.
- `FarmPlot`: Represents a plot of land within the cooperative.
- `FarmingActivity`: Represents activities related to farm plots.
- `Resource`: Represents resources available in the cooperative.
- `Event`: Represents events associated with the cooperative.

### Core Functions

- `create_farmer_profile(payload: FarmerPayload) -> Result<Farmer, Message>`: Creates a new farmer profile.
- `update_farmer_profile(farmer_id: String, payload: FarmerPayload) -> Result<Farmer, Message>`: Updates an existing farmer profile.
- `get_farmer_profile(farmer_id: String) -> Result<Farmer, Message>`: Retrieves a farmer profile by ID.
- `get_all_farmers() -> Result<Vec<Farmer>, Message>`: Retrieves all farmer profiles.
- `create_plot(payload: PlotPayload) -> Result<FarmPlot, Message>`: Creates a new farm plot.
- `get_all_plots() -> Result<Vec<FarmPlot>, Message>`: Retrieves all farm plots.
- `create_activity(payload: ActivityPayload) -> Result<FarmingActivity, Message>`: Creates a new farming activity.
- `get_all_activities() -> Result<Vec<FarmingActivity>, Message>`: Retrieves all farming activities.
- `create_resource(payload: ResourcePayload) -> Result<Resource, Message>`: Creates a new resource.
- `get_all_resources() -> Result<Vec<Resource>, Message>`: Retrieves all resources.
- `create_event(payload: EventPayload) -> Result<Event, Message>`: Creates a new event.
- `get_all_events() -> Result<Vec<Event>, Message>`: Retrieves all events.

## Error Handling

The backend uses the `Message` enum to handle various types of responses, including success, error, not found, and invalid payload scenarios.

## Contributing

If you wish to contribute to this project, please fork the repository and submit a pull request with your changes. Ensure to follow the project's coding standards and include tests for new features or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Feel free to adjust the content as necessary to better fit the specifics of your project or any additional instructions you might want to include.
# farming_sacco-rust
