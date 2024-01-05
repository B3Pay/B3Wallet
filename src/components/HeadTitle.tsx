import Head from "next/head"

interface HeadTitleProps {
  title: string
}

const HeadTitle: React.FC<HeadTitleProps> = ({ title }) => {
  return (
    <Head>
      <title>{title}</title>
    </Head>
  )
}

export default HeadTitle
