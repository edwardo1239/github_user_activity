# GitHub User Activity Viewer

This project is a Rust-based application that fetches and displays GitHub user activity. It's designed to help improve programming skills and is inspired by the [GitHub User Activity project](https://roadmap.sh/projects/github-user-activity) from roadmap.sh.

## Features

- Fetch recent GitHub activity for a specified user
- Process and display events such as pushes, creates, and other GitHub activities
- Console-based output for easy viewing

## Installation

To install and run this project, follow these steps:

1. Ensure you have Rust and Cargo installed on your system. If not, you can install them from [rustup.rs](https://rustup.rs/).

2. Clone this repository:
   ```
   git clon https://github.com/edwardo1239/github_user_activity
   cd github-user-activity-viewer
   ```

3. Build the project:
   ```
   cargo build --release
   ```

4. Run the application:
   ```
   cargo run --release
   ```

## Usage

After running the application, you will be prompted to enter a GitHub username. The application will then fetch and display the recent activity for that user.

## How It Works

1. The application uses the GitHub API to fetch recent events for a specified user.
2. It processes the JSON response and extracts relevant information such as the event type, actor, repository, and description.
3. The processed data is then formatted and displayed in the console.

## Project Structure

- `main.rs`: Contains the main application logic and user interface.
- `api.rs`: Handles API requests to GitHub.
- `models.rs`: Defines data structures used in the application.
- `utils.rs`: Contains utility functions for data processing and display.

## Dependencies

This project uses several Rust crates:
- `reqwest`: For making HTTP requests
- `tokio`: For asynchronous runtime
- `serde` and `serde_json`: For JSON parsing
- (List any other major dependencies here)

## Contributing

Contributions to this project are welcome! Please feel free to submit a Pull Request.

## Acknowledgements

This project is inspired by the [GitHub User Activity project](https://roadmap.sh/projects/github-user-activity) from roadmap.sh. We are grateful for their project ideas and resources for learning and improving programming skills.

## License

This project is open source and available under the [MIT License](LICENSE).
