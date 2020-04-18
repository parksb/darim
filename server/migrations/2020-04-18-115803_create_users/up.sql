CREATE TABLE users (
    id BIGINT(20) UNSIGNED AUTO_INCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    avatar_url VARCHAR(255),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME,
    PRIMARY KEY (id),
    UNIQUE INDEX ux_email (email)
) CHARACTER SET 'utf8mb4'
  COLLATE 'utf8mb4_general_ci';
