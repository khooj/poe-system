import { BuildInfo } from '@bindings/domain/bindings/BuildInfo';
import { Button, Container, Spinner } from 'react-bootstrap';
import { ItemListConfig } from '@/components/ItemListConfig';
import { useForm, router } from '@inertiajs/react';
// @ts-expect-error import type check
import Routes from '@routes';
// eslint-disable-next-line @typescript-eslint/no-unused-vars
import _ from 'lodash';
import { useCallback, useEffect, useRef, useState } from 'react';
import { ToggleButton } from 'react-bootstrap';
import { createItemsStore, ItemsContext } from '@states/preview';

type BuildPreviewData = {
  id: string,
  itemset: string,
  skillset: string,
  pob: string,
  data: BuildInfo,
};

type InertiaFormType = {
  config: BuildInfo | null,
  id: string
};

type Props = {
  build_data: BuildPreviewData
};

const Preview = ({ build_data }: Props) => {
  const store = useRef(createItemsStore({ data: build_data.data })).current;

  const { setDefaults, patch, setData, data, isDirty, errors, processing } = useForm({
    config: build_data.data,
    id: build_data.id,
  } as InertiaFormType);

  const [autosave, setAutosave] = useState(false);
  const [save, setSave] = useState<'haveChanges' | 'saving' | 'noChanges'>('noChanges');

  useEffect(() => {
    if (isDirty) {
      setSave('haveChanges');
    } else if (processing) {
      setSave('saving');
    } else if (!isDirty) {
      setSave('noChanges');
    }
  }, [isDirty, processing, setSave]);

  const renderSave = () => {
    switch (save) {
      case 'noChanges': return <>Up to date</>
      case 'saving': return <>Saving<Spinner animation="border" size="sm" role="status"></Spinner></>
      case 'haveChanges': return <>Save</>
    }
  };

  const patchForm = useCallback(() => {
    patch(Routes.path('poe1.preview.update_preview'), {
      onSuccess: () => setDefaults(),
    });
  }, [patch, setDefaults]);

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

  useEffect(() => {
    console.log('current data is null', data.config === null);
    if (autosave) {
      patchForm();
    }
    // we dont want to trigger saving on autosave flag toggle
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [data, patchForm]);

  return (
    <Container fluid className='d-flex flex-column align-content-center main-color'>
      <div>itemset: {build_data.itemset} skillset: {build_data.skillset}
        {errors.config && <p>Error occured: {errors.config}</p>}
      </div>
      <div className='d-flex'>
        <ToggleButton id="autosave-toggle" value="1" type="checkbox" variant="outline-success" checked={autosave} onChange={(e) => setAutosave(e.currentTarget.checked)}>
          {autosave && "Autosave enabled" || "Enable autosave"}
        </ToggleButton>
        {isDirty && <p>Changes not saved</p>}
      </div>
      <div>
        <Button
          disabled={save !== 'haveChanges'}
          onClick={patchForm}>
          {renderSave()}
        </Button>
        <Button disabled={save !== 'noChanges'} onClick={() => router.post(Routes.path('poe1.new.new', { id: build_data.id }))}>Submit build</Button>
      </div>
      <ItemsContext.Provider value={store}>
        <ItemListConfig />
      </ItemsContext.Provider>
    </Container>
  )
}

export default Preview;
