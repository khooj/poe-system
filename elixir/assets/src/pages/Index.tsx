import { Container, Flex } from "@mantine/core";

type Props = {
  build_ids: string[],
};

const Index = ({ build_ids }: Props) => {
  return (
    <Container className="d-flex flex-column align-items-center justify-content-center">
      <Flex align='center' justify='center' direction='column'>
        <div>Welcome to website with tools for Path of Exile videogames.<br />At this moment we don't have many tools. You can select them in top navigation menu.</div>
      </Flex>
    </Container>
  )
}

export default Index
