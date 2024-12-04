defmodule LlamaBinding do
  use Rustler, otp_app: :llama_binding, crate: :llm_cpp

  defmodule Model do
    defstruct [:resource]
  end

  defmodule Session do
    defstruct [:resource]
  end

  @doc """
  Load a model from the specified path.
  """
  def load_model(path), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Create a session using a loaded model.
  """
  def create_session(model), do: :erlang.nif_error(:nif_not_loaded)

  @spec set_context(any(), any()) :: any()
  @doc """
  Set the context for a session.
  """
  def set_context(session, prompt), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Start a completion and return a stream of tokens.
  """
  def complete(session, max_tokens \\ 1024) do
    Stream.resource(
      fn -> start_stream(session, max_tokens) end,
      fn
        {:ok, receiver} ->
          case receive_token(receiver) do
            {:ok, token} -> {[token], {:ok, receiver}}
            :done -> {:halt, receiver}
          end

        _ -> {:halt, nil}
      end,
      fn _ -> :ok end
    )
  end

  defp start_stream(session, max_tokens) do
    case :llama.complete(session, max_tokens) do
      {:ok, receiver} -> {:ok, receiver}
      error -> error
    end
  end

  defp receive_token(receiver) do
    receive do
      token -> {:ok, token}
    after
      100 -> :done
    end
  end
end
