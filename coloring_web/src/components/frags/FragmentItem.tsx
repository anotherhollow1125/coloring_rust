import { useSortable } from "@dnd-kit/sortable";
import { HighlightTarget } from "./types";
import { Box, Checkbox, ListItem, ListItemIcon, ListItemText } from "@mui/material";
import { DragIndicator } from "@mui/icons-material";

interface FragmentItemProps {
  fragInfo: HighlightTarget;
  setFragInfo: (info: HighlightTarget) => void;
}

export default function FragmentItem(
  {
    fragInfo,
    setFragInfo,
  }: FragmentItemProps
) {
  const {attributes, listeners, setNodeRef, transform} = useSortable({
    id: fragInfo.name,
  });
  const style = transform ? {
    transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
  } : undefined;

  return (<ListItem
    sx={{ display: 'flex', alignItems: 'center', maxWidth: '600px' }}
    id={fragInfo.name}
    ref={setNodeRef}
    style={style}
    {...attributes}
  >
    <ListItemIcon>
      <Checkbox
        edge="start"
        checked={fragInfo.is_target}
        tabIndex={-1}
        disableRipple
        onClick={() => setFragInfo({...fragInfo, is_target: !fragInfo.is_target})}
      />
    </ListItemIcon>

    <ListItemText primary={fragInfo.name} className={fragInfo.name}/>

    <Box
      sx={{ ml: 'auto' }}
      {...listeners}
    >
      <DragIndicator />
    </Box>
  </ListItem>);
}