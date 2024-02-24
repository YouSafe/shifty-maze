import type { Result as R } from "game-core/pkg";

export type NoneError = "NoneError";
export type Option<T> = Result<T, NoneError>;
export type SplitResult<T, E> = R<T, E>;

export function Ok<T, E = never>(value: T): Result<T, E> {
  return Result.ok(value);
}

export function Err<T, E>(error: E): Result<T, E> {
  return Result.err(error);
}

export function Some<T>(value: T): Option<T> {
  return Result.ok(value);
}

export function None<T>(): Option<T> {
  return Result.err("NoneError");
}

export function fromNullable<T>(value: T | null | undefined): Option<T> {
  if (value === null || value === undefined) {
    return None();
  }
  return Some(value);
}

export class Result<T, E> {
  constructor(private _value: T | E, private _isOk: boolean) { }
  static ok<T, E = never>(value: T): Result<T, E> {
    return new Result<T, E>(value, true);
  }
  static err<T, E>(error: E): Result<T, E> {
    return new Result<T, E>(error, false);
  }

  static fromSplitResult<T, E>(split: SplitResult<T, E>): Result<T, E> {
    if (split.type === "Ok") {
      return Result.ok(split.value);
    }

    return Result.err(split.value);
  }

  isErr(): boolean {
    return !this._isOk;
  }
  isOk(): boolean {
    return this._isOk;
  }

  split(): SplitResult<T, E> {
    if (this.isOk()) {
      return { type: "Ok", value: this._value as T };
    }
    return { type: "Err", value: this._value as E };
  }

  unwrap(): T {
    if (this.isErr()) {
      throw new Error("Called unwrap on an error value");
    }
    return this._value as T;
  }

  unwrapErr(): E {
    if (this.isOk()) {
      throw new Error("Called unwrapErr on an ok value");
    }
    return this._value as E;
  }

  map<U>(fn: (value: T) => U): Result<U, E> {
    if (this.isErr()) {
      return Result.err(this._value as E);
    }
    return Result.ok(fn(this._value as T));
  }

  mapErr<U>(fn: (error: E) => U): Result<T, U> {
    if (this.isOk()) {
      return Result.ok(this._value as T);
    }
    return Result.err(fn(this._value as E));
  }

  and<U>(result: Result<U, E>): Result<U, E> {
    if (this.isOk()) {
      return result;
    }
    return Result.err(this._value as E);
  }

  andThen<U>(fn: (value: T) => Result<U, E>): Result<U, E> {
    if (this.isOk()) {
      return fn(this._value as T);
    }
    return Result.err(this._value as E);
  }

  unwrapOr<U>(value: U): T | U {
    if (this.isOk()) {
      return this._value as T;
    }
    return value;
  }

  static all<T, E>(results: Result<T, E>[]): Result<T[], E[]> {
    let errors: E[] = [];
    let values: T[] = [];
    for (let result of results) {
      if (result.isOk()) {
        values.push(result.unwrap());
      } else {
        errors.push(result.unwrapErr());
      }
    }
    if (errors.length > 0) {
      return Result.err(errors);
    }
    return Result.ok(values);
  }
}
