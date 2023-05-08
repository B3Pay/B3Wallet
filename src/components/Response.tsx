import { CanisterStatus } from "declarations/b3_user/b3_user.did"

interface ResponseProps {
  response?: CanisterStatus
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
    <section>
      <label>Loading...</label>
    </section>
  ) : response ? (
    <section>
      <label>Response: &nbsp;</label>
      {loading}
      <ul>
        {response &&
          Object.entries(response).map(([key, value]) => parent(key, value))}
      </ul>
    </section>
  ) : (
    <section>
      <label>No response</label>
    </section>
  )
}
