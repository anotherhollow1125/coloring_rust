import Head from "next/head";
import { Source_Code_Pro } from "next/font/google";
import styles from "@/styles/Home.module.css";
import InputField from "@/components/InputField";
import { Button, createTheme, ThemeProvider } from "@mui/material";
import { useState } from "react";
import OutputField from "@/components/OutputField";
import { colored } from "coloring_wasm";
import Grid from "@mui/material/Grid2";
import FilterList, { Filter } from "@/components/filter/FilterList";

const SourceCodePro = Source_Code_Pro({
  weight: "400",
});

const theme = createTheme({
  typography: {
    fontFamily: SourceCodePro.style.fontFamily,
  },
  colorSchemes: {
    dark: true,
  },
});

const filterNew= (name: string) => {
  return {
    name,
    active: true,
  };
}

const initFilterList = (): Filter[] => {
  return [
    filterNew("file"),
    filterNew("item"),
    filterNew("block"),
    filterNew("stmt"),
    filterNew("expr"),
    filterNew("type"),
    filterNew("path"),
    filterNew("visibility"),
    filterNew("ident"),
    filterNew("lifetime"),
    filterNew("lit"),
    filterNew("meta"),
  ];
};

export default function Home() {
  const [input, setInput] = useState("");
  const [filterList, setFilterList] = useState(initFilterList());

  const { hit_top_filter, hit_filters, colored: output } = colored({
    code: input,
    filters: filterList
      .flatMap(item => item.active ? [item.name] : []),
  });

  return (
    <ThemeProvider theme={theme}>
      <Head>
        <title>Rust フラグメント識別子判別器 Fragment Specifiers Highlighter</title>
        <meta name="description" content="Rustコードを解析してフラグメント識別子を判断。宣言マクロ作成に役立ちます。" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div
        className={`${styles.page} ${SourceCodePro.className}`}
      >
        <main className={styles.main}>
          <h1>Rust フラグメント識別子判別器</h1>
          <InputField input={input} setInput={setInput} />
          <>Whole Match: {hit_top_filter}</>
          <OutputField output={output} />
          <Grid container spacing={10}>
          <Grid size={4}>
            <FilterList
              filterList={filterList}
              setFilterList={setFilterList}
              hitTopFilter={hit_top_filter}
              hitFilters={hit_filters}
            />
            <Button
              variant="outlined"
              onClick={() => {
                setFilterList(initFilterList());
              }}
            >
              Reset Order
            </Button>
          </Grid>
          <Grid size={4}>
            preparing...
          </Grid>
        </Grid>
        </main>
        <footer className={styles.footer}>
          ©️ namnium 2024
        </footer>
      </div>
    </ThemeProvider>
  );
}
