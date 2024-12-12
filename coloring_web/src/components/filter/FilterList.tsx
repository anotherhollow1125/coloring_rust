import { closestCenter, DndContext, DragEndEvent, KeyboardSensor, PointerSensor, useSensor, useSensors } from "@dnd-kit/core";
import FilterItem from "./FilterItem";
import { arrayMove, SortableContext, sortableKeyboardCoordinates, verticalListSortingStrategy } from "@dnd-kit/sortable";
import { useCallback } from "react";
import { List } from "@mui/material";

export interface Filter {
  name: string,
  active: boolean,
}

interface FilterListProps {
  filterList: Filter[];
  setFilterList: React.Dispatch<React.SetStateAction<Filter[]>>;
  hitTopFilter: string,
  hitFilters: string[],
}

// ref: https://docs.dndkit.com/presets/sortable
export default function FilterList({filterList, setFilterList, hitTopFilter, hitFilters}: FilterListProps) {
  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    }),
  );

  const handleDragEnd = useCallback(({active, over}: DragEndEvent) => {
    if (over !== null && active.id !== over.id) {
      setFilterList((list) => {
        const oldIndex = list.findIndex((item) => item.name === active.id);
        const newIndex = list.findIndex((item) => item.name === over.id);

        return arrayMove(list, oldIndex, newIndex);
      })
    }
  }, [filterList]);

  return (
    <List>
      <DndContext
        sensors={sensors}
        collisionDetection={closestCenter}
        onDragEnd={handleDragEnd}
      >
        <SortableContext
          items={filterList.map((filter) => filter.name)}
          strategy={verticalListSortingStrategy}
        >
          {
            filterList.map(filter => {
              const setActive = (active: boolean) => {
                const newList = filterList.map((item) => {
                  if (item.name === filter.name) {
                    return { ...item, active };
                  }

                  return item;
                });

                setFilterList((_list) => { return newList; });
              };

              return (<FilterItem
                filterName={filter.name}
                active={filter.active}
                setActive={setActive}
                hit={hit(hitTopFilter, hitFilters, filter.name)}
              />);
            })
          }
        </SortableContext>
      </DndContext>
    </List>
  );
}

const hit = (hitTopFilter: string, hitFilters: string[], filterName: string) => {
  if (hitTopFilter === filterName) {
    return 'top';
  }

  if (hitFilters.includes(filterName)) {
    return 'yes';
  }

  return 'no';
};
