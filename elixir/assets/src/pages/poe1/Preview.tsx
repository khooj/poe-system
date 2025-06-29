import { BuildInfo } from '@bindings/domain/bindings/BuildInfo';
import { ItemListConfig } from '@/components/ItemListConfig';
import { useForm, router } from '@inertiajs/react';
// @ts-expect-error import type check
import Routes from '@routes';
// eslint-disable-next-line @typescript-eslint/no-unused-vars
import _ from 'lodash';
import { useCallback, useEffect, useRef, useState } from 'react';
import { createItemsStore, ItemsContext } from '@states/preview';
import { useStore } from 'zustand';
import { Button, Container, Flex } from '@mantine/core';

export type BuildPreviewData = {
  itemset: string,
  skillset: string,
  pobData: string,
  data: BuildInfo,
};

type InertiaFormType = {
  config: BuildInfo,
  itemset: string,
  skillset: string,
  pobData: string,
};

export type Props = {
  build_data: BuildPreviewData
};

export const Preview = ({ build_data }: Props) => {
  const store = useRef(createItemsStore({ data: build_data.data, enabled: true })).current;

  const { setDefaults, post, setData, data, isDirty, errors, processing } = useForm({
    config: build_data.data,
    itemset: build_data.itemset,
    skillset: build_data.skillset,
    pobData: build_data.pobData,
  } as InertiaFormType);

  const [save, setSave] = useState<'haveChanges' | 'saving' | 'noChanges'>('noChanges');
  const setEnabledEdit = useStore(store, s => s.enableEdit);
  const setDisableEdit = useStore(store, s => s.disableEdit);

  useEffect(() => {
    if (isDirty) {
      setSave('haveChanges');
    } else if (processing) {
      setSave('saving');
    } else if (!isDirty) {
      setSave('noChanges');
    }
  }, [isDirty, processing, setSave]);

  const patchForm = useCallback(() => {
    setDisableEdit();
    post(Routes.path('poe1.build-calc.new.new'), {
      onFinish: () => {
        setEnabledEdit();
      },
    })

  }, [post, setEnabledEdit, setDisableEdit]);

  useEffect(() => {
    console.log('subscribe for data');
    const unsub = store.subscribe((state) => {
      console.log('subscribed cb');
      // make prev data as default to trigger isDirty flag on every change
      // even if it returned to previous state
      setDefaults();
      setData('config', state.data);
    });
    return () => {
      unsub();
    };
  }, [setDefaults, setData, store]);

  return (
    <Container fluid>
      <Flex
        direction='column'
      >
        <div>itemset: {build_data.itemset} skillset: {build_data.skillset}
          {errors.config && <p>Error occured: {errors.config}</p>}
        </div>
        <div>
          <Button onClick={patchForm} variant='filled'>Submit build</Button>
        </div>
        <ItemsContext.Provider value={store}>
          <ItemListConfig />
        </ItemsContext.Provider>
      </Flex>
    </Container>
  )
}

export default Preview;
