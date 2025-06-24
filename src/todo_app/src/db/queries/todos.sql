-- name: get_all_todos
SELECT * FROM todos ORDER BY created_at DESC;

-- name: create_todo
INSERT INTO todos (title, completed) VALUES (?, ?);

-- name: delete_todo
DELETE FROM todos WHERE id = ?;

-- name: update_todo
UPDATE todos SET title = ?, completed = ? WHERE id = ?;
-- name: get_todo_by_id
SELECT * FROM todos WHERE id = ?;
-- name: get_completed_todos
SELECT * FROM todos WHERE completed = TRUE ORDER BY created_at DESC;
-- name: get_incomplete_todos
SELECT * FROM todos WHERE completed = FALSE ORDER BY created_at DESC;
-- name: get_todos_by_title
SELECT * FROM todos WHERE title LIKE ? ORDER BY created_at DESC;
-- name: get_todos_by_date_range
SELECT * FROM todos WHERE created_at BETWEEN ? AND ? ORDER BY created_at DESC;
-- name: get_todos_by_completion_and_date
SELECT * FROM todos WHERE completed = ? AND created_at BETWEEN ? AND ? ORDER BY created_at DESC;

-- name: get_todos_count
SELECT COUNT(*) FROM todos;
-- name: get_completed_todos_count
SELECT COUNT(*) FROM todos WHERE completed = TRUE;
-- name: get_incomplete_todos_count
SELECT COUNT(*) FROM todos WHERE completed = FALSE;
-- name: get_todos_by_completion_count
SELECT COUNT(*) FROM todos WHERE completed = ?;
-- name: get_todos_by_title_count
SELECT COUNT(*) FROM todos WHERE title LIKE ?;
-- name: get_todos_by_date_range_count
SELECT COUNT(*) FROM todos WHERE created_at BETWEEN ? AND ?;
-- name: get_todos_by_completion_and_date_count
SELECT COUNT(*) FROM todos WHERE completed = ? AND created_at BETWEEN ? AND ?;
-- name: get_todos_by_title_and_completion
SELECT * FROM todos WHERE title LIKE ? AND completed = ? ORDER BY created_at DESC;
-- name: get_todos_by_title_and_completion_count
SELECT COUNT(*) FROM todos WHERE title LIKE ? AND completed = ?;
-- name: get_todos_by_title_and_date_range
SELECT * FROM todos WHERE title LIKE ? AND created_at BETWEEN ? AND ? ORDER BY created_at DESC;
-- name: get_todos_by_title_and_date_range_count
SELECT COUNT(*) FROM todos WHERE title LIKE ? AND created_at BETWEEN ? AND ?;
-- name: get_todos_by_completion_and_title
SELECT * FROM todos WHERE completed = ? AND title LIKE ? ORDER BY created_at DESC;
-- name: get_todos_by_completion_and_title_count
SELECT COUNT(*) FROM todos WHERE completed = ? AND title LIKE ?;
-- name: get_todos_by_completion_and_date_range
SELECT * FROM todos WHERE completed = ? AND created_at BETWEEN ? AND ? ORDER BY created_at DESC;
-- name: get_todos_by_completion_and_date_range_count
SELECT COUNT(*) FROM todos WHERE completed = ? AND created_at BETWEEN ? AND ?;
-- name: get_todos_by_title_completion_and_date_range
SELECT * FROM todos WHERE title LIKE ? AND completed = ? AND created_at BETWEEN ? AND ? ORDER BY created_at DESC;
-- name: get_todos_by_title_completion_and_date_range_count
SELECT COUNT(*) FROM todos WHERE title LIKE ? AND completed = ? AND created_at BETWEEN ? AND ?; 
-- name: get_todos_by_title_completion_and_date_range_and_id
SELECT * FROM todos WHERE title LIKE ? AND completed = ? AND created_at BETWEEN ? AND ? AND id = ? ORDER BY created_at DESC;
-- name: get_todos_by_title_completion_and_date_range_and_id_count  
SELECT COUNT(*) FROM todos WHERE title LIKE ? AND completed = ? AND created_at BETWEEN ? AND ? AND id = ?;
-- name: get_todos_by_title_completion_and_date_range_and_id_or_title
SELECT * FROM todos WHERE (title LIKE ? OR id = ?) AND completed = ? AND created_at BETWEEN ? AND ? ORDER BY created_at DESC;
-- name: get_todos_by_title_completion_and_date_range_and_id_or_title_count
SELECT COUNT(*) FROM todos WHERE (title LIKE ? OR id = ?) AND completed = ? AND created_at BETWEEN ? AND ?;
-- name: get_todos_by_title_completion_and_date_range_and_id_or_title_and_completed
SELECT * FROM todos WHERE (title LIKE ? OR id = ?) AND completed = ? AND created_at BETWEEN ? AND ? ORDER BY created_at DESC;
-- name: get_todos_by_title_completion_and_date_range_and_id_or_title_and_completed_count
SELECT COUNT(*) FROM todos WHERE (title LIKE ? OR id = ?) AND completed = ? AND created_at BETWEEN ? AND ?;
-- name: get_todos_by_title_completion_and_date_range_and_id_or_title_and_completed_and_created_at
SELECT * FROM todos WHERE (title LIKE ? OR id = ?) AND completed = ? AND created_at BETWEEN ? AND ? AND created_at = ? ORDER BY created_at DESC;
-- name: get_todos_by_title_completion_and_date_range_and_id_or_title_and_completed_and_created_at_count
SELECT COUNT(*) FROM todos WHERE (title LIKE ? OR id = ?) AND completed = ? AND created_at BETWEEN ? AND ? AND created_at = ?;
-- name: get_todos_by_title_completion_and_date_range_and_id_or_title_and_completed_and_created_at_and_id
SELECT * FROM todos WHERE (title LIKE ? OR id = ?) AND completed = ? AND created_at BETWEEN ? AND ? AND created_at = ? AND id = ? ORDER BY created_at DESC;
-- name: get_todos_by_title_completion_and_date_range_and_id_or_title_and_completed_and_created_at_and_id_count
SELECT COUNT(*) FROM todos WHERE (title LIKE ? OR id = ?) AND completed = ? AND created_at BETWEEN ? AND ? AND created_at = ? AND id = ?;
-- name: get_todos_by_title_completion_and_date_range_and_id_or_title_and_completed_and_created_at_and_id_or_title
SELECT * FROM todos WHERE (title LIKE ? OR id = ?) AND completed = ? AND created_at BETWEEN ? AND ? AND created_at = ? AND (title LIKE ? OR id = ?) ORDER BY created_at DESC;
-- name: get_todos_by_title_completion_and_date_range_and_id_or_title_and_completed_and_created_at_and_id_or_title_count
SELECT COUNT(*) FROM todos WHERE (title LIKE ? OR id = ?) AND completed = ? AND created_at BETWEEN ? AND ? AND created_at = ? AND (title LIKE ? OR id = ?);