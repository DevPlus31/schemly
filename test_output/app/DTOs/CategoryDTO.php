<?php

namespace App\DTOs;


class CategoryDTO {

    public function __construct
    (
        public int $id,
        public int $userId,
        public string $name,
        public int $user,
        public int $links,
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
                $data['name'],
                $data['user'],
                $data['links'],
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
            'name' => $this->name,
            'user' => $this->user,
            'links' => $this->links,
            'created_at' => $this->created_at,
            'updated_at' => $this->updated_at,
            'deleted_at' => $this->deleted_at
        ];
    }

}