

def counter_flag(x: int, y: int, location_points: list) -> int:
    """Счётчик подобранных флагов игроком"""
    global counter
    for i in range(0, 2):
        if x == location_points[i][0] and y == location_points[i][1]:
            counter += 1
            return counter

counter = 0
location_points = [[100, 200], [123, 876], [5674, 342]]
print(counter_flag(100, 200, location_points))
print(counter_flag(100, 200, location_points))
print(counter_flag(100, 200, location_points))
