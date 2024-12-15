import { Button, Checkbox, Divider, ListItem, ListItemIcon, ListItemText } from "@mui/material";
import { HighlightTarget, initHighlightTargetArray } from "./types";
import { useState } from "react";
import { SwapVert } from "@mui/icons-material";
import FragmentList from "./FragmentList";

interface FragmentColumnProps {
  fragmentList: HighlightTarget[];
  setFragmentList: React.Dispatch<React.SetStateAction<HighlightTarget[]>>;
  isDarkMode: boolean;
}

export default function FragmentColumn({fragmentList, setFragmentList, isDarkMode}: FragmentColumnProps) {
  const [allTarget, setAllTarget] = useState(true);

  return (
    <>
      <ListItem
        sx={{ display: 'flex', alignItems: 'center', maxWidth: '400px' }}
      >
        <ListItemIcon>
          <Checkbox
            edge="start"
            checked={allTarget}
            tabIndex={-1}
            disableRipple
            onClick={() => {
              setFragmentList(list => {
                return list.map((item) => {
                  return { ...item, is_target: !allTarget };
                });
              });

              setAllTarget(!allTarget);
            }}
          />
        </ListItemIcon>

        <ListItemText primary={"Highlighting Priority"} />

        <SwapVert />
      </ListItem>
      <Divider orientation="horizontal" />
      <FragmentList
        fragmentList={fragmentList}
        setFragmentList={setFragmentList}
      />
      <Button
        variant="outlined"
        onClick={() => {
          setFragmentList(initHighlightTargetArray(isDarkMode));
          setAllTarget(true);
        }}
      >
        Reset All
      </Button>
    </>
  );
}