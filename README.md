# LlamaBinding

## Start

```bash
mix deps.get && iex -S mix
```

Once rust lib compiled, you can start using llama_binding

## Using llama_binding

Testing with llama3.2:1B model (./llm_models)

```elixir

{:ok, model} = LlamaBinding.load_model("./llm_models/path_to_model.gguf")

```

Once it loaded you can use it:

```elixir
{:ok, session} = LLama.create_session(model)

LLama.set_context(session, "This is the story of a man named Stanley.")

output = LLama.complete(session, 1024)

IO.puts(output)

```

I wanted to get in output Elixir Stream so i can stream directly to liveview.
## 