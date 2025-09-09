# Database API

## Overview

The Database API provides interfaces for data persistence, retrieval, and management within the RWKV Agent Kit ecosystem.

## Classes

### DatabaseManager

Main class for database operations.

#### Constructor

```typescript
constructor(config: DatabaseConfig)
```

**Parameters:**
- `config`: Database configuration object

#### Methods

##### connect()

Establishes connection to the database.

```typescript
connect(): Promise<void>
```

##### disconnect()

Closes the database connection.

```typescript
disconnect(): Promise<void>
```

##### save()

Saves data to the database.

```typescript
save<T>(collection: string, data: T): Promise<string>
```

**Parameters:**
- `collection`: Collection/table name
- `data`: Data to save

**Returns:**
- `Promise<string>`: ID of the saved record

##### find()

Finds records in the database.

```typescript
find<T>(collection: string, query: Query): Promise<T[]>
```

**Parameters:**
- `collection`: Collection/table name
- `query`: Query object

**Returns:**
- `Promise<T[]>`: Array of matching records

##### findById()

Finds a record by ID.

```typescript
findById<T>(collection: string, id: string): Promise<T | null>
```

**Parameters:**
- `collection`: Collection/table name
- `id`: Record ID

**Returns:**
- `Promise<T | null>`: The record or null if not found

##### update()

Updates a record in the database.

```typescript
update<T>(collection: string, id: string, data: Partial<T>): Promise<boolean>
```

**Parameters:**
- `collection`: Collection/table name
- `id`: Record ID
- `data`: Data to update

**Returns:**
- `Promise<boolean>`: Success status

##### delete()

Deletes a record from the database.

```typescript
delete(collection: string, id: string): Promise<boolean>
```

**Parameters:**
- `collection`: Collection/table name
- `id`: Record ID

**Returns:**
- `Promise<boolean>`: Success status

### VectorDatabase

Specialized database for vector operations.

#### Methods

##### addVector()

Adds a vector to the database.

```typescript
addVector(vector: Vector, metadata?: Record<string, any>): Promise<string>
```

##### searchSimilar()

Searches for similar vectors.

```typescript
searchSimilar(query: Vector, limit?: number): Promise<VectorSearchResult[]>
```

##### deleteVector()

Deletes a vector from the database.

```typescript
deleteVector(id: string): Promise<boolean>
```

## Functions

### createDatabase()

Creates a new database instance.

```typescript
function createDatabase(type: DatabaseType, config: DatabaseConfig): DatabaseManager
```

**Parameters:**
- `type`: Type of database
- `config`: Database configuration

**Returns:**
- `DatabaseManager`: Database manager instance

### migrateDatabase()

Runs database migrations.

```typescript
function migrateDatabase(manager: DatabaseManager): Promise<void>
```

**Parameters:**
- `manager`: Database manager instance

## Types

### DatabaseConfig

```typescript
interface DatabaseConfig {
  type: DatabaseType;
  connectionString?: string;
  host?: string;
  port?: number;
  database?: string;
  username?: string;
  password?: string;
  options?: Record<string, any>;
}
```

### DatabaseType

```typescript
type DatabaseType = 'sqlite' | 'postgresql' | 'mysql' | 'mongodb' | 'memory';
```

### Query

```typescript
interface Query {
  where?: Record<string, any>;
  orderBy?: string;
  limit?: number;
  offset?: number;
}
```

### Vector

```typescript
type Vector = number[];
```

### VectorSearchResult

```typescript
interface VectorSearchResult {
  id: string;
  vector: Vector;
  similarity: number;
  metadata?: Record<string, any>;
}
```

### DatabaseRecord

```typescript
interface DatabaseRecord {
  id: string;
  createdAt: Date;
  updatedAt: Date;
  [key: string]: any;
}
```

## Example Usage

```typescript
import { DatabaseManager, createDatabase } from 'rwkv-agent-kit';

// Create database instance
const db = createDatabase('sqlite', {
  type: 'sqlite',
  connectionString: './data.db'
});

// Connect to database
await db.connect();

// Save data
const id = await db.save('conversations', {
  agentId: 'agent-1',
  message: 'Hello, world!',
  timestamp: new Date()
});

// Find data
const conversations = await db.find('conversations', {
  where: { agentId: 'agent-1' },
  orderBy: 'timestamp',
  limit: 10
});

// Update data
await db.update('conversations', id, {
  message: 'Updated message'
});

// Disconnect
await db.disconnect()