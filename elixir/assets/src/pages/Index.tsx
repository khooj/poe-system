import { Head } from '@inertiajs/react'

type Props = {
  text: string,
}

const Index = ({ text }: Props) => {
  return (
    <div>
      <Head title='hi vite+inertia+react' />
      <p>{text}</p>
    </div>
  )
}

export default Index
