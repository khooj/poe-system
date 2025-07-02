import { usePage } from '@inertiajs/react';
import { ReactNode } from 'react';
import { ActionIcon, Anchor, AppShell, Group, Text, ThemeIcon, Title, useComputedColorScheme, useMantineColorScheme } from '@mantine/core';
import { TypedLink } from './TypedLink';
import cx from 'clsx';
import classes from './Layout.module.css';
import sun from '@icons/sun.svg';
import moon from '@icons/moon.svg';

export type Props = {
  children: ReactNode,
};

export default function Layout({ children }: Props) {
  // FIXME: inertia page contexts for storybook
  // const { flash } = usePage().props;

  const { setColorScheme } = useMantineColorScheme();
  const computedColorScheme = useComputedColorScheme('light', { getInitialValueInEffect: true });

  return (
    <>
      <AppShell
        padding="md"
        header={{
          height: 60
        }}
        footer={{
          height: 60
        }}
      >
        <AppShell.Header>
          <Group mt={10} ml={10} justify='space-between'>
            <Group>
              <Title order={2}>SomePoeTools</Title>
              <Title order={3}>Path of Exile 1</Title>
              <Anchor component={TypedLink} to='poe1.build-calc.index'>Build calculator</Anchor>
            </Group>
            <Group>
              <Title order={4}>Change mode</Title>
              <ActionIcon
                onClick={() => setColorScheme(computedColorScheme === 'light' ? 'dark' : 'light')}
                variant='default'
                size='md'
                aria-label='Toggle color scheme'
              >
                <img src={sun} className={cx(classes.icon, classes.light)} />
                <img src={moon} className={cx(classes.icon, classes.dark)} />
              </ActionIcon>
            </Group>
          </Group>
        </AppShell.Header>
        <AppShell.Main>
          {children}
        </AppShell.Main>
        <AppShell.Footer withBorder={false}>
          <Text pl='xs'>somepoetools.xyz isn't affiliated with or endorsed by Grinding Gear Games in any way.</Text>
        </AppShell.Footer>
      </AppShell>
    </>
  )
}
