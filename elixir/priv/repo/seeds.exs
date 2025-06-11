alias Ecto.UUID
alias PoeSystem.Testdata
alias PoeSystem.Items.Item
alias PoeSystem.Build
alias PoeSystem.Repo

if Mix.env() == :test do
  for item <- Testdata.items() do
    Item.changeset(%Item{}, item)
    |> Repo.insert_or_update!()
  end

  {itemset, skillset} = Testdata.config_info()

  Build.changeset(%Build{}, %{
    id: UUID.bingenerate(),
    itemset: itemset,
    skillset: skillset,
    pob: Testdata.pobdata_file(),
    data: Testdata.extract_config()
  })
  |> Repo.insert!()
end
