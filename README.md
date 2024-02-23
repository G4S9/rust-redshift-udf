# Rust Redshift UDF

## TL;DR

This project has two parts:

1) a lambda function written in Rust that can be used as a UDF for geocoding addresses in Redshift as well as
2) a CDK app for deploying the lambda

## Why a Lambda UDF

Joining datasets with a "geolocation table" is generally not feasible for multiple reasons:

- size
- ambiguity
- etc.

## Why Rust

"Because it is fast, that's why" : )

[See excellent Re:Invent speach by Efi Merdler-Kravitz](https://www.youtube.com/watch?v=Mdh_2PXe9i8)

## Demo

https://github.com/G4S9/rust-redshift-udf/assets/96652361/335bcce0-c7ee-445f-8ea2-ada9d7cf9c4f
