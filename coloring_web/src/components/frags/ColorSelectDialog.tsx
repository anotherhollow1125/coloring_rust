import { Box, Checkbox, Dialog, FormControl, FormControlLabel, FormGroup, InputLabel, MenuItem, Select, SelectChangeEvent, Typography } from "@mui/material";
import { Color, namedColorNew, HighlightStyle } from "./types";

const namedColorArray: Color[] = [
  namedColorNew("lightcoral"),
  namedColorNew("orange"),
  namedColorNew("lime"),
  namedColorNew("lightgreen"),
  namedColorNew("green"),
  namedColorNew("aquamarine"),
  namedColorNew("pink"),
  namedColorNew("cyan"),
  namedColorNew("lightskyblue"),
  namedColorNew("violet"),
  namedColorNew("lightblue"),
  namedColorNew("darkcyan"),
  namedColorNew("white"),
  namedColorNew("darkblue"),
  {
    type_: "custom",
    hex: "#aa0000",
  },
];

interface ColorSelectDialogProps {
  open: boolean;
  handleClose: () => void;
  highlightStyle: HighlightStyle;
  setHighlightStyle: (style: HighlightStyle) => void;
}

export default function ColorSelectDialog({
  open,
  handleClose,
  highlightStyle,
  setHighlightStyle,
}: ColorSelectDialogProps) {
  const color = highlightStyle.color;
  const is_custom = color.type_ === "custom";
  const name = is_custom ? "custom" : color.name;

  const is_background = highlightStyle.is_background;

  const bgHandle = () => {
    setHighlightStyle({...highlightStyle, is_background: !is_background});
  };

  const selectHandle = (e: SelectChangeEvent) => {
    const name = e.target.value as string;

    if (name === "custom") {
      setHighlightStyle({
        ...highlightStyle,
        color: {
          type_: "custom",
          hex: "#aa0000",
        },
      });
    } else {
      setHighlightStyle({
        ...highlightStyle,
        color: {
          type_: "named",
          name,
        },
      });
    }
  };

  const pickerHandle = (e: React.ChangeEvent<HTMLInputElement>) => {
    const hex = e.target.value;

    setHighlightStyle({
      ...highlightStyle,
      color: {
        type_: "custom",
        hex,
      },
    });
  };

  return (
    <Dialog
      open={open}
      onClose={handleClose}
    >
      <Box
        sx={{
          padding: "20px",
          gap: "10px",
        }}
      >
        <Typography>
          Color Setting
        </Typography>
        <FormGroup>
          <FormControlLabel control={
            <Checkbox
              checked={is_background}
              tabIndex={-1}
              disableRipple
              onClick={bgHandle}
            />
          } label="bg" />
        </FormGroup>
        <FormControl variant="standard">
          <InputLabel>Color</InputLabel>
          <Select
            labelId="color-select-label"
            value={name}
            tabIndex={-1}
            onChange={selectHandle}
            renderValue={value => (
              <Box sx={{ color: color.type_ === "named" ? value : color.hex }}>
                {value}
              </Box>
            )}
          >
            {namedColorArray.map((c: Color) => {
              const name = c.type_ === "named" ? c.name : "custom";

              return <MenuItem value={name} key={name}>{name}</MenuItem>
            })}
          </Select>
        </FormControl>
        {is_custom ? <input
          type="color"
          value={color.hex}
          onChange={pickerHandle}
        />: <></>}
      </Box>
    </Dialog>
  );
}