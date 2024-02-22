import { type Ref, type ShallowRef, customRef } from "vue";
import { useUrlSearchParams } from "@vueuse/core";

const params = useUrlSearchParams("history");

type UrlRefValue =
  | {
      type: "boolean";
      value: ShallowRef<boolean>;
    }
  | {
      type: "string";
      value: ShallowRef<string>;
    }
  | {
      type: "number";
      value: ShallowRef<number>;
    };

function urlRef(name: string, defaultValue: boolean): Ref<boolean>;
function urlRef(name: string, defaultValue: string): Ref<string>;
function urlRef(name: string, defaultValue: number): Ref<number>;
function urlRef<T extends UrlRefValue["type"]>(
  name: string,
  defaultValue: T
): Ref<T> {
  const type = typeof defaultValue as UrlRefValue["type"];

  return customRef<T>((track, trigger) => ({
    get() {
      track();
      const valueAsString: string | string[] | undefined = params[name];
      const parsedValue = parseValue(
        type,
        valueAsString !== undefined && Array.isArray(valueAsString)
          ? valueAsString.at(0)
          : valueAsString
      );
      if (parsedValue === undefined) {
        return defaultValue;
      }
      return parsedValue as T;
    },
    set(newValue) {
      params[name] = newValue + ""; // Serializing code, maybe this is good enough, not sure
      trigger();
    },
  }));
}

function parseValue(
  type: UrlRefValue["type"],
  value: string | undefined
): string | number | boolean | undefined {
  if (type === "string") {
    if (value !== undefined) {
      return value;
    }
  } else if (type === "boolean") {
    if (value === "1" || value === "true") {
      return true;
    } else if (value === "0" || value === "false") {
      return false;
    }
  } else if (type === "number") {
    if (value !== undefined && /^[0-9]+$/.test(value)) {
      return Number.parseFloat(value);
    }
  }

  return undefined;
}

export { urlRef };
