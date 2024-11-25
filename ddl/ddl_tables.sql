
CREATE TABLE folders (
    folder_id UUID PRIMARY KEY,                             -- Primary Key for folders
    folder_name VARCHAR NOT NULL,                           -- Folder name (VARCHAR)
    folder_parent_id UUID,                                  -- Parent folder ID (self-referencing foreign key)
    user_id VARCHAR NOT NULL,                               -- User ID (VARCHAR)
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,       -- Created timestamp (default to current timestamp)
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,       -- Updated timestamp (default to current timestamp)
    
    -- Self-referencing foreign key (folder_parent_id references folder_id)
    CONSTRAINT fk_folder_parent_id FOREIGN KEY (folder_parent_id) 
        REFERENCES folders (folder_id) ON DELETE CASCADE    -- Cascade delete on parent folder deletion
);

-- Create files table
CREATE TABLE files (
    file_id UUID PRIMARY KEY,                               -- Primary key for files
    filename VARCHAR NOT NULL,                               -- File name (VARCHAR)
    size BIGINT NOT NULL,                                    -- File size (BIGINT)
    path VARCHAR NOT NULL,                                   -- File path (VARCHAR)
    folder_id UUID NOT NULL,                                 -- Foreign key to folders
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,        -- Created timestamp (default to current timestamp)
    modified_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,       -- Modified timestamp (default to current timestamp)
    user_id VARCHAR NOT NULL,                                -- User ID (VARCHAR)
    
    -- Foreign key to the folders table (folder_id references folders)
    CONSTRAINT fk_folder_id FOREIGN KEY (folder_id) 
        REFERENCES folders (folder_id) ON DELETE CASCADE    -- Cascade delete on folder deletion
);