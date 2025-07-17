import { BuildInfo } from '@bindings/domain/bindings/BuildInfo';
import { ItemListConfig } from '@/components/ItemListConfig';
import { useForm } from '@inertiajs/react';
// @ts-expect-error import type check
import Routes from '@routes';
import { useCallback, useRef } from 'react';
import { createItemsStore, ItemsContext } from '@states/preview';
import { useStore } from 'zustand';
import { Button, Container, Flex } from '@mantine/core';
import { FillRules } from './Index';

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
  build_data: BuildPreviewData,
  profile: FillRules,
};

export const Preview = ({ build_data, profile }: Props) => {
  const store = useRef(createItemsStore({ data: build_data.data, enabled: true })).current;

  const { post, errors } = useForm({
    config: build_data.data,
    itemset: build_data.itemset,
    skillset: build_data.skillset,
    pobData: build_data.pobData,
  } as InertiaFormType);

  const setEnabledEdit = useStore(store, s => s.enableEdit);
  const setDisableEdit = useStore(store, s => s.disableEdit);

  const patchForm = useCallback(() => {
    setDisableEdit();
    post(Routes.path('poe1.build-calc.new.new'), {
      onFinish: () => {
        setEnabledEdit();
      },
    })

  }, []);

  return (
    <Container fluid>
      <Flex
        direction='column'
      >
        <div>itemset: {build_data.itemset} skillset: {build_data.skillset} profile: {profile}
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
