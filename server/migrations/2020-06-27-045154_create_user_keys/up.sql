CREATE TABLE user_keys (
    id BIGINT(20) UNSIGNED AUTO_INCREMENT NOT NULL,
    user_id BIGINT(20) UNSIGNED NOT NULL,
    public_key VARCHAR(255) NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME,
    PRIMARY KEY (id),
    UNIQUE INDEX ux_user_id (user_id),
    UNIQUE INDEX ux_public_key (public_key),
    CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users(id)
) CHARACTER SET 'utf8mb4'
  COLLATE 'utf8mb4_general_ci';
