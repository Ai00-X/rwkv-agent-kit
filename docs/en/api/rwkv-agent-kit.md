# RWKV Agent Kit API

## Overview

The RWKV Agent Kit is the core module that provides the main functionality for creating and managing RWKV-based AI agents.

## Classes

### RWKVAgentKit

The main class for creating RWKV agent instances.

#### Constructor

```typescript
constructor(config: AgentConfig)
```

**Parameters:**
- `config`: Configuration object for the agent

#### Methods

##### createAgent()

Creates a new agent instance.

```typescript
createAgent(options: AgentOptions): Promise<Agent>
```

**Parameters:**
- `options`: Agent creation options

**Returns:**
- `Promise<Agent>`: The created agent instance

##### getAgent()

Retrieves an existing agent by ID.

```typescript
getAgent(id: string): Agent | null
```

**Parameters:**
- `id`: Agent identifier

**Returns:**
- `Agent | null`: The agent instance or null if not found

##### listAgents()

Lists all available agents.

```typescript
listAgents(): Agent[]
```

**Returns:**
- `Agent[]`: Array of all agent instances

## Types

### AgentConfig

Configuration interface for the RWKV Agent Kit.

```typescript
interface AgentConfig {
  modelPath: string;
  maxTokens?: number;
  temperature?: number;
  topP?: number;
}
```

### AgentOptions

Options for creating a new agent.

```typescript
interface AgentOptions {
  name: string;
  description?: string;
  systemPrompt?: string;
  tools?: Tool[];
}
```

## Example Usage

```typescript
import { RWKVAgentKit } from 'rwkv-agent-kit';

const kit = new RWKVAgentKit({
  modelPath: './models/rwkv-model.bin',
  maxTokens: 2048,
  temperature: 0.7
});

const agent = await kit.createAgent({
  name: 'Assistant',
  description: 'A helpful AI assistant',
  systemPrompt: 'You are a helpful assistant.'
});
```