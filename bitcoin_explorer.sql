
CREATE TABLE blocksummary (
                              hash VARCHAR(64) PRIMARY KEY,
                              height BIGINT NOT NULL,
                              timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE bitcoin_prices (
                                id INT(11) NOT NULL AUTO_INCREMENT,
                                price DECIMAL(20,8) NOT NULL,
                                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                                PRIMARY KEY (id)
);
CREATE TABLE bitcoin_volumes ( id INT(11) NOT NULL AUTO_INCREMENT, volume DECIMAL(20,8) NOT NULL, timestamp DATETIME DEFAULT CURRENT_TIMESTAMP, PRIMARY KEY (id) );

CREATE TABLE blocksummary (
                              hash VARCHAR(64) NOT NULL PRIMARY KEY,
                              height BIGINT NOT NULL,
                              timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
