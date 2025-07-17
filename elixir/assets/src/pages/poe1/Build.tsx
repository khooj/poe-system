import { BuildItemsWithConfig } from '@bindings/domain/bindings/BuildItemsWithConfig';
import { FoundBuildItems } from '@bindings/domain/bindings/FoundBuildItems';
import MultipleItems from '@/components/MultipleItems';
import Item from '@/components/Item';
import { ModConfig } from '@bindings/domain/bindings/ModConfig';
import { StoredItemComponent } from '@/components/StoredItemComponent';
import { Price } from '@bindings/domain/bindings/Price';
import useSSE from '../../utils/useSSE.ts';
import { router } from '@inertiajs/react';
import { useEffect } from 'react';
import { Container, Flex, Grid, Group, Loader } from '@mantine/core';
import Routes from '@routes.js';

export type RenderConfigProps = {
  cf: ModConfig | null,
};

export const RenderConfig = ({ cf }: RenderConfigProps) => {
  const content = (() => {
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
  })();

  return content;
};

export type Props = {
  id: string,
  provided: BuildItemsWithConfig,
  found: FoundBuildItems | null,
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

export const Build = ({ id, provided, found, processed }: Props) => {
  // TODO: render info that build processing still in progress
  // with spinner 
  const sub = useSSE(Routes.path('sse.subscribe'), {
    method: 'POST',
    body: `topics=build:${id}`,
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded'
    }
  });

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
        {provided[k].map(i => <Item
          item={i.item}
          modConfigComponent={(mcf) => <RenderConfig cf={mcf[1]} />}
        />)}
      </MultipleItems>
    } else {
      return <Item
        item={provided[k].item}
        modConfigComponent={(mcf) => <RenderConfig cf={mcf[1]} />}
      />
    }
  };
  const renderFound = (k: keyof FoundBuildItems) => {
    if (Array.isArray(found[k])) {
      if (found[k].length !== 0) {
        return <MultipleItems itemKey={k}>
          {found[k].map(i => <StoredItemComponent item={i} />)}
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

  const cost: { [x: string]: number } = Object.entries(found || {}).flatMap(it => it[1]).filter(it => !!it)
    .reduce((acc, prev) => {
      const { name, value } = priceCurrency(prev.price);
      acc[name] = (acc[name] ?? 0) + value;
      return acc;
    }, {});

  const costString = Object.entries(cost).reduce((acc, x) => acc + ` ${x[0]}: ${x[1]}`, "");

  return (
    <Container fluid>
      <Flex justify='space-evenly'>
        <Flex direction='column'>
          <Grid columns={2}>
            <Grid.Col span={1}><Group>Provided</Group></Grid.Col>
            <Grid.Col span={1}>
              <Group>{processed && `Found (with cost: ${costString})` || <Group>
                <Loader />
                <span>Processing, page should reload automatically</span>
              </Group>}
              </Group>
            </Grid.Col>
            {itemsOrder.map(k => <>
              <Grid.Col span={1}>{renderProvided(k)}</Grid.Col>
              <Grid.Col span={1}>{processed && renderFound(k)}</Grid.Col>
            </>)}
          </Grid>
        </Flex>
      </Flex>
    </Container>
  )
}

export default Build
