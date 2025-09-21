import { invoke } from '@tauri-apps/api/core';

export interface DatabaseInfo {
  path: string;
  name: string;
  table_count: number;
  is_connected: boolean;
  alias?: string;  // Add this line
}

export interface TableInfo {
  name: string;
  row_count: number;
  columns: ColumnInfo[];
}

export interface ColumnInfo {
  name: string;
  data_type: string;
  is_nullable: boolean;
  default_value?: string;
  is_primary_key: boolean;
}

export interface TableData {
  columns: string[];
  rows: any[][];
  total_count: number;
}

export interface SchemaComparison {
  database1: string;
  database2: string;
  added_tables: string[];
  removed_tables: string[];
  modified_tables: TableDiff[];
  identical_tables: string[];
}

export interface TableDiff {
  table_name: string;
  added_columns: ColumnInfo[];
  removed_columns: string[];
  modified_columns: ColumnDiff[];
}

export interface ColumnDiff {
  column_name: string;
  old_type: string;
  new_type: string;
  changes: string[];
}

export class DatabaseService {
  async testConnection(): Promise<string> {
    return await invoke('test_connection');
  }

  async connectDatabase(path: string, password: string): Promise<DatabaseInfo> {
    console.log('Connecting to database:', path);
    return await invoke('connect_database', { path, password });
  }

  async getDatabaseTables(dbPath: string): Promise<TableInfo[]> {
    console.log('Getting tables for database:', dbPath);
    const result = await invoke('get_database_tables', { dbPath });
    console.log('Tables received from backend:', result);
    return result as TableInfo[];
  }

  async getTableData(dbPath: string, tableName: string, limit?: number): Promise<TableData> {
    console.log('Getting table data:', tableName, 'from', dbPath);
    return await invoke('get_table_data', { 
      dbPath, 
      tableName, 
      limit: limit || 100 
    });
  }

  async compareDatabaseSchemas(db1Path: string, db2Path: string): Promise<SchemaComparison> {
    console.log('Comparing schemas:', db1Path, 'vs', db2Path);
    return await invoke('compare_database_schemas', { db1Path, db2Path });
  }
}