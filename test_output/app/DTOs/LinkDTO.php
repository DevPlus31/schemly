<?php

namespace App\DTOs;


class LinkDTO {

    public function __construct
    (
        public int $id,
        public int $userId,
        public ?int $categoryId,
        public string $title,
        public string $url,
        public bool $isActive,
        public ?int $order,
        public int $user,
        public ?int $category,
        public ?string $created_at,
        public ?string $updated_at,
        public ?string $deleted_at
    )
    {
    }


     public static function fromArray(array $data): self
     {
            return new self(
                $data['id'],
                $data['userId'],
                $data['categoryId'],
                $data['title'],
                $data['url'],
                $data['isActive'],
                $data['order'],
                $data['user'],
                $data['category'],
                $data['created_at'],
                $data['updated_at'],
                $data['deleted_at']
            );
     }


    public function toArray(): array
    {
        return [
            'id' => $this->id,
            'userId' => $this->userId,
            'categoryId' => $this->categoryId,
            'title' => $this->title,
            'url' => $this->url,
            'isActive' => $this->isActive,
            'order' => $this->order,
            'user' => $this->user,
            'category' => $this->category,
            'created_at' => $this->created_at,
            'updated_at' => $this->updated_at,
            'deleted_at' => $this->deleted_at
        ];
    }

}