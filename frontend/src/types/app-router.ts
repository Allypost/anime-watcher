export type PageProps<
  TParams extends Record<string, string> = Record<string, string>,
> = {
  params: TParams;
  searchParams: Record<string, string | string[] | undefined>;
};
