import { Container, Row } from 'react-bootstrap'
import { type BuildInfo } from '@bindings/domain/bindings/BuildInfo';
import { BuildItemsWithConfig } from '@bindings/domain/bindings/BuildItemsWithConfig';
import { FoundBuildItems } from '@bindings/domain/bindings/FoundBuildItems';
import MultipleItems from '@/components/MultipleItems';
import RequiredItemComponent from '@/components/RequiredItemComponent';
import { ModConfig } from '@bindings/domain/bindings/ModConfig';
import { StoredItemComponent } from '@/components/StoredItemComponent';
import { Col } from 'react-bootstrap';

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
  const renderProvided = (k: keyof BuildItemsWithConfig) => {
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
  };
  const renderFound = (k: keyof FoundBuildItems) => {
    if (Array.isArray(data.found[k])) {
      if (data.found[k].length !== 0) {
        return <MultipleItems itemKey={k}>
          {data.found[k].map(i => <RequiredItemComponent
            item={i.item}
            modConfigComponent={(mcf) => <RenderConfig cf={mcf[1]} />}
          />)}
        </MultipleItems>
      } else {
        return <div>Items ({k}) not found</div>
      }
    } else {
      if (data.found[k]) {
        return <StoredItemComponent item={data.found[k]} />
      } else {
        return <div>Item ({k}) not found</div>
      }
    }
  };

  return (
    <Container fluid className="d-flex flex-fill flex-row justify-content-evenly border main-color text-light">
      <div className='d-flex flex-column vw-50 border'>
        <Row>
          <Col className='d-flex justify-content-center'>Provided</Col>
          <Col className='d-flex justify-content-center'>Found</Col>
        </Row>
        {itemsOrder.map(k => <Row>
          <Col>{renderProvided(k)}</Col>
          <Col>{renderFound(k)}</Col>
        </Row>)}
      </div>
    </Container>
  )
}

export default Build
