.global __morestack
__morestack:
    ret

# This is just a placeholder to make rustc happy.
# Some commit around the time several hours before GMT+08 Jan. 18 15:00
# seems to cause a regression that the eh_personality language item
# is always needed regardless the '-Z no-landing-pads' flag.
.global __gcc_personality_v0
__gcc_personality_v0:
    ret
