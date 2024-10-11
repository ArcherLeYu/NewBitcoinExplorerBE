
CREATE TABLE bitcoin_explorer (
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

CREATE TABLE blockinfo (
                           hash VARCHAR(64) NOT NULL,
                           confirmations BIGINT UNSIGNED NOT NULL,
                           size BIGINT UNSIGNED NOT NULL,
                           height BIGINT UNSIGNED NOT NULL,
                           version BIGINT UNSIGNED NOT NULL,
                           version_hex VARCHAR(16),
                           merkleroot VARCHAR(64) NOT NULL,
                           time BIGINT UNSIGNED NOT NULL,
                           mediantime BIGINT UNSIGNED NOT NULL,
                           nonce BIGINT UNSIGNED NOT NULL,
                           bits VARCHAR(16) NOT NULL,
                           difficulty DOUBLE NOT NULL,
                           chainwork VARCHAR(128) NOT NULL,
                           n_tx BIGINT UNSIGNED,
                           previousblockhash VARCHAR(64),
                           nextblockhash VARCHAR(64),
                           PRIMARY KEY (hash)
);
