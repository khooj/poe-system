import { useState } from 'react';
import { Alert as BAlert } from 'react-bootstrap';

export default function Alert({ children }) {
  const [show, setShow] = useState(true);
  if (show) {
    return <BAlert variant='primary' dismissible onClose={() => setShow(false)}>
      {children}
    </BAlert>
  }

  return <></>
}
