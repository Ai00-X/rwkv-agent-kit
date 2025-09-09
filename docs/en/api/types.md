# Types API

## Overview

The Types API provides TypeScript type definitions and interfaces used throughout the RWKV Agent Kit ecosystem.

## Core Types

### Agent Types

#### Agent

Base agent interface.

```typescript
interface Agent {
  id: string;
  name: string;
  status: AgentStatus;
  config: AgentConfig;
  start(): Promise<void>;
  stop(): Promise<void>;
  sendMessage(message: string): Promise<AgentResponse>;
}
```

#### AgentConfig

Configuration for agent instances.

```typescript
interface AgentConfig {
  name: string;
  description?: string;
  systemPrompt?: string;
  modelPath: string;
  maxTokens?: number;
  temperature?: number;
  topP?: number;
  topK?: number;
  repeatPenalty?: number;
  maxHistory?: number;
  timeout?: number;
}
```

#### AgentStatus

Agent status enumeration.

```typescript
enum AgentStatus {
  IDLE = 'idle',
  BUSY = 'busy',
  ERROR = 'error',
  STOPPED = 'stopped'
}
```

#### AgentResponse

Response from agent interactions.

```typescript
interface AgentResponse {
  content: string;
  timestamp: Date;
  metadata?: Record<string, any>;
  tokens?: TokenUsage;
}
```

### Message Types

#### Message

Base message interface.

```typescript
interface Message {
  id: string;
  role: MessageRole;
  content: string;
  timestamp: Date;
  metadata?: Record<string, any>;
}
```

#### MessageRole

Message role enumeration.

```typescript
enum MessageRole {
  USER = 'user',
  AGENT = 'agent',
  SYSTEM = 'system'
}
```

#### ConversationHistory

Conversation history interface.

```typescript
interface ConversationHistory {
  messages: Message[];
  totalCount: number;
  agentId: string;
}
```

### Tool Types

#### Tool

Base tool interface.

```typescript
interface Tool {
  name: string;
  description: string;
  parameters: ToolParameters;
  execute(args: Record<string, any>): Promise<ToolResult>;
}
```

#### ToolParameters

Tool parameter schema.

```typescript
interface ToolParameters {
  type: 'object';
  properties: Record<string, ParameterSchema>;
  required?: string[];
}
```

#### ParameterSchema

Parameter schema definition.

```typescript
interface ParameterSchema {
  type: 'string' | 'number' | 'boolean' | 'array' | 'object';
  description?: string;
  enum?: any[];
  items?: ParameterSchema;
  properties?: Record<string, ParameterSchema>;
}
```

#### ToolResult

Result from tool execution.

```typescript
interface ToolResult {
  success: boolean;
  result?: any;
  error?: string;
  executionTime?: number;
}
```

### Memory Types

#### MemoryEntry

Memory entry interface.

```typescript
interface MemoryEntry {
  id: string;
  content: string;
  vector?: number[];
  metadata: Record<string, any>;
  timestamp: Date;
  agentId: string;
}
```

#### MemoryQuery

Memory query interface.

```typescript
interface MemoryQuery {
  content?: string;
  vector?: number[];
  metadata?: Record<string, any>;
  limit?: number;
  threshold?: number;
}
```

#### MemorySearchResult

Memory search result.

```typescript
interface MemorySearchResult {
  entry: MemoryEntry;
  similarity: number;
}
```

### Configuration Types

#### Config

Main configuration interface.

```typescript
interface Config {
  model: ModelConfig;
  agent: AgentConfig;
  memory: MemoryConfig;
  tools: ToolsConfig;
  logging: LoggingConfig;
  database: DatabaseConfig;
}
```

#### ModelConfig

Model configuration.

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

#### MemoryConfig

Memory system configuration.

```typescript
interface MemoryConfig {
  enabled: boolean;
  maxSize: number;
  persistPath?: string;
  vectorDimension: number;
  similarityThreshold: number;
}
```

#### ToolsConfig

Tools configuration.

```typescript
interface ToolsConfig {
  enabled: string[];
  disabled: string[];
  customTools: CustomToolConfig[];
}
```

#### CustomToolConfig

Custom tool configuration.

```typescript
interface CustomToolConfig {
  name: string;
  path: string;
  enabled: boolean;
  config?: Record<string, any>;
}
```

#### LoggingConfig

Logging configuration.

```typescript
interface LoggingConfig {
  level: LogLevel;
  file?: string;
  console: boolean;
  format?: string;
}
```

#### LogLevel

Logging level enumeration.

```typescript
enum LogLevel {
  DEBUG = 'debug',
  INFO = 'info',
  WARN = 'warn',
  ERROR = 'error'
}
```

### Utility Types

#### TokenUsage

Token usage information.

```typescript
interface TokenUsage {
  promptTokens: number;
  completionTokens: number;
  totalTokens: number;
}
```

#### ValidationResult

Validation result interface.

```typescript
interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}
```

#### EventCallback

Event callback function type.

```typescript
type EventCallback<T = any> = (data: T) => void | Promise<void>;
```

#### AsyncFunction

Generic async function type.

```typescript
type AsyncFunction<T = any, R = any> = (args: T) => Promise<R>;
```

## Type Guards

### isAgent()

Type guard for Agent interface.

```typescript
function isAgent(obj: any): obj is Agent {
  return obj && typeof obj.id === 'string' && typeof obj.sendMessage === 'function';
}
```

### isMessage()

Type guard for Message interface.

```typescript
function isMessage(obj: any): obj is Message {
  return obj && typeof obj.id === 'string' && typeof obj.content === 'string';
}
```

### isTool()

Type guard for Tool interface.

```typescript
function isTool(obj: any): obj is Tool {
  return obj && typeof obj.name === 'string' && typeof obj.execute === 'function';
}
```

## Example Usage

```typescript
import { Agent, AgentConfig, AgentStatus, Message } from 'rwkv-agent-kit';

// Type-safe agent configuration
const config: AgentConfig = {
  name: 'Assistant',
  modelPath: './models/rwkv-model.bin',
  maxTokens: 2048,
  temperature: 0.7
};

// Type-safe message creation
const message: Message = {
  id: 'msg-1',
  role: MessageRole.USER,
  content: 'Hello!',
  timestamp: new Date()
};

// Type guard usage
if (isAgent(someObject)) {
  await someObject.sendMessage('Hello');
}
```