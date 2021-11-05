# Compiler for LANGAUGE in Rust
Team Members: Abhay Patel & Mohammad Ghaemi

## Introduction
---

A compiler is a program that maps some source code from one programming language to another programming language (this is called the target language). Usually, the target language is a lower level programming language. The intent behind this is to use the target language to build a higher level language with more features. We will be borrowing many ideas from CMSC430, an introductory class for implementing compilers at the University of Maryland.<sup>1

This process will involve a few steps. We will first need to create an Abstract Syntax Tree (AST), which is a data structure allowing for representations of expressions in the target language which can be fed through either an interpreter (which would evaluate the parsed code in Rust) or through a compiler (which would convert the target language into another programming language).<sup>1

In order to parse the target language, there are a few steps involved. First, a lexer must be created which can tokenize the target code. This token stream would then be passed through the parser, where recursive descent parsing (or a variety of parsing strategies) would allow for this token stream to be converted into the defined AST.<sup>2</sup> There are many different types of parsers, including recursive descent, non-recursive descent, LR parsers, and operator precedence parsers.<sup>3

## Goals
---

### **75%**
- Define our target language to be a subset of ??? which includes the following features:
- Create a tokenizer and a parser which can accept code of our target language.
- Create a compiler which can convert our target language into ???


### **100%**
- Define our target language to be a subset of ??? which includes the following features:
- Create a tokenizer and a parser which can accept code of our target language.
- Create a compiler which can convert our target language into ???


### **125%**
- Define our target language to be a subset of ??? which includes the following features:
- Create a tokenizer and a parser which can accept code of our target language.
- Create a compiler which can convert our target language into ???


## Specific Aims & Objectives
---

## Cited References
---

1. https://www.cs.umd.edu/class/fall2021/cmsc430/
2. https://www.cs.umd.edu/class/fall2020/cmsc330/lectures/20-parsing.pdf
3. https://www.geeksforgeeks.org/types-of-parsers-in-compiler-design/