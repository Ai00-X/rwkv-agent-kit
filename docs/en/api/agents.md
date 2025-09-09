# Agents API

## Overview

The Agents API provides interfaces and classes for creating, managing, and interacting with individual agent instances in the RWKV Agent Kit ecosystem.

## Classes

### Agent

Base class representing an individual agent.

#### Constructor

```typescript
constructor(config: AgentConfig)
```

#### Properties

- `id: string` - Unique identifier for the agent
- `name: string` - Human-readable name of the agent
- `status: AgentStatus` - Current status of the agent
- `config: AgentConfig` - Agent configuration

#### Methods

##### start()

Starts the agent and makes it ready to receive messages.

```typescript
start(): Promise<void>
```

##### stop()

Stops the agent and cleans up resources.

```typescript
stop(): Promise<void>
```

##### sendMessage()

Sends a message to the agent.

```typescript
sendMessage(message: string): Promise<AgentResponse>
```

**Parameters:**
- `message`: The message to send to the agent

**Returns:**
- `Promise<AgentResponse>`: The agent's response

##### getHistory()

Retrieves the conversation history.

```typescript
getHistory(): ConversationHistory
```

**Returns:**
- `ConversationHistory`: The conversation history

##### clearHistory()

Clears the conversation history.

```typescript
clearHistory(): void
```

### SpecializedAgent

Extended agent class with specialized capabilities.

#### Methods

##### executeTask()

Executes a specific task.

```typescript
executeTask(task: Task): Promise<TaskResult>
```

##### addTool()

Adds a tool to the agent's toolkit.

```typescript
addTool(tool: Tool): void
```

##### removeTool()

Removes a tool from the agent's toolkit.

```typescript
removeTool(toolName: string): boolean
```

## Enums

### AgentStatus

```typescript
enum AgentStatus {
  IDLE = 'idle',
  BUSY = 'busy',
  ERROR = 'error',
  STOPPED = 'stopped'
}
```

## Types

### AgentResponse

```typescript
interface AgentResponse {
  content: string;
  timestamp: Date;
  metadata?: Record<string, any>;
}
```

### ConversationHistory

```typescript
interface ConversationHistory {
  messages: HistoryMessage[];
  totalCount: number;
}
```

### HistoryMessage

```typescript
interface HistoryMessage {
  role: 'user' | 'agent';
  content: string;
  timestamp: Date;
}
```

### Task

```typescript
interface Task {
  id: string;
  type: string;
  parameters: Record<string, any>;
  priority?: number;
}
```

### TaskResult

```typescript
interface TaskResult {
  success: boolean;
  result?: any;
  error?: string;
  executionTime: number;
}
```

## Example Usage

```typescript
import { Agent, AgentStatus } from 'rwkv-agent-kit';

const agent = new Agent({
  name: 'Assistant',
  modelPath: './models/rwkv-model.bin'
});

await agent.start();

const response = await agent.sendMessage('Hello, how are you?');
console.log(response.content);

const history = agent.getHistory();
console.log(`Total messages: ${history.totalCount}`);
```