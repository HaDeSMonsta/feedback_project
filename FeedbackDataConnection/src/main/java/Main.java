import java.io.*;
import java.net.ServerSocket;
import java.net.Socket;
import java.time.LocalDateTime;
import java.time.ZoneOffset;
import java.time.format.DateTimeFormatter;
import java.util.ArrayList;
import java.util.List;

public class Main {

	private static final String FILE_ENDING = ".txt";
	private static final String FILE_NAME = "/feedback/feedback" + FILE_ENDING;
	private static final String PWD = System.getenv("PWD");
	private static final int PORT = 8080;

	public static void main(String[] args) {
		System.out.println("Starting Server");

		try (ServerSocket server = new ServerSocket(PORT)) {

			// noinspection InfiniteLoopStatement
			while(true) {
				Socket sock = server.accept();
				new Thread(() -> authenticate(sock)).start();
			}

		} catch(IOException e) {
			throw new RuntimeException(e);
		}
	}

	private static synchronized void logic(BufferedReader reader) {
		DateTimeFormatter dtf = DateTimeFormatter.ofPattern("yyyy-MM-dd");
		String fileName = FILE_NAME.replace(FILE_ENDING, "-")
				+ dtf.format(LocalDateTime.now(ZoneOffset.UTC))
				+ FILE_ENDING;

		try (BufferedWriter writer = new BufferedWriter(new FileWriter(fileName, true))) {

			writer.write("-".repeat(50));
			writer.newLine();

			dtf = DateTimeFormatter.ofPattern("[yyyy-MM-dd - HH:mm:ss]");
			LocalDateTime now = LocalDateTime.now(ZoneOffset.UTC);
			writer.write(dtf.format(now) + "z");

			writer.newLine();

			List<String> lines = new ArrayList<>();
			String read;
			while((read = reader.readLine()) != null) lines.add(read);

			for(String line : lines) {
				writer.write(line);
				writer.newLine();
			}
			writer.write("-".repeat(50));
			writer.newLine();
			writer.newLine();
			writer.flush();

		} catch(IOException e) {
			throw new RuntimeException(e);
		}
	}

	private static void authenticate(Socket sock) {
		try (BufferedReader reader = new BufferedReader(new InputStreamReader(sock.getInputStream()))) {

			String in = reader.readLine();
			if(in.equals(PWD)) logic(reader);
			else Thread.sleep(10_000);

		} catch(IOException | InterruptedException ignored) {
		}
	}
}