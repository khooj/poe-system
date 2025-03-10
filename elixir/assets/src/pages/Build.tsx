import { Container } from 'react-bootstrap'
import { type BuildInfo } from '@bindings/domain/bindings/BuildInfo';

type Props = {
  data: BuildInfo,
}

const Build = ({ data }: Props) => {
  return (
    <Container className="d-flex flex-column align-items-center justify-content-center">
      {JSON.stringify(data.provided)}
    </Container>
  )
}

export default Build
