-- ============================================================
-- CERTIFICATES
-- ============================================================

-- name: GetCertificateByID :one
SELECT * FROM certificates WHERE id = ?;

-- name: GetCertificateByPath :one
SELECT * FROM certificates WHERE certificate_path = ?;

-- name: ListCertificatesByOrg :many
SELECT * FROM certificates WHERE organization_id = ? ORDER BY created_at DESC;

-- name: ListCertificatesByServer :many
SELECT * FROM certificates WHERE server_id = ? ORDER BY created_at DESC;

-- name: CreateCertificate :one
INSERT INTO certificates (
    name, certificate_data, private_key,
    certificate_path, auto_renew, server_id, organization_id
) VALUES (?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateCertificate :one
UPDATE certificates
SET
    name             = ?,
    certificate_data = ?,
    private_key      = ?,
    certificate_path = ?,
    auto_renew       = ?
WHERE id = ?
RETURNING *;

-- name: DeleteCertificate :exec
DELETE FROM certificates WHERE id = ?;
