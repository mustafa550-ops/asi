import { type InputHTMLAttributes } from "react";

interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  label?: string;
}

export function Input({ label, className = "", id, ...rest }: InputProps) {
  return (
    <div className="ui-input-group">
      {label && (
        <label className="ui-input-label" htmlFor={id}>
          {label}
        </label>
      )}
      <input id={id} className={`ui-input ${className}`} {...rest} />
    </div>
  );
}
