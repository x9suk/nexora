package nexora;

import com.intellij.psi.tree.IElementType;
import org.jetbrains.annotations.NonNls;
import org.jetbrains.annotations.NotNull;

public class NexoraTokenTypes extends IElementType {
    public static final NexoraTokenTypes KEYWORD = new NexoraTokenTypes("KEYWORD");
    public static final NexoraTokenTypes FUNCTION = new NexoraTokenTypes("FUNCTION");
    public static final NexoraTokenTypes STRING = new NexoraTokenTypes("STRING");
    public static final NexoraTokenTypes NUMBER = new NexoraTokenTypes("NUMBER");
    public static final NexoraTokenTypes COMMENT = new NexoraTokenTypes("COMMENT");
    public static final NexoraTokenTypes OPERATOR = new NexoraTokenTypes("OPERATOR");
    public static final NexoraTokenTypes BUILTIN = new NexoraTokenTypes("BUILTIN");
    public static final NexoraTokenTypes HTML_HELPER = new NexoraTokenTypes("HTML_HELPER");
    public static final NexoraTokenTypes IDENTIFIER = new NexoraTokenTypes("IDENTIFIER");
    public static final NexoraTokenTypes WHITE_SPACE = new NexoraTokenTypes("WHITE_SPACE");

    public NexoraTokenTypes(@NotNull @NonNls String debugName) {
        super(debugName, NexoraLanguage.INSTANCE);
    }
}
