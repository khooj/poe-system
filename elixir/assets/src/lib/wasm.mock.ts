import { fn } from "storybook/test";
import * as actual from 'wasm';

export const get_pob_itemsets = fn(actual.get_pob_itemsets).mockName('get_pob_itemsets');
