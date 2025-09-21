use crate::database::DatabaseManager;
use crate::models::*;
use std::sync::Mutex;
use std::time::Duration;
use tauri::State;
use tokio::time::timeout;

// Global database manager
type DbManager = Mutex<DatabaseManager>;

#[tauri::command]
pub async fn connect_database(
    path: String,
    password: String,
    manager: State<'_, DbManager>,
) -> Result<DatabaseInfo, String> {
    let mut db_manager = manager.lock().unwrap();
    
    match db_manager.connect_database(&path, &password) {
        Ok(db_info) => {
            println!("Successfully connected to database: {}", path);
            Ok(db_info)
        },
        Err(e) => {
            println!("Failed to connect to database {}: {}", path, e);
            Err(format!("Connection failed: {}", e))
        }
    }
}

#[tauri::command]
pub async fn get_database_tables(
    db_path: String,
    manager: State<'_, DbManager>,
) -> Result<Vec<TableInfo>, String> {
    let db_manager = manager.lock().unwrap();
    
    match db_manager.get_tables(&db_path) {
        Ok(tables) => {
            println!("Retrieved {} tables from {}", tables.len(), db_path);
            Ok(tables)
        },
        Err(e) => {
            println!("Failed to get tables from {}: {}", db_path, e);
            Err(format!("Failed to get tables: {}", e))
        }
    }
}


#[tauri::command]
pub async fn get_table_data(
    db_path: String,
    table_name: String,
    limit: Option<i64>,
    manager: State<'_, DbManager>,
) -> Result<TableData, String> {
    // Add timeout for large queries (60 seconds)
    let timeout_duration = Duration::from_secs(60);
    
    let result = timeout(timeout_duration, async {
        let db_manager = manager.lock().unwrap();
        
        // Log the request with more details
        println!("Fetching table '{}' from '{}' with limit: {:?}", 
                table_name, db_path, limit);
        
        db_manager.get_table_data(&db_path, &table_name, limit)
    }).await;
    
    match result {
        Ok(data_result) => {
            match data_result {
                Ok(data) => {
                    println!("Successfully retrieved {} rows from table '{}' in '{}'", 
                            data.rows.len(), table_name, db_path);
                    Ok(data)
                },
                Err(e) => {
                    println!("Database error for table '{}': {}", table_name, e);
                    Err(format!("Database error: {}", e))
                }
            }
        },
        Err(_) => {
            println!("Timeout occurred while fetching table '{}' (limit: {:?})", 
                    table_name, limit);
            Err(format!("Query timeout - table '{}' took too long to fetch. Try using a smaller row limit.", 
                       table_name))
        }
    }
}


#[tauri::command]
pub async fn compare_database_schemas(
    db1_path: String,
    db2_path: String,
    manager: State<'_, DbManager>,
) -> Result<SchemaComparison, String> {
    let db_manager = manager.lock().unwrap();
    
    match db_manager.compare_schemas(&db1_path, &db2_path) {
        Ok(comparison) => {
            println!("Schema comparison completed between {} and {}", db1_path, db2_path);
            Ok(comparison)
        },
        Err(e) => {
            println!("Failed to compare schemas: {}", e);
            Err(format!("Schema comparison failed: {}", e))
        }
    }
}

#[tauri::command]
pub async fn test_connection() -> Result<String, String> {
    Ok("Tauri backend is working!".to_string())
}


// struct SchemaObject {
//     name: String,
//     object_type: String,
//     sql: String,
//     columns: Option<Vec<ColumnInfo>>,
// }