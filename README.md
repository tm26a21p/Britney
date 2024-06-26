# Britney - GitHub Issues Bulk Creator

Britney is a powerful tool designed to streamline the process of creating multiple GitHub issues. Leveraging AI and customizable templates, Britney allows you to generate bulk issues efficiently, saving time and effort.

## Features

- **Bulk Issue Creation**: Generate multiple issues at once using predefined templates.
- **AI Integration**: Utilize AI to customize issue content dynamically.
- **Customizable Templates**: Define your own templates for issue titles and bodies.

## Getting Started

### Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [ollama](https://ollama.com)

### Installation

1. **Clone the repository**:
   ```sh
   git clone https://github.com/yourusername/Britney.git
   cd Britney
    ```

2. Set up environment variables:
Create a .env file in the root directory and add your GitHub token and other configuration settings:
```sh
    GITHUB_TOKEN=your_github_token
    GITHUB_OWNER=your_github_username
    GITHUB_REPO=your_repository_name
    OLLAMA_MODEL=desired_model_name
```

3. **Build the project**:
```sh
   cargo build --release
```

## Usage

**Run the project**
```sh
   cargo run
```

Britney will read the configuration from the .env file and use the specified templates and AI model to generate issues. You will be prompted to validate the issues before they are created and pushed to your GitHub repository.

## Contributing

Contributions are welcome! Please refer to the [Contribution Guidelines](CONTRIBUTING.md) for more details.
