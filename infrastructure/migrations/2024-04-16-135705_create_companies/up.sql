-- Your SQL goes here
CREATE TABLE IF NOT EXISTS hris.companies (
    id serial PRIMARY KEY,
    company_code VARCHAR(255),
    company_name VARCHAR(255),
    photo VARCHAR(255),
    address VARCHAR(255),
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    status VARCHAR(255),
    created_at TIMESTAMP NULL,
    updated_at TIMESTAMP NULL
);

CREATE TABLE IF NOT EXISTS hris.projects (
    id serial PRIMARY KEY,
    project_code VARCHAR(255),
    project_name VARCHAR(255),
    company_id BIGINT,
    created_at TIMESTAMP NULL,
    updated_at TIMESTAMP NULL

);

ALTER TABLE hris.projects
ADD CONSTRAINT fk_projects_company_id
FOREIGN KEY (company_id)
REFERENCES hris.companies(id)
ON DELETE CASCADE;

-- Seed data companies
INSERT INTO hris.companies (company_code, company_name, photo, address, latitude, longitude, status, created_at, updated_at)
VALUES
    ('COMP001', 'Company 1', NULL, 'Address 1', 12.345, 67.890, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP002', 'Company 2', NULL, 'Address 2', 23.456, 78.901, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP003', 'Company 3', NULL, 'Address 3', 34.567, 89.012, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP004', 'Company 4', NULL, 'Address 4', 45.678, 90.123, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP005', 'Company 5', NULL, 'Address 5', 56.789, 01.234, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP006', 'Company 6', NULL, 'Address 6', 67.890, 12.345, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP007', 'Company 7', NULL, 'Address 7', 78.901, 23.456, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP008', 'Company 8', NULL, 'Address 8', 89.012, 34.567, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP009', 'Company 9', NULL, 'Address 9', 90.123, 45.678, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('COMP010', 'Company 10', NULL, 'Address 10', 01.234, 56.789, 'Active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);


-- Seed data projects
INSERT INTO hris.projects (project_code, project_name, company_id, created_at, updated_at)
SELECT 
    CONCAT('PROJ00', ROW_NUMBER() OVER(PARTITION BY c.id ORDER BY p.id)),
    CONCAT('Project ', ROW_NUMBER() OVER(PARTITION BY c.id ORDER BY p.id)),
    c.id,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
FROM hris.companies c
CROSS JOIN (
    SELECT GENERATE_SERIES(1, 2) AS id
) AS p;