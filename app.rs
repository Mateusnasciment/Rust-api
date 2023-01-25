/// It's importing the necessary modules.
use warp::{Filter, Rejection, Reply};
use serde_json::json;

/// A Todo is a struct with an id field of type i32 and a task field of type String.
/// 
/// Properties:
/// 
/// * `id`: i32 - The id of the todo item.
/// * `task`: The task that the user wants to complete.
struct Todo {
    id: i32,
    task: String,
}
/// It's creating a mutable vector of Todo structs.
let mut todos = vec![
    Todo { id: 1, task: "Take out the trash".to_string() },
    Todo { id: 2, task: "Buy groceries".to_string() },
];
/// `get_todos` is a function that returns a `Result` of either a `Reply` or a `Rejection`
/// 
/// Returns:
/// 
/// A function that takes a `Rejection` and returns a `Result<impl Reply, Rejection>`
fn get_todos() -> Result<impl Reply, Rejection> {
    let todos_json = serde_json::to_string(&todos)?;
    Ok(warp::reply::json(&todos_json))
}
/// `create_todo` takes a `Todo` struct as an argument, and returns a `Result` that either contains a
/// `Reply` or a `Rejection`
/// 
/// Arguments:
/// 
/// * `todo`: Todo - This is the type of the request body.
/// 
/// Returns:
/// 
/// A function that takes a Todo and returns a Result<impl Reply, Rejection>
fn create_todo(todo: Todo) -> Result<impl Reply, Rejection> {
    let new_id = todos.len() as i32 + 1;
    let new_todo = Todo { id: new_id, task: todo.task };
    todos.push(new_todo);
    Ok(warp::reply::json(&json!({ "id": new_id })))
}

/// If the todo with the given id exists, remove it from the todos list and return an empty response. If
/// the todo doesn't exist, return a 404 Not Found error
/// 
/// Arguments:
/// 
/// * `id`: i32 - The id of the todo to delete.
/// 
/// Returns:
/// 
/// a Result<impl Reply, Rejection>
/// Question: What is the type of the Ok()?
/// Answer: The Ok() is a warp::reply()
/// Question: What is the type of the Err()?
/// Answer: The Err() is a warp::reject::not_found()
/// Question: What is the type of the warp::reply()?
"/todos/:id" endpoint
fn delete_todo(id: i32) -> Result<impl Reply, Rejection> {
    let index = todos.iter().position(|t| t.id == id);
    if let Some(index) = index {
        todos.remove(index);
        Ok(warp::reply())
    } else {
        Err(warp::reject::not_found())
    }
}

/// It's creating a route that matches the path `/todos` and then it's creating a route that matches the
/// path `/todos` and then it's creating a route that matches the path `/todos` and then it's creating a
/// route that matches the path `/todos` and then it's creating a route that matches the path `/todos`
/// and then it's creating a route that matches the path `/todos` and then it's creating a route that
/// matches the path `/todos` and then it's creating a route that matches the path `/todos` and then
/// it's creating a route that matches the path `/todos` and then it's creating a route that matches the
/// path `/todos` and then it's creating a route that matches the path `/todos` and then it's creating a
/// route that matches the path `/todos` and then it's creating a route that matches the path `/todos`
/// and then it's creating a route that matches the path `/todos` and then it's
let routes = warp::path("todos")
    .and(warp::get().and_then(get_todos))
    .or(warp::path("todos")
        .and(warp::post().and(warp::body::json()).and_then(create_todo)))
    .or(warp::path("todos")
        .and(warp::delete().and(warp::path::param()).and_then(delete_todo)));]

 /// `warp::serve(routes).run(([127, 0, 0, 1], 3030 )).await;`
 /// 
 /// This function is a server that listens on port 3030 and runs the routes function
        
async fn main() {
    warp::serve(routes).run(([127, 0, 0, 1], 3030 )).await;
}


/   / This code is defining a web application that manages a list of to-do items using the Rust programming language. The application uses the warp web framework to handle HTTP requests and responses, as well as the serde_json library for serializing and deserializing data in JSON format.

// At the top of the code, the necessary modules are imported, including the warp library and the serde_json library. A struct called Todo is defined, which has fields for an id (i32) and a task (String). The code then creates a mutable vector of Todo structs, which represents the list of to-do items.

// The code defines several functions for handling different types of HTTP requests. The get_todos function returns a JSON representation of the to-do list. The create_todo function takes a Todo struct as an argument and adds it to the to-do list, returning a JSON response with the new id. The delete_todo function takes an id as an argument and removes the corresponding to-do item from the list, returning an empty response if successful or a 404 Not Found error if the to-do item is not found.

// Finally, the code creates routes that match different paths, such as "/todos" and "/todos/:id", and associates them with the appropriate functions for handling requests. These routes allow the application to handle various types of HTTP requests, such as GET, POST, and DELETE, and respond with the appropriate data or error messages.