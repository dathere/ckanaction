import { createAPIPage } from "fumadocs-openapi/ui";
import client from "@/components/api-page.client";
import { openapi } from "@/lib/openapi";

export const APIPage = createAPIPage(openapi, {
  client,
});
