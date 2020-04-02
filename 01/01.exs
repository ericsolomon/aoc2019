defmodule D1 do
  def p1 do
    File.read!("input.txt")
    |> String.trim()
    |> String.split("\n")
    |> Enum.reduce(0, fn x, acc -> floor(String.to_integer(x) / 3.0) - 2 + acc end)
  end

  def p2 do
    File.read!("input.txt")
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&String.to_integer(&1))
    |> Enum.reduce(0, fn x, acc -> get_fuel(x) + acc end)
  end

  defp get_fuel(x) when x <= 0, do: 0

  defp get_fuel(x) do
    fuel = floor(x / 3.0) - 2
    if fuel <= 0, do: 0, else: fuel + get_fuel(fuel)
  end
end

IO.puts("Part 1: #{D1.p1()}")
IO.puts("Part 2: #{D1.p2()}")
