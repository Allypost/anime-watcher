import { env } from "~/env.mjs";

import { type ApiResponse } from "./types/base";

export const BASE_URL = env.NEXT_PUBLIC_API_BASE_URL.replace(/\/*$/g, "");

export const cacheTag = (...parts: unknown[]) => JSON.stringify(["api", parts]);

export function fetchApi<TVersion extends `v${number}`, TData>(
  url: `/${string}`,
  init?: RequestInit,
): Promise<ApiResponse<TVersion, TData> | null> {
  const method = init?.method?.toUpperCase() ?? "GET";

  return fetch(`${BASE_URL}${url}`, {
    mode: "cors",
    credentials: "include",
    redirect: "follow",
    method,
    ...init,
    next: {
      ...init?.next,
      tags: [`${method} ${url}`, ...(init?.next?.tags ?? [])],
      revalidate: 0,
    },
  })
    .then((res) => res.json() as Promise<ApiResponse<TVersion, TData>>)
    .catch((e) => {
      console.error(e);
      return null;
    });
}
