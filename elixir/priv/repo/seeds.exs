alias PoeSystem.Testdata
alias PoeSystem.Items.Item
alias PoeSystem.Repo

for item <- Testdata.items() do
  Item.changeset(%Item{}, item)
  |> Repo.insert_or_update!()
end
