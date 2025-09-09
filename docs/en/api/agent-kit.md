# Agent Kit API

## Overview

The Agent Kit provides utilities and helper functions for working with RWKV agents, including agent management, communication, and lifecycle operations.

## Functions

### createAgentKit()

Creates a new agent kit instance with the specified configuration.

```typescript
function createAgentKit(config: AgentKitConfig): AgentKit
```

**Parameters:**
- `config`: Configuration object for the agent kit

**Returns:**
- `AgentKit`: A new agent kit instance

### initializeAgent()

Initializes an agent with default settings.

```typescript
function initializeAgent(options: InitOptions): Promise<Agent>
```

**Parameters:**
- `options`: Initialization options

**Returns:**
- `Promise<Agent>`: The initialized agent

### destroyAgent()

Cleans up and destroys an agent instance.

```typescript
function destroyAgent(agent: Agent): Promise<void>
```

**Parameters:**
- `agent`: The agent instance to destroy

**Returns:**
- `Promise<void>`: Promise that resolves when cleanup is complete

## Classes

### AgentKit

Main class for managing multiple agents.

#### Methods

##### addAgent()

Adds an agent to the kit.

```typescript
addAgent(agent: Agent): void
```

##### removeAgent()

Removes an agent from the kit.

```typescript
removeAgent(agentId: string): boolean
```

##### getAgent()

Retrieves an agent by ID.

```typescript
getAgent(agentId: string): Agent | undefined
```

##### getAllAgents()

Gets all agents in the kit.

```typescript
getAllAgents(): Agent[]
```

## Types

### AgentKitConfig

```typescript
interface AgentKitConfig {
  maxAgents?: number;
  defaultTimeout?: number;
  enableLogging?: boolean;
}
```

### InitOptions

```typescript
interface InitOptions {
  name: string;
  type?: AgentType;
  config?: Partial<AgentConfig>;
}
```

## Example Usage

```typescript
import { createAgentKit, initializeAgent } from 'rwkv-agent-kit';

const kit = createAgentKit({
  maxAgents: 10,
  enableLogging: true
});

const agent = await initializeAgent({
  name: 'MyAgent',
  type: 'assistant'
});

kit.addAgent(agent);
```