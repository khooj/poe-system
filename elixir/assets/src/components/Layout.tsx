import { usePage } from '@inertiajs/react';
import { ReactNode } from 'react';
import { AppShell, Title } from '@mantine/core';

export type Props = {
  children: ReactNode,
};

export default function Layout({ children }: Props) {
  // FIXME: inertia page contexts for storybook
  // const { flash } = usePage().props;

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
          <Title order={2} mt={10} ml={10}>SomePoeTools</Title>
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
