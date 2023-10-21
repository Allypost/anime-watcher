import { cacheTagV1, fetchV1 } from "./$fetch";

export const cacheTagAnimeSources = (...parts: unknown[]) =>
  cacheTagV1("anime", parts);

export const add = (data: {
  seriesId: number;
  seriesSite: string;
  seriesSiteId: string;
}) => {
  return fetchV1("/anime/sources", {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
    next: {
      tags: [cacheTagAnimeSources("add")],
    },
  });
};

export const remove = (id: number) => {
  return fetchV1<{ sourceId: number }>(`/anime/sources/${id}`, {
    method: "DELETE",
    next: {
      tags: [cacheTagAnimeSources("remove")],
    },
  });
};
