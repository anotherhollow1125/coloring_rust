import React from "react";
import { HighlightTarget } from "./types";
import { KeyboardSensor, PointerSensor, useSensor, useSensors, DragEndEvent, DndContext, closestCenter } from "@dnd-kit/core";
import { arrayMove, SortableContext, sortableKeyboardCoordinates, verticalListSortingStrategy } from "@dnd-kit/sortable";
import { List } from "@mui/material";
import FragmentItem from "./FragmentItem";

interface FragmentListProps {
  fragmentList: HighlightTarget[];
  setFragmentList: React.Dispatch<React.SetStateAction<HighlightTarget[]>>;
}

// ref: https://docs.dndkit.com/presets/sortable
export default function FragmentList({fragmentList, setFragmentList}: FragmentListProps) {
  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates
    }),
  );

  const handleDragEnd = ({active, over}: DragEndEvent) => {
    if (over !== null && active.id !== over.id) {
      setFragmentList((list) => {
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
        items={fragmentList.map(frag => frag.name)}
        strategy={verticalListSortingStrategy}
      >
        <List>
          {
            fragmentList.map(frag => {
              const setFragInfo = (info: HighlightTarget) => {
                const newList = fragmentList.map(item => {
                  if (item.name === frag.name) {
                    return { ...item, ...info };
                  }

                  return item;
                });

                setFragmentList(_list => { return newList; });
              };

              return (<FragmentItem
                key={frag.name}
                fragInfo={frag}
                setFragInfo={setFragInfo}
              />);
            })
          }
        </List>
      </SortableContext>
    </DndContext>
  );
}