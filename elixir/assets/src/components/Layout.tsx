import { Link } from '@inertiajs/react';
import { ReactNode } from 'react';
import { Container, Nav, Navbar } from 'react-bootstrap';

type Props = {
  children: ReactNode
};

export default function Layout({ children }: Props) {
  return (
    <Container fluid className="d-flex flex-row border border-info justify-content-start">
      <Nav defaultActiveKey="/" className='flex-column vh-100 border border-warning'>
        <Nav.Item>
          <Nav.Link disabled>Path of Exile 1</Nav.Link>
        </Nav.Item>
        <Nav.Item>
          <Nav.Link href="/">New build</Nav.Link>
        </Nav.Item>
      </Nav>
      <div className='d-flex border border-danger'>
        {children}
      </div>
    </Container>
  )
}
