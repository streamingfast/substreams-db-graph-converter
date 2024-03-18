# Converter database changes to entity changes

This substreams contains `dbout_to_graphout` module enabling database changes conversion into entity changes. 

## Usage 

This substreams package could be imported in another substreams, using imports, in the substreams manifest. After being imported, the module `dbout_to_graphout` can be used by another module implementing the `use` attribute. The module using `dbout_to_graphout` could override its inputs, and its initial block.

**Note**: Each input should have the same output type than the corresponding overrided input 

## Example 

```yaml
specVersion: v0.1.0
package:
  name: byac
  version: v0.1.0

imports:
  converter: https://spkg.io/streamingfast/substreams-db-graph-converter-v0.1.0.spkg

modules:
...
  - name: db_out
    kind: map
    initialBlock: 12287507
    inputs:
      - map: map_events
    output:
      type: proto:sf.substreams.sink.database.v1.DatabaseChanges

  - name: graph_out
    use: converter:dbout_to_graphout
    inputs:
      - map: db_out
```

In this example, the `graph_out` module, uses `dbout_to_graphout` imported module, and overrides its inputs by the map module `db_out` 
