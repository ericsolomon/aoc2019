defmodule Part1 do
  def run() do
    File.read!("input.txt")
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&String.to_integer(&1))
    |> Enum.map(&div(&1, 3))
    |> Enum.map(&(&1 - 2))
    |> Enum.reduce(0, fn x, acc -> acc + x end)
  end
end

defmodule Part2 do
  def run() do
    File.read!("input.txt")
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&String.to_integer(&1))
    |> calc_fuel()
  end

  defp calc_fuel(list, acc \\ 0)
  defp calc_fuel([], acc), do: acc

  defp calc_fuel([mass | rem_masses], acc) do
    calc_fuel(rem_masses, acc + fuel_sum(div(mass, 3) - 2, 0))
  end

  defp fuel_sum(req_fuel, acc) when req_fuel <= 0, do: acc

  defp fuel_sum(req_fuel, acc) do
    fuel_sum(div(req_fuel, 3) - 2, req_fuel + acc)
  end
end

IO.inspect(Part1.run(), label: "P1 sum of fuel requirements")
IO.inspect(Part2.run(), label: "P2 sum of fuel requirements")
