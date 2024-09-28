# Transaction Manager

A toy transaction manager that can handle reading a CSV containing a listing of transactions.

## How to run

We can run it like so:

```bash
cargo run -- input.csv > output.csv
```

where `input.csv` is a CSV containing a listing of transactions.

## Supported Transactions

We have the following transactions which are supported in the input CSV file.

### Deposit

deposit, <client>, <tx>, <amount>

where

* deposit - the type
* <client> - the client id
* <tx> - transaction id
* <amount> - the amount to deposit

### Withdrawal

withdrawal, <client>, <tx>, <amount>

where

* withdrawal - the type
* <client> - the client id
* <tx> - transaction id
* <amount> - the amount to withdraw

### Dispute

dispute, <client>, <tx>, 

where

* dispute - the type
* <client> - the client id whose account has the disputed transaction
* <tx> - transaction id of the transaction being disputed

### Resolve

resolve, <client>, <tx>, 

where

* resolve - the type
* <client> - the client id whose account has the disputed transaction we want to resolve
* <tx> - transaction id of the transaction being resolved

### Chargeback

chargeback, <client>, <tx>, 

where

* chargeback - the type
* <client> - the client id whose account has the disputed transaction we want to chargeback
* <tx> - transaction id of the transaction being charged back

