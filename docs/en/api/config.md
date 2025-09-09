# Configuration API

## Overview

The Configuration API provides interfaces and utilities for managing configuration settings across the RWKV Agent Kit ecosystem.

## Classes

### ConfigManager

Main class for managing configuration settings.

#### Constructor

```typescript
constructor(configPath?: string)
```

**Parameters:**
- `configPath`: Optional path to configuration file

#### Methods

##### load()

Loads configuration from file or default settings.

```typescript
load(): Promise<Config>
```

**Returns:**
- `Promise<Config>`: The loaded configuration

##### save()

Saves current configuration to file.

```typescript
save(config: Config): Promise<void>
```

**Parameters:**
- `config`: Configuration object to save

##### get()

Retrieves a configuration value by key.

```typescript
get<T>(key: string): T | undefined
```

**Parameters:**
- `key`: Configuration key

**Returns:**
- `T | undefined`: The configuration value or undefined

##### set()

Sets a configuration value.

```typescript
set<T>(key: string, value: T): void
```

**Parameters:**
- `key`: Configuration key
- `value`: Value to set

##### validate()

Validates the current configuration.

```typescript
validate(): ValidationResult
```

**Returns:**
- `ValidationResult`: Validation result

## Functions

### createDefaultConfig()

Creates a default configuration object.

```typescript
function createDefaultConfig(): Config
```

**Returns:**
- `Config`: Default configuration object

### mergeConfigs()

Merges multiple configuration objects.

```typescript
function mergeConfigs(...configs: Partial<Config>[]): Config
```

**Parameters:**
- `configs`: Configuration objects to merge

**Returns:**
- `Config`: Merged configuration

### validateConfig()

Validates a configuration object.

```typescript
function validateConfig(config: Config): ValidationResult
```

**Parameters:**
- `config`: Configuration to validate

**Returns:**
- `ValidationResult`: Validation result

## Types

### Config

Main configuration interface.

```typescript
interface Config {
  model: ModelConfig;
  agent: AgentConfig;
  memory: MemoryConfig;
  tools: ToolsConfig;
  logging: LoggingConfig;
}
```

### ModelConfig

```typescript
interface ModelConfig {
  path: string;
  maxTokens: number;
  temperature: number;
  topP: number;
  topK: number;
  repeatPenalty: number;
}
```

### AgentConfig

```typescript
interface AgentConfig {
  name: string;
  description?: string;
  systemPrompt?: string;
  maxHistory: number;
  timeout: number;
}
```

### MemoryConfig

```typescript
interface MemoryConfig {
  enabled: boolean;
  maxSize: number;
  persistPath?: string;
  vectorDimension: number;
}
```

### ToolsConfig

```typescript
interface ToolsConfig {
  enabled: string[];
  disabled: string[];
  customTools: CustomToolConfig[];
}
```

### LoggingConfig

```typescript
interface LoggingConfig {
  level: 'debug' | 'info' | 'warn' | 'error';
  file?: string;
  console: boolean;
}
```

### ValidationResult

```typescript
interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}
```

### CustomToolConfig

```typescript
interface CustomToolConfig {
  name: string;
  path: string;
  enabled: boolean;
}
```

## Example Usage

```typescript
import { ConfigManager, createDefaultConfig } from 'rwkv-agent-kit';

const configManager = new ConfigManager('./config.json');

// Load configuration
const config = await configManager.load();

// Update a setting
configManager.set('model.temperature', 0.8);

// Validate configuration
const validation = configManager.validate();
if (!validation.valid) {
  console.error('Configuration errors:', validation.errors);
}

// Save configuration
await configManager.save(config);
```