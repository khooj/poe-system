import { Link as InertiaLink, InertiaLinkProps } from "@inertiajs/react";
import React from "react";
// @ts-expect-error no default in routes
import type { PathParamsWithQuery, RouteName } from "@routes";
import Routes from "@routes";

type TypedLinkProps<T extends RouteName> = Omit<InertiaLinkProps, "href"> & {
  to: T;
  params?: PathParamsWithQuery<T>;
  children: React.ReactNode;
};

export const TypedLink = <T extends RouteName>({
  to,
  params,
  children,
  ...props
}: TypedLinkProps<T>) => {
  const href = Routes.path(to, params);

  return (
    <InertiaLink href={href} {...props}>
      {children}
    </InertiaLink>
  );
};
