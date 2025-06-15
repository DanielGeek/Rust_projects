# LLM Ollama examples

## Ollama links

<https://github.com/ollama/ollama>
<https://github.com/ollama/ollama?tab=readme-ov-file>
<https://ollama.com/library/llama3.2>
<https://github.com/ollama/ollama/blob/main/docs/api.md>

curl <http://localhost:11434/api/generate> -d '{
  "model": "llama3.2:1b",
  "prompt": "Why is the sky blue?",
  "stream": false
}'
