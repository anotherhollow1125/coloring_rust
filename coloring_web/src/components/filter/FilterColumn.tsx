import { Button, Checkbox, Divider, ListItem, ListItemIcon, ListItemText } from "@mui/material";
import FilterList from "./FilterList";
import { useState } from "react";
import { SwapVert } from "@mui/icons-material";
import { Filter, initFilterArray } from "./types";

interface FilterColumnProps {
  filterList: Filter[];
  setFilterList: React.Dispatch<React.SetStateAction<Filter[]>>;
  hit_top_filter: string,
  hit_filters: string[],
}

export default function FilterColumn({
  filterList,
  setFilterList,
  hit_top_filter,
  hit_filters,
}: FilterColumnProps) {
  const [allActive, setAllActive] = useState(true);

  return (
    <>
      <ListItem
        sx={{ display: 'flex', alignItems: 'center', maxWidth: '400px' }}
      >
        <ListItemIcon>
          <Checkbox
            edge="start"
            checked={allActive}
            tabIndex={-1}
            disableRipple
            onClick={() => {
              setFilterList(list => {
                return list.map((item) => {
                  return { ...item, active: !allActive };
                });
              });

              setAllActive(!allActive);
            }}
          />
        </ListItemIcon>

        <ListItemText primary={"Whole Matcher"} />

        <SwapVert />
      </ListItem>
      <Divider orientation="horizontal" />
      <FilterList
        filterList={filterList}
        setFilterList={setFilterList}
        hitTopFilter={hit_top_filter}
        hitFilters={hit_filters}
      />
      <Button
        variant="outlined"
        onClick={() => {
          setFilterList(initFilterArray());
        }}
      >
        Reset Order
      </Button>
    </>
  );
}