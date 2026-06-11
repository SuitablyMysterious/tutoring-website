-- Enquiries from the public contact form.
-- PII columns (name, email, message) are stored plaintext for now; the
-- zero-trust spec calls for application-layer encryption before launch.
CREATE TABLE IF NOT EXISTS enquiries (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    message TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
