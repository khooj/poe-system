// @ts-expect-error type error
import Routes from '@routes';
import { router, useForm } from '@inertiajs/react'
import { ChangeEventHandler, FormEvent, useEffect, useState } from 'react';
import useSWR from 'swr';
import axios from 'axios';
import { Button, Container, Flex, Loader, NativeSelect, TextInput } from '@mantine/core';

const wasmLoader = async () => await import('wasm');

export type Props = {
  build_ids: string[],
};

export const Index = ({ build_ids }: Props) => {
  const { data: formData, setData } = useForm({
    pobData: null,
    itemset: null,
    skillset: null,
  } as { pobData: string | null, itemset: string | null, skillset: string | null });

  const [itemsets, setItemsets] = useState([] as string[]);
  const [skillsets, setSkillsets] = useState([] as string[]);
  const [parsing, setParsing] = useState(false);
  const [pobFormError, setPobFormError] = useState(null as string | null);
  const { data: wasm, error: wasmError, isLoading: wasmLoading } = useSWR('wasm', wasmLoader);

  // const [wasm, setWasm] = useState();
  // const [wasmError, setWasmError] = useState();
  // const [wasmLoading, setWasmLoading] = useState(true);

  // useEffect(() => {
  //   (async () => {
  //     try {
  //       setWasm(await wasmLoader());
  //     } catch (err) {
  //       setWasmError(err);
  //     } finally {
  //       setWasmLoading(false);
  //     }
  //   })()
  // });

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

  const itemsetSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const resp = await axios.post(Routes.path('api.extract.extract'), formData);
    console.log(resp);
    if (resp.status === 200) {
      router.push({
        url: '/poe1',
        component: 'poe1/Preview',
        props: {
          build_data: {
            data: resp.data.config,
            ...formData,
          }
        },
      })
    }
  };

  return (
    <Container fluid>
      <Flex
        justify="center"
        align="center"
        direction="column"
      >
        <span>This tool can calculate price for itemset and skill gems exported from Path of Building app</span>
        {wasmLoading && <>
          <Loader />
          <span>Loading wasm bundle</span>
        </>}
        {wasmError && <span>Wasm loading error, try to reload page</span>}
        {!wasmLoading && !wasmError && <>
          <form onSubmit={itemsetSubmit} encType='multipart/form-data'>
            <TextInput
              label="Path of Building data"
              onChange={onChange}
            />
            {parsing && <div>
              <span>Parsing itemsets...</span>
              <Loader />
            </div>}
            {pobFormError && <span>Please provide correct path of building encoded in base64<br />(typically provided at export menu or in code blocks)</span>}
            {itemsets.length > 0 && <NativeSelect
              label="Select desired itemset"
              data={itemsets}
              onChange={e => setData('itemset', e.target.value)}
            />}
            {skillsets.length > 0 && <NativeSelect
              label="Select desired skillset"
              data={skillsets}
              onChange={e => setData('skillset', e.target.value)}
            />}
            <Flex justify="center">
              <Button type="submit" disabled={!(formData.pobData && formData.itemset && formData.skillset)}>Proceed</Button>
            </Flex>
          </form>
        </>}
      </Flex>
    </Container>
  )
}

export default Index
