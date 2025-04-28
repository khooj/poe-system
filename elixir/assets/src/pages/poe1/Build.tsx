import { Container } from 'react-bootstrap'
import { type BuildInfo } from '@bindings/domain/bindings/BuildInfo';
import { BuildItemsWithConfig } from '@bindings/domain/bindings/BuildItemsWithConfig';
import { FoundBuildItems } from '@bindings/domain/bindings/FoundBuildItems';
import MultipleItems from '@/components/MultipleItems';
import RequiredItemComponent from '@/components/RequiredItemComponent';
import { ModConfig } from '@bindings/domain/bindings/ModConfig';
import { StoredItemComponent } from '@/components/StoredItemComponent';

type RenderConfigProps = {
  cf: ModConfig | null,
};

const RenderConfig = ({ cf }: RenderConfigProps) => {
  if (!cf) {
    return <></>
  }

  if (cf === 'Exist') {
    return <span>(exist)</span>
  } else if ('Exact' in cf) {
    return <span>(exact: {cf.Exact})</span>
  } else if ('Range' in cf) {
    return <span>(range: {cf.Range.start}-{cf.Range.end})</span>
  } else if ('Min' in cf) {
    return <span>(min: {cf.Min})</span>
  } else if ('Max' in cf) {
    return <span>(max: {cf.Max})</span>
  }
};

type Props = {
  data: BuildInfo,
}

const itemsOrder: (keyof (BuildItemsWithConfig & FoundBuildItems))[] = [
  'helmet', 'body', 'gloves', 'boots',
  'weapon1', 'weapon2', 'belt', 'amulet',
  'ring1', 'ring2', 'jewels', 'gems',
  'flasks'
];

const Build = ({ data }: Props) => {
  return (
    <Container fluid className="d-flex flex-fill flex-row justify-content-evenly border border-danger">
      <div className='d-flex flex-column vw-50 border'>
        <span>Provided</span>
        {itemsOrder.map(k => {
          if (Array.isArray(data.provided[k])) {
            return <MultipleItems itemKey={k}>
              {data.provided[k].map(i => <RequiredItemComponent
                item={i.item}
                modConfigComponent={(mcf) => <RenderConfig cf={mcf[1]} />}
              />)}
            </MultipleItems>
          } else {
            return <RequiredItemComponent
              item={data.provided[k].item}
              modConfigComponent={(mcf) => <RenderConfig cf={mcf[1]} />}
            />
          }
        })}
      </div>
      <div className='d-flex flex-column vw-50 border'>
        <span>Found</span>
        {itemsOrder.map(k => {
          if (Array.isArray(data.found[k])) {
            return <MultipleItems itemKey={k}>
              {data.found[k].map(i => <RequiredItemComponent
                item={i.item}
                modConfigComponent={(mcf) => <RenderConfig cf={mcf[1]} />}
              />)}
            </MultipleItems>
          } else {
            if (data.found[k]) {
              return <StoredItemComponent item={data.found[k]} />
            } else {
              return <div>Item ({k}) not found</div>
            }
          }
        })}
      </div>
    </Container>
  )
}

export default Build
