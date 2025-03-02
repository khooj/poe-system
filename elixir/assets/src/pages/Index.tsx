import { Head, useForm } from '@inertiajs/react'
import { Container, Form } from 'react-bootstrap'

type Props = {
  text: string,
}

const Index = ({ text }: Props) => {
  const { data, setData, post, processing, errors } = useForm({
    pobLink: null,
    pobFile: null,
  });

  const submit = (e) => {
    e.preventDefault();
    post('/new');
  };

  return (
    <Container className="d-flex flex-column align-items-center justify-content-center">
      <Form onSubmit={submit}>
        <Form.Group className="mb-3" controlId="formBuildUrl">
          <Form.Label>Pobb.in/Poe.ninja links</Form.Label>
          <Form.Control type="text" placeholder="Enter url" value={data.pobLink} onChange={e => setData('pobLink', e.target.value)} />
          <Form.Control type="submit" value="Calculate" />
        </Form.Group>
      </Form>

      <Form onSubmit={submit} encType='multipart/form-data'>
        <Form.Group className="mb-3" controlId="formBuildFile">
          <Form.Label>Path of Building file</Form.Label>
          <Form.Control type="file" onChange={e => setData('pobFile', e.target.files[0])} />
          <Form.Control type="submit" value="Calculate" />
        </Form.Group>
      </Form>
    </Container>
  )
}

export default Index
