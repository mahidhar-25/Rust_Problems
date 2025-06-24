-- name: get_all_todos
SELECT * FROM todos ORDER BY created_at DESC;

-- name: create_todo
INSERT INTO todos (title, completed) VALUES (?, ?);

-- name: delete_todo
DELETE FROM todos WHERE id = ?;
