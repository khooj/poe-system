interface Route {
  readonly name: string;
  readonly action: string;
  readonly path: string;
  readonly method: string;
  readonly controller: string;
  readonly params: readonly string[];
}

type HTTPMethod = GET | POST;

type QueryParam = string | number | boolean | null | undefined;
type QueryParams = Record<string, QueryParam | QueryParam[]>;

type RouteParams = {
  "index": Record<string, never>;
  "poe1.build-calc.index": Record<string, never>;
  "poe1.build-calc.new.new": Record<string, never>;
  "poe1.build-calc.get_build": {id: string | number};
  "sse.subscribe": Record<string, never>;
  "api.v1.extract.extract": Record<string, never>
}

type RouteName = "index" | "poe1.build-calc.index" | "poe1.build-calc.new.new" | "poe1.build-calc.get_build" | "sse.subscribe" | "api.v1.extract.extract";

type RouteParamsWithQuery<T extends Record<string, any>> = T & {
  _query?: QueryParams;
}

type RoutePathConfig = {
  "/": Record<string, never>;
      "/poe1/build-calc": Record<string, never>;
      "/poe1/build-calc/new": Record<string, never>;
      "/poe1/build-calc/:id": {id: string | number};
      "/sse": Record<string, never>;
      "/api/v1/extract": Record<string, never>
}

type RoutePath = keyof RoutePathConfig;

type PathParamsWithQuery<T extends RoutePath> = RoutePathConfig[T] & {
  _query?: QueryParams;
}

declare const Routes: {
  readonly routes: readonly Route[];

  route<T extends RouteName>(
    name: T,
    params?: RouteParamsWithQuery<RouteParams[T]>
  ): string;

  path<T extends RouteName>(
    name: T,
    params?: RouteParamsWithQuery<RouteParams[T]>
  ): string;

  replaceParams<T extends RoutePath>(
    path: T,
    params?: PathParamsWithQuery<T>
  ): string;

  method(name: RouteName): HTTPMethod;

  hasRoute(name: string): name is RouteName;
};

export as namespace Routes;
export { RoutePath, PathParamsWithQuery };
export = Routes;
