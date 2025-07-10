// @ts-expect-error type error
import Routes from '@routes';
import { router } from '@inertiajs/react'
import { useEffect, useState } from 'react';
import useSWR from 'swr';
import axios from 'axios';
import { Button, Container, Flex, Loader, NativeSelect, Text, TextInput } from '@mantine/core';
import { useForm } from '@mantine/form';
import wasmInit, { get_pob_itemsets, get_pob_skillsets } from 'wasm';

export type FillRules = 'simpleeverything' | 'simplenores';

type FormValues = {
  pobData: string,
  itemset: string,
  skillset: string,
  profile: FillRules
};

export const Index = () => {
  const [itemsets, setItemsets] = useState<Array<string>>([]);
  const [skillsets, setSkillsets] = useState<Array<string>>([]);
  const [parsing, setParsing] = useState(false);
  const { error: wasmError, isLoading: wasmLoading } = useSWR('wasm', async () => {
    const ret = await wasmInit();
    return ret;
  });

  const form = useForm<FormValues>({
    mode: 'uncontrolled',
    initialValues: {
      pobData: '',
      itemset: '',
      skillset: '',
      profile: 'simpleeverything'
    },
    validateInputOnChange: [
      'pobData',
    ],
    validate: {
      pobData: (d) => {
        // FIXME: validates only on first time
        try {
          console.log('check')
          get_pob_itemsets(d);
          return null;
        } catch (e) {
          console.log('error', e);
          // @ts-expect-error unknown exception from wasm
          console.log('error stack', e.stack);
          return 'Please provide correct path of building encoded in base64 (typically provided at export menu or in code blocks)';
        }
      }
    },
  });

  form.watch('pobData', ({ value }) => {
    try {
      setParsing(true);
      const itemsets = get_pob_itemsets(value);
      setItemsets(itemsets);
      form.setFieldValue('itemset', itemsets[0]);
      const skillsets = get_pob_skillsets(value);
      setSkillsets(skillsets);
      form.setFieldValue('skillset', skillsets[0]);
      form.isValid();
    } catch (e) {
      console.log('caught smth i shouldnt', e);
    } finally {
      setParsing(false);
    }
  });

  const onSubmit = async (values: typeof form.values) => {
    const resp = await axios.post(Routes.path('api.v1.extract.extract'), values);

    console.log(resp);
    if (resp.status === 200) {
      router.push({
        url: '/poe1',
        component: 'poe1/Preview',
        props: {
          build_data: {
            data: resp.data.config,
            ...values,
          },
          profile: values.profile
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
          <form onSubmit={form.onSubmit(onSubmit)} encType='multipart/form-data'>
            <TextInput
              label="Path of Building data"
              placeholder='base64-encoded string'
              key={form.key('pobData')}
              {...form.getInputProps('pobData')}
            />
            <NativeSelect
              label="Select profile"
              key={form.key('profile')}
              {...form.getInputProps('profile')}
            >
              <option value='simpleeverything'>Simple</option>
              <option value='simplenores'>Simple w/o Elemental resistances</option>
            </NativeSelect>
            <Text>
              Simple profile sets predefined search options such as: <br />
              - every unique item searched by it's name (ignoring stats) <br />
              - every non-unique item searched by it's mods with 'exist' option <br />
              Simple w/o elemental resistances profile sets predefined search options such as: <br />
              - everything as in simple profile except elemental resistances on non-unique items ignored <br />
            </Text>
            {parsing && <div>
              <span>Parsing itemsets...</span>
              <Loader />
            </div>}
            {itemsets.length > 0 && <NativeSelect
              label="Select desired itemset"
              key={form.key('itemset')}
              data={itemsets}
              {...form.getInputProps('itemset')}
            />}
            {skillsets.length > 0 && <NativeSelect
              label="Select desired skillset"
              key={form.key('skillset')}
              data={skillsets}
              {...form.getInputProps('skillset')}
            />}
            <Flex justify="center">
              <Button type="submit">Proceed</Button>
            </Flex>
          </form>
        </>}
      </Flex>
    </Container>
  )
}

export default Index
