import { Container } from 'react-bootstrap'
import { type BuildInfo } from '@bindings/domain/bindings/BuildInfo';
import { ItemList } from '@/components/ItemList';

type Props = {
  data: BuildInfo,
}

const Build = ({ data }: Props) => {
  return (
    <Container fluid className="d-flex flex-fill flex-row justify-content-evenly border border-danger">
      <div className='d-flex flex-column vw-50 border'>
        <span>Provided</span>
        <ItemList {...data.provided} />
      </div>
      <div className='d-flex flex-column vw-50 border'>
        <span>Found</span>
        <ItemList {...data.found} />
      </div>
    </Container>
  )
}

export default Build
