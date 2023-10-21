import { type WithoutMeta } from "../types/base";
import { cacheTagV1, fetchV1 } from "./$fetch";
import { type Anime, type AnimeSource } from "./$models";

export const cacheTagAnime = (...parts: unknown[]) =>
  cacheTagV1("anime", parts);

export const list = () => {
  return fetchV1<
    {
      anime: Anime;
      sources: AnimeSource[];
    }[]
  >("/anime", {
    next: {
      tags: [cacheTagAnime("list"), "$anime/list"],
    },
  });
};

export const info = (id: number) => {
  type TResp = {
    anime: Anime;
    sources: AnimeSource[];
  };

  return fetchV1<TResp>(`/anime/${id}`, {
    next: {
      tags: [cacheTagAnime("info"), `$anime/info/${id}`],
    },
  });
};

export const malInfo = (id: number) => {
  return fetchV1(`/anime/${id}/mal-info`, {
    next: {
      tags: [cacheTagAnime("malInfo"), `$anime/mal-info/${id}`],
    },
  });
};

export const update = (id: number, data: WithoutMeta<Anime>) => {
  type TResp = {
    payload: typeof data;
    result: Anime;
  };

  if (!data.malId) {
    data.malId = null;
  } else {
    data.malId = Number(data.malId);
  }

  return fetchV1<TResp>(`/anime/${id}`, {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
    next: {
      tags: [cacheTagAnime("update"), `$anime/update/${id}`],
    },
  });
};
