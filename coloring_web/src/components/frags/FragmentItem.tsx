import { useSortable } from "@dnd-kit/sortable";
import { HighlightTarget } from "./types";
import { Box, Checkbox, IconButton, ListItem, ListItemIcon, ListItemText } from "@mui/material";
import { ColorLens, DragIndicator } from "@mui/icons-material";
import ColorSelectDialog from "./ColorSelectDialog";
import { useState } from "react";

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
  const [colorSelectOpen, setColorSelectOpen] = useState(false);

  return (<ListItem
    sx={{
      display: 'flex',
      alignItems: 'center',
      maxWidth: '400px',
      gap: '16px',
    }}
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

    <IconButton onClick={() => setColorSelectOpen(true)}>
      <ColorLens />
    </IconButton>

    <ColorSelectDialog
      open={colorSelectOpen}
      handleClose={() => setColorSelectOpen(false)}
      highlightStyle={fragInfo.style}
      setHighlightStyle={(style) => { setFragInfo({...fragInfo, style}); }}
    />

    <Box
      sx={{
        ml: 'auto',
        touchAction: 'none'
      }}
      {...listeners}
    >
      <DragIndicator />
    </Box>
  </ListItem>);
}