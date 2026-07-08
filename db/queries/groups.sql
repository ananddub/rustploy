-- ============================================================
-- GROUPS
-- ============================================================

-- name: GetGroupByID :one
SELECT * FROM groups WHERE id = ?;

-- name: GetGroupByName :one
SELECT * FROM groups WHERE name = ?;

-- name: ListGroups :many
SELECT * FROM groups ORDER BY created_at DESC;

-- name: CreateGroup :one
INSERT INTO groups (name) VALUES (?) RETURNING *;

-- name: UpdateGroup :one
UPDATE groups SET name = ? WHERE id = ? RETURNING *;

-- name: DeleteGroup :exec
DELETE FROM groups WHERE id = ?;

-- ============================================================
-- POLICIES
-- ============================================================

-- name: GetPolicyByID :one
SELECT * FROM policy WHERE id = ?;

-- name: GetPolicyByAction :one
SELECT * FROM policy WHERE action = ?;

-- name: ListPolicies :many
SELECT * FROM policy ORDER BY action ASC;

-- name: CreatePolicy :one
INSERT INTO policy (action) VALUES (?) RETURNING *;

-- name: DeletePolicy :exec
DELETE FROM policy WHERE id = ?;

-- ============================================================
-- GROUP POLICIES
-- ============================================================

-- name: ListPoliciesByGroup :many
SELECT p.* FROM policy p
JOIN group_policy gp ON gp.policy_id = p.id
WHERE gp.group_id = ?
ORDER BY p.action ASC;

-- name: AddPolicyToGroup :one
INSERT INTO group_policy (group_id, policy_id) VALUES (?, ?) RETURNING *;

-- name: RemovePolicyFromGroup :exec
DELETE FROM group_policy WHERE group_id = ? AND policy_id = ?;

-- name: ClearGroupPolicies :exec
DELETE FROM group_policy WHERE group_id = ?;
