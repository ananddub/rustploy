-- name: GetUserByID :one
SELECT * FROM users WHERE id = ?;