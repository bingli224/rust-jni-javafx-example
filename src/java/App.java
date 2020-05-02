
import javafx.scene.layout.BorderPane;
import javafx.scene.control.*;
import javafx.scene.Scene;
import javafx.stage.Stage;
import javafx.application.Application;

public class App extends Application {
	static {
		System.loadLibrary ( "test_javafx" );
	}

	@Override
	public void start ( Stage stage ) {
		stage.setTitle ( "Test JavaFX" );
		BorderPane borderPane = new BorderPane ( );
		borderPane.setCenter (
			new Button ( "Click me" )
		);
		stage.setScene (
			new Scene (
				borderPane,
				480.0,
				320.0
			)
		);
		stage.show ( );
	}

	public static void main ( String[] argv ) {
		System.out.println ( "Launching.." );
		Application.launch ( App.class );
		System.out.println ( "End" );
	}
}
