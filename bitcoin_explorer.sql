
CREATE TABLE Blocks (
                        automatic_pruning TINYINT(1) NOT NULL DEFAULT 0,
                        bestblockhash VARCHAR(64) PRIMARY KEY,
                        blocks BIGINT,
                        chain VARCHAR(32),
                        chainwork VARCHAR(128),
                        difficulty DOUBLE,
                        headers BIGINT,
                        initialblockdownload TINYINT(1),
                        mediantime BIGINT,
                        prune_target_size BIGINT,
                        pruned TINYINT(1),
                        pruneheight BIGINT,
                        size_on_disk BIGINT,
                        time BIGINT,
                        verificationprogress DOUBLE,
                        warnings TEXT
);
