package nexora;

import com.intellij.lang.Language;

public class NexoraLanguage extends Language {
    public static final NexoraLanguage INSTANCE = new NexoraLanguage();

    private NexoraLanguage() {
        super("Nexora");
    }

    @Override
    public String getFileExtension() {
        return "nx";
    }
}
