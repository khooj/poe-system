import { BuildInfo } from '@bindings/domain/bindings/BuildInfo';
import { Container, Spinner } from 'react-bootstrap';
import { ItemListConfig } from '@/components/ItemListConfig';
import { useForm } from '@inertiajs/react';
import Routes from '../../routes.js';
import { BuildItemsWithConfig } from '@bindings/domain/bindings/BuildItemsWithConfig.js';
import { ItemWithConfig } from '@bindings/domain/bindings/ItemWithConfig.js';
import * as _ from 'lodash';

type BuildPreviewData = {
  id: string,
  itemset: string,
  skillset: string,
  pob: string,
  data: BuildInfo,
};

type Props = {
  buildData: BuildPreviewData
};

type InertiaFormType = {
  config: BuildInfo | null,
  id: string
};

const Preview = ({ buildData }: Props) => {
  const { patch, setData, errors, processing, data, isDirty } = useForm({
    config: buildData.data,
    id: buildData.id,
  } as InertiaFormType);
  const debouncedPatch = _.debounce(patch, 1000, { trailing: true });

  const setItemCb = (k: keyof BuildItemsWithConfig, it: ItemWithConfig | ItemWithConfig[]) => {
    const d = { ...data.config! };
    setData('config', { ...d, provided: { ...data.config!.provided, [k]: it } });
    debouncedPatch(Routes.path('poe1.preview.update_preview'));
  };

  return (
    <Container fluid className='d-flex flex-column'>
      <span>itemset: {buildData.itemset} skillset: {buildData.skillset}{
        processing && <><Spinner animation='border' role='status'></Spinner><p>Saving...</p></>
      }{errors.config && <p>Error occured: {errors.config}</p>}{isDirty && <p>Changes not saved</p>}
      </span>
      <ItemListConfig data={buildData.data} setItemCb={setItemCb} />
    </Container>
  )
}

export default Preview
