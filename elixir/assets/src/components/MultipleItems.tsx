import { JSX, useId, useState } from "react";
import { Collapse } from "react-bootstrap";
import { Button } from "react-bootstrap";

type MultipleItemsProps = {
  itemKey: string,
  children: JSX.Element[],
};

const MultipleItems = ({ itemKey, children }: MultipleItemsProps) => {
  const [open, setOpen] = useState(false);
  const id = useId();

  return <div className='m-3'>
    <Button onClick={() => setOpen(!open)} aria-controls={id} aria-expanded={open}>{itemKey}</Button>
    <Collapse in={open}>
      <div id={id}>
        {children}
      </div>
    </Collapse>
  </div>
};

export default MultipleItems;
