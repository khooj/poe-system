import { useForm } from '@inertiajs/react'
import { useState } from 'react';
import { Container, Form, Spinner } from 'react-bootstrap'
import * as wasm from 'wasm';

const Index = () => {
  const { data, setData, post, processing, errors } = useForm({
    pobData: null,
    itemset: null,
  });

  const [itemsets, setItemsets] = useState([]);
  const [parsing, setParsing] = useState(false);

  const submit = (e) => {
    e.preventDefault();
    try {
      setParsing(true);
      const itemsets = wasm.get_pob_itemsets(data.pobData);
      setItemsets(itemsets);
      setData('itemset', itemsets[0]);
    } catch (e) {
      console.log(e);
    } finally {
      setParsing(false);
    }
  };

  const itemsetSubmit = (e) => {
    e.preventDefault();
    post('/new');
  };

  return (
    <Container className="d-flex flex-column align-items-center justify-content-center">
      <Form onSubmit={submit}>
        <Form.Group className="mb-3" controlId="formBuildFile">
          <Form.Label>Path of Building data</Form.Label>
          <Form.Control type="text" onChange={e => setData('pobData', e.target.value)} />
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
