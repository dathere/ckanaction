"use client";

import Script from "next/script";
import { useEffect, useState } from "react";

export const DatHereAnalyticsScript = () => {
  const [hostname, setHostname] = useState<string>();

  useEffect(() => {
    if (process) {
      setHostname(window.location.hostname);
    }
  }, []);

  return hostname === "ckanaction.dathere.com" ? (
    <Script
      src="https://mk-analytics.dathere.com/api/script.js"
      data-site-id="10"
      strategy="afterInteractive"
    />
  ) : (
    <></>
  );
};
