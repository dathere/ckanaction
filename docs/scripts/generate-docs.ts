import { generateFiles } from "fumadocs-openapi";
import { createOpenAPI } from "fumadocs-openapi/server";

void generateFiles({
  input: createOpenAPI({
    input: ["./lib/openapi.yml"],
    disableCache: true
  }),
  output: "./content/docs",
  // we recommend to enable it
  // make sure your endpoint description doesn't break MDX syntax.
  includeDescription: true,
});
