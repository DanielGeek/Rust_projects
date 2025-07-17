---
theme: apple-basic
title: Building AI Agents
transition: fade
# layout: center
---

## LLM based Agents

## By making things that have already been solved

---

## First a question

How many of you code?

- doesn't matter what language
- doesn't matter what framework

---

## Objectives

- To inspire you to create your own agents
  - without Langchain

---

## AI (LLM) Agents

## What do we mean by AI Agents anyways?

There isn't really a good definition. I've multiple descriptions of agents, from

- **thin wrapper around LLMS**

to

- **app that has LLMs call tools**

---

layout: image
image: ./slide_images/demo_screenshot.png
backgroundSize: contain

---

## demo - A todo app

---

layout: image-right
image: ./slide_images/high_level_overview.png
backgroundSize: contain

---

## How it works from a high level

## Tech Stack

- Ollama
- Rust
- Docker (Postgres Database)

---

## The Key, tool calling

When using the API, we can tell the LLM that a function exists, what it takes in, and what it will return.

- the LLM and the actual function are not intertwined, the LLM **cannot** interface with the code directly
- The LLM can choose to call a function or not
- Ollama based LLMs almost always call a function when given the chance to...even if it doesn't make sense

---

## Tool Calling Example

### requesting a tool call from ollama

```json
{
 "model": "qwen2:7b-instruct-fp16",
 "messages": [
  {
   "role": "user",
   "content": "what is the weather like in Denver, Colorado?"
  }
 ],
 "stream": false,
 "tools": [
  {
   "type": "function",
   "function": {
    "name": "get_weather",
    "description": "queries an api that will return the current temperature in Farenheight",
    "parameters": {
     "type": "object",
     "properties": {
      "location": {
       "type": "string",
       "description": "The location that you want to check the weather of. The format should be 'City, State'. for example 'Berkely, CA'"
      }
     }
    }
   }
  }
 ]
}
```

### response from ollama

```json
{
 "model": "qwen2:7b-instruct-fp16",
 "created_at": "2024-12-04T22:33:07.605992Z",
 "message": {
  "role": "assistant",
  "content": "",
  "tool_calls": [
   {
    "function": {
     "name": "get_weather",
     "arguments": {
      "location": "Denver, CO"
     }
    }
   }
  ]
 },
 "done_reason": "stop",
 "done": true,
 "total_duration": 2638946583,
 "load_duration": 563932916,
 "prompt_eval_count": 184,
 "prompt_eval_duration": 648000000,
 "eval_count": 27,
 "eval_duration": 1177000000
}
```

---

## Challenge

Try creating your own agent without Langchain

- start with using a tool like postman to learn tool calling
- identify a simple problem that is already solved
- create a very simple version of that solution but using LLMs as the core
- don't worry about having the LLM do everything

---
