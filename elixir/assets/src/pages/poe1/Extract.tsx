import { BuildInfo } from '@bindings/domain/bindings/BuildInfo';
import { Container } from 'react-bootstrap';
import { ItemListConfig } from '@/components/ItemListConfig';

type Props = {
  buildInfo: BuildInfo
};

const Extract = ({ buildInfo }: Props) => {
  return (
    <Container fluid className='d-flex flex-column'>
      <ItemListConfig {...buildInfo.provided} />
    </Container>
  )
}

export default Extract
