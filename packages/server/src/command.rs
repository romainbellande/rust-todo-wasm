use uuid::Uuid;

pub struct CreateTodoDto {
    title: String,
    text: String,
}

pub enum TodoCommand {
    GetTodos,
    CreateTodo(CreateTodoDto),
    GetTodo(Uuid),
}

pub enum Command {
    TodoCommand(TodoCommand),
}
