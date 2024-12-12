import { Box, Checkbox, IconButton, ListItem, ListItemIcon, ListItemText } from "@mui/material";
import { Check, CheckCircle, DragIndicator, Filter } from "@mui/icons-material";
import { useSortable } from "@dnd-kit/sortable";

export type FilterHit = 'yes' | 'no' | 'top';

interface FilterItemProps {
  filterName: string;
  active: boolean;
  setActive: (active: boolean) => void;
  hit: FilterHit;
}

export default function FilterItem(
  { filterName, active, setActive, hit }: FilterItemProps
) {
  const {attributes, listeners, setNodeRef, transform} = useSortable({
    id: filterName,
  });
  const style = transform ? {
    transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
  } : undefined;

  return (<ListItem
    sx={{ display: 'flex', alignItems: 'center' }}
    key={filterName}
    id={filterName}
    ref={setNodeRef}
    style={style}
    {...listeners}
    {...attributes}
  >
    <ListItemIcon>
      <Checkbox
        edge="start"
        checked={active}
        tabIndex={-1}
        disableRipple
        onClick={() => setActive(!active)}
      />
    </ListItemIcon>

    <ListItemText primary={filterName} />

    <Box sx={{ ml: 'auto' }}>
      {icon(hit)}
    </Box>
  </ListItem>);
}

const icon = (hit: FilterHit) => {
  if (hit == 'top') {
    return <CheckCircle />;
  }

  if (hit == 'yes') {
    return <Check />;
  }

  if (hit == 'no') {
    return (<IconButton edge="end">
      <DragIndicator />
    </IconButton>);
  }
};