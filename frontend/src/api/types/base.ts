import { type Simplify } from "type-fest";

export type ApiResponse<TVersion extends `v${number}`, TData> = {
  meta: {
    /**
     * Date and time of the response in ISO 8601 format.
     */
    at: string;
    /**
     * Version of the API.
     */
    v: TVersion;
    /**
     * Status code of the response.
     */
    status: number;
  };

  body:
    | {
        type: "success";
        data: TData;
      }
    | {
        type: "error";
        data: string;
      }
    | {
        type: "empty";
        data: null;
      };
};

export type WithoutMeta<T, TAlsoOmit extends keyof T = never> = Simplify<
  Omit<T, "id" | "createdAt" | "updatedAt" | TAlsoOmit>
>;
