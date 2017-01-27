class Clock(object):
    def __init__(self, hours, minutes):
        self.hours = hours
        self.minutes = minutes
        self.__normalize()

    def add(self, minutes):
        self.minutes += minutes
        self.__normalize()
        return self

    def __normalize(self):
        self.hours += self.minutes // 60
        self.minutes %= 60
        self.hours %= 24
    
    def __str__(self):
        return "%02d:%02d" % (self.hours, self.minutes)

    def __eq__(self, other):
        return isinstance(other, self.__class__) and \
            self.hours == other.hours and \
            self.minutes == other.minutes
