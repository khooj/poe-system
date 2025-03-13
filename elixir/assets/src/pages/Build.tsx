import { Container } from 'react-bootstrap'
import { type BuildInfo } from '@bindings/domain/bindings/BuildInfo';
import { ItemList } from '@/components/ItemList';

type Props = {
  data: BuildInfo,
}

const Build = ({ data }: Props) => {
  return (
    <Container className="d-flex flex-row align-items-center justify-content-center">
      <div className='d-flex flex-column'>
        <span>Provided</span>
        <ItemList {...data.provided} />
      </div>
      <div className='d-flex flex-column'>
        <span>Found</span>
        <ItemList {...data.found} />
      </div>
    </Container>
  )
}

export default Build
