// Ragavan Kalyanaraaman
// Period 5
// Fraction Calculator Project
// This project is to make a calculator that can calculate fraction operations between whole numbers, mixed fractions and improper fractions.
import java.util.*;

// TODO: Description of what this program does goes here.
public class Main {

  // It is best if we have only one console object for input
  public static Scanner console = new Scanner(System.in);

  // This main method will loop through user input and then call the
  // correct method to execute the user's request for help, test, or
  // the mathematical operation on fractions. or, quit.
  // DO NOT CHANGE THIS METHOD!!
  public static void main(String[] args) {
    // initialize to false so that we start our loop
    boolean done = false;

    // When the user types in "quit", we are done.
    while (!done) {
      // prompt the user for input
      String input = getInput();

      // special case the "quit" command
      if (input.equalsIgnoreCase("quit")) {
        done = true;
    //   } else if (
    //     !UnitTestRunner.processCommand(input, Main::processCommand)
    //   ) {
        // We allowed the UnitTestRunner to handle the command first.
        // If the UnitTestRunner didn't handled the command, process normally.
        String result = processCommand(input);

        // print the result of processing the command
        System.out.println(result);
      }
    }

    System.out.println("Goodbye!");
    console.close();
  }

  // Prompt the user with a simple, "Enter: " and get the line of input.
  // Return the full line that the user typed in.
  public static String getInput() {
    // TODO: Implement this method
    System.out.print("Enter: ");
    String input = console.nextLine();
    return input;
  }

  // processCommand will process every user command except for "quit".
  // It will return the String that should be printed to the console.
  // This method won't print anything.
  // DO NOT CHANGE THIS METHOD!!!
  public static String processCommand(String input) {
    if (input.equalsIgnoreCase("help")) {
      return provideHelp();
    }

    // if the command is not "help", it should be an expression.
    // Of course, this is only if the user is being nice.
    return processExpression(input);
  }

  // Lots work for this project is handled in here.
  // Of course, this method will call LOTS of helper methods
  // so that this method can be shorter.
  // This will calculate the expression and RETURN the answer.
  // This will NOT print anything!
  // Input: an expression to be evaluated
  //    Examples:
  //        1/2 + 1/2
  //        2_1/4 - 0_1/8
  //        1_1/8 * 2
  // Return: the fully reduced mathematical result of the expression
  //    Value is returned as a String. Results using above examples:
  //        1
  //        2 1/8
  //        2 1/4
  public static String processExpression(String input) {
      try {
        String[] parts = input.split(" ");
        int[] currentValues = getValues(parts[0]);
        String currentOperator = null;

        for (int i = 1; i < parts.length; i += 2) {
            currentOperator = parts[i];
            int[] nextValues = getValues(parts[i + 1]);

            switch (currentOperator) {
                case "+":
                    currentValues[0] = currentValues[0] * nextValues[1] + nextValues[0] * currentValues[1];
                    currentValues[1] *= nextValues[1];
                    break;
                case "-":
                    currentValues[0] = currentValues[0] * nextValues[1] - nextValues[0] * currentValues[1];
                    currentValues[1] *= nextValues[1];
                    break;
                case "*":
                    currentValues[0] *= nextValues[0];
                    currentValues[1] *= nextValues[1];
                    break;
                case "/":
                    currentValues[0] *= nextValues[1];
                    currentValues[1] *= nextValues[0];
                    break;
                default:
                    return "Invalid Operator";
            }
        }

        int newWhole = currentValues[0] / currentValues[1];
        currentValues[0] %= currentValues[1];
        String resultOfSimp = simpFraction(currentValues[0], currentValues[1], newWhole);

        if (currentValues[0] == 0) {
            return String.valueOf(newWhole);
        } else if (newWhole == 0) {
            return resultOfSimp;
        } else {
            return newWhole + " " + resultOfSimp;
        }
    } catch (ArithmeticException e) {
          return "Undefind - Cannot divide by zero";     
      }
    }

  // Finds the Greatest Common Denominator of the 2 fractions
  public static int findGCD(int a, int b) {
    return b == 0 ? a : findGCD(b, Math.abs(a) % Math.abs(b));
  }
  // Simplifies the end fraction.
  public static String simpFraction(int a, int b, int newWhole) {
    int c = newWhole;
    int greatestCommonDenom = findGCD(a, b);

    if (b < 0) {
      a *= -1;
      b *= -1;
    }
    a = fixSignNumerator(c, a);

    String simpFrac = (a / greatestCommonDenom) + "/" + (b / greatestCommonDenom);
    return simpFrac;
  }
  // If there is a whole number in the result and it is negative, make the numerator positive.
  public static int fixSignNumerator(int c, int a){
    if (c < 0 && a < 0){
      a *= -1;
    }
    return a;
  }
  public static String provideHelp() {
    // TODO: Update this help text!

    String help = "Do you need help? \n";
    help += "Enter an equation buddy, and I can help you! ";

    return help;
  }
  // Returns the numerators and denominator of the improper fractions that have been deduced from the original 2 fractions.
  public static int[] getValues(String num) {
    // Parse the first number
    int whole = 0;
    int numerator = 0;
    int denominator = 1;

    if (num.contains("_")) {
      String[] fracParts = num.split("[_/]");
      whole = Integer.parseInt(fracParts[0]);
      numerator = Integer.parseInt(fracParts[1]);
      denominator = Integer.parseInt(fracParts[2]);
    } else if (num.contains("/")) {
      String[] fracParts = num.split("/");
      numerator = Integer.parseInt(fracParts[0]);
      denominator = Integer.parseInt(fracParts[1]);
    } else {
      whole = Integer.parseInt(num);
    }
    int multiplier = whole < 0 ? -1 : 1;
    numerator = (Math.abs(whole) * denominator + numerator) * multiplier;
    int[] numAndDenom = new int[] { numerator, denominator };
    return numAndDenom;
  }
}

