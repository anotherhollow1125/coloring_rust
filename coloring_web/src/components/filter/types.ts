export interface Filter {
  name: string,
  active: boolean,
}

const filterNew= (name: string) => {
  return {
    name,
    active: true,
  };
};

export const initFilterArray = (): Filter[] => {
  return [
    filterNew("file"),
    filterNew("item"),
    filterNew("block"),
    filterNew("stmt"),
    filterNew("expr"),
    filterNew("ty"),
    filterNew("path"),
    filterNew("vis"),
    filterNew("ident"),
    filterNew("lifetime"),
    filterNew("literal"),
    filterNew("meta"),
  ];
};