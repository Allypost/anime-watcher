import { cacheTag, fetchApi } from "../fetch";
import { type ApiV1Response } from "./$models";

export const cacheTagV1 = (...parts: unknown[]) => cacheTag("v1", parts);

export function fetchV1<T>(
  url: `/${string}`,
  init?: RequestInit,
): Promise<ApiV1Response<T> | null> {
  return fetchApi<"v1", T>(`/v1${url}`, init);
}
