package nexora;

import com.intellij.openapi.fileTypes.LanguageFileType;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import javax.swing.*;

public class NexoraFileType extends LanguageFileType {
    public static final NexoraFileType INSTANCE = new NexoraFileType();

    private NexoraFileType() {
        super(NexoraLanguage.INSTANCE);
    }

    @Override
    @NotNull
    public String getName() {
        return "Nexora";
    }

    @Override
    @NotNull
    public String getDescription() {
        return "Nexora language file";
    }

    @Override
    @NotNull
    public String getDefaultExtension() {
        return "nx";
    }

    @Override
    @Nullable
    public Icon getIcon() {
        return null;
    }
}
