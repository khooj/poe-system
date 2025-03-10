import { Head, useForm } from '@inertiajs/react'
import { useRef, useState } from 'react';
import { Container, Form, Spinner } from 'react-bootstrap'
import * as wasm from 'wasm';

type Props = {
  text: string,
}

const Index = ({ text }: Props) => {
  const { data, setData, post, processing, errors } = useForm({
    pobFile: null,
    itemset: null,
  });

  const [pob, setPob] = useState(null);
  const [itemsets, setItemsets] = useState([]);
  const fileRef = useRef(null);
  const [parsing, setParsing] = useState(false);

  const submit = (e) => {
    e.preventDefault();
    const reader = new FileReader();
    reader.onload = e => {
      const itemsets = wasm.get_pob_itemsets(e.target?.result);
      setItemsets(itemsets);
      setData('itemset', itemsets[0]);
      setParsing(false);
    };
    setParsing(true);
    reader.readAsText(data.pobFile);
  };

  const itemsetSubmit = (e) => {
    e.preventDefault();
    post('/new');
  };

  return (
    <Container className="d-flex flex-column align-items-center justify-content-center">
      <Form onSubmit={submit}>
        <Form.Group className="mb-3" controlId="formBuildFile">
          <Form.Label>Path of Building file</Form.Label>
          <Form.Control ref={fileRef} type="file" onChange={e => setData('pobFile', e.target.files[0])} />
          <Form.Control type="submit" value="Proceed" />
        </Form.Group>
      </Form>
      {parsing && <div>
        <span>Parsing itemsets...</span>
        <Spinner animation='border' role='status'>
        </Spinner>
      </div>}
      {itemsets.length > 0 && <Form onSubmit={itemsetSubmit} encType='multipart/form-data'>
        <Form.Group className='mb-3' controlId='formItemset'>
          <Form.Label>Select desired itemset</Form.Label>
          <Form.Select aria-label='Itemset selection' onChange={e => setData('itemset', e.target.value)}>
            {itemsets.map(is => <option>{is}</option>)}
          </Form.Select>
          <Form.Control type="submit" value="Calculate" />
        </Form.Group>
      </Form>}
    </Container>
  )
}

export default Index
