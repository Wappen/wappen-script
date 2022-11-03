#!/usr/bin/env wappen-script

( # loop.ws )


( n = 0 )


( @condition = "
    ( ( $ n ) < 5 )
" )
( @body = "
    ( n = ( ( $ n ) + 1 ) )
" )
( @ while )


( @n = 10 )
( @body = "
    ( n = ( ( $ n ) + 1 ) )
" )
( @ for )


{
    { "We executed a few loops a total of" ( $ n ) "times" }
    { "And our PID is" ( ~ 39 ) }
}
