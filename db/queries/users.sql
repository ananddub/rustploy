-- ============================================================
-- USERS
-- ============================================================

-- name: GetUserByID :one
SELECT * FROM users WHERE id = ?;

-- name: GetUserByEmail :one
SELECT * FROM users WHERE email = ?;

-- name: ListUsers :many
SELECT * FROM users ORDER BY created_at DESC;

-- name: ListUsersByGroup :many
SELECT * FROM users WHERE group_id = ? ORDER BY created_at DESC;

-- name: CreateUser :one
INSERT INTO users (
    email, first_name, last_name, avatar,
    role, password, is_registered, group_id, added_by
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateUser :one
UPDATE users
SET first_name = ?, last_name = ?, avatar = ?, about_me = ?
WHERE id = ?
RETURNING *;

-- name: UpdateUserRole :exec
UPDATE users SET role = ? WHERE id = ?;

-- name: UpdateUserPassword :exec
UPDATE users SET password = ? WHERE id = ?;

-- name: VerifyUserEmail :exec
UPDATE users
SET is_email_verify = 1, email_verify_at = strftime('%s', 'now')
WHERE id = ?;

-- name: EnableTwoFactor :exec
UPDATE users SET two_factor_enable = 1 WHERE id = ?;

-- name: DisableTwoFactor :exec
UPDATE users SET two_factor_enable = 0 WHERE id = ?;

-- name: DeleteUser :exec
DELETE FROM users WHERE id = ?;

-- ============================================================
-- TWO FACTOR AUTH
-- ============================================================

-- name: GetTwoFactorByUserID :one
SELECT * FROM two_factor WHERE user_id = ?;

-- name: CreateTwoFactor :one
INSERT INTO two_factor (secret, backup_codes, user_id)
VALUES (?, ?, ?)
RETURNING *;

-- name: UpdateTwoFactor :exec
UPDATE two_factor SET secret = ?, backup_codes = ? WHERE user_id = ?;

-- name: DeleteTwoFactor :exec
DELETE FROM two_factor WHERE user_id = ?;

-- ============================================================
-- JWT TOKENS
-- ============================================================

-- name: CreateJwtToken :one
INSERT INTO jwt_tokens (jti, role, user_id, expired_at)
VALUES (?, ?, ?, ?)
RETURNING *;

-- name: GetJwtTokenByJti :one
SELECT * FROM jwt_tokens WHERE jti = ?;

-- name: ListActiveTokensByUser :many
SELECT * FROM jwt_tokens
WHERE user_id = ? AND is_blacklist = 0
ORDER BY created_at DESC;

-- name: BlacklistJwtToken :exec
UPDATE jwt_tokens
SET is_blacklist = 1, blacklist_at = strftime('%s', 'now')
WHERE jti = ?;

-- name: BlacklistAllUserTokens :exec
UPDATE jwt_tokens
SET is_blacklist = 1, blacklist_at = strftime('%s', 'now')
WHERE user_id = ? AND is_blacklist = 0;

-- name: DeleteExpiredTokens :exec
DELETE FROM jwt_tokens WHERE expired_at < strftime('%s', 'now');
