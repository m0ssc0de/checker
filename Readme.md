# a service

1. block ranges, 
    1. start, latest, missed
2. block range, 
    1. headers, sorted, data integrity
3. latest block on chain
4. fetching jobs
    1. launch fetching historical job
    3. launch catching up job

Project name could be 'Glacier'?
The main duty is offering a stability data source.

# Data source service

Named Glacier?

A service to maintain reliable blockchain data, ethereum for now.

The data structure is as the graph demonstrates.

The filesystem structure will be based on continuous block number intervals. For example, a range '0012700000-0012709999' means including block '12700000' to '12709999'. A range '0015924902-0015924902' means data of one block '15924902'.

```shell
$ tree .
.
├── 0012940000-0012949999
│   ├── blocks.csv
│   ├── contract_addresses.txt
│   ├── contracts.csv
│   ├── logs.csv
│   ├── receipts.csv
│   ├── token_addresses.txt
│   ├── token_transfers.csv
│   ├── tokens.csv
│   ├── transaction_hashes.txt
│   └── transactions.csv
└── 0012950000-0012959999
    ├── blocks.csv
    ├── contract_addresses.txt
    ├── contracts.csv
    ├── logs.csv
    ├── receipts.csv
    ├── token_addresses.txt
    ├── token_transfers.csv
    ├── tokens.csv
    ├── transaction_hashes.txt
    └── transactions.csv
```

Every file will be sorted by block number as much as possible without csv header.

Main features
- Check latest range and block fechted
- Check data integrity for every range
- Check the gap between ranges
- Launch jobs to fetch missed or new range/block
- Expose metrics of all ranges and running jobs