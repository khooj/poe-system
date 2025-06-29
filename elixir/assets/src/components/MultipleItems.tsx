import { Box, Button, Collapse, Spoiler } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { JSX, useId, useState } from "react";

type MultipleItemsProps = {
  itemKey: string,
  children: JSX.Element[],
};

const MultipleItems = ({ itemKey, children }: MultipleItemsProps) => {
  const [open, { toggle }] = useDisclosure(false);

  return <Box>
    <Button onClick={toggle}>{itemKey}</Button>

    <Collapse in={open}>
      <div>
        {children}
      </div>
    </Collapse>
  </Box>
};

export default MultipleItems;
