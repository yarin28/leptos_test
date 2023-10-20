-- return {
-- 	cron_string = "1/20 * * * * * *",
-- 	seconds_to_pump_water = 5,
-- 	gpio_table = {
-- 		name = "pump1 - water pump",
-- 		gpio_pin = 4,
-- 		gpio_type = "pump",
-- 		active_seconds = 5,
-- 	},
-- }
cron_string = "1/20 * * * * * *"
seconds_to_pump_water = 5
gpio_table = {
	name = "pump1 - water pump",
	gpio_pin = 4,
	gpio_type = "pump",
	active_seconds = 5,
	cron_string = "1/20 * * * * * *",
}
