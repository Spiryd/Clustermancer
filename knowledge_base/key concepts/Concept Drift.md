## What is it?
Concept drift is the unforeseen change in statistical properties of data stream instances over time. 
## Concept Drift Types:
 - [[#Sudden]]
 - [[#Gradual]]
 - [[#Incremental]]
 - [[#Recurring]]
### Sudden
Between two consecutive instances, the change occurs at once, and after this time only instances of the new class are received. An instance that has properties of the previous class never arrives again. A data stream containing sudden concept drift might look like as follows, where different colors indicate different classes.
$S = \{\dots, \textcolor{purple}{x_0}, \textcolor{purple}{x_1}, \textcolor{purple}{x_2}, \textcolor{purple}{x_3}, \textcolor{purple}{x_4}, \textcolor{purple}{x_5}, \textcolor{teal}{x_6}, \textcolor{teal}{x_7}, \textcolor{teal}{x_8}, \textcolor{teal}{x_9}, \textcolor{teal}{x_{10}}, \textcolor{teal}{x_{11}}, \dots\}$
### Gradual
Data instances belonging to the previous class evolves to a new class step by step. After the concept drift is completed, the previous class disappears. The instances that arrive during the concept drift are of transitional forms and they do not have to belong to either of the classes. A data stream containing incremental concept drift might look like as follows, where different colors indicate different classes.
$S = \{\dots, \textcolor{purple}{x_0}, \textcolor{purple}{x_1}, \textcolor{purple}{x_2}, \textcolor{teal}{x_3}, \textcolor{purple}{x_4}, \textcolor{purple}{x_5}, \textcolor{teal}{x_6}, \textcolor{teal}{x_7}, \textcolor{purple}{x_8}, \textcolor{teal}{x_9}, \textcolor{teal}{x_{10}}, \textcolor{teal}{x_{11}}, \dots\}$
### Incremental
Data instances belonging to the previous class evolves to a new class step by step. After the concept drift is completed, the previous class disappears. The instances that arrive during the concept drift are of transitional forms and they do not have to belong to either of the classes. A data stream containing incremental concept drift might look like as follows, where different colors indicate different classes.
 $S = \{\dots, \textcolor{purple}{x_0}, \textcolor{purple}{x_1}, \textcolor{purple}{x_2}, \textcolor{magenta}{x_3}, \textcolor{magenta}{x_4}, \textcolor{violet}{x_5}, \textcolor{violet}{x_6}, \textcolor{cyan}{x_7}, \textcolor{cyan}{x_8}, \textcolor{teal}{x_9}, \textcolor{teal}{x_{10}}, \textcolor{teal}{x_{11}}, \dots\}$
### Recurring
The data instances change between two or more statistical characteristics several times. Neither of the classes disappears permanently but both of them arrive in turns. A data stream containing recurring concept drift might look like as follows, where different colors indicate different classes.
 $S = \{\dots, \textcolor{purple}{x_0}, \textcolor{purple}{x_1}, \textcolor{purple}{x_2}, \textcolor{teal}{x_3}, \textcolor{teal}{x_4}, \textcolor{teal}{x_5}, \textcolor{purple}{x_6}, \textcolor{purple}{x_7}, \textcolor{purple}{x_8}, \textcolor{teal}{x_9}, \textcolor{teal}{x_{10}}, \textcolor{teal}{x_{11}}, \dots\}$