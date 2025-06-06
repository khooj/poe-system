import { TypedLink } from '@/components/TypedLink';
import { usePage } from '@inertiajs/react';
import { ReactNode } from 'react';
import { Container, Nav, Navbar, NavDropdown } from 'react-bootstrap';
import Alert from '@/components/Alert';

type Props = {
  children: ReactNode
};

export default function Layout({ children }: Props) {
  const { flash } = usePage().props;

  return (
    <>
      <Navbar expand='lg' className=''>
        <Container>
          <TypedLink to='index' className='navbar-brand'>Home</TypedLink>
          <Navbar.Toggle aria-controls='basic-navbar-nav' />
          <Navbar.Collapse id='basic-navbar-nav'>
            <Nav className='me-auto'>
              <NavDropdown title='Path of Exile 1' id='poe1-nav-dropdown'>
                <NavDropdown.Item disabled>Build price calculator</NavDropdown.Item>
                <TypedLink to='poe1.index' className='dropdown-item'>New build</TypedLink>
              </NavDropdown>
            </Nav>
          </Navbar.Collapse>
        </Container>
      </Navbar>
      {flash?.info && <Alert>{flash.info}</Alert>}
      {children}
    </>
  )
}
