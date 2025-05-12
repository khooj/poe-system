import { Container, Row } from 'react-bootstrap'
import { type BuildInfo } from '@bindings/domain/bindings/BuildInfo';
import { BuildItemsWithConfig } from '@bindings/domain/bindings/BuildItemsWithConfig';
import { FoundBuildItems } from '@bindings/domain/bindings/FoundBuildItems';
import MultipleItems from '@/components/MultipleItems';
import RequiredItemComponent from '@/components/RequiredItemComponent';
import { ModConfig } from '@bindings/domain/bindings/ModConfig';
import { StoredItemComponent } from '@/components/StoredItemComponent';
import { Col } from 'react-bootstrap';
import { Price } from '@bindings/domain/bindings/Price';

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
  processed: boolean,
}

const itemsOrder: (keyof (BuildItemsWithConfig & FoundBuildItems))[] = [
  'helmet', 'body', 'gloves', 'boots',
  'weapon1', 'weapon2', 'belt', 'amulet',
  'ring1', 'ring2', 'jewels', 'gems',
  'flasks'
];

const priceCurrency = (x: Price) => {
  if ("Chaos" in x) {
    return { name: "chaos", value: x.Chaos };
  } else if ("Divine" in x) {
    return { name: "divine", value: x.Divine };
  } else {
    return { name: x.Custom[0], value: x.Custom[1] };
  }
};

const Build = ({ data, processed }: Props) => {
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

  const cost: { [x: string]: number } = Object.entries(data.found).flatMap(it => it[1]).filter(it => !!it)
    .reduce((acc, prev) => {
      const { name, value } = priceCurrency(prev.price);
      acc[name] = (acc[name] ?? 0) + value;
      return acc;
    }, {});

  const costString = Object.entries(cost).reduce((acc, x) => acc + ` ${x[0]}: ${x[1]}`, "");

  return (
    <Container fluid className="d-flex flex-fill flex-row justify-content-evenly border main-color text-light">
      <div className='d-flex flex-column vw-50 border'>
        <Row>
          <Col className='d-flex justify-content-center'>Provided</Col>
          <Col className='d-flex justify-content-center'>{processed && `Found (with cost: ${costString})` || "Build not processed, please return later"}</Col>
        </Row>
        {itemsOrder.map(k => <Row>
          <Col>{renderProvided(k)}</Col>
          <Col>{processed && renderFound(k)}</Col>
        </Row>)}
      </div>
    </Container>
  )
}

export default Build
