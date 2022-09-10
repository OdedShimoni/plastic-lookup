
# Elastic Lookup

Ultra fast, in-memory db to look for a word in a word list and data related to it.


## Installation

Clone the repository, then

```bash
  cargo build --release
```
    
## Usage
Currently via stdin and stdout.

Run the executable through your app or directly.

### Actions
```bash
store add {store_name} # Creates a store
word add {store_name} {word} {data} # Adds a word
store lookup {store_name} {word} # Retrieves a word

store ls # Lists all stores and data
```

Data is json-parsed using serde-json.

### Answers
Answer has 2 lines:

#### Line 1, Status:
* 2 - found
* 1 - partially found / successfuly inserted
* 0 - not found

#### Line 2, Data
If exists
## Contributing

Contributions are always welcome!

The next feature is going to be a CLI client for easier communication.
