// @ts-expect-error type error
import Routes from '@routes';
import { Link, useForm } from '@inertiajs/react'
import { ChangeEventHandler, FormEvent, useState } from 'react';
import { Container, Form, Spinner } from 'react-bootstrap'
import useSWR from 'swr';

const wasmLoader = async () => await import('wasm');

type Props = {
  build_ids: string[],
};

const Index = ({ build_ids }: Props) => {
  const { setData, post } = useForm({
    pobData: null,
    itemset: null,
    skillset: null,
  } as { pobData: string | null, itemset: string | null, skillset: string | null });

  const [itemsets, setItemsets] = useState([] as string[]);
  const [skillsets, setSkillsets] = useState([] as string[]);
  const [parsing, setParsing] = useState(false);
  const [pobFormError, setPobFormError] = useState(null as string | null);
  const { data: wasm, error: wasmError, isLoading: wasmLoading } = useSWR('wasm', wasmLoader);

  const onChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    e.preventDefault();
    try {
      setParsing(true);
      // @ts-expect-error undefined wasm
      const itemsets = wasm.get_pob_itemsets(e.target.value);
      setItemsets(itemsets);
      setData('itemset', itemsets[0]);
      // @ts-expect-error undefined wasm
      const skillsets = wasm.get_pob_skillsets(e.target.value);
      setSkillsets(skillsets);
      setData('skillset', skillsets[0]);
      setData('pobData', e.target.value);
      setPobFormError(null);
    } catch (e) {
      setPobFormError(e as string);
      setItemsets([]);
      setSkillsets([]);
      console.log(e);
    } finally {
      setParsing(false);
    }
  };

  const itemsetSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    post(Routes.path('poe1.extract.extract'))
  };

  return (
    <Container fluid className="d-flex flex-column align-items-center justify-content-center border border-warning">
      {wasmLoading && <><Spinner animation='border' role='status'>
      </Spinner>
        <span>Loading wasm bundle</span>
      </>}
      {wasmError && <span>Wasm loading error, try to reload page: {wasmError}</span>}
      {!wasmLoading && !wasmError && <>
        <Form onSubmit={itemsetSubmit} encType='multipart/form-data'>
          <Form.Group className="mb-3" controlId="formBuildFile">
            <Form.Label>Path of Building data</Form.Label>
            <Form.Control type="text" required onChange={onChange} />
            {parsing && <div>
              <span>Parsing itemsets...</span>
              <Spinner animation='border' role='status'>
              </Spinner>
            </div>}
            {pobFormError && <span>{pobFormError}</span>}
            {itemsets.length > 0 && <><Form.Label>Select desired itemset</Form.Label>
              <Form.Select aria-label='Itemset selection' onChange={e => setData('itemset', e.target.value)}>
                {itemsets.map(is => <option>{is}</option>)}
              </Form.Select>
            </>}
            {skillsets.length > 0 && <><Form.Label>Select desired skillset</Form.Label>
              <Form.Select aria-label='Skillset selection' onChange={e => setData('skillset', e.target.value)}>
                {skillsets.map(is => <option>{is}</option>)}
              </Form.Select>
            </>}
            <Form.Control type="submit" value="Proceed" />
          </Form.Group>
        </Form>
        <div className='d-flex flex-column'>
          <div>or select build id</div>
          {build_ids.map(id => <div><Link href={`/build/${id}`}>{id}</Link></div>)}
        </div>
      </>}
    </Container>
  )
}

export default Index
