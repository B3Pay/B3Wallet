interface ResponseProps {
  response?: any
  loading?: string
}

const parent = (key: string, value: any) =>
  value && typeof value === "object" ? (
    <li key={key}>
      <label>{key}: &nbsp;</label>
      {child(value)}
    </li>
  ) : (
    <li key={key}>
      <label>{key}: &nbsp;</label>
      {value?.toString()}
    </li>
  )

const child = (value: any) =>
  value &&
  (value._isPrincipal ? (
    value.toText()
  ) : typeof value === "object" ? (
    Array.isArray(value) || typeof value[0] === "number" ? (
      value.toString()
    ) : (
      <ul>{Object.entries(value).map(([key, value]) => parent(key, value))}</ul>
    )
  ) : (
    value.toString()
  ))

export const Response: React.FC<ResponseProps> = ({ response, loading }) => {
  return loading ? (
    <label>Loading...</label>
  ) : response ? (
    <div
      style={{
        width: "100%",
        overflow: "hidden"
      }}
    >
      <label>Status: &nbsp;</label>
      {loading}
      <ul>
        {response &&
          Object.entries(response).map(([key, value]) => parent(key, value))}
      </ul>
    </div>
  ) : (
    <section>
      <label>No response</label>
    </section>
  )
}
