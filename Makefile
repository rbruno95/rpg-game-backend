run-migrations:
	diesel database reset

seed-db:
	psql rpg_game < seed.sql

reset-db: run-migrations seed-db
