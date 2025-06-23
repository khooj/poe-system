import { useEffect } from "react";

export const ReloadFrame = (Story) => {
  useEffect(() => {
    return () => window.location.reload();
  });

  return <Story />;
};
