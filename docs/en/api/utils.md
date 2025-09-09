---
title: utils
createTime: 2025/09/08 22:31:16
permalink: /en/article/j516dys6/
---
# Utils API

The Utils API provides utility functions and helpers for common operations in RWKV Agent Kit.

## Logger

Logging utilities for debugging and monitoring.

### Constructor

#### `Logger::new(level: LogLevel) -> Logger`

Creates a new logger with specified level.

**Parameters:**
- `level`: Minimum log level

**Returns:**
- `Logger`: New logger instance

#### `Logger::with_file(level: LogLevel, file_path: &str) -> Result<Logger, LogError>`

Creates a logger that writes to a file.

**Parameters:**
- `level`: Minimum log level
- `file_path`: Path to log file

**Returns:**
- `Result<Logger, LogError>`: Logger or error

### Methods

#### `debug(&self, message: &str)`

Logs a debug message.

**Parameters:**
- `message`: Debug message

#### `info(&self, message: &str)`

Logs an info message.

**Parameters:**
- `message`: Info message

#### `warn(&self, message: &str)`

Logs a warning message.

**Parameters:**
- `message`: Warning message

#### `error(&self, message: &str)`

Logs an error message.

**Parameters:**
- `message`: Error message

### Example Usage

```rust
use rwkv_agent_kit::utils::{Logger, LogLevel};

let logger = Logger::new(LogLevel::Info);
logger.info("Agent initialized successfully");
logger.warn("Memory usage is high");
logger.error("Failed to load model");

// File logging
let file_logger = Logger::with_file(LogLevel::Debug, "agent.log")?;
file_logger.debug("Detailed debug information");
```

## Config

Configuration management utilities.

### ConfigLoader

Loads configuration from various sources.

#### Constructor

##### `ConfigLoader::new() -> ConfigLoader`

Creates a new config loader.

**Returns:**
- `ConfigLoader`: New config loader

#### Methods

##### `load_from_file(&self, path: &str) -> Result<Config, ConfigError>`

Loads configuration from a file.

**Parameters:**
- `path`: Configuration file path

**Returns:**
- `Result<Config, ConfigError>`: Configuration or error

##### `load_from_env(&self) -> Result<Config, ConfigError>`

Loads configuration from environment variables.

**Returns:**
- `Result<Config, ConfigError>`: Configuration or error

##### `merge_configs(&self, configs: Vec<Config>) -> Config`

Merges multiple configurations.

**Parameters:**
- `configs`: List of configurations to merge

**Returns:**
- `Config`: Merged configuration

### Config

Represents application configuration.

#### Fields

- `model_path: String` - Path to RWKV model
- `max_tokens: usize` - Maximum tokens per response
- `temperature: f32` - Sampling temperature
- `top_p: f32` - Top-p sampling parameter
- `memory_size: usize` - Memory buffer size
- `tools_enabled: bool` - Whether tools are enabled
- `log_level: LogLevel` - Logging level

#### Methods

##### `default() -> Config`

Creates default configuration.

**Returns:**
- `Config`: Default configuration

##### `validate(&self) -> Result<(), ConfigError>`

Validates the configuration.

**Returns:**
- `Result<(), ConfigError>`: Success or validation error

##### `to_toml(&self) -> Result<String, ConfigError>`

Serializes configuration to TOML.

**Returns:**
- `Result<String, ConfigError>`: TOML string or error

##### `from_toml(toml: &str) -> Result<Config, ConfigError>`

Deserializes configuration from TOML.

**Parameters:**
- `toml`: TOML string

**Returns:**
- `Result<Config, ConfigError>`: Configuration or error

### Example Usage

```rust
use rwkv_agent_kit::utils::{ConfigLoader, Config};

// Load from file
let loader = ConfigLoader::new();
let config = loader.load_from_file("config.toml")?;

// Load from environment
let env_config = loader.load_from_env()?;

// Merge configurations
let final_config = loader.merge_configs(vec![config, env_config]);

// Validate
final_config.validate()?;

// Use configuration
println!("Model path: {}", final_config.model_path);
println!("Max tokens: {}", final_config.max_tokens);
```

## Tokenizer

Text tokenization utilities.

### RwkvTokenizer

Tokenizer for RWKV models.

#### Constructor

##### `RwkvTokenizer::new(vocab_path: &str) -> Result<RwkvTokenizer, TokenizerError>`

Creates a new RWKV tokenizer.

**Parameters:**
- `vocab_path`: Path to vocabulary file

**Returns:**
- `Result<RwkvTokenizer, TokenizerError>`: Tokenizer or error

#### Methods

##### `encode(&self, text: &str) -> Result<Vec<u32>, TokenizerError>`

Encodes text to token IDs.

**Parameters:**
- `text`: Text to encode

**Returns:**
- `Result<Vec<u32>, TokenizerError>`: Token IDs or error

##### `decode(&self, tokens: &[u32]) -> Result<String, TokenizerError>`

Decodes token IDs to text.

**Parameters:**
- `tokens`: Token IDs to decode

**Returns:**
- `Result<String, TokenizerError>`: Decoded text or error

##### `count_tokens(&self, text: &str) -> Result<usize, TokenizerError>`

Counts tokens in text.

**Parameters:**
- `text`: Text to count

**Returns:**
- `Result<usize, TokenizerError>`: Token count or error

##### `truncate(&self, text: &str, max_tokens: usize) -> Result<String, TokenizerError>`

Truncates text to maximum tokens.

**Parameters:**
- `text`: Text to truncate
- `max_tokens`: Maximum number of tokens

**Returns:**
- `Result<String, TokenizerError>`: Truncated text or error

### Example Usage

```rust
use rwkv_agent_kit::utils::RwkvTokenizer;

let tokenizer = RwkvTokenizer::new("vocab.json")?;

// Encode text
let text = "Hello, world!";
let tokens = tokenizer.encode(text)?;
println!("Tokens: {:?}", tokens);

// Decode tokens
let decoded = tokenizer.decode(&tokens)?;
println!("Decoded: {}", decoded);

// Count tokens
let count = tokenizer.count_tokens("This is a longer text")?;
println!("Token count: {}", count);

// Truncate text
let truncated = tokenizer.truncate("Very long text...", 10)?;
println!("Truncated: {}", truncated);
```

## FileUtils

File system utilities.

### Methods

#### `read_file(path: &str) -> Result<String, FileError>`

Reads entire file content.

**Parameters:**
- `path`: File path

**Returns:**
- `Result<String, FileError>`: File content or error

#### `write_file(path: &str, content: &str) -> Result<(), FileError>`

Writes content to file.

**Parameters:**
- `path`: File path
- `content`: Content to write

**Returns:**
- `Result<(), FileError>`: Success or error

#### `append_file(path: &str, content: &str) -> Result<(), FileError>`

Appends content to file.

**Parameters:**
- `path`: File path
- `content`: Content to append

**Returns:**
- `Result<(), FileError>`: Success or error

#### `file_exists(path: &str) -> bool`

Checks if file exists.

**Parameters:**
- `path`: File path

**Returns:**
- `bool`: True if file exists

#### `create_dir_all(path: &str) -> Result<(), FileError>`

Creates directory and all parent directories.

**Parameters:**
- `path`: Directory path

**Returns:**
- `Result<(), FileError>`: Success or error

#### `list_files(dir: &str, extension: Option<&str>) -> Result<Vec<String>, FileError>`

Lists files in directory.

**Parameters:**
- `dir`: Directory path
- `extension`: Optional file extension filter

**Returns:**
- `Result<Vec<String>, FileError>`: File list or error

### Example Usage

```rust
use rwkv_agent_kit::utils::FileUtils;

// Read file
let content = FileUtils::read_file("data.txt")?;
println!("Content: {}", content);

// Write file
FileUtils::write_file("output.txt", "Hello, world!")?;

// Check if file exists
if FileUtils::file_exists("config.toml") {
    println!("Config file found");
}

// Create directories
FileUtils::create_dir_all("data/models")?;

// List files
let rust_files = FileUtils::list_files("src", Some("rs"))?;
for file in rust_files {
    println!("Rust file: {}", file);
}
```

## JsonUtils

JSON manipulation utilities.

### Methods

#### `parse(json: &str) -> Result<serde_json::Value, JsonError>`

Parses JSON string.

**Parameters:**
- `json`: JSON string

**Returns:**
- `Result<serde_json::Value, JsonError>`: Parsed JSON or error

#### `stringify(value: &serde_json::Value) -> Result<String, JsonError>`

Stringifies JSON value.

**Parameters:**
- `value`: JSON value

**Returns:**
- `Result<String, JsonError>`: JSON string or error

#### `pretty_print(value: &serde_json::Value) -> Result<String, JsonError>`

Formats JSON with indentation.

**Parameters:**
- `value`: JSON value

**Returns:**
- `Result<String, JsonError>`: Formatted JSON string or error

#### `merge(a: &serde_json::Value, b: &serde_json::Value) -> serde_json::Value`

Merges two JSON objects.

**Parameters:**
- `a`: First JSON object
- `b`: Second JSON object

**Returns:**
- `serde_json::Value`: Merged JSON object

#### `get_nested(value: &serde_json::Value, path: &str) -> Option<&serde_json::Value>`

Gets nested value using dot notation.

**Parameters:**
- `value`: JSON object
- `path`: Dot-separated path (e.g., "user.profile.name")

**Returns:**
- `Option<&serde_json::Value>`: Nested value or None

### Example Usage

```rust
use rwkv_agent_kit::utils::JsonUtils;
use serde_json::json;

// Parse JSON
let json_str = r#"{"name": "Alice", "age": 30}"#;
let parsed = JsonUtils::parse(json_str)?;

// Stringify JSON
let value = json!({"key": "value"});
let stringified = JsonUtils::stringify(&value)?;

// Pretty print
let pretty = JsonUtils::pretty_print(&value)?;
println!("Pretty JSON:\n{}", pretty);

// Merge objects
let obj1 = json!({"a": 1, "b": 2});
let obj2 = json!({"b": 3, "c": 4});
let merged = JsonUtils::merge(&obj1, &obj2);
// Result: {"a": 1, "b": 3, "c": 4}

// Get nested value
let data = json!({
    "user": {
        "profile": {
            "name": "Alice"
        }
    }
});
let name = JsonUtils::get_nested(&data, "user.profile.name");
println!("Name: {:?}", name);
```

## TimeUtils

Time and date utilities.

### Methods

#### `now() -> SystemTime`

Gets current system time.

**Returns:**
- `SystemTime`: Current time

#### `timestamp() -> u64`

Gets current Unix timestamp.

**Returns:**
- `u64`: Unix timestamp in seconds

#### `timestamp_millis() -> u128`

Gets current Unix timestamp in milliseconds.

**Returns:**
- `u128`: Unix timestamp in milliseconds

#### `format_duration(duration: Duration) -> String`

Formats duration as human-readable string.

**Parameters:**
- `duration`: Duration to format

**Returns:**
- `String`: Formatted duration (e.g., "2h 30m 15s")

#### `parse_duration(duration_str: &str) -> Result<Duration, TimeError>`

Parses duration from string.

**Parameters:**
- `duration_str`: Duration string (e.g., "1h30m", "45s")

**Returns:**
- `Result<Duration, TimeError>`: Parsed duration or error

### Example Usage

```rust
use rwkv_agent_kit::utils::TimeUtils;
use std::time::{Duration, Instant};

// Get current time
let now = TimeUtils::now();
let timestamp = TimeUtils::timestamp();
println!("Current timestamp: {}", timestamp);

// Measure execution time
let start = Instant::now();
// ... some operation ...
let elapsed = start.elapsed();
let formatted = TimeUtils::format_duration(elapsed);
println!("Operation took: {}", formatted);

// Parse duration
let duration = TimeUtils::parse_duration("2h30m")?;
println!("Parsed duration: {:?}", duration);
```

## HashUtils

Hashing and checksum utilities.

### Methods

#### `sha256(data: &[u8]) -> String`

Computes SHA-256 hash.

**Parameters:**
- `data`: Data to hash

**Returns:**
- `String`: Hexadecimal hash string

#### `sha256_string(text: &str) -> String`

Computes SHA-256 hash of string.

**Parameters:**
- `text`: Text to hash

**Returns:**
- `String`: Hexadecimal hash string

#### `md5(data: &[u8]) -> String`

Computes MD5 hash.

**Parameters:**
- `data`: Data to hash

**Returns:**
- `String`: Hexadecimal hash string

#### `md5_string(text: &str) -> String`

Computes MD5 hash of string.

**Parameters:**
- `text`: Text to hash

**Returns:**
- `String`: Hexadecimal hash string

#### `verify_checksum(data: &[u8], expected: &str, algorithm: HashAlgorithm) -> bool`

Verifies data against checksum.

**Parameters:**
- `data`: Data to verify
- `expected`: Expected hash
- `algorithm`: Hash algorithm to use

**Returns:**
- `bool`: True if checksum matches

### Example Usage

```rust
use rwkv_agent_kit::utils::{HashUtils, HashAlgorithm};

// Hash string
let text = "Hello, world!";
let sha256_hash = HashUtils::sha256_string(text);
let md5_hash = HashUtils::md5_string(text);

println!("SHA-256: {}", sha256_hash);
println!("MD5: {}", md5_hash);

// Hash binary data
let data = b"binary data";
let hash = HashUtils::sha256(data);

// Verify checksum
let is_valid = HashUtils::verify_checksum(
    data,
    &hash,
    HashAlgorithm::Sha256
);
println!("Checksum valid: {}", is_valid);
```

## ValidationUtils

Input validation utilities.

### Methods

#### `is_valid_email(email: &str) -> bool`

Validates email address format.

**Parameters:**
- `email`: Email address to validate

**Returns:**
- `bool`: True if valid email format

#### `is_valid_url(url: &str) -> bool`

Validates URL format.

**Parameters:**
- `url`: URL to validate

**Returns:**
- `bool`: True if valid URL format

#### `is_valid_json(json: &str) -> bool`

Validates JSON format.

**Parameters:**
- `json`: JSON string to validate

**Returns:**
- `bool`: True if valid JSON

#### `sanitize_filename(filename: &str) -> String`

Sanitizes filename for filesystem safety.

**Parameters:**
- `filename`: Filename to sanitize

**Returns:**
- `String`: Sanitized filename

#### `validate_range<T: PartialOrd>(value: T, min: T, max: T) -> bool`

Validates value is within range.

**Parameters:**
- `value`: Value to validate
- `min`: Minimum allowed value
- `max`: Maximum allowed value

**Returns:**
- `bool`: True if value is in range

### Example Usage

```rust
use rwkv_agent_kit::utils::ValidationUtils;

// Validate email
let email = "user@example.com";
if ValidationUtils::is_valid_email(email) {
    println!("Valid email: {}", email);
}

// Validate URL
let url = "https://example.com/api";
if ValidationUtils::is_valid_url(url) {
    println!("Valid URL: {}", url);
}

// Validate JSON
let json = r#"{"key": "value"}"#;
if ValidationUtils::is_valid_json(json) {
    println!("Valid JSON");
}

// Sanitize filename
let unsafe_name = "file<>name?.txt";
let safe_name = ValidationUtils::sanitize_filename(unsafe_name);
println!("Safe filename: {}", safe_name);

// Validate range
let temperature = 0.7;
if ValidationUtils::validate_range(temperature, 0.0, 1.0) {
    println!("Temperature in valid range");
}
```

## Error Types

### LogError

Logging-related errors.

### ConfigError

Configuration-related errors.

### TokenizerError

Tokenization-related errors.

### FileError

File operation errors.

### JsonError

JSON processing errors.

### TimeError

Time parsing errors.

### HashAlgorithm

Enum for hash algorithms.

#### Variants

- `Sha256` - SHA-256 algorithm
- `Md5` - MD5 algorithm
- `Sha1` - SHA-1 algorithm

## LogLevel

Enum for logging levels.

#### Variants

- `Trace` - Most verbose logging
- `Debug` - Debug information
- `Info` - General information
- `Warn` - Warning messages
- `Error` - Error messages only