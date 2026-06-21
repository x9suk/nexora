package nexora;

import com.intellij.lexer.Lexer;
import com.intellij.openapi.editor.colors.TextAttributesKey;
import com.intellij.openapi.fileTypes.SyntaxHighlighterBase;
import com.intellij.psi.TokenType;
import com.intellij.psi.tree.IElementType;
import org.jetbrains.annotations.NotNull;

import static com.intellij.editor.colors.TextAttributesKey.createTextAttributesKey;

public class NexoraSyntaxHighlighter extends SyntaxHighlighterBase {
    public static final TextAttributesKey KEYWORD =
            createTextAttributesKey("NEXORA_KEYWORD");
    public static final TextAttributesKey FUNCTION =
            createTextAttributesKey("NEXORA_FUNCTION");
    public static final TextAttributesKey STRING =
            createTextAttributesKey("NEXORA_STRING");
    public static final TextAttributesKey NUMBER =
            createTextAttributesKey("NEXORA_NUMBER");
    public static final TextAttributesKey COMMENT =
            createTextAttributesKey("NEXORA_COMMENT");
    public static final TextAttributesKey OPERATOR =
            createTextAttributesKey("NEXORA_OPERATOR");
    public static final TextAttributesKey BUILTIN =
            createTextAttributesKey("NEXORA_BUILTIN");
    public static final TextAttributesKey HTML_HELPER =
            createTextAttributesKey("NEXORA_HTML_HELPER");
    public static final TextAttributesKey INVALID =
            createTextAttributesKey("NEXORA_INVALID", TokenType.BAD_CHARACTER);

    private static final TextAttributesKey[] KEYWORD_KEYS = new TextAttributesKey[]{KEYWORD};
    private static final TextAttributesKey[] FUNCTION_KEYS = new TextAttributesKey[]{FUNCTION};
    private static final TextAttributesKey[] STRING_KEYS = new TextAttributesKey[]{STRING};
    private static final TextAttributesKey[] NUMBER_KEYS = new TextAttributesKey[]{NUMBER};
    private static final TextAttributesKey[] COMMENT_KEYS = new TextAttributesKey[]{COMMENT};
    private static final TextAttributesKey[] OPERATOR_KEYS = new TextAttributesKey[]{OPERATOR};
    private static final TextAttributesKey[] BUILTIN_KEYS = new TextAttributesKey[]{BUILTIN};
    private static final TextAttributesKey[] HTML_HELPER_KEYS = new TextAttributesKey[]{HTML_HELPER};
    private static final TextAttributesKey[] INVALID_KEYS = new TextAttributesKey[]{INVALID};
    private static final TextAttributesKey[] EMPTY_KEYS = new TextAttributesKey[0];

    @NotNull
    @Override
    public Lexer getHighlightingLexer() {
        return new NexoraLexerAdapter();
    }

    @NotNull
    @Override
    public TextAttributesKey[] getTokenHighlights(@NotNull IElementType tokenType) {
        if (tokenType instanceof NexoraTokenTypes) {
            NexoraTokenTypes type = (NexoraTokenTypes) tokenType;
            switch (type) {
                case KEYWORD:
                    return KEYWORD_KEYS;
                case FUNCTION:
                    return FUNCTION_KEYS;
                case STRING:
                    return STRING_KEYS;
                case NUMBER:
                    return NUMBER_KEYS;
                case COMMENT:
                    return COMMENT_KEYS;
                case OPERATOR:
                    return OPERATOR_KEYS;
                case BUILTIN:
                    return BUILTIN_KEYS;
                case HTML_HELPER:
                    return HTML_HELPER_KEYS;
                default:
                    return EMPTY_KEYS;
            }
        }
        if (tokenType.equals(TokenType.BAD_CHARACTER)) {
            return INVALID_KEYS;
        }
        return EMPTY_KEYS;
    }
}
