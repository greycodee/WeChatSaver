import React from 'react';
import Image from 'next/image';

export default function AboutPage() {
    return (
        <div>
            <Image
                className="dark:invert"
                src="/next.svg"
                alt="Next.js logo"
                width={180}
                height={38}
                priority
            />
            <ol className="list-inside list-decimal text-sm text-center sm:text-left font-[family-name:var(--font-geist-mono)]">
                <li className="mb-2">
                    wechat backup project.
                </li>
                <li>About</li>
            </ol>
        </div>
    );
}