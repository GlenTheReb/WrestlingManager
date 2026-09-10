ALTER TABLE workers ADD COLUMN wrestling_style TEXT
    CHECK(wrestling_style IS NULL OR json_valid(wrestling_style));
