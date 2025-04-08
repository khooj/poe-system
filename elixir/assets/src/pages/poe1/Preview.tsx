import { BuildInfo } from '@bindings/domain/bindings/BuildInfo';
import { Button, Container, Spinner } from 'react-bootstrap';
import { ItemListConfig } from '@/components/ItemListConfig';
import { useForm, router } from '@inertiajs/react';
import Routes from '../../routes.js';
import { BuildItemsWithConfig } from '@bindings/domain/bindings/BuildItemsWithConfig.js';
import { ItemWithConfig } from '@bindings/domain/bindings/ItemWithConfig.js';
// eslint-disable-next-line @typescript-eslint/no-unused-vars
import * as _ from 'lodash';
import { useCallback, useEffect, useState } from 'react';
import { ToggleButton } from 'react-bootstrap';

type BuildPreviewData = {
  id: string,
  itemset: string,
  skillset: string,
  pob: string,
  data: BuildInfo,
};

type Props = {
  build_data: BuildPreviewData
};

type InertiaFormType = {
  config: BuildInfo,
  id: string
};

const Preview = ({ build_data }: Props) => {
  // isDirty probably does not work because of using shallow equal
  const { patch, setData, errors, processing, data } = useForm({
    config: build_data.data,
    id: build_data.id,
  } as InertiaFormType);

  const [autosave, setAutosave] = useState(true);
  const [save, setSave] = useState<'haveChanges' | 'saving' | 'noChanges'>('noChanges');
  const [isDirty, setIsDirty] = useState(false);

  useEffect(() => {
    if (isDirty) {
      setSave('haveChanges');
    } else if (processing) {
      setSave('saving');
    } else if (!isDirty) {
      setSave('noChanges');
    }
  }, [isDirty, processing]);

  const renderSave = useCallback(() => {
    switch (save) {
      case 'noChanges': return <>Up to date</>
      case 'saving': return <>Saving<Spinner animation="border" size="sm" role="status"></Spinner></>
      case 'haveChanges': return <>Save</>
    }
  }, [save]);

  const patchForm = useCallback(() => {
    patch(Routes.path('poe1.preview.update_preview'), {
      onSuccess: () => setIsDirty(false),
    });
  }, [patch, setIsDirty]);

  const setItemCb = useCallback((k: keyof BuildItemsWithConfig, it: ItemWithConfig | ItemWithConfig[]) => {
    console.log('setitemcb');
    const d = { ...data.config };
    setData('config', { found: d.found, provided: { ...data.config.provided, [k]: it } });
    setIsDirty(true);
    if (autosave) {
      patchForm();
    }
  }, [autosave, setData, data, patchForm, setIsDirty]);

  // const { errors: submitErrors, isLoading: submitIsLoading } = useSWR(
  //   [Routes.path('poe1.new.new'), buildData.id], 
  //   (u, i) => axios)

  return (
    <Container fluid className='d-flex flex-column'>
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

      <ItemListConfig data={data.config!} setItemCb={setItemCb} />
    </Container>
  )
}

export default Preview
