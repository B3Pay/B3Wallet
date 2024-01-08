import { ReleaseView } from "@src/declarations/b3system/b3system.did"
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableFooter,
  TableHead,
  TableHeader,
  TableRow
} from "@src/components/ui/table"
import { cn } from "@src/lib/utils"
import {
  sizeToMbWithUnit,
  truncateString,
  nanoTimeToDate
} from "@src/lib/converter"
import { Checkbox } from "./ui/checkbox"

interface ReleaseTableProps {
  releases: ReleaseView[]
  selectedRelease: ReleaseView
  selectHandler: (release: ReleaseView) => void
}

const ReleaseTable: React.FC<ReleaseTableProps> = ({
  releases,
  selectedRelease,
  selectHandler
}) => {
  const handleRowClick = (release: ReleaseView) => {
    selectHandler(release)
  }

  return (
    <Table>
      <TableCaption>
        A list of recent releases. Click a row to select.
      </TableCaption>
      <TableHeader>
        <TableRow>
          <TableHead>
            <Checkbox />
          </TableHead>
          <TableHead>Version</TableHead>
          <TableHead>Size</TableHead>
          <TableHead>Hash</TableHead>
          <TableHead>Date</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {releases.map(release => (
          <TableRow
            key={release.wasm_hash}
            className={cn(release.deprecated ? "opacity-50" : "")}
            onClick={() => handleRowClick(release)}
          >
            <TableCell>
              <Checkbox
                name="version"
                value={release.version}
                checked={selectedRelease.wasm_hash === release.wasm_hash}
                disabled={release.deprecated}
                onChange={() => handleRowClick(release)}
              />
            </TableCell>
            <TableCell>{release.version}</TableCell>
            <TableCell>{sizeToMbWithUnit(release.size)}MB</TableCell>
            <TableCell>{truncateString(release.wasm_hash)}</TableCell>
            <TableCell>
              {nanoTimeToDate(release.date).toLocaleString()}
            </TableCell>
          </TableRow>
        ))}
      </TableBody>
      <TableFooter>
        <TableRow>
          <TableCell colSpan={5}>Total Releases</TableCell>
          <TableCell>{releases.length}</TableCell>
        </TableRow>
      </TableFooter>
    </Table>
  )
}

export default ReleaseTable
