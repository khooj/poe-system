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
import useSSE from '../../utils/useSSE.ts';
import { router } from '@inertiajs/react';
import { useEffect, useState } from 'react';

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
  id: string,
  provided: BuildItemsWithConfig,
  found: FoundBuildItems,
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

const Build = ({ id, provided, found, processed }: Props) => {
  const sub = useSSE('/poe1/sse', {
    method: 'POST',
    body: `topics=build:${id}`,
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded'
    }
  });
  const [changeProcessed, setChangeProcessed] = useState(processed);

  useEffect(() => {
    if (!processed) {
      return sub.subscribe(msg => {
        console.log('msg received', msg);
        if (msg === "done") {
          router.reload({ only: ['found', 'processed'] });
        }
      });
    }
  }, [processed]);

  const renderProvided = (k: keyof BuildItemsWithConfig) => {
    if (Array.isArray(provided[k])) {
      return <MultipleItems itemKey={k}>
        {provided[k].map(i => <RequiredItemComponent
          item={i.item}
          modConfigComponent={(mcf) => <RenderConfig cf={mcf[1]} />}
        />)}
      </MultipleItems>
    } else {
      return <RequiredItemComponent
        item={provided[k].item}
        modConfigComponent={(mcf) => <RenderConfig cf={mcf[1]} />}
      />
    }
  };
  const renderFound = (k: keyof FoundBuildItems) => {
    if (Array.isArray(found[k])) {
      if (found[k].length !== 0) {
        return <MultipleItems itemKey={k}>
          {found[k].map(i => <RequiredItemComponent
            item={i.item}
            modConfigComponent={(mcf) => <RenderConfig cf={mcf[1]} />}
          />)}
        </MultipleItems>
      } else {
        return <div>Items ({k}) not found</div>
      }
    } else {
      if (found[k]) {
        return <StoredItemComponent item={found[k]} />
      } else {
        return <div>Item ({k}) not found</div>
      }
    }
  };

  const cost: { [x: string]: number } = Object.entries(found).flatMap(it => it[1]).filter(it => !!it)
    .reduce((acc, prev) => {
      const { name, value } = priceCurrency(prev.price);
      acc[name] = (acc[name] ?? 0) + value;
      return acc;
    }, {});

  const costString = Object.entries(cost).reduce((acc, x) => acc + ` ${x[0]}: ${x[1]}`, "");

  return (
    <Container fluid className="d-flex flex-fill flex-row justify-content-evenly">
      <div className='d-flex flex-column vw-50'>
        <Row>
          <Col className='d-flex justify-content-center'>Provided</Col>
          <Col className='d-flex justify-content-center'>{processed && `Found (with cost: ${costString})` || "Build not processed, please return later"}</Col>
        </Row>
        {itemsOrder.map(k => <Row className='border'>
          <Col>{renderProvided(k)}</Col>
          <Col>{processed && renderFound(k)}</Col>
        </Row>)}
      </div>
    </Container>
  )
}

export default Build
