import { usePage } from '@inertiajs/react';
import { ReactNode } from 'react';
import { ActionIcon, Anchor, AppShell, Group, Title, useComputedColorScheme, useMantineColorScheme } from '@mantine/core';
import { TypedLink } from './TypedLink';
import { IconSun, IconMoon } from '@tabler/icons-react';
import cx from 'clsx';
import classes from './Layout.module.css';

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
          <Group mt={10} ml={10}>
            <Title order={2}>SomePoeTools</Title>
            <Anchor component={TypedLink} to='poe1.index'>Build calculator</Anchor>
            <ActionIcon
              onClick={() => setColorScheme(computedColorScheme === 'light' ? 'dark' : 'light')}
              variant='default'
              size='xl'
              aria-label='Toggle color scheme'
            >
              <IconSun className={cx(classes.icon, classes.light)} stroke={1.5} />
              <IconMoon className={cx(classes.icon, classes.dark)} stroke={1.5} />
            </ActionIcon>
          </Group>
        </AppShell.Header>
        <AppShell.Main>
          {children}
        </AppShell.Main>
        <AppShell.Footer withBorder={false}>
          <p>somepoetools.xyz isn't affiliated with or endorsed by Grinding Gear Games in any way.</p>
        </AppShell.Footer>
      </AppShell>
    </>
  )
}
