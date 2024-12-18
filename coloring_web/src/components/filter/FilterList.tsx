import { closestCenter, DndContext, DragEndEvent, KeyboardSensor, PointerSensor, useSensor, useSensors } from "@dnd-kit/core";
import FilterItem from "./FilterItem";
import { arrayMove, SortableContext, sortableKeyboardCoordinates, verticalListSortingStrategy } from "@dnd-kit/sortable";
import { List } from "@mui/material";
import { Filter } from "./types";

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

  const handleDragEnd = ({active, over}: DragEndEvent) => {
    if (over !== null && active.id !== over.id) {
      setFilterList((list) => {
        const oldIndex = list.findIndex((item) => item.name === active.id);
        const newIndex = list.findIndex((item) => item.name === over.id);

        return arrayMove(list, oldIndex, newIndex);
      })
    }
  };

  return (
      <DndContext
        sensors={sensors}
        collisionDetection={closestCenter}
        onDragEnd={handleDragEnd}
      >
        <SortableContext
          items={filterList.map(filter => filter.name)}
          strategy={verticalListSortingStrategy}
        >
          <List>
          {
            filterList.map(filter => {
              const setActive = (active: boolean) => {
                const newList = filterList.map(item => {
                  if (item.name === filter.name) {
                    return { ...item, active };
                  }

                  return item;
                });

                setFilterList(_list => { return newList; });
              };

              return (<FilterItem
                key={filter.name}
                filterName={filter.name}
                active={filter.active}
                setActive={setActive}
                hit={hit(hitTopFilter, hitFilters, filter.name)}
              />);
            })
          }
          </List>
        </SortableContext>
      </DndContext>
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
