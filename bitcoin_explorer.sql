CREATE TABLE Blocks (
                        automatic_pruning BOOLEAN NOT NULL DEFAULT false,
                        block_hash VARCHAR(64) PRIMARY KEY,
                        blocks BIGINT,
                        chain VARCHAR(32),
                        chainwork VARCHAR(128),
                        difficulty DOUBLE,
                        headers BIGINT,
                        initialblockdownload BOOLEAN,
                        mediantime BIGINT,
                        prune_target_size BIGINT,
                        pruned BOOLEAN,
                        pruneheight BIGINT,
                        size_on_disk BIGINT,
                        time BIGINT,
                        verificationprogress DOUBLE,
                        warnings TEXT
);

