from Tkinter import Tk
import re

r = Tk()
r.withdraw()
result = r.selection_get(selection = "CLIPBOARD")

if re.match("Rarity:", result):
  print result
else:
  print "None"
r.destroy()

