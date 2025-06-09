import clsx from 'clsx';

interface Tab {
  id: string;
  label: string;
  icon?: string;
}

interface TabsProps {
  tabs: Tab[];
  activeTab: string;
  onTabChange: (tabId: string) => void;
  className?: string;
}

export function Tabs({ tabs, activeTab, onTabChange, className }: TabsProps) {
  return (
    <div className={clsx('flex border-b border-amber-600/20', className)}>
      {tabs.map((tab) => (
        <button
          key={tab.id}
          onClick={() => onTabChange(tab.id)}
          className={clsx(
            'px-4 py-2 font-medium text-sm transition-colors relative flex items-center gap-2',
            activeTab === tab.id
              ? 'text-amber-800 after:absolute after:bottom-[-1px] after:left-0 after:right-0 after:h-[2px] after:bg-amber-600'
              : 'text-gray-600 hover:text-amber-700'
          )}
        >
          {tab.icon && <span className="text-lg">{tab.icon}</span>}
          {tab.label}
        </button>
      ))}
    </div>
  );
} 