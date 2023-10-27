To use:

1. Manually write an adjacencies graph YAML. See `example-graph.yaml`
2. cargo run `[adjacencies-graph.yaml]` to generate a directory containing a `shadow.yaml` and a `configs` directory tree containing relevant earendil configs
3. modify the `shadow.yaml` to have nodes run desired programs. Then `shadow shadow.yaml` to run the shadow simulation.