import Link from "next/link";

export default function HomePage() {
  return (
    <main className="flex flex-1 flex-col justify-center text-center">
      <h1 className="mb-4 text-2xl font-bold">ckanaction</h1>
      <p className="text-fd-muted-foreground">
        You can open{" "}
        <Link
          href="/docs"
          className="text-fd-foreground font-semibold underline"
        >
          /docs
        </Link>{" "}
        and see the interactive documentation.
      </p>
      <footer className="absolute bottom-2 ml-4 flex flex-col bg-brand-secondary text-brand-secondary-foreground rounded-2xl">
        <p className="text-xs">
          Provided by{" "}
          <a
            href="https://dathere.com"
            target="_blank"
            className="font-medium text-blue-400"
            rel="noopener"
          >
            datHere
          </a>
          .{" "}
          <a
            href="https://dathere.com/privacy-policy/"
            target="_blank"
            className="font-medium text-blue-400"
            rel="noopener"
          >
            Privacy Policy
          </a>
          .
        </p>
      </footer>
    </main>
  );
}
