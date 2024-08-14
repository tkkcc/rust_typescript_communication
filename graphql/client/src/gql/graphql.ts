/* eslint-disable */
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
};

export type Query = {
  __typename?: 'Query';
  /** Adds two `a` and `b` numbers. */
  add: User;
  /** Adds two `a` and `b` numbers. */
  add2: User;
};


export type QueryAddArgs = {
  a: Scalars['Int']['input'];
  b: Scalars['Int']['input'];
};


export type QueryAdd2Args = {
  a: Scalars['Int']['input'];
  b: Scalars['Int']['input'];
};

export type Subscription = {
  __typename?: 'Subscription';
  /** Counts seconds. */
  count: User;
};

export type User = {
  __typename?: 'User';
  name: Scalars['String']['output'];
  related: Array<User>;
};
